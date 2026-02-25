# Roam MC Server Manager 🌍🎮

A high-performance, lightweight Minecraft server manager built for local hosting with cloud synchronization. Designed with **Tauri (Rust)**, **Svelte**, and **Tailwind CSS + DaisyUI** for maximum efficiency and a modern look.

## 🚀 The Concept
Hosting Minecraft servers can be expensive. **Roam MC Server Manager** allows you and your friends to "pass the torch" of hosting using a shared cloud folder (e.g., MEGA, Dropbox, OneDrive).

- **Locking System:** Prevents two people from hosting at once to avoid world corruption.
- **Port Forwarding:** Built-in support for **Playit.gg** for easy, free tunneling without port forwarding.
- **High Performance:** Uses almost zero background RAM, leaving more resources for your Minecraft server.

---

## 🛠️ Prerequisites
Before you begin, ensure you have the following installed:
- **[Node.js](https://nodejs.org/)** (v18 or higher)
- **[Rust & Cargo](https://rustup.rs/)** (v1.75 or higher)
- **[Java](https://www.oracle.com/java/technologies/downloads/)** (The version required for your Minecraft server, e.g., Java 17 for MC 1.18+)
- **[Playit.gg](https://playit.gg/download)** (Download the CLI executable and place it in your path or project folder)

---

## 🏗️ Getting Started

### 1. Install Dependencies
Clone this repository (or use your current folder) and install the necessary Node.js packages:
```bash
npm install
```

### 2. Run in Development Mode
Launch the application in development mode with hot-reloading:
```bash
npm run tauri dev
```

### 3. Build from Source (Compile)
To create a high-performance executable (`.exe` on Windows, `.app` on Mac, or a binary on Linux):
```bash
npm run tauri build
```
The compiled application will be located in `src-tauri/target/release/bundle/`.

---

## 📂 Configuration (Cloud Sync Setup)
1. **Shared Folder:** Create a folder for your Minecraft server on a service like **MEGA**.
2. **Syncing:** Ensure all friends are syncing the same folder.
3. **The Lock File:** When a host starts the server, the app creates a `.lock` file in the shared folder. Other users will see a "Locked" status and will be unable to start the server until the current host stops.

---

## 🛡️ Key Features
- **Real-time Console:** View live server logs directly in the app.
- **System Monitoring:** Monitor RAM and CPU usage in real-time.
- **Auto-Tunneling:** Automatically starts the Playit.gg tunnel when the Minecraft server goes live.
- **Safety First:** Prevents accidental double-hosting across different PCs.

---

## 🤝 Contributing
Feel free to open issues or submit pull requests to improve performance or add new features!

## 📄 License
MIT License. Play on! ⛏️💎
