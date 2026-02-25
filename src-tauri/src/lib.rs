use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use sysinfo::{Pid, System};
use tauri::{Emitter, State, Window};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ServerConfig {
    pub name: Option<String>,
    pub path: String,
    pub jar_name: String,
    pub min_ram: String,
    pub max_ram: String,
}

#[derive(Serialize, Clone, Debug)]
pub enum ServerStatus {
    Offline,
    Starting,
    Running,
    Stopping,
}

pub struct AppState {
    pub config: Mutex<Option<ServerConfig>>,
    pub child_process: Mutex<Option<Child>>,
    pub player_count: Arc<Mutex<i32>>,
    pub status: Arc<Mutex<ServerStatus>>,
}

#[tauri::command]
async fn select_jar_file(app: tauri::AppHandle) -> Result<Option<ServerConfig>, String> {
    use tauri_plugin_dialog::DialogExt;
    let (tx, rx) = std::sync::mpsc::channel();
    app.dialog().file().add_filter("Minecraft Server", &["jar"]).pick_file(move |f| { tx.send(f).unwrap(); });
    let selected = rx.recv().unwrap();
    if let Some(path_buf) = selected {
        let path_str = path_buf.to_string().replace("\\", "/");
        let path = Path::new(&path_str);
        let parent = path.parent().ok_or("Invalid path")?.to_string_lossy().to_string();
        let file_name = path.file_name().ok_or("Invalid filename")?.to_string_lossy().to_string();
        Ok(Some(ServerConfig { name: None, path: parent, jar_name: file_name, min_ram: "1G".into(), max_ram: "2G".into() }))
    } else {
        Ok(None)
    }
}

#[tauri::command]
async fn set_server_config(config: ServerConfig, state: State<'_, AppState>) -> Result<(), String> {
    let mut state_config = state.config.lock().unwrap();
    *state_config = Some(config);
    Ok(())
}

#[tauri::command]
async fn start_server(app: tauri::AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    let config = {
        let conf = state.config.lock().unwrap();
        conf.clone().ok_or("Server not configured")?
    };

    let mut child_process = state.child_process.lock().unwrap();
    if child_process.is_some() { return Err("Server already running".into()); }

    {
        let mut status = state.status.lock().unwrap();
        *status = ServerStatus::Starting;
        app.emit("status-update", ServerStatus::Starting).unwrap();
    }

    let eula_path = Path::new(&config.path).join("eula.txt");
    fs::write(eula_path, "eula=true").map_err(|e| e.to_string())?;

    let mut child = Command::new("java")
        .arg(format!("-Xms{}", config.min_ram))
        .arg(format!("-Xmx{}", config.max_ram))
        .arg("-jar").arg(&config.jar_name).arg("nogui")
        .current_dir(&config.path)
        .stdout(Stdio::piped()).stderr(Stdio::piped())
        .spawn().map_err(|e| {
            let mut status = state.status.lock().unwrap();
            *status = ServerStatus::Offline;
            app.emit("status-update", ServerStatus::Offline).unwrap();
            format!("Failed to start: {}", e)
        })?;

    let stdout = child.stdout.take().unwrap();
    let player_count = Arc::clone(&state.player_count);
    let status_clone = Arc::clone(&state.status);
    let app_clone = app.clone();

    std::thread::spawn(move || {
        let reader = BufReader::new(stdout);
        let mut started = false;
        for line in reader.lines() {
            if let Ok(l) = line {
                app_clone.emit("server-log", &l).unwrap();
                if !started && (l.contains("Done") || l.contains("For help, type \"help\"")) {
                    started = true;
                    let mut status = status_clone.lock().unwrap();
                    *status = ServerStatus::Running;
                    app_clone.emit("status-update", ServerStatus::Running).unwrap();
                }
                let mut pc = player_count.lock().unwrap();
                if l.contains("joined the game") {
                    *pc += 1; app_clone.emit("player-update", *pc).unwrap();
                } else if l.contains("left the game") {
                    if *pc > 0 { *pc -= 1; } app_clone.emit("player-update", *pc).unwrap();
                }
            }
        }
        let mut status = status_clone.lock().unwrap();
        *status = ServerStatus::Offline;
        app_clone.emit("status-update", ServerStatus::Offline).unwrap();
    });

    *child_process = Some(child);
    Ok(())
}

#[tauri::command]
async fn stop_server(app: tauri::AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    let mut child_process = state.child_process.lock().unwrap();
    if let Some(mut child) = child_process.take() {
        let mut status = state.status.lock().unwrap();
        *status = ServerStatus::Stopping;
        app.emit("status-update", ServerStatus::Stopping).unwrap();
        child.kill().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[derive(Serialize)]
pub struct ServerStats {
    pub cpu: f32,
    pub memory: u64,
    pub status: ServerStatus,
    pub player_count: i32,
}

#[tauri::command]
async fn get_server_stats(state: State<'_, AppState>) -> Result<ServerStats, String> {
    let mut child_lock = state.child_process.lock().unwrap();
    let pc = *state.player_count.lock().unwrap();
    let status = state.status.lock().unwrap().clone();

    if let Some(child) = child_lock.as_mut() {
        match child.try_wait() {
            Ok(Some(_)) => *child_lock = None,
            Ok(None) => {
                let pid = Pid::from(child.id() as usize);
                let mut sys = System::new_all();
                sys.refresh_all();
                if let Some(process) = sys.process(pid) {
                    return Ok(ServerStats { cpu: process.cpu_usage(), memory: process.memory(), status, player_count: pc });
                }
            }
            Err(_) => *child_lock = None,
        }
    }
    Ok(ServerStats { cpu: 0.0, memory: 0, status, player_count: pc })
}

#[tauri::command]
async fn read_properties(path: String) -> Result<HashMap<String, String>, String> {
    let prop_path = Path::new(&path).join("server.properties");
    if !prop_path.exists() { return Ok(HashMap::new()); }
    let content = fs::read_to_string(prop_path).map_err(|e| e.to_string())?;
    let mut props = HashMap::new();
    for line in content.lines() {
        if line.starts_with('#') || !line.contains('=') { continue; }
        let parts: Vec<&str> = line.splitn(2, '=').collect();
        props.insert(parts[0].trim().to_string(), parts[1].trim().to_string());
    }
    Ok(props)
}

#[tauri::command]
async fn write_properties(path: String, props: HashMap<String, String>) -> Result<(), String> {
    let prop_path = Path::new(&path).join("server.properties");
    let mut content = String::from("# Updated by Roam MC Manager\n");
    for (key, value) in props { content.push_str(&format!("{}={}\n", key, value)); }
    fs::write(prop_path, content).map_err(|e| e.to_string())
}

#[tauri::command]
fn close_window(window: Window) { window.close().unwrap(); }
#[tauri::command]
fn minimize_window(window: Window) { window.minimize().unwrap(); }
#[tauri::command]
fn maximize_window(window: Window) {
    if window.is_maximized().unwrap() { window.unmaximize().unwrap(); }
    else { window.maximize().unwrap(); }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            config: Mutex::new(None),
            child_process: Mutex::new(None),
            player_count: Arc::new(Mutex::new(0)),
            status: Arc::new(Mutex::new(ServerStatus::Offline)),
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            set_server_config, start_server, stop_server, get_server_stats,
            read_properties, write_properties, select_jar_file,
            close_window, minimize_window, maximize_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
