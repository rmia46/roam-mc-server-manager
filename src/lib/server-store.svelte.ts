import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { ServerConfig, ServerStats, ServerProperties, ServerStatus, PlayerInfo, WorldInfo } from "./types";

const isTauri = () => !!(window as any).__TAURI_INTERNALS__;

class ServerStore {
  servers = $state<ServerConfig[]>([]);
  config = $state<ServerConfig | null>(null);
  stats = $state<ServerStats>({ cpu: 0, core_count: 1, memory: 0, status: "Offline", player_count: 0, tunnel_status: "Offline" });
  players = $state<PlayerInfo[]>([]);
  worlds = $state<WorldInfo[]>([]);
  properties = $state<ServerProperties>({});
  logs = $state<string[]>([]);
  isDownloading = $state(false);
  
  constructor() {
    if (isTauri()) {
      this.setupListeners();
    } else {
      console.warn("Tauri internals not found. Running in browser mode.");
    }
    this.loadServers();
  }

  async refreshWorlds() {
    if (this.config && isTauri()) {
      try {
        const w = await invoke("get_worlds", { path: this.config.path });
        this.worlds = w as WorldInfo[];
      } catch (e) {
        console.error("Failed to fetch world data", e);
      }
    }
  }

  async backupWorld(worldName: string) {
    if (this.config && isTauri()) {
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
    if (this.config && isTauri()) {
      try {
        const p = await invoke("get_players_data", { path: this.config.path });
        this.players = p as PlayerInfo[];
      } catch (e) {
        console.error("Failed to fetch player data", e);
      }
    }
  }

  async setupListeners() {
    if (!isTauri()) return;

    await listen<string>("server-log", (event) => {
      this.logs = [...this.logs.slice(-500), event.payload];
    });

    await listen<number>("player-update", (event) => {
      this.stats.player_count = event.payload;
    });

    await listen<ServerStatus>("status-update", (event) => {
      this.stats.status = event.payload;
    });

    await listen<any>("tunnel-status-update", (event) => {
      this.stats.tunnel_status = event.payload;
    });
  }

  async updateTunnelConfig(provider: "none" | "playit" | "ngrok", token: string) {
    if (this.config) {
      const updatedConfig = { 
        ...this.config, 
        tunnel: { provider, token, public_address: "" } 
      };
      const index = this.servers.findIndex(s => s.path === this.config?.path);
      if (index !== -1) {
        this.servers[index] = updatedConfig;
        this.config = updatedConfig;
        this.saveServers();
        if (isTauri()) {
          await invoke("set_server_config", { config: updatedConfig });
        }
      }
    }
  }

  async loadServers() {
    try {
      const saved = localStorage.getItem("mc_servers");
      if (saved) this.servers = JSON.parse(saved);
    } catch (e) {
      console.error(e);
    }
  }

  saveServers() {
    try {
      const data = JSON.parse(JSON.stringify(this.servers));
      localStorage.setItem("mc_servers", JSON.stringify(data));
    } catch (e) {
      console.error("Failed to save servers:", e);
    }
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
    
    if (this.config && deletedServer && this.config.path === deletedServer.path) {
      this.config = null;
    }
  }

  async selectServer(index: number) {
    const server = this.servers[index];
    if (server) {
      this.config = server;
      if (isTauri()) {
        await invoke("set_server_config", { config: server });
        await this.refreshProperties();
      }
      this.logs = [`[System] Selected server: ${server.name}`];
    }
  }

  async handleSelectJar() {
    if (!isTauri()) return null;
    try {
      const config = await invoke("select_jar_file");
      return config as ServerConfig;
    } catch (err) {
      console.error(err);
      return null;
    }
  }

  async toggleServer() {
    if (!isTauri()) return;
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
    if (!isTauri()) return;
    this.logs = [...this.logs.slice(-500), "[System] Taking control of orphaned process..."];
    await invoke("stop_server");
    await this.toggleServer();
  }

  async refreshProperties() {
    if (this.config && isTauri()) {
      const props = await invoke("read_properties", { path: this.config.path });
      this.properties = props as ServerProperties;
    }
  }

  async saveProperties(props: ServerProperties) {
    if (this.config) {
      this.properties = props;
      if (isTauri()) {
        await invoke("write_properties", { path: this.config.path, props });
      }
    }
  }

  async sendCommand(command: string) {
    if (this.stats.status === "Running" && isTauri()) {
      try {
        await invoke("send_server_command", { command });
        this.logs = [...this.logs.slice(-500), `[Input] > ${command}`];
      } catch (e) {
        console.error("Command failed:", e);
        this.logs = [...this.logs.slice(-500), `[System] Error: ${e}`];
      }
    }
  }

  async refreshStats() {
    if (!isTauri()) return;
    try {
      const s = await invoke("get_server_stats");
      this.stats = s as ServerStats;
    } catch (e) {
      console.error(e);
    }
  }
}

export const serverStore = new ServerStore();
