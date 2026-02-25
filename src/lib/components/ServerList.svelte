<script lang="ts">
  import { serverStore } from "../server-store.svelte";
  import { Plus, Server, ChevronRight, X, FolderSearch } from "lucide-svelte";

  let newServerName = $state("");
  let newServerPath = $state("");
  let newServerJar = $state("");
  let newServerRam = $state(2);
  let showModal = $state(false);

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

  async function addNewServer() {
    if (!newServerName || !newServerPath || !newServerJar) return;
    await serverStore.addServer(newServerName, newServerPath, newServerJar, newServerRam);
    showModal = false;
    newServerName = ""; newServerPath = ""; newServerJar = "";
  }
</script>

<div class="flex flex-col gap-4">
  <div class="flex items-center justify-between px-2">
    <h3 class="text-[10px] font-black uppercase tracking-[0.2em] opacity-40">Active Servers</h3>
    <button 
      class="btn btn-xs btn-circle btn-primary" 
      onclick={() => showModal = true}
      aria-label="Add Server"
    >
      <Plus size={14} />
    </button>
  </div>

  <div class="grid grid-cols-1 gap-2">
    {#each serverStore.servers as server, i}
      <button 
        class="w-full flex items-center justify-between p-3 rounded-xl transition-all duration-200 group
               {serverStore.config?.path === server.path ? 'bg-primary text-primary-content shadow-lg shadow-primary/20' : 'hover:bg-base-200 border border-transparent opacity-70'}"
        onclick={() => serverStore.selectServer(i)}
      >
        <div class="flex items-center gap-3 overflow-hidden">
          <Server size={16} class="shrink-0 {serverStore.config?.path === server.path ? 'opacity-100' : 'opacity-50'}" />
          <div class="text-left truncate">
            <p class="text-xs font-bold leading-none truncate">{server.name}</p>
            <p class="text-[9px] mt-1 font-mono truncate {serverStore.config?.path === server.path ? 'opacity-70' : 'opacity-30'}">{server.jar_name}</p>
          </div>
        </div>
        <ChevronRight size={14} class="shrink-0 {serverStore.config?.path === server.path ? 'opacity-100' : 'opacity-0 group-hover:opacity-50'} transition-all" />
      </button>
    {/each}

    {#if serverStore.servers.length === 0}
      <div class="p-6 border-2 border-dashed border-base-300 rounded-2xl flex flex-col items-center gap-2 opacity-20">
        <Server size={20} />
        <p class="text-[9px] font-bold uppercase tracking-wider text-center leading-tight">Create your first<br/>server entry</p>
      </div>
    {/if}
  </div>
</div>

<!-- Add Server Modal -->
{#if showModal}
  <div class="modal modal-open" role="dialog" aria-modal="true">
    <div class="modal-box bg-base-100 border border-base-200 shadow-2xl relative max-w-sm">
      <button 
        class="btn btn-sm btn-circle btn-ghost absolute right-4 top-4" 
        onclick={() => showModal = false}
        aria-label="Close"
      >
        <X size={16} />
      </button>

      <h3 class="font-black text-xl mb-6 flex items-center gap-3">
        <div class="p-2 bg-primary/10 rounded-lg text-primary"><Server size={20} /></div>
        Import Server
      </h3>
      
      <div class="space-y-4">
        <div class="form-control">
          <label class="label" for="new-server-name"><span class="label-text text-xs font-bold uppercase opacity-50 tracking-widest">Display Name</span></label>
          <input id="new-server-name" type="text" class="input input-bordered focus:input-primary transition-all" placeholder="Survival World" bind:value={newServerName} />
        </div>

        <div class="form-control">
          <label class="label" for="new-server-jar"><span class="label-text text-xs font-bold uppercase opacity-50 tracking-widest">Binary (JAR)</span></label>
          <div class="join">
            <input id="new-server-jar" type="text" class="input input-bordered join-item flex-1 text-xs" readonly placeholder="Browse server.jar..." bind:value={newServerJar} />
            <button class="btn join-item btn-primary" onclick={pickJar} type="button"><FolderSearch size={18} /></button>
          </div>
        </div>

        <div class="form-control">
          <label class="label" for="new-server-ram">
            <span class="label-text text-xs font-bold uppercase opacity-50 tracking-widest">RAM Limit</span>
            <span class="badge badge-primary font-mono">{newServerRam}GB</span>
          </label>
          <input id="new-server-ram" type="range" min="1" max="16" class="range range-primary range-xs" bind:value={newServerRam} />
        </div>
      </div>

      <div class="modal-action">
        <button class="btn btn-block btn-primary shadow-lg" onclick={addNewServer} type="button" disabled={!newServerName || !newServerJar}>Create Entry</button>
      </div>
    </div>
    <button class="modal-backdrop bg-black/80 backdrop-blur-sm border-none cursor-default" onclick={() => showModal = false} aria-label="Close backdrop"></button>
  </div>
{/if}
