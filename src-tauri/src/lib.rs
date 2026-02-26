use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader, Write, Read};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use sysinfo::{Pid, System};
use tauri::{Emitter, Manager, State, Window, WindowEvent};
use walkdir::WalkDir;
use zip::write::SimpleFileOptions;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ServerConfig {
    pub name: Option<String>,
    pub path: String,
    pub jar_name: String,
    pub min_ram: String,
    pub max_ram: String,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
pub enum ServerStatus {
    Offline,
    Starting,
    Running,
    Stopping,
}

#[derive(Serialize)]
pub struct ServerStats {
    pub cpu: f32,
    pub memory: u64,
    pub status: ServerStatus,
    pub player_count: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerInfo {
    pub uuid: String,
    pub name: String,
    pub time_played: f64,
    pub steps: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorldInfo {
    pub name: String,
    pub size_mb: f64,
    pub last_modified: String,
}

pub struct AppState {
    pub config: Mutex<Option<ServerConfig>>,
    pub child_process: Mutex<Option<Child>>,
    pub player_count: Arc<Mutex<i32>>,
    pub status: Arc<Mutex<ServerStatus>>,
}

fn find_orphaned_java_process(server_path: &str) -> Option<Pid> {
    let mut sys = System::new_all();
    sys.refresh_all();
    for (pid, process) in sys.processes() {
        let name = process.name().to_string_lossy().to_lowercase();
        if name.contains("java") {
            if let Some(cwd) = process.cwd() {
                if cwd.to_string_lossy() == server_path { return Some(*pid); }
            }
            for arg in process.cmd() {
                if arg.to_string_lossy().contains(server_path) { return Some(*pid); }
            }
        }
    }
    None
}

#[tauri::command]
async fn get_worlds(path: String) -> Result<Vec<WorldInfo>, String> {
    let mut worlds = Vec::new();
    let entries = fs::read_dir(&path).map_err(|e| e.to_string())?;

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_dir() && path.join("level.dat").exists() {
            let name = path.file_name().unwrap().to_string_lossy().to_string();
            let metadata = fs::metadata(&path).map_err(|e| e.to_string())?;
            
            // Format time using chrono
            let last_modified = if let Ok(time) = metadata.modified() {
                let dt: chrono::DateTime<chrono::Local> = time.into();
                dt.format("%d %b %Y, %H:%M").to_string()
            } else {
                "Unknown".to_string()
            };
            
            // Calculate total size of directory
            let total_size: u64 = WalkDir::new(&path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter_map(|e| e.metadata().ok())
                .filter(|m| m.is_file())
                .map(|m| m.len())
                .sum();

            worlds.push(WorldInfo {
                name,
                size_mb: (total_size as f64) / 1024.0 / 1024.0,
                last_modified,
            });
        }
    }
    Ok(worlds)
}

#[tauri::command]
async fn backup_world(server_path: String, world_name: String) -> Result<String, String> {
    let world_dir = Path::new(&server_path).join(&world_name);
    let backup_dir = Path::new(&server_path).join("roam_backups");
    
    if !backup_dir.exists() {
        fs::create_dir_all(&backup_dir).map_err(|e| e.to_string())?;
    }

    let timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let backup_filename = format!("{}_{}.zip", world_name, timestamp);
    let backup_path = backup_dir.join(&backup_filename);

    let file = fs::File::create(&backup_path).map_err(|e| e.to_string())?;
    let mut zip = zip::ZipWriter::new(file);
    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in WalkDir::new(&world_dir) {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        let name = path.strip_prefix(&world_dir).map_err(|e| e.to_string())?;

        if path.is_file() {
            zip.start_file(name.to_string_lossy().to_string(), options).map_err(|e| e.to_string())?;
            let mut f = fs::File::open(path).map_err(|e| e.to_string())?;
            f.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
            zip.write_all(&buffer).map_err(|e| e.to_string())?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            zip.add_directory(name.to_string_lossy().to_string(), options).map_err(|e| e.to_string())?;
        }
    }

    zip.finish().map_err(|e| e.to_string())?;
    Ok(backup_filename)
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
    } else { Ok(None) }
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
    if let Some(pid) = find_orphaned_java_process(&config.path) {
        return Err(format!("Existing process found (PID {}). Please stop it.", pid));
    }
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
        .stdout(Stdio::piped()).stderr(Stdio::piped()).stdin(Stdio::piped())
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
async fn stop_server(app: tauri::AppHandle, state: State<'_, AppState>) -> Result<bool, String> {
    {
        let mut status = state.status.lock().unwrap();
        *status = ServerStatus::Stopping;
        app.emit("status-update", ServerStatus::Stopping).unwrap();
    }
    let mut child_process = state.child_process.lock().unwrap();
    let mut stopped = false;
    if let Some(mut child) = child_process.take() {
        child.kill().map_err(|e| e.to_string())?;
        let _ = child.wait();
        stopped = true;
    } else {
        let config = {
            let conf = state.config.lock().unwrap();
            conf.clone().ok_or("Server not configured")?
        };
        if let Some(pid) = find_orphaned_java_process(&config.path) {
            let mut sys = System::new_all();
            sys.refresh_all();
            if let Some(process) = sys.process(pid) {
                process.kill();
                let mut attempts = 0;
                while attempts < 10 {
                    std::thread::sleep(std::time::Duration::from_millis(500));
                    sys.refresh_all();
                    if sys.process(pid).is_none() { break; }
                    attempts += 1;
                }
                stopped = true;
            }
        }
    }
    {
        let mut status = state.status.lock().unwrap();
        *status = ServerStatus::Offline;
        app.emit("status-update", ServerStatus::Offline).unwrap();
    }
    Ok(stopped)
}

#[tauri::command]
async fn get_server_stats(state: State<'_, AppState>) -> Result<ServerStats, String> {
    let mut child_lock = state.child_process.lock().unwrap();
    let pc = *state.player_count.lock().unwrap();
    let mut status = state.status.lock().unwrap().clone();
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
    let config_lock = state.config.lock().unwrap();
    if let Some(config) = config_lock.as_ref() {
        if let Some(pid) = find_orphaned_java_process(&config.path) {
            let mut sys = System::new_all();
            sys.refresh_all();
            if let Some(process) = sys.process(pid) {
                if status == ServerStatus::Offline { status = ServerStatus::Running; }
                return Ok(ServerStats { cpu: process.cpu_usage(), memory: process.memory(), status, player_count: pc });
            }
        }
    }
    Ok(ServerStats { cpu: 0.0, memory: 0, status: ServerStatus::Offline, player_count: pc })
}

#[tauri::command]
async fn read_properties(path: String) -> Result<HashMap<String, String>, String> {
    let prop_path = Path::new(&path).join("server.properties");
    if !prop_path.exists() { return Ok(HashMap::new()); }
    let content = fs::read_to_string(prop_path).map_err(|e| e.to_string())?;
    let mut props = HashMap::new();
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with('#') || !line.contains('=') { continue; }
        if let Some((key, value)) = line.split_once('=') {
            props.insert(key.trim().to_string(), value.trim().to_string());
        }
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

#[tauri::command]
async fn get_players_data(path: String) -> Result<Vec<PlayerInfo>, String> {
    let prop_path = Path::new(&path).join("server.properties");
    let mut world_name = String::from("world");
    if prop_path.exists() {
        if let Ok(content) = fs::read_to_string(prop_path) {
            for line in content.lines() {
                if line.starts_with("level-name=") {
                    world_name = line.split_once('=').unwrap().1.trim().to_string();
                    break;
                }
            }
        }
    }
    let stats_path = Path::new(&path).join(&world_name).join("stats");
    let cache_path = Path::new(&path).join("usercache.json");
    if !stats_path.exists() { return Ok(Vec::new()); }
    let mut uuid_to_name = HashMap::new();
    if cache_path.exists() {
        if let Ok(cache_content) = fs::read_to_string(cache_path) {
            if let Ok(cache) = serde_json::from_str::<serde_json::Value>(&cache_content) {
                if let Some(arr) = cache.as_array() {
                    for entry in arr {
                        if let (Some(u), Some(n)) = (entry["uuid"].as_str(), entry["name"].as_str()) {
                            uuid_to_name.insert(u.to_string(), n.to_string());
                        }
                    }
                }
            }
        }
    }
    let mut players = Vec::new();
    for entry in fs::read_dir(stats_path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let file_path = entry.path();
        if file_path.extension().map_or(false, |ext| ext == "json") {
            let uuid = file_path.file_stem().unwrap().to_string_lossy().to_string();
            let name = uuid_to_name.get(&uuid).cloned().unwrap_or_else(|| uuid.clone());
            if let Ok(content) = fs::read_to_string(&file_path) {
                if let Ok(stats) = serde_json::from_str::<serde_json::Value>(&content) {
                    let custom = &stats["stats"]["minecraft:custom"];
                    let ticks = custom["minecraft:play_time"].as_u64()
                        .or(custom["minecraft:play_one_minute"].as_u64())
                        .unwrap_or(0);
                    let hours = (ticks as f64) / 20.0 / 3600.0;
                    let cm_walked = custom["minecraft:walk_one_cm"].as_u64().unwrap_or(0);
                    let steps = cm_walked / 75;
                    players.push(PlayerInfo { uuid, name, time_played: hours, steps });
                }
            }
        }
    }
    Ok(players)
}

#[tauri::command]
async fn send_server_command(command: String, state: State<'_, AppState>) -> Result<(), String> {
    let mut child_process = state.child_process.lock().unwrap();
    if let Some(child) = child_process.as_mut() {
        let stdin = child.stdin.as_mut().ok_or("Failed to open stdin")?;
        let cmd_with_newline = format!("{}\n", command.trim());
        stdin.write_all(cmd_with_newline.as_bytes()).map_err(|e: std::io::Error| e.to_string())?;
        stdin.flush().map_err(|e: std::io::Error| e.to_string())?;
        Ok(())
    } else {
        Err("Cannot send commands to an orphaned process. Only servers started via this manager support direct commands.".into())
    }
}

#[tauri::command]
fn open_folder(app: tauri::AppHandle, path: String) -> Result<(), String> {
    use tauri_plugin_opener::OpenerExt;
    app.opener().open_path(path, None::<&str>).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_directory(path: String) -> Result<(), String> {
    fs::remove_dir_all(path).map_err(|e| e.to_string())
}

#[tauri::command]
fn is_server_initialized(path: String) -> bool {
    let prop_path = Path::new(&path).join("server.properties");
    prop_path.exists()
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
        .on_window_event(|window, event| {
            if let WindowEvent::Destroyed = event {
                let state: State<AppState> = window.state();
                let mut child_process = state.child_process.lock().unwrap();
                if let Some(mut child) = child_process.take() {
                    let _ = child.kill().map_err(|e: std::io::Error| e.to_string());
                }
            }
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            set_server_config, start_server, stop_server, get_server_stats,
            read_properties, write_properties, select_jar_file,
            close_window, minimize_window, maximize_window,
            is_server_initialized, delete_directory, get_players_data,
            send_server_command, open_folder, get_worlds, backup_world
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
