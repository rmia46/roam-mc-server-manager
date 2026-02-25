<script lang="ts">
  import { serverStore } from "../lib/server-store.svelte";
  import { onMount } from "svelte";
  import { 
    LayoutDashboard, 
    Settings2, 
    Terminal, 
    Play, 
    Square, 
    Users, 
    Cpu, 
    HardDrive,
    Activity,
    ChevronRight,
    Server,
    Loader2
  } from "lucide-svelte";
  import PropertiesManager from "../lib/components/PropertiesManager.svelte";
  import ServerList from "../lib/components/ServerList.svelte";
  import TitleBar from "../lib/components/TitleBar.svelte";

  let activePage = $state("dashboard");
  let maxPlayers = $state(20);

  $effect(() => {
    maxPlayers = parseInt(serverStore.properties["max-players"] || "20");
  });

  onMount(() => {
    const interval = setInterval(() => {
      serverStore.refreshStats();
    }, 1000);
    return () => clearInterval(interval);
  });

  const getStatusColor = (status: string) => {
    switch (status) {
      case "Running": return "badge-success";
      case "Starting": return "badge-warning";
      case "Stopping": return "badge-warning";
      case "Offline": return "badge-error";
      default: return "badge-ghost";
    }
  };
</script>

<div class="h-screen bg-base-300 flex flex-col overflow-hidden font-sans text-base-content border border-base-200 shadow-2xl">
  <!-- Custom Title Bar -->
  <TitleBar />

  <div class="flex-1 flex overflow-hidden">
    <!-- Left Sidebar Navigation -->
    <aside class="w-72 bg-base-100 border-r border-base-200 flex flex-col z-20">
      <div class="p-6 flex items-center gap-3 border-b border-base-200">
        <div class="bg-primary text-primary-content p-2 rounded-xl shadow-lg ring-4 ring-primary/10">
          <Server size={24} />
        </div>
        <div>
          <h1 class="text-lg font-black tracking-tight leading-none">Roam MC</h1>
          <p class="text-[10px] uppercase font-bold opacity-40 tracking-widest mt-1">Manager v1.0</p>
        </div>
      </div>

      <!-- Page Navigation -->
      <nav class="p-4 space-y-1">
        <button 
          class="flex items-center justify-between w-full px-4 py-2.5 rounded-xl transition-all duration-200 
                 {activePage === 'dashboard' ? 'bg-primary text-primary-content shadow-lg shadow-primary/20' : 'hover:bg-base-200 opacity-70'}"
          onclick={() => activePage = 'dashboard'}
        >
          <div class="flex items-center gap-3">
            <LayoutDashboard size={18} />
            <span class="font-bold text-sm">Dashboard</span>
          </div>
        </button>

        <button 
          class="flex items-center justify-between w-full px-4 py-2.5 rounded-xl transition-all duration-200 
                 {activePage === 'settings' ? 'bg-primary text-primary-content shadow-lg shadow-primary/20' : 'hover:bg-base-200 opacity-70'}"
          onclick={() => activePage = 'settings'}
        >
          <div class="flex items-center gap-3">
            <Settings2 size={18} />
            <span class="font-bold text-sm">Properties</span>
          </div>
        </button>
      </nav>

      <div class="divider px-6 my-1 opacity-10"></div>

      <!-- Server List Section -->
      <div class="flex-1 overflow-y-auto p-4 custom-scrollbar">
        <ServerList />
      </div>

      <!-- Quick Status Bottom -->
      <div class="p-4 bg-base-200/50 m-4 rounded-2xl border border-base-200">
        <div class="flex items-center justify-between mb-3">
          <span class="text-[10px] font-bold uppercase opacity-40 tracking-wider">Status</span>
          <div class="badge {getStatusColor(serverStore.stats.status)} badge-xs"></div>
        </div>
        <button 
          class="btn btn-sm w-full {serverStore.stats.status === 'Running' || serverStore.stats.status === 'Starting' ? 'btn-error' : 'btn-primary'} gap-2 shadow-md border-none"
          onclick={() => serverStore.toggleServer()}
          disabled={!serverStore.config || serverStore.isDownloading || serverStore.stats.status === 'Stopping'}
        >
          {#if serverStore.stats.status === 'Starting'}
            <Loader2 size={12} class="animate-spin" /> Starting...
          {:else if serverStore.stats.status === 'Stopping'}
            <Loader2 size={12} class="animate-spin" /> Stopping...
          {:else if serverStore.stats.status === 'Running'}
            <Square size={12} fill="currentColor" /> Stop
          {:else}
            <Play size={12} fill="currentColor" /> Start Server
          {/if}
        </button>
      </div>
    </aside>

    <!-- Main Content Area -->
    <main class="flex-1 flex flex-col min-w-0 relative">
      <header class="h-16 bg-base-100/50 backdrop-blur-md border-b border-base-200 px-8 flex items-center justify-between shrink-0 z-10">
        <div class="flex items-center gap-4">
          <Activity size={18} class="text-primary" />
          <h2 class="font-bold text-sm uppercase tracking-widest opacity-60">
            {serverStore.config?.name || 'Select a Server'}
          </h2>
        </div>
        <div class="flex items-center gap-3">
          {#if serverStore.config}
            <div class="badge {getStatusColor(serverStore.stats.status)} badge-outline font-black text-[10px] tracking-widest">
              {serverStore.stats.status.toUpperCase()}
            </div>
            <div class="text-[10px] font-mono opacity-40 bg-base-200 px-3 py-1 rounded-full border border-base-300">
              {serverStore.config.jar_name}
            </div>
          {/if}
        </div>
      </header>

      <div class="flex-1 overflow-hidden">
        {#if !serverStore.config}
          <div class="h-full flex flex-col items-center justify-center opacity-20 gap-4">
            <Server size={64} />
            <p class="text-xl font-black uppercase tracking-[0.2em]">Select a Server from Sidebar</p>
          </div>
        {:else if activePage === "dashboard"}
          <div class="h-full p-8 flex flex-col gap-6">
            <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
              <div class="stats shadow-xl bg-base-100 border border-base-200">
                <div class="stat">
                  <div class="stat-figure text-info opacity-30"><Users size={32} /></div>
                  <div class="stat-label text-[10px] font-bold uppercase opacity-50 mb-1">Active Players</div>
                  <div class="stat-value font-mono text-3xl">{serverStore.stats.player_count}</div>
                  <div class="stat-desc font-bold opacity-40">Max: {maxPlayers}</div>
                </div>
              </div>

              <div class="stats shadow-xl bg-base-100 border border-base-200">
                <div class="stat">
                  <div class="stat-figure text-primary opacity-30"><Cpu size={32} /></div>
                  <div class="stat-label text-[10px] font-bold uppercase opacity-50 mb-1">CPU Utilization</div>
                  <div class="stat-value font-mono text-3xl text-primary">{serverStore.stats.cpu.toFixed(0)}%</div>
                  <progress class="progress progress-primary w-full h-1 mt-2 opacity-30" value={serverStore.stats.cpu} max="100"></progress>
                </div>
              </div>

              <div class="stats shadow-xl bg-base-100 border border-base-200">
                <div class="stat">
                  <div class="stat-figure text-secondary opacity-30"><HardDrive size={32} /></div>
                  <div class="stat-label text-[10px] font-bold uppercase opacity-50 mb-1">RAM Usage</div>
                  <div class="stat-value font-mono text-3xl text-secondary">{(serverStore.stats.memory / 1024 / 1024).toFixed(0)}<span class="text-sm font-normal opacity-50 ml-1">MB</span></div>
                  <progress class="progress progress-secondary w-full h-1 mt-2 opacity-30" value={serverStore.stats.memory / 1024 / 1024} max={4096}></progress>
                </div>
              </div>
            </div>

            <div class="flex-1 bg-neutral rounded-3xl shadow-2xl overflow-hidden flex flex-col border border-white/5">
              <div class="bg-base-100/10 p-4 flex justify-between items-center border-b border-white/5">
                <div class="flex items-center gap-3">
                  <div class="w-2 h-2 rounded-full {serverStore.stats.status === 'Running' ? 'bg-success animate-ping' : 'bg-error'}"></div>
                  <span class="text-[10px] font-black tracking-[0.2em] opacity-50">CONSOLE_FEED</span>
                </div>
                <button class="btn btn-ghost btn-xs opacity-30 hover:opacity-100" onclick={() => serverStore.logs = []}>Clear</button>
              </div>
              <div class="flex-1 p-6 overflow-y-auto font-mono text-xs flex flex-col-reverse gap-2 select-text custom-scrollbar">
                {#each serverStore.logs.slice().reverse() as log}
                  <div class="flex gap-4 border-l border-white/10 pl-4 py-0.5">
                    <span class="opacity-20 select-none shrink-0">{new Date().toLocaleTimeString([], {hour12: false})}</span>
                    <span class="text-neutral-content/90 leading-relaxed whitespace-pre-wrap">{log}</span>
                  </div>
                {/each}
              </div>
            </div>
          </div>
        {:else if activePage === "settings"}
          <div class="h-full p-8 overflow-hidden">
            <div class="max-w-4xl mx-auto h-full flex flex-col gap-6">
              <h3 class="text-lg font-black uppercase tracking-widest opacity-40">Server Properties Editor</h3>
              <div class="flex-1 overflow-hidden">
                <PropertiesManager />
              </div>
            </div>
          </div>
        {/if}
      </div>
    </main>
  </div>
</div>

<style>
  :global(body) {
    background-color: transparent;
  }
  .custom-scrollbar::-webkit-scrollbar {
    width: 4px;
  }
  .custom-scrollbar::-webkit-scrollbar-track {
    background: transparent;
  }
  .custom-scrollbar::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.1);
    border-radius: 10px;
  }
</style>
