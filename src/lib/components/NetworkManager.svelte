<script lang="ts">
  import { serverStore } from "../server-store.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { Globe, ExternalLink, Wifi, FolderSearch, Terminal, Search, Info, CheckCircle2, CloudSync, Share2 } from "lucide-svelte";
  import { open } from "@tauri-apps/plugin-dialog";

  let activeTab = $state("tunnel"); // "tunnel", "sync"
  let provider = $state<"none" | "playit" | "ngrok">("none");
  let binaryPath = $state("");
  let isSearching = $state(false);
  let saveState = $state<"idle" | "saved">("idle");

  $effect(() => {
    if (serverStore.config?.tunnel) {
      provider = serverStore.config.tunnel.provider as any;
      binaryPath = (serverStore.config.tunnel as any).binary_path || "";
    } else {
      provider = "none";
      binaryPath = "";
    }
  });

  async function autoDetect() {
    if (provider === "none") return;
    isSearching = true;
    try {
      const path = await invoke("find_binary", { name: provider });
      if (path) binaryPath = path as string;
      else alert(`${provider} not found in system PATH. Please select manually.`);
    } finally {
      isSearching = false;
    }
  }

  async function selectBinary() {
    const selected = await open({ multiple: false, title: `Select ${provider} Executable` });
    if (selected) binaryPath = selected as string;
  }

  async function handleSave() {
    if (!serverStore.config) return;
    try {
      const updatedTunnel = { provider, token: "", binary_path: binaryPath, public_address: "" };
      const updatedConfig = JSON.parse(JSON.stringify(serverStore.config));
      updatedConfig.tunnel = updatedTunnel;
      
      const index = serverStore.servers.findIndex(s => s.path === serverStore.config?.path);
      if (index !== -1) {
        serverStore.servers[index] = updatedConfig;
        serverStore.config = updatedConfig;
        serverStore.saveServers();
        await invoke("set_server_config", { config: updatedConfig });
      }
      saveState = "saved";
      setTimeout(() => { saveState = "idle"; }, 2000);
    } catch (err) {
      console.error("Save failed:", err);
    }
  }
</script>

