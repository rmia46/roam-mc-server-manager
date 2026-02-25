<script lang="ts">
  import { serverStore } from "../server-store.svelte";
  import { Settings, Save, RefreshCw, Search, CheckCircle2, ChevronDown } from "lucide-svelte";

  let editedProps = $state<Record<string, string>>({});
  let searchQuery = $state("");
  let saveState = $state<"idle" | "saving" | "saved">("idle");
  let saveFeedback = $state(false);

  $effect(() => {
    editedProps = { ...serverStore.properties };
  });

  async function save() {
    if (saveState !== "idle") return;
    saveState = "saving";
    try {
      await serverStore.saveProperties(editedProps);
      saveState = "saved";
      saveFeedback = true;
      setTimeout(() => { saveState = "idle"; }, 2500);
    } catch (e) {
      console.error(e);
      saveState = "idle";
    }
  }

  async function refresh() {
    await serverStore.refreshProperties();
  }

  const filteredProps = $derived(
    Object.entries(editedProps)
      .filter(([key]) => key.toLowerCase().includes(searchQuery.toLowerCase()))
      .sort(([a], [b]) => a.localeCompare(b))
  );
</script>

<div class="h-full flex flex-col gap-4 relative">
  <!-- Toolbar -->
  <div class="flex flex-col md:flex-row justify-between items-center gap-4 bg-base-100 p-4 rounded-2xl border border-base-200 shadow-sm">
    <div class="relative w-full md:w-64">
      <Search size={14} class="absolute left-3 top-1/2 -translate-y-1/2 opacity-30" />
      <input 
        type="text" 
        placeholder="Search settings..." 
        class="input input-sm input-bordered w-full pl-10 bg-base-200 border-none focus:ring-1 ring-primary/50" 
        bind:value={searchQuery}
      />
    </div>

    <div class="flex items-center gap-2 w-full md:w-auto">
      {#if saveFeedback}
        <div class="flex items-center gap-2 text-[10px] font-bold text-warning uppercase tracking-widest animate-pulse pr-4">
          <RefreshCw size={12} /> Restart required for some changes
        </div>
      {/if}

      <button class="btn btn-sm btn-ghost gap-2 opacity-40 hover:opacity-100" onclick={refresh}>
        <RefreshCw size={14} />
      </button>
      
      <button 
        class="btn btn-sm min-w-[160px] gap-2 shadow-lg border-none transition-all duration-300
               {saveState === 'saved' ? 'bg-success text-success-content scale-105' : 'bg-primary text-primary-content hover:bg-primary/90'}
               {saveState === 'saving' ? 'cursor-wait opacity-80' : 'cursor-default'}" 
        onclick={save}
      >
        {#if saveState === "saving"}
          <span class="loading loading-spinner loading-xs"></span>
          <span class="text-[10px] font-black uppercase tracking-widest">Saving...</span>
        {:else if saveState === "saved"}
          <div class="flex items-center gap-2 animate-in zoom-in-90 duration-300">
            <CheckCircle2 size={14} class="animate-bounce" />
            <span class="text-[10px] font-black uppercase tracking-widest">Config Saved!</span>
          </div>
        {:else}
          <Save size={14} />
          <span class="text-[10px] font-black uppercase tracking-widest">Save Properties</span>
        {/if}
      </button>
    </div>
  </div>

  <!-- Properties List -->
  <div class="flex-1 overflow-y-auto pr-2 custom-scrollbar bg-base-100 rounded-3xl border border-base-200">
    <div class="divide-y divide-base-200">
      {#each filteredProps as [key, value]}
        <div class="flex flex-col md:flex-row items-start md:items-center justify-between p-4 hover:bg-primary/5 transition-colors group">
          <div class="flex flex-col gap-1 mb-2 md:mb-0">
            <span class="text-[10px] font-black tracking-widest uppercase text-primary/60">{key.replace(/-/g, ' ')}</span>
            <span class="text-[9px] font-mono opacity-20 italic lowercase">{key}</span>
          </div>
          
          <div class="w-full md:w-64">
            {#if value === "true" || value === "false"}
              <!-- 
                Custom DaisyUI Boolean Selector
                Replaces native <select> to prevent OS takeover.
              -->
              <div class="join w-full">
                <button 
                  class="btn btn-xs join-item flex-1 {editedProps[key] === 'true' ? 'btn-primary' : 'btn-ghost opacity-40'}"
                  onclick={() => editedProps[key] = 'true'}
                >true</button>
                <button 
                  class="btn btn-xs join-item flex-1 {editedProps[key] === 'false' ? 'btn-primary' : 'btn-ghost opacity-40'}"
                  onclick={() => editedProps[key] = 'false'}
                >false</button>
              </div>
            {:else}
              <input 
                type="text" 
                class="input input-sm input-bordered w-full font-mono text-xs bg-base-200 border-none focus:ring-1 ring-primary/50 transition-all" 
                bind:value={editedProps[key]} 
              />
            {/if}
          </div>
        </div>
      {/each}

      {#if filteredProps.length === 0}
        <div class="p-20 text-center opacity-20">
          <Settings size={48} class="mx-auto mb-4" />
          <p class="text-xl font-black uppercase tracking-widest">No Matches</p>
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  .custom-scrollbar::-webkit-scrollbar {
    width: 4px;
  }
  .custom-scrollbar::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.05);
    border-radius: 10px;
  }
</style>
