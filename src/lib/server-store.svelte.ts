import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { ServerConfig, ServerStats, ServerProperties, ServerStatus } from "./types";

class ServerStore {
  servers = $state<ServerConfig[]>([]);
  config = $state<ServerConfig | null>(null);
  stats = $state<ServerStats>({ cpu: 0, core_count: 1, memory: 0, status: "Offline", player_count: 0 });
  players = $state<PlayerInfo[]>([]);
  worlds = $state<WorldInfo[]>([]);
  properties = $state<ServerProperties>({});
  logs = $state<string[]>([]);
  isDownloading = $state(false);
  
  constructor() {
    this.setupListeners();
    this.loadServers();
  }

  async refreshWorlds() {
    if (this.config) {
      try {
        const w = await invoke("get_worlds", { path: this.config.path });
        this.worlds = w as WorldInfo[];
      } catch (e) {
        console.error("Failed to fetch world data", e);
      }
    }
  }

  async backupWorld(worldName: string) {
    if (this.config) {
      this.logs = [...this.logs.slice(-500), `[System] Starting backup for: ${worldName}...`];
      try {
        const filename = await invoke("backup_world", { 
          serverPath: this.config.path, 
          worldName 
        });
        this.logs = [...this.logs.slice(-500), `[System] Backup successful: ${filename}`];
        return filename;
      } catch (e) {
        console.error("Backup failed", e);
        this.logs = [...this.logs.slice(-500), `[System] Backup failed: ${e}`];
        throw e;
      }
    }
  }

  async refreshPlayers() {
    if (this.config) {
      try {
        const p = await invoke("get_players_data", { path: this.config.path });
        this.players = p as PlayerInfo[];
      } catch (e) {
        console.error("Failed to fetch player data", e);
      }
    }
  }

  async setupListeners() {
    await listen<string>("server-log", (event) => {
      this.logs = [...this.logs.slice(-500), event.payload];
    });

    await listen<number>("player-update", (event) => {
      this.stats.player_count = event.payload;
    });

    await listen<ServerStatus>("status-update", (event) => {
      this.stats.status = event.payload;
    });
  }

  async loadServers() {
    try {
      // We will implement persistent storage using tauri-plugin-fs or similar later
      // For now, let's keep it in memory or simple local storage
      const saved = localStorage.getItem("mc_servers");
      if (saved) this.servers = JSON.parse(saved);
    } catch (e) {
      console.error(e);
    }
  }

  saveServers() {
    localStorage.setItem("mc_servers", JSON.stringify(this.servers));
  }

  async addServer(name: string, path: string, jar: string, ram: number) {
    const newServer: ServerConfig = {
      name,
      path,
      jar_name: jar,
      min_ram: "1G",
      max_ram: `${ram}G`
    };
    this.servers.push(newServer);
    this.saveServers();
  }

  async deleteServer(index: number) {
    const deletedServer = this.servers[index];
    this.servers.splice(index, 1);
    this.saveServers();
    
    // Clear current config if it was the one deleted
    if (this.config && deletedServer && this.config.path === deletedServer.path) {
      this.config = null;
    }
  }

  async selectServer(index: number) {
    const server = this.servers[index];
    if (server) {
      this.config = server;
      await invoke("set_server_config", { config: server });
      await this.refreshProperties();
      this.logs = [`[System] Selected server: ${server.name}`];
    }
  }

  async handleSelectJar() {
    try {
      const config = await invoke("select_jar_file");
      return config as ServerConfig;
    } catch (err) {
      console.error(err);
      return null;
    }
  }

  async toggleServer() {
    if (this.stats.status === "Running" || this.stats.status === "Starting") {
      await invoke("stop_server");
    } else {
      if (!this.config) return;
      this.logs = ["[System] Initializing startup..."];
      await invoke("start_server");
    }
    await this.refreshStats();
  }

  async takeOverOrphan() {
    this.logs = [...this.logs.slice(-500), "[System] Taking control of orphaned process..."];
    await invoke("stop_server");
    await this.toggleServer();
  }

  async refreshProperties() {
    if (this.config) {
      const props = await invoke("read_properties", { path: this.config.path });
      this.properties = props as ServerProperties;
    }
  }

  async saveProperties(props: ServerProperties) {
    if (this.config) {
      await invoke("write_properties", { path: this.config.path, props });
      this.properties = props;
    }
  }

  async sendCommand(command: string) {
    if (this.stats.status === "Running") {
      try {
        await invoke("send_server_command", { command });
        // Immediate visual feedback in console
        this.logs = [...this.logs.slice(-500), `[Input] > ${command}`];
      } catch (e) {
        console.error("Command failed:", e);
        this.logs = [...this.logs.slice(-500), `[System] Error: ${e}`];
      }
    }
  }

  async refreshStats() {
    try {
      const s = await invoke("get_server_stats");
      this.stats = s as ServerStats;
    } catch (e) {
      console.error(e);
    }
  }
}

export const serverStore = new ServerStore();
