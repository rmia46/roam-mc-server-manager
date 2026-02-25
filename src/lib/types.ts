export interface ServerConfig {
  name?: string;
  path: string;
  jar_name: string;
  min_ram: string;
  max_ram: string;
}

export interface ServerStats {
  cpu: number;
  memory: number;
  status: "Offline" | "Starting" | "Running" | "Stopping";
  player_count: number;
}

export type ServerProperties = Record<string, string>;

export interface PlayerInfo {
  uuid: string;
  name: string;
  time_played: number;
  steps: number;
}