<div class="h-full flex flex-col gap-6">
  <!-- Precise Sliding Tabs -->
  <div class="grid grid-cols-2 w-64 bg-base-100 p-1 rounded-2xl border border-base-200 shadow-sm relative self-start overflow-hidden">
    <div 
      class="absolute top-1 bottom-1 bg-primary rounded-xl transition-all duration-300 ease-out z-0"
      style="width: calc(50% - 2px); transform: translateX({activeTab === 'tunnel' ? '2px' : 'calc(100% + 2px)'});"
    ></div>

    <button 
      class="relative z-10 py-2 text-[10px] font-black uppercase tracking-widest transition-colors duration-300 {activeTab === 'tunnel' ? 'text-primary-content' : 'opacity-40 hover:opacity-100'}"
      onclick={() => activeTab = 'tunnel'}
    >
      Tunneling
    </button>
    <button 
      class="relative z-10 py-2 text-[10px] font-black uppercase tracking-widest transition-colors duration-300 {activeTab === 'sync' ? 'text-primary-content' : 'opacity-40 hover:opacity-100'}"
      onclick={() => activeTab = 'sync'}
    >
      Cloud Sync
    </button>
  </div>

  <!-- Tab Content -->
  <div class="flex-1 min-h-0 bg-base-100 rounded-3xl border border-base-200 p-8 shadow-sm">
    {#if activeTab === "tunnel"}
      <div class="h-full flex flex-col gap-8 animate-in fade-in duration-200">
        <div class="flex justify-between items-start">
          <div class="space-y-2">
            <h3 class="text-2xl font-black tracking-tight italic uppercase">Tunnel Bridge</h3>
            <p class="text-[10px] opacity-40 uppercase font-bold tracking-widest">External Connectivity</p>
          </div>
          
          {#if serverStore.stats.tunnel_status === "Online"}
            <div class="badge badge-success gap-2 py-3 px-4 font-black text-[10px] uppercase tracking-widest shadow-lg shadow-success/20">
              <Wifi size={12} /> Link Active
            </div>
          {/if}
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
          <!-- Setup Config -->
          <div class="space-y-6">
            <div class="form-control">
              <label class="label" for="provider-select"><span class="label-text text-[10px] font-black uppercase opacity-40">Select Engine</span></label>
              <div id="provider-select" class="join w-full bg-base-200 p-1 rounded-xl">
                <button class="btn btn-xs join-item flex-1 {provider === 'none' ? 'btn-primary' : 'btn-ghost opacity-40'}" onclick={() => provider = "none"}>None</button>
                <button class="btn btn-xs join-item flex-1 {provider === 'playit' ? 'btn-primary' : 'btn-ghost opacity-40'}" onclick={() => provider = "playit"}>Playit</button>
                <button class="btn btn-xs join-item flex-1 {provider === 'ngrok' ? 'btn-primary' : 'btn-ghost opacity-40'}" onclick={() => provider = "ngrok"}>Ngrok</button>
              </div>
            </div>

            {#if provider !== "none"}
              <div class="form-control animate-in slide-in-from-top-2 duration-300">
                <div class="flex justify-between items-center mb-1">
                  <label class="label p-0" for="path-input"><span class="label-text text-[10px] font-black uppercase opacity-40">Binary Location</span></label>
                  <button class="btn btn-xs btn-ghost gap-1 text-[9px] font-bold text-primary" onclick={autoDetect} disabled={isSearching}>
                    {#if isSearching}<span class="loading loading-spinner loading-xs"></span>{:else}<Search size={10} />{/if}
                    DETECT
                  </button>
                </div>
                <div class="join">
                  <input id="path-input" type="text" placeholder={`Path to ${provider}...`} class="input input-bordered input-sm join-item flex-1 bg-base-200 border-none text-xs font-mono" bind:value={binaryPath} />
                  <button class="btn btn-sm join-item btn-primary" onclick={selectBinary}><FolderSearch size={14} /></button>
                </div>
              </div>
            {/if}

            <button 
              class="btn btn-primary btn-block shadow-lg transition-all duration-300 border-none
                     {saveState === 'saved' ? 'bg-success text-success-content scale-[1.02]' : ''}" 
              onclick={handleSave} 
              disabled={provider !== 'none' && !binaryPath}
            >
              {#if saveState === "saved"}<CheckCircle2 size={16} class="animate-bounce" /> SUCCESS{:else}<Wifi size={16} /> SAVE BRIDGE CONFIG{/if}
            </button>
          </div>

          <!-- Guide -->
          <div class="bg-base-200/30 p-6 rounded-3xl space-y-4 border border-base-200">
            <h4 class="text-[10px] font-black uppercase tracking-widest opacity-40">Bridge Documentation</h4>
            <div class="space-y-4">
              <div class="flex gap-3">
                <div class="p-2 bg-primary/10 rounded-lg text-primary h-fit"><Info size={14} /></div>
                <p class="text-[10px] opacity-60 leading-relaxed">The manager will automatically launch your tunnel alongside the server and extract the public address from logs.</p>
              </div>
              <div class="divider opacity-5"></div>
              <p class="text-[9px] opacity-30 italic">Make sure you've authorized the binary in your terminal at least once before using it here.</p>
            </div>
          </div>
        </div>
      </div>

    {:else if activeTab === "sync"}
      <div class="h-full flex flex-col items-center justify-center opacity-30 gap-6 animate-in fade-in duration-200">
        <Share2 size={80} />
        <div class="text-center space-y-2">
          <h3 class="text-2xl font-black uppercase tracking-[0.3em]">Cloud Synchronization</h3>
          <p class="text-xs font-bold uppercase tracking-widest text-primary">Incoming Feature</p>
        </div>
        <div class="max-w-md bg-base-200 p-6 rounded-3xl border border-base-100 text-center">
          <p class="text-[10px] leading-relaxed uppercase tracking-wider font-medium">
            Sync your worlds across MEGA, Dropbox, or OneDrive. 
            Includes a <span class="text-primary font-black">Lock System</span> to prevent world corruption when multiple friends host.
          </p>
        </div>
      </div>
    {/if}
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
