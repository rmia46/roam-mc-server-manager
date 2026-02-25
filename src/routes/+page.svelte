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
    Loader2,
    Beaker,
    Globe2,
    Box,
    ExternalLink,
    ShieldAlert
  } from "lucide-svelte";
  import ServerSettings from "../lib/components/ServerSettings.svelte";
  import ServerList from "../lib/components/ServerList.svelte";
  import TitleBar from "../lib/components/TitleBar.svelte";
  import Lab from "../lib/components/Lab.svelte";

  let activeSubPage = $state("dashboard"); // "dashboard", "players", "worlds", "mods", "settings"
  let globalPage = $state("manager"); // "manager", "lab"
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

<div class="h-screen bg-base-300 flex flex-col overflow-hidden font-sans text-base-content border border-base-200 shadow-2xl rounded-[var(--radius-box)]">
  <TitleBar />

  <div class="flex-1 flex overflow-hidden">
    <!-- Navigation Sidebar -->
    <aside class="w-72 bg-base-100 border-r border-base-200 flex flex-col z-20">
      <!-- App Header -->
      <div class="p-6 flex items-center gap-3 border-b border-base-200 bg-base-200/30">
        <div class="bg-primary text-primary-content p-2 rounded-xl shadow-lg ring-4 ring-primary/10">
          <Server size={20} />
        </div>
        <div>
          <h1 class="text-sm font-black tracking-tight leading-none uppercase italic">Roam MC</h1>
          <p class="text-[9px] uppercase font-bold opacity-30 tracking-[0.2em] mt-1">Management Hub</p>
        </div>
      </div>

      <div class="flex-1 flex flex-col min-h-0">
        <!-- Global Nav -->
        <nav class="p-4 space-y-1">
          <button 
            class="flex items-center justify-between w-full px-4 py-2 rounded-xl transition-all duration-200 
                   {globalPage === 'manager' ? 'bg-primary/10 text-primary font-bold border border-primary/20' : 'hover:bg-base-200 opacity-60'}"
            onclick={() => globalPage = 'manager'}
          >
            <div class="flex items-center gap-3"><Activity size={16} /> <span class="text-xs uppercase tracking-wider">Manager</span></div>
          </button>
          <button 
            class="flex items-center justify-between w-full px-4 py-2 rounded-xl transition-all duration-200 
                   {globalPage === 'lab' ? 'bg-primary/10 text-primary font-bold border border-primary/20' : 'hover:bg-base-200 opacity-60'}"
            onclick={() => globalPage = 'lab'}
          >
            <div class="flex items-center gap-3"><Beaker size={16} /> <span class="text-xs uppercase tracking-wider">Theme Lab</span></div>
          </button>
        </nav>

        <div class="divider px-6 my-0 opacity-10"></div>

        <!-- Dynamic Content: Server Selection or Server Functions -->
        <div class="flex-1 overflow-y-auto custom-scrollbar">
          {#if globalPage === 'manager'}
            <div class="p-4 space-y-6">
              <!-- Server Selector -->
              <ServerList />

              {#if serverStore.config}
                <!-- Server Context Nav -->
                <div class="space-y-4 pt-4 border-t border-base-200">
                  <h3 class="text-[10px] font-black uppercase tracking-[0.2em] opacity-40 px-2">Control Panel</h3>
                  <div class="space-y-1">
                    <button 
                      class="flex items-center gap-3 w-full px-4 py-2.5 rounded-xl transition-all {activeSubPage === 'dashboard' ? 'bg-primary text-primary-content font-bold shadow-lg shadow-primary/20' : 'hover:bg-base-200 opacity-70'}"
                      onclick={() => activeSubPage = 'dashboard'}
                    >
                      <LayoutDashboard size={16} /> <span class="text-xs">Dashboard</span>
                    </button>
                    <button 
                      class="flex items-center gap-3 w-full px-4 py-2.5 rounded-xl transition-all {activeSubPage === 'players' ? 'bg-primary text-primary-content font-bold shadow-lg shadow-primary/20' : 'hover:bg-base-200 opacity-70'}"
                      onclick={() => activeSubPage = 'players'}
                    >
                      <Users size={16} /> <span class="text-xs">Players</span>
                    </button>
                    <button 
                      class="flex items-center gap-3 w-full px-4 py-2.5 rounded-xl transition-all {activeSubPage === 'worlds' ? 'bg-primary text-primary-content font-bold shadow-lg shadow-primary/20' : 'hover:bg-base-200 opacity-70'}"
                      onclick={() => activeSubPage = 'worlds'}
                    >
                      <Globe2 size={16} /> <span class="text-xs">Worlds</span>
                    </button>
                    <button 
                      class="flex items-center gap-3 w-full px-4 py-2.5 rounded-xl transition-all {activeSubPage === 'mods' ? 'bg-primary text-primary-content font-bold shadow-lg shadow-primary/20' : 'hover:bg-base-200 opacity-70'}"
                      onclick={() => activeSubPage = 'mods'}
                    >
                      <Box size={16} /> <span class="text-xs">Mods & Plugins</span>
                    </button>
                    <button 
                      class="flex items-center gap-3 w-full px-4 py-2.5 rounded-xl transition-all {activeSubPage === 'settings' ? 'bg-primary text-primary-content font-bold shadow-lg shadow-primary/20' : 'hover:bg-base-200 opacity-70'}"
                      onclick={() => activeSubPage = 'settings'}
                    >
                      <Settings2 size={16} /> <span class="text-xs">Server Settings</span>
                    </button>
                  </div>
                </div>
              {/if}
            </div>
          {/if}
        </div>
      </div>

      <!-- Compact Footer Status -->
      {#if serverStore.config}
        <div class="p-4 border-t border-base-200 bg-base-200/20">
          <div class="flex items-center justify-between text-[10px] font-bold uppercase tracking-widest opacity-40 mb-2 px-1">
            <span>{serverStore.stats.status}</span>
            <span>{serverStore.stats.player_count}/{maxPlayers}</span>
          </div>
          <progress class="progress {getStatusColor(serverStore.stats.status).replace('badge-', 'progress-')} w-full h-1" value={serverStore.stats.status === 'Running' ? 100 : (serverStore.stats.status === 'Offline' ? 0 : 50)} max="100"></progress>
        </div>
      {/if}
    </aside>

    <!-- Main Content Area -->
    <main class="flex-1 flex flex-col min-w-0 relative bg-base-300">
      {#if globalPage === "lab"}
        <header class="h-16 bg-base-100/50 backdrop-blur-md border-b border-base-200 px-8 flex items-center justify-between shrink-0 z-10">
          <div class="flex items-center gap-4"><Beaker size={18} class="text-primary" /> <h2 class="font-bold text-sm uppercase tracking-widest opacity-60">Theme Laboratory</h2></div>
        </header>
        <div class="flex-1 overflow-y-auto p-8 custom-scrollbar"><Lab /></div>
      {:else if !serverStore.config}
        <div class="h-full flex flex-col items-center justify-center opacity-20 gap-4">
          <Server size={64} />
          <p class="text-xl font-black uppercase tracking-[0.2em]">Select a Server Instance</p>
        </div>
      {:else}
        <!-- Server Specific Header -->
        <header class="h-16 bg-base-100/50 backdrop-blur-md border-b border-base-200 px-8 flex items-center justify-between shrink-0 z-10">
          <div class="flex items-center gap-4">
            <div class="badge {getStatusColor(serverStore.stats.status)} badge-sm animate-pulse"></div>
            <h2 class="font-black text-lg tracking-tight uppercase italic">{serverStore.config.name}</h2>
            <div class="divider divider-horizontal mx-0 h-4 opacity-20"></div>
            <span class="text-[10px] font-bold uppercase opacity-40 tracking-widest">{activeSubPage}</span>
          </div>
          
          <div class="flex items-center gap-4">
            <div class="text-[9px] font-mono opacity-30 bg-base-200 px-3 py-1.5 rounded-lg border border-base-200 hidden md:block">
              {serverStore.config.path}
            </div>
            <button class="btn btn-xs btn-ghost gap-2 opacity-40 hover:opacity-100"><ExternalLink size={12}/> View Files</button>
          </div>
        </header>

        <div class="flex-1 overflow-hidden relative">
          {#if activeSubPage === "dashboard"}
            <div class="h-full p-8 flex flex-col gap-6">
              <!-- Dashboard Control Row -->
              <div class="grid grid-cols-1 xl:grid-cols-4 gap-6 shrink-0">
                <!-- Main Control Card -->
                <div class="card bg-base-100 shadow-xl border border-base-200 p-6 xl:col-span-2">
                  <div class="flex items-center justify-between gap-6">
                    <div class="flex items-center gap-4">
                      <div class="p-4 {serverStore.stats.status === 'Running' ? 'bg-success/10 text-success' : 'bg-error/10 text-error'} rounded-2xl shadow-inner">
                        <Server size={32} />
                      </div>
                      <div>
                        <p class="text-[10px] font-black uppercase opacity-40 tracking-widest">Instance Engine</p>
                        <p class="text-xl font-black">{serverStore.stats.status}</p>
                      </div>
                    </div>
                    <div class="join shadow-lg ring-4 ring-base-200/50">
                      <button 
                        class="btn btn-lg join-item {serverStore.stats.status === 'Running' || serverStore.stats.status === 'Starting' ? 'btn-error' : 'btn-primary'} gap-3 px-8"
                        onclick={() => serverStore.toggleServer()}
                        disabled={serverStore.stats.status === 'Stopping'}
                      >
                        {#if serverStore.stats.status === 'Starting'}
                          <Loader2 size={20} class="animate-spin" /> STARTING
                        {:else if serverStore.stats.status === 'Stopping'}
                          <Loader2 size={20} class="animate-spin" /> STOPPING
                        {:else if serverStore.stats.status === 'Running'}
                          <Square size={20} fill="currentColor" /> STOP
                        {:else}
                          <Play size={20} fill="currentColor" /> START SERVER
                        {/if}
                      </button>
                    </div>
                  </div>
                </div>

                <!-- Metrics -->
                <div class="stats shadow-xl bg-base-100 border border-base-200">
                  <div class="stat p-4">
                    <div class="stat-label text-[10px] font-black uppercase opacity-40 mb-1">Processor</div>
                    <div class="flex items-end gap-2">
                      <div class="stat-value text-2xl font-mono text-primary">{serverStore.stats.cpu.toFixed(0)}%</div>
                      <progress class="progress progress-primary w-full h-1 mb-2 opacity-20" value={serverStore.stats.cpu} max="100"></progress>
                    </div>
                  </div>
                </div>
                <div class="stats shadow-xl bg-base-100 border border-base-200">
                  <div class="stat p-4">
                    <div class="stat-label text-[10px] font-black uppercase opacity-40 mb-1">Memory (RSS)</div>
                    <div class="flex items-end gap-2">
                      <div class="stat-value text-2xl font-mono text-secondary">{(serverStore.stats.memory / 1024 / 1024).toFixed(0)}<span class="text-xs font-normal">M</span></div>
                      <progress class="progress progress-secondary w-full h-1 mb-2 opacity-20" value={serverStore.stats.memory / 1024 / 1024} max={4096}></progress>
                    </div>
                  </div>
                </div>
              </div>

              <!-- Console Section -->
              <div class="flex-1 bg-neutral rounded-[2.5rem] shadow-2xl overflow-hidden flex flex-col border border-white/5 relative group">
                <div class="bg-base-100/20 p-4 px-8 flex justify-between items-center border-b border-white/5 backdrop-blur-md">
                  <div class="flex items-center gap-4">
                    <div class="flex gap-1.5">
                      <div class="w-2.5 h-2.5 rounded-full bg-error/40 border border-error/20"></div>
                      <div class="w-2.5 h-2.5 rounded-full bg-warning/40 border border-warning/20"></div>
                      <div class="w-2.5 h-2.5 rounded-full bg-success/40 border border-success/20"></div>
                    </div>
                    <span class="text-[10px] font-black tracking-[0.3em] opacity-30 uppercase ml-2">Secure_Shell_Output</span>
                  </div>
                  <button class="btn btn-ghost btn-xs opacity-20 hover:opacity-100 hover:bg-white/5 rounded-lg px-4" onclick={() => serverStore.logs = []}>Clear Buffer</button>
                </div>
                <div class="flex-1 p-8 overflow-y-auto font-mono text-[11px] flex flex-col-reverse gap-2 select-text custom-scrollbar">
                  {#each serverStore.logs.slice().reverse() as log}
                    <div class="flex gap-6 group/line hover:bg-white/5 transition-colors -mx-8 px-8 py-0.5 border-l-2 border-transparent hover:border-primary/40">
                      <span class="opacity-10 select-none shrink-0 font-bold w-20">{new Date().toLocaleTimeString([], {hour12: false, fractionalSecondDigits: 1})}</span>
                      <span class="text-neutral-content/80 leading-relaxed whitespace-pre-wrap">{log}</span>
                    </div>
                  {/each}
                </div>
              </div>
            </div>
          {:else if activeSubPage === "settings"}
            <div class="h-full p-8 overflow-hidden">
              <div class="max-w-5xl mx-auto h-full overflow-y-auto custom-scrollbar pr-4">
                <ServerSettings />
              </div>
            </div>
          {:else}
            <!-- Placeholder for other subpages -->
            <div class="h-full flex flex-col items-center justify-center opacity-10 gap-6">
              {#if activeSubPage === 'players'}<Users size={80} />{/if}
              {#if activeSubPage === 'worlds'}<Globe2 size={80} />{/if}
              {#if activeSubPage === 'mods'}<Box size={80} />{/if}
              <p class="text-2xl font-black uppercase tracking-[0.3em]">{activeSubPage} Module</p>
              <div class="badge badge-outline badge-lg p-4 font-bold opacity-50 italic">Development In Progress</div>
            </div>
          {/if}
        </div>
      {/if}
    </main>
  </div>
</div>

<style>
  :global(body) {
    background-color: transparent;
  }
  .custom-scrollbar::-webkit-scrollbar {
    width: 4px;
    height: 4px;
  }
  .custom-scrollbar::-webkit-scrollbar-track {
    background: transparent;
  }
  .custom-scrollbar::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.05);
    border-radius: 10px;
  }
  .custom-scrollbar::-webkit-scrollbar-thumb:hover {
    background: rgba(255, 255, 255, 0.15);
  }
</style>
