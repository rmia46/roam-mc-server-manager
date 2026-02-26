export interface TunnelConfig {
  provider: "none" | "playit" | "ngrok";
  token: string;
  public_address: string;
}

export interface ServerConfig {
  name?: string;
  path: string;
  jar_name: string;
  min_ram: string;
  max_ram: string;
  tunnel?: TunnelConfig;
}

export interface ServerStats {
  cpu: number;
  core_count: number;
  memory: number;
  status: "Offline" | "Starting" | "Running" | "Stopping";
  player_count: number;
  tunnel_status: "Offline" | "Connecting" | "Online" | "Error";
}

export type ServerProperties = Record<string, string>;

export interface PlayerInfo {
  uuid: string;
  name: string;
  time_played: number;
  steps: number;
}

export interface WorldInfo {
  name: string;
  size_mb: number;
  last_modified: string;
}
