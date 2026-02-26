# Project Context: Roam MC Server Manager 🌍🎮

## 📌 Overview
**Roam MC Server Manager** is a high-performance, lightweight desktop application designed to manage local Minecraft server instances with integrated tunneling and future cloud-synchronization capabilities. It is built to allow groups of friends to "decentralize" their hosting by sharing a cloud folder and using this manager to handle process lifecycles and port-forwarding.

## 🛠️ Tech Stack
- **Backend:** Rust (Tauri v2)
    - Responsibilities: Process management (Java/Sidecars), File I/O, System monitoring (sysinfo), World backups (zip).
- **Frontend:** Svelte 5 (Runes)
    - Responsibilities: Reactive UI, State management (ServerStore), Dashboard rendering.
- **Styling:** Tailwind CSS v4 + DaisyUI v5 (Custom "emowitch" theme).
- **Architecture:** Frameless window with a custom-built, draggable TitleBar.

## 📂 Project Structure
```text
roam-mc-server-manager/
├── src-tauri/              # Rust Backend
│   ├── src/lib.rs          # Core Logic (Server & Tunnel processes, IPC commands)
│   ├── capabilities/       # Tauri 2.0 Security/Permissions
│   └── tauri.conf.json     # App Configuration (Frameless, Window size, etc.)
├── src/                    # Frontend (Svelte 5)
│   ├── lib/
│   │   ├── components/     # Modular UI (PlayerManager, WorldManager, etc.)
│   │   ├── server-store.ts # Centralized Svelte State (Runes)
│   │   └── types.ts        # Shared TypeScript Interfaces
│   ├── routes/
│   │   ├── +layout.svelte  # Global setup (No context menu, theme import)
│   │   └── +page.svelte    # Subpage Router (Dashboard, Settings, Network, etc.)
│   ├── app.css             # Tailwind v4 entry & global styles
│   └── theme.css           # Modular DaisyUI Theme (Pasted from generator)
└── progress.md             # Detailed feature roadmap & status
```

## 🧠 Key Features & Logic
1. **Sidecar Process Runner:** Binds the lifecycle of tunneling binaries (Playit.gg/Ngrok) to the Minecraft Java process.
2. **Orphaned Process Detection:** Uses `sysinfo` to detect Java servers already running in a folder, allowing the manager to "Take Control" via a safe restart.
3. **High-Precision Metrics:** Normalizes CPU usage across logical cores and provides floating-point precision for low-activity states.
4. **Intelligent Import:** Automatically detects if a folder is a valid server directory (checking for `server.properties`) and provides a Setup Wizard only for new initializations.
5. **World Management:** Scans world folders, calculates size, and creates compressed `.zip` backups in a dedicated `roam_backups` directory.
6. **Frameless UI:** Custom window controls (Minimize, Maximize, Close) with dynamic rounding that sharpens edges when the window is maximized.

## 🚦 Communication (IPC)
The frontend communicates with Rust using Tauri's `invoke` for commands and `listen` for real-time events (`server-log`, `status-update`, `player-update`, `tunnel-status-update`).

## 🎨 Design System
- **Theme:** "emowitch" (Dark, Purple/Blue palette).
- **Radius:** Defined by `--radius-box` (standardized across modals and containers).
- **Performance:** Optimized for Linux webviews by avoiding backdrop-blurs and heavy CSS transitions.
