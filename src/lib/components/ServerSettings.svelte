<script lang="ts">
  import { serverStore } from "../server-store.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { Trash2, AlertTriangle, Settings2, FileWarning } from "lucide-svelte";
  import PropertiesManager from "./PropertiesManager.svelte";

  let showDeleteConfirm = $state(false);
  let deleteFilesFromDisk = $state(false);
  let isDeleting = $state(false);

  async function handleDelete() {
    const serverPath = serverStore.config?.path;
    const index = serverStore.servers.findIndex(s => s.path === serverPath);
    
    if (index !== -1 && serverPath) {
      isDeleting = true;
      if (deleteFilesFromDisk) {
        try {
          await invoke("delete_directory", { path: serverPath });
        } catch (err) {
          alert("Failed to delete files from disk: " + err);
        }
      }
      
      await serverStore.deleteServer(index);
      isDeleting = false;
      showDeleteConfirm = false;
      deleteFilesFromDisk = false;
    }
  }
</script>

<div class="h-full flex flex-col gap-8 pb-10">
  <section class="flex-1 min-h-0 flex flex-col gap-4">
    <div class="flex items-center gap-3 opacity-40">
      <Settings2 size={16} />
      <h3 class="text-xs font-bold uppercase tracking-widest">Server Properties</h3>
    </div>
    <div class="flex-1 min-h-0">
      <PropertiesManager />
    </div>
  </section>

  <section class="space-y-4">
    <div class="flex items-center gap-3 text-error opacity-60">
      <AlertTriangle size={16} />
      <h3 class="text-xs font-bold uppercase tracking-widest">Danger Zone</h3>
    </div>
    
    <div class="card bg-error/5 border border-error/20 p-6">
      <div class="flex flex-col md:flex-row justify-between items-center gap-6">
        <div>
          <h4 class="font-bold text-error">Remove Server Instance</h4>
          <p class="text-xs opacity-60 mt-1 max-w-sm">
            Unregisters this server from the manager. You can choose to keep or delete the actual files on your disk.
          </p>
        </div>
        <button 
          class="btn btn-error btn-outline gap-2 px-8"
          onclick={() => showDeleteConfirm = true}
        >
          <Trash2 size={16} /> Remove Entry
        </button>
      </div>
    </div>
  </section>
</div>

{#if showDeleteConfirm}
  <div class="modal modal-open" role="dialog">
    <div class="modal-box bg-base-100 border border-error/30 shadow-2xl">
      <div class="flex items-center gap-4 text-error mb-6">
        <div class="p-3 bg-error/10 rounded-2xl"><Trash2 size={24} /></div>
        <div>
          <h3 class="font-black text-xl">Confirm Removal</h3>
          <p class="text-[10px] uppercase font-bold opacity-40 tracking-widest">Action cannot be undone</p>
        </div>
      </div>
      
      <div class="py-4 space-y-4 border-y border-base-200">
        <p class="text-sm">
          You are removing <span class="font-bold text-error">"{serverStore.config?.name}"</span>. 
        </p>

        <div class="form-control bg-error/10 p-4 rounded-xl border border-error/20">
          <label class="label cursor-pointer justify-start gap-4">
            <input type="checkbox" class="checkbox checkbox-error" bind:checked={deleteFilesFromDisk} />
            <div class="flex flex-col">
              <span class="label-text font-black text-error text-xs uppercase tracking-wider">Permanent Deletion</span>
              <span class="text-[10px] opacity-60 italic">Delete all server files, worlds, and JARs from my disk.</span>
            </div>
          </label>
        </div>
      </div>

      {#if deleteFilesFromDisk}
        <div class="alert alert-error rounded-xl mt-4 py-2 flex gap-2">
          <FileWarning size={16} />
          <span class="text-[10px] font-bold uppercase tracking-widest">WARNING: ALL FILES WILL BE LOST FOREVER</span>
        </div>
      {/if}

      <div class="modal-action gap-2">
        <button class="btn btn-ghost" onclick={() => showDeleteConfirm = false} disabled={isDeleting}>Cancel</button>
        <button class="btn btn-error px-10 shadow-lg" onclick={handleDelete} disabled={isDeleting}>
          {#if isDeleting}
            <span class="loading loading-spinner loading-xs"></span> DELETING...
          {:else}
            {deleteFilesFromDisk ? 'PERMANENT DELETE' : 'REMOVE ENTRY'}
          {/if}
        </button>
      </div>
    </div>
    <div class="modal-backdrop bg-black/80 backdrop-blur-sm" onclick={() => showDeleteConfirm = false}></div>
  </div>
{/if}
