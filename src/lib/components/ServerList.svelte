<script lang="ts">
  import { serverStore } from "../server-store.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { Plus, Server, ChevronRight, X, FolderSearch, Settings2, Sparkles } from "lucide-svelte";

  let newServerName = $state("");
  let newServerPath = $state("");
  let newServerJar = $state("");
  let newServerRam = $state(2);
  let showModal = $state(false);
  
  // Setup Wizard State
  let showWizard = $state(false);
  let worldName = $state("world");
  let difficulty = $state("easy");
  let gamemode = $state("survival");
  let serverPort = $state("25565");

  async function pickJar() {
    try {
      const config = await serverStore.handleSelectJar();
      if (config) {
        newServerPath = config.path;
        newServerJar = config.jar_name;
        if (!newServerName) {
          newServerName = newServerJar.replace(".jar", "").split(/[_-]/).map(word => word.charAt(0).toUpperCase() + word.slice(1)).join(" ");
        }
      }
    } catch (err) {
      console.error("Failed to pick JAR:", err);
    }
  }

  async function handleMainSubmit() {
    if (!newServerName || !newServerPath || !newServerJar) return;
    
    // Check if server already exists
    const exists = await invoke("is_server_initialized", { path: newServerPath });
    
    if (exists) {
      // Direct Import
      await finishCreation();
    } else {
      // New Server - Show Wizard
      showModal = false;
      showWizard = true;
    }
  }

  async function finishCreation() {
    // If wizard was shown, pre-write properties
    if (showWizard) {
      const initialProps = {
        "level-name": worldName,
        "difficulty": difficulty,
        "gamemode": gamemode,
        "server-port": serverPort,
        "motd": `A Roam MC Managed Server: ${newServerName}`
      };
      await invoke("write_properties", { path: newServerPath, props: initialProps });
    }

    await serverStore.addServer(newServerName, newServerPath, newServerJar, newServerRam);
    
    // Reset
    showModal = false;
    showWizard = false;
    newServerName = ""; newServerPath = ""; newServerJar = "";
  }
</script>

