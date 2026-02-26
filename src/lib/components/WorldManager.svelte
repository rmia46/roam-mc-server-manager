<script lang="ts">
  import { serverStore } from "../server-store.svelte";
  import { onMount } from "svelte";
  import { Globe2, Archive, HardDrive, RefreshCw, Clock, CheckCircle2, AlertTriangle } from "lucide-svelte";

  // Use a string to track which world is currently backing up
  let currentAction = $state<{ name: string, status: "idle" | "loading" | "success" | "error" }>({
    name: "",
    status: "idle"
  });

  async function refresh() {
    await serverStore.refreshWorlds();
  }

  async function handleBackup(worldName: string) {
    // Start Loading
    currentAction = { name: worldName, status: "loading" };
    
    try {
      await serverStore.backupWorld(worldName);
      // Set Success
      currentAction = { name: worldName, status: "success" };
      
      // Wait 3 seconds then Reset
      setTimeout(() => {
        currentAction = { name: "", status: "idle" };
      }, 3000);
    } catch (e) {
      console.error(e);
      currentAction = { name: worldName, status: "error" };
      setTimeout(() => {
        currentAction = { name: "", status: "idle" };
      }, 3000);
    }
  }

  onMount(() => {
    refresh();
  });
</script>

<div class="h-full flex flex-col gap-6">
  <!-- Header -->
  <div class="flex justify-between items-center bg-base-100 p-4 rounded-2xl border border-base-200 shadow-sm">
    <div class="flex items-center gap-3">
      <div class="p-2 bg-primary/10 rounded-lg text-primary"><Globe2 size={18} /></div>
      <div>
        <h3 class="font-black text-sm uppercase tracking-widest">World Instances</h3>
        <p class="text-[10px] opacity-60">Manage and backup your Minecraft worlds</p>
      </div>
    </div>
    <button class="btn btn-sm btn-ghost gap-2 opacity-60 hover:opacity-100" onclick={refresh}>
      <RefreshCw size={14} /> Refresh List
    </button>
  </div>

  <!-- World Grid -->
  <div class="flex-1 overflow-y-auto pr-2 custom-scrollbar">
    <div class="grid grid-cols-1 gap-4">
      {#each serverStore.worlds as world}
        <div class="card bg-base-100 border border-base-200 shadow-md hover:border-primary/30 transition-all group">
          <div class="card-body p-6 flex flex-row items-center justify-between gap-6">
            <div class="flex items-center gap-6">
              <div class="p-4 bg-base-200 rounded-3xl text-primary/40 group-hover:text-primary transition-colors">
                <Globe2 size={32} />
              </div>
              
              <div class="space-y-1">
                <h4 class="font-black text-xl tracking-tight leading-none">{world.name}</h4>
                <div class="flex items-center gap-4 text-[10px] uppercase font-bold tracking-widest opacity-60">
                  <span class="flex items-center gap-1.5"><HardDrive size={12}/> {world.size_mb.toFixed(1)} MB</span>
                  <span class="flex items-center gap-1.5"><Clock size={12}/> {world.last_modified}</span>
                </div>
              </div>
            </div>

            <div class="flex items-center gap-3">
              <!-- 
                Dynamic Button Styling 
                Matches only if this specific world is being acted upon
              -->
              <button 
                class="btn min-w-[160px] gap-3 shadow-lg transition-all duration-300
                       {currentAction.name === world.name && currentAction.status === 'loading' ? 'btn-primary opacity-80' : 
                        (currentAction.name === world.name && currentAction.status === 'success' ? 'btn-success text-success-content scale-105' : 
                         (currentAction.name === world.name && currentAction.status === 'error' ? 'btn-error text-error-content' : 'btn-primary shadow-primary/10'))}"
                onclick={() => handleBackup(world.name)}
                disabled={currentAction.status === 'loading'}
              >
                {#if currentAction.name === world.name && currentAction.status === "loading"}
                  <span class="loading loading-spinner loading-xs"></span>
                  <span class="text-[10px] font-black uppercase tracking-widest">Archiving...</span>
                {:else if currentAction.name === world.name && currentAction.status === "success"}
                   <CheckCircle2 size={16} class="animate-bounce" />
                   <span class="text-[10px] font-black uppercase tracking-widest">Success!</span>
                {:else if currentAction.name === world.name && currentAction.status === "error"}
                   <AlertTriangle size={16} />
                   <span class="text-[10px] font-black uppercase tracking-widest">Failed!</span>
                {:else}
                  <Archive size={16} />
                  <span class="text-[10px] font-black uppercase tracking-widest">Backup World</span>
                {/if}
              </button>
            </div>
          </div>
        </div>
      {/each}

      {#if serverStore.worlds.length === 0}
        <div class="py-20 flex flex-col items-center justify-center opacity-10 gap-4 border-2 border-dashed border-base-200 rounded-3xl">
          <Globe2 size={64} />
          <p class="text-xl font-black uppercase tracking-[0.2em]">No World Directories Detected</p>
        </div>
      {/if}
    </div>

    <!-- Backup Info Card -->
    <div class="mt-8 p-6 bg-primary/5 rounded-3xl border border-primary/10 flex items-start gap-4">
      <div class="p-2 bg-primary/10 rounded-lg text-primary"><Archive size={18} /></div>
      <div>
        <h5 class="text-xs font-black uppercase tracking-widest text-primary/80">Backup Information</h5>
        <p class="text-[10px] opacity-60 mt-1 leading-relaxed">
          Backups are stored as <strong>.zip</strong> archives in the <code class="bg-base-200 px-1 rounded">roam_backups</code> folder within your server directory.
          Each backup includes the entire world data including player data, regions, and level settings.
        </p>
      </div>
    </div>
  </div>
</div>

<style>
  .custom-scrollbar::-webkit-scrollbar {
    width: 4px;
  }
  .custom-scrollbar::-webkit-scrollbar-track {
    background: transparent;
  }
  .custom-scrollbar::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.05);
    border-radius: 10px;
  }
</style>
