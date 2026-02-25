<script lang="ts">
  import { serverStore } from "../server-store.svelte";
  import { Trash2, AlertTriangle, ShieldCheck, Database, Settings2 } from "lucide-svelte";
  import PropertiesManager from "./PropertiesManager.svelte";

  let showDeleteConfirm = $state(false);

  async function handleDelete() {
    // Find index of current config in servers list
    const index = serverStore.servers.findIndex(s => s.path === serverStore.config?.path);
    if (index !== -1) {
      await serverStore.deleteServer(index);
      showDeleteConfirm = false;
    }
  }
</script>

<div class="h-full flex flex-col gap-8 pb-10">
  <!-- Properties Editor Section -->
  <section class="flex-1 min-h-0 flex flex-col gap-4">
    <div class="flex items-center gap-3 opacity-40">
      <Settings2 size={16} />
      <h3 class="text-xs font-bold uppercase tracking-widest">Server Properties</h3>
    </div>
    <div class="flex-1 min-h-0">
      <PropertiesManager />
    </div>
  </section>

  <!-- Danger Zone Section -->
  <section class="space-y-4">
    <div class="flex items-center gap-3 text-error opacity-60">
      <AlertTriangle size={16} />
      <h3 class="text-xs font-bold uppercase tracking-widest">Danger Zone</h3>
    </div>
    
    <div class="card bg-error/5 border border-error/20 p-6">
      <div class="flex flex-col md:flex-row justify-between items-center gap-6">
        <div>
          <h4 class="font-bold text-error">Delete Server Entry</h4>
          <p class="text-xs opacity-60 mt-1 max-w-sm">
            This will remove the server from Roam Manager. <span class="font-bold underline">No world files or JARs will be deleted</span> on your disk.
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
          <h3 class="font-black text-xl">Are you sure?</h3>
          <p class="text-xs opacity-60">Confirming will unregister this server.</p>
        </div>
      </div>
      
      <p class="text-sm py-4 border-y border-base-200">
        You are about to remove <span class="font-bold text-error">"{serverStore.config?.name}"</span> from your list. 
        You can always re-import it later by selecting the JAR file again.
      </p>

      <div class="modal-action gap-2">
        <button class="btn btn-ghost" onclick={() => showDeleteConfirm = false}>Cancel</button>
        <button class="btn btn-error px-10" onclick={handleDelete}>Delete Entry</button>
      </div>
    </div>
    <div class="modal-backdrop bg-black/80 backdrop-blur-md" onclick={() => showDeleteConfirm = false}></div>
  </div>
{/if}
