import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { ServerConfig, ServerStats, ServerProperties, ServerStatus } from "./types";

class ServerStore {
  servers = $state<ServerConfig[]>([]);
  config = $state<ServerConfig | null>(null);
  stats = $state<ServerStats>({ cpu: 0, memory: 0, status: "Offline", player_count: 0 });
  properties = $state<ServerProperties>({});
  logs = $state<string[]>([]);
  isDownloading = $state(false);
  
  constructor() {
    this.setupListeners();
    this.loadServers();
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
    this.servers.splice(index, 1);
    this.saveServers();
    if (this.config && this.servers[index] && this.config.path === this.servers[index].path) {
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
