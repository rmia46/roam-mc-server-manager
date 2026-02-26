<script lang="ts">
  import { serverStore } from "../server-store.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { 
    Trash2, AlertTriangle, Settings2, FileWarning, 
    ShieldCheck, Save, SlidersHorizontal, Info, Wrench 
  } from "lucide-svelte";
  import PropertiesManager from "./PropertiesManager.svelte";

  let activeTab = $state("properties"); // "identity", "properties", "maintenance"
  let showDeleteConfirm = $state(false);
  let deleteFilesFromDisk = $state(false);
  let isDeleting = $state(false);
  let newName = $state("");

  $effect(() => {
    if (serverStore.config && !newName) {
      newName = serverStore.config.name || "";
    }
  });

  async function handleRename() {
    if (!newName.trim() || !serverStore.config) return;
    const index = serverStore.servers.findIndex(s => s.path === serverStore.config?.path);
    if (index !== -1) {
      const updatedServer = { ...serverStore.servers[index], name: newName.trim() };
      serverStore.servers[index] = updatedServer;
      serverStore.config = updatedServer;
      serverStore.saveServers();
    }
  }

  async function handleDelete() {
    const serverPath = serverStore.config?.path;
    const index = serverStore.servers.findIndex(s => s.path === serverPath);
    if (index !== -1 && serverPath) {
      isDeleting = true;
      if (deleteFilesFromDisk) {
        try { await invoke("delete_directory", { path: serverPath }); } 
        catch (err) { alert("Failed: " + err); }
      }
      await serverStore.deleteServer(index);
      isDeleting = false;
      showDeleteConfirm = false;
    }
  }
</script>

<div class="h-full flex flex-col gap-6">
  <!-- Grid-Based Precise Sliding Tabs -->
  <div class="grid grid-cols-3 w-80 bg-base-100 p-1 rounded-2xl border border-base-200 shadow-sm relative self-start overflow-hidden">
    <!-- Sliding Background Indicator - Aligned to Grid -->
    <div 
      class="absolute top-1 bottom-1 bg-primary rounded-xl transition-all duration-300 ease-out z-0"
      style="width: calc(33.33% - 2px); transform: translateX({
        activeTab === 'identity' ? '2px' : 
        activeTab === 'properties' ? 'calc(100% + 2px)' : 'calc(200% + 2px)'
      });"
    ></div>

    <button 
      class="relative z-10 py-2.5 text-[10px] font-black uppercase tracking-widest transition-colors duration-300 {activeTab === 'identity' ? 'text-primary-content' : 'opacity-40 hover:opacity-100'}"
      onclick={() => activeTab = 'identity'}
    >
      Identity
    </button>
    <button 
      class="relative z-10 py-2.5 text-[10px] font-black uppercase tracking-widest transition-colors duration-300 {activeTab === 'properties' ? 'text-primary-content' : 'opacity-40 hover:opacity-100'}"
      onclick={() => activeTab = 'properties'}
    >
      Properties
    </button>
    <button 
      class="relative z-10 py-2.5 text-[10px] font-black uppercase tracking-widest transition-colors duration-300 {activeTab === 'maintenance' ? 'text-primary-content' : 'opacity-40 hover:opacity-100'}"
      onclick={() => activeTab = 'maintenance'}
    >
      Cleanup
    </button>
  </div>

  <!-- Tab Content -->
  <div class="flex-1 min-h-0 bg-base-100 rounded-3xl border border-base-200 p-8 shadow-sm">
    {#if activeTab === "identity"}
      <div class="max-w-2xl space-y-8 will-change-transform animate-in fade-in duration-200">
        <div class="space-y-2">
          <h3 class="text-2xl font-black tracking-tight italic uppercase">Instance Identity</h3>
          <p class="text-[10px] opacity-40 uppercase font-bold tracking-widest">Labels & Metadata</p>
        </div>
        
        <div class="space-y-6">
          <div class="form-control">
            <label class="label" for="rename-input"><span class="label-text text-[10px] font-black uppercase opacity-40 tracking-widest">New Label</span></label>
            <input id="rename-input" type="text" class="input input-bordered bg-base-200 border-none focus:ring-1 ring-primary/50 text-sm font-bold" bind:value={newName} />
          </div>
          <button class="btn btn-primary px-10 gap-2 shadow-lg" onclick={handleRename} disabled={!newName.trim() || newName === serverStore.config?.name}>
            <ShieldCheck size={16} /> Update Label
          </button>
        </div>
      </div>

    {:else if activeTab === "properties"}
      <div class="h-full flex flex-col will-change-transform animate-in fade-in duration-200">
        <PropertiesManager />
      </div>

    {:else if activeTab === "maintenance"}
      <div class="max-w-2xl space-y-10 will-change-transform animate-in fade-in duration-200">
        <div class="space-y-2">
          <h3 class="text-2xl font-black tracking-tight uppercase italic">Server Cleanup</h3>
          <p class="text-[10px] opacity-40 uppercase font-bold tracking-widest">Instance Management</p>
        </div>

        <div class="bg-error/5 border border-error/10 p-8 rounded-3xl space-y-6">
          <div class="flex items-start gap-4">
            <div class="p-3 bg-error/10 rounded-2xl text-error"><Trash2 size={24} /></div>
            <div>
              <h4 class="font-black text-lg uppercase leading-none">Remove Server</h4>
              <p class="text-[10px] opacity-60 mt-2 uppercase tracking-widest leading-relaxed">Remove this server from your list. You can choose to keep or delete your world files.</p>
            </div>
          </div>
          <button class="btn btn-error btn-outline btn-block gap-2 h-14" onclick={() => showDeleteConfirm = true}>
            <Trash2 size={18} /> DELETE SERVER ENTRY
          </button>
        </div>
      </div>
    {/if}
  </div>
</div>

{#if showDeleteConfirm}
  <div class="modal modal-open" role="dialog">
    <div class="modal-box bg-base-100 border border-error/30 shadow-2xl">
      <div class="flex items-center gap-4 text-error mb-6">
        <div class="p-3 bg-error/10 rounded-2xl"><Trash2 size={24} /></div>
        <h3 class="font-black text-xl uppercase italic tracking-tighter">Confirm Deletion</h3>
      </div>
      <div class="py-4 space-y-4 border-y border-base-200">
        <p class="text-sm">Deleting entry for: <span class="font-black text-error">"{serverStore.config?.name}"</span></p>
        <div class="form-control bg-error/5 p-4 rounded-xl border border-error/10">
          <label class="label cursor-pointer justify-start gap-4">
            <input type="checkbox" class="checkbox checkbox-error checkbox-sm" bind:checked={deleteFilesFromDisk} />
            <div class="flex flex-col">
              <span class="label-text font-black text-error text-[10px] uppercase tracking-wider">Delete files from disk</span>
              <span class="text-[9px] opacity-40 italic">Check this to also delete world folders and JARs.</span>
            </div>
          </label>
        </div>
      </div>
      <div class="modal-action gap-2">
        <button class="btn btn-ghost px-8" onclick={() => showDeleteConfirm = false} disabled={isDeleting}>CANCEL</button>
        <button class="btn btn-error flex-1 shadow-lg" onclick={handleDelete} disabled={isDeleting}>
          {#if isDeleting}
            <span class="loading loading-spinner loading-xs"></span>
          {:else}
            CONFIRM DELETE
          {/if}
        </button>
      </div>
    </div>
    <div class="modal-backdrop bg-black/80" role="presentation" onclick={() => showDeleteConfirm = false}></div>
  </div>
{/if}
