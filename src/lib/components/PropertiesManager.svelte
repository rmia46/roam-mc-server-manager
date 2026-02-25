<script lang="ts">
  import { serverStore } from "../server-store.svelte";
  import { Settings, Save, RefreshCw } from "lucide-svelte";

  let editedProps = $state<Record<string, string>>({});

  $effect(() => {
    editedProps = { ...serverStore.properties };
  });

  async function save() {
    await serverStore.saveProperties(editedProps);
  }

  async function refresh() {
    await serverStore.refreshProperties();
  }
</script>

<div class="card bg-base-100 shadow-xl p-6 h-full flex flex-col">
  <div class="flex justify-between items-center mb-6">
    <h2 class="text-xl font-bold flex items-center gap-2">
      <Settings size={20} /> Properties Editor
    </h2>
    <div class="join">
      <button class="btn btn-sm btn-ghost join-item" onclick={refresh}><RefreshCw size={14} /></button>
      <button class="btn btn-sm btn-primary join-item" onclick={save}><Save size={14} /> Save</button>
    </div>
  </div>

  <div class="overflow-y-auto flex-1 pr-2 scrollbar-thin scrollbar-thumb-base-300">
    <div class="grid grid-cols-1 gap-3">
      {#each Object.entries(editedProps) as [key, value]}
        <div class="flex flex-col md:flex-row items-start md:items-center justify-between border-b border-base-200 pb-2">
          <span class="text-sm font-mono opacity-70 break-all mb-1 md:mb-0">{key}</span>
          <input 
            type="text" 
            class="input input-bordered input-sm w-full md:w-48 font-mono text-xs" 
            bind:value={editedProps[key]} 
          />
        </div>
      {/each}
    </div>
  </div>
</div>
