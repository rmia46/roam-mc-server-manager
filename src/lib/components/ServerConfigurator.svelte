<script lang="ts">
  import { serverStore } from "../server-store.svelte";
  import { Folder, Settings, Download, Zap, FileCode } from "lucide-svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";

  const isTauri = () => !!(window as any).__TAURI_INTERNALS__;

  let path = $state("");
  let jarName = $state("server.jar");
  let maxRamGb = $state(2);
  let minRamGb = $state(1);

  async function handleSelectJar() {
    if (!isTauri()) return;
    try {
      const config = await invoke("select_jar_file");
      if (config) {
        const c = config as any;
        path = c.path;
        jarName = c.jar_name;
        // Parse existing RAM if possible, or default to slider
        await serverStore.setConfig({
          ...c,
          max_ram: `${maxRamGb}G`,
          min_ram: `${minRamGb}G`
        });
      }
    } catch (err) {
      console.error(err);
    }
  }

  async function selectFolder() {
    if (!isTauri()) return;
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: "Select Minecraft Server Folder"
      });
      if (selected) {
        path = selected as string;
      }
    } catch (err) {
      console.error(err);
    }
  }

  async function handleSave() {
    if (!path) return;
    await serverStore.setConfig({
      path,
      jar_name: jarName,
      min_ram: `${minRamGb}G`,
      max_ram: `${maxRamGb}G`,
    });
  }

  async function quickDownload(type: "vanilla" | "paper") {
    if (!path) {
      alert("Please specify a path first!");
      return;
    }
    await serverStore.downloadServer(path, type);
  }
</script>

<div class="space-y-4">
  <!-- JAR Selection Card -->
  <div class="card bg-primary text-primary-content shadow-xl p-6">
    <div class="flex items-center gap-4 mb-4">
      <div class="p-3 bg-white/20 rounded-2xl"><FileCode size={24} /></div>
      <div>
        <h2 class="text-xl font-black uppercase tracking-tight">Import Server</h2>
        <p class="text-xs opacity-70">Select an existing server.jar</p>
      </div>
    </div>
    <button 
      class="btn btn-white w-full gap-2 border-none shadow-lg hover:scale-[1.02] transition-transform" 
      onclick={handleSelectJar}
    >
      <Folder size={18} /> Browse JAR File
    </button>
  </div>

  <div class="card bg-base-100 shadow-xl p-6 border border-base-200">
    <h2 class="text-xl font-bold mb-4 flex items-center gap-2">
      <Settings size={20} /> Advanced Config
    </h2>

    <div class="space-y-4">
      <div class="form-control">
        <label class="label" for="server-path">
          <span class="label-text text-xs font-bold uppercase opacity-50">Server Folder Path</span>
        </label>
        <div class="join">
          <input 
            id="server-path"
            type="text" 
            placeholder="/path/to/server" 
            class="input input-bordered join-item flex-1 text-sm" 
            bind:value={path} 
          />
          <button class="btn join-item btn-primary" onclick={selectFolder} type="button">
            <Folder size={18} />
          </button>
        </div>
      </div>

      <div class="form-control">
        <label class="label" for="jar-name">
          <span class="label-text text-xs font-bold uppercase opacity-50">JAR Name</span>
        </label>
        <input id="jar-name" type="text" class="input input-bordered input-sm" bind:value={jarName} />
      </div>

      <!-- RAM Selectors -->
      <div class="space-y-4 pt-2">
        <div class="form-control">
          <label class="label" for="max-ram">
            <span class="label-text text-xs font-bold uppercase opacity-50">Max Memory Allocation</span>
            <span class="badge badge-primary font-mono">{maxRamGb}GB</span>
          </label>
          <input 
            id="max-ram" 
            type="range" 
            min="1" 
            max="32" 
            class="range range-primary range-xs" 
            bind:value={maxRamGb} 
          />
          <div class="w-full flex justify-between text-[10px] px-2 opacity-30 font-mono mt-1">
            <span>1G</span>
            <span>8G</span>
            <span>16G</span>
            <span>24G</span>
            <span>32G</span>
          </div>
        </div>
      </div>

      <button class="btn btn-block btn-primary shadow-lg mt-2" onclick={handleSave}>Save Changes</button>
    </div>
  </div>

  <!-- Quick Setup Card -->
  <div class="card bg-base-100 shadow-xl p-6 border border-primary/10">
    <h2 class="text-lg font-bold mb-4 flex items-center gap-2 text-primary">
      <Zap size={18} /> Install Template
    </h2>
    <div class="grid grid-cols-2 gap-3">
      <button class="btn btn-sm btn-outline btn-primary" onclick={() => quickDownload("paper")}>
        <Download size={14} /> Paper 1.21.1
      </button>
      <button class="btn btn-sm btn-outline" onclick={() => quickDownload("vanilla")}>
        <Download size={14} /> Vanilla 1.21.1
      </button>
    </div>
  </div>
</div>