<div class="flex flex-col gap-4">
  <div class="flex items-center justify-between px-2">
    <h3 class="text-[10px] font-black uppercase tracking-[0.2em] opacity-40">Active Servers</h3>
    <button class="btn btn-xs btn-circle btn-primary" onclick={() => showModal = true} aria-label="Add Server">
      <Plus size={14} />
    </button>
  </div>

  <div class="grid grid-cols-1 gap-2">
    {#each serverStore.servers as server, i}
      <button 
        class="w-full flex items-center justify-between p-3 rounded-xl transition-colors group
               {serverStore.config?.path === server.path ? 'bg-primary text-primary-content shadow-sm' : 'hover:bg-base-200 border border-transparent opacity-70'}"
        onclick={() => serverStore.selectServer(i)}
      >
        <div class="flex items-center gap-3 overflow-hidden">
          <Server size={16} class="shrink-0 {serverStore.config?.path === server.path ? 'opacity-100' : 'opacity-50'}" />
          <div class="text-left truncate">
            <p class="text-xs font-bold leading-none truncate">{server.name}</p>
            <p class="text-[9px] mt-1 font-mono truncate {serverStore.config?.path === server.path ? 'opacity-70' : 'opacity-30'}">{server.jar_name}</p>
          </div>
        </div>
        <ChevronRight size={14} class="shrink-0 {serverStore.config?.path === server.path ? 'opacity-100' : 'opacity-0 group-hover:opacity-50'}" />
      </button>
    {/each}
  </div>
</div>

<!-- Step 1: Basic Import Modal -->
{#if showModal}
  <div class="modal modal-open" role="dialog">
    <div class="modal-box bg-base-100 border border-base-200 shadow-2xl relative max-w-sm">
      <button class="btn btn-sm btn-circle btn-ghost absolute right-4 top-4" onclick={() => showModal = false} aria-label="Close"><X size={16} /></button>
      <h3 class="font-black text-xl mb-6 flex items-center gap-3">Import Server</h3>
      
      <div class="space-y-4">
        <div class="form-control">
          <label class="label" for="new-server-name"><span class="label-text text-xs font-bold uppercase opacity-50">Display Name</span></label>
          <input id="new-server-name" type="text" class="input input-bordered" placeholder="Survival World" bind:value={newServerName} />
        </div>
        <div class="form-control">
          <label class="label" for="new-server-jar"><span class="label-text text-xs font-bold uppercase opacity-50">Binary (JAR)</span></label>
          <div class="join">
            <input id="new-server-jar" type="text" class="input input-bordered join-item flex-1 text-xs" readonly placeholder="Browse JAR..." bind:value={newServerJar} />
            <button class="btn join-item btn-primary" onclick={pickJar}><FolderSearch size={18} /></button>
          </div>
        </div>
        <div class="form-control">
          <label class="label" for="new-server-ram"><span class="label-text text-xs font-bold uppercase opacity-50">RAM: {newServerRam}GB</span></label>
          <input id="new-server-ram" type="range" min="1" max="16" class="range range-primary range-xs" bind:value={newServerRam} />
        </div>
      </div>
      <div class="modal-action">
        <button class="btn btn-block btn-primary" onclick={handleMainSubmit} disabled={!newServerName || !newServerJar}>Next Step</button>
      </div>
    </div>
    <div class="modal-backdrop bg-black/80" role="presentation" onclick={() => showModal = false}></div>
  </div>
{/if}

<!-- Step 2: New Server Wizard -->
{#if showWizard}
  <div class="modal modal-open" role="dialog">
    <div class="modal-box bg-base-100 border border-primary/20 shadow-2xl relative max-w-md">
      <div class="flex items-center gap-4 mb-8">
        <div class="p-3 bg-primary/10 rounded-2xl text-primary"><Sparkles size={24} /></div>
        <div>
          <h3 class="font-black text-xl leading-none">Initialize World</h3>
          <p class="text-[10px] uppercase font-bold opacity-40 mt-1 tracking-widest">New Server Detected</p>
        </div>
      </div>
      
      <div class="grid grid-cols-2 gap-4">
        <div class="form-control col-span-2">
          <label class="label" for="world-name"><span class="label-text text-xs font-bold opacity-50">WORLD NAME</span></label>
          <input id="world-name" type="text" class="input input-bordered" bind:value={worldName} />
        </div>
        
        <div class="form-control">
          <label class="label" for="difficulty"><span class="label-text text-xs font-bold opacity-50">DIFFICULTY</span></label>
          <select id="difficulty" class="select select-bordered" bind:value={difficulty}>
            <option value="peaceful">Peaceful</option>
            <option value="easy">Easy</option>
            <option value="normal">Normal</option>
            <option value="hard">Hard</option>
          </select>
        </div>

        <div class="form-control">
          <label class="label" for="gamemode"><span class="label-text text-xs font-bold opacity-50">GAME MODE</span></label>
          <select id="gamemode" class="select select-bordered" bind:value={gamemode}>
            <option value="survival">Survival</option>
            <option value="creative">Creative</option>
            <option value="adventure">Adventure</option>
            <option value="spectator">Spectator</option>
          </select>
        </div>

        <div class="form-control col-span-2">
          <label class="label" for="server-port"><span class="label-text text-xs font-bold opacity-50">SERVER PORT</span></label>
          <input id="server-port" type="text" class="input input-bordered font-mono" bind:value={serverPort} />
        </div>
      </div>

      <div class="modal-action gap-2">
        <button class="btn btn-ghost" onclick={() => showWizard = false}>Back</button>
        <button class="btn btn-primary flex-1 shadow-lg" onclick={finishCreation}>Initialize & Start</button>
      </div>
    </div>
    <div class="modal-backdrop bg-black/80" role="presentation" onclick={() => showWizard = false}></div>
  </div>
{/if}
