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
    Globe2,
    Box,
    ExternalLink,
    TrendingUp
  } from "lucide-svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import ServerSettings from "../lib/components/ServerSettings.svelte";
  import ServerList from "../lib/components/ServerList.svelte";
  import TitleBar from "../lib/components/TitleBar.svelte";
  import PlayerManager from "../lib/components/PlayerManager.svelte";
  import WorldManager from "../lib/components/WorldManager.svelte";

  const appWindow = getCurrentWindow();
  let activeSubPage = $state("dashboard");
  let maxPlayers = $state(20);
  let isMaximized = $state(false);
  let commandInput = $state("");

  let normalizedCpu = $derived(serverStore.stats.cpu / (serverStore.stats.core_count || 1));

  $effect(() => {
    maxPlayers = parseInt(serverStore.properties["max-players"] || "20");
  });

  onMount(() => {
    const checkMaximized = async () => {
      isMaximized = await appWindow.isMaximized();
    };
    checkMaximized();
    const unlisten = appWindow.onResized(() => { checkMaximized(); });

    const interval = setInterval(() => {
      serverStore.refreshStats();
    }, 1000);

    return () => {
      clearInterval(interval);
      unlisten.then(fn => fn());
    };
  });

  async function handleCommand(e: KeyboardEvent) {
    if (e.key === "Enter" && commandInput.trim()) {
      await serverStore.sendCommand(commandInput);
      commandInput = "";
    }
  }

  async function openServerFolder() {
    if (serverStore.config) {
      await invoke("open_folder", { path: serverStore.config.path });
    }
  }

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

<div class="h-screen bg-base-300 flex flex-col overflow-hidden font-sans text-base-content border border-base-200 shadow-lg {isMaximized ? 'rounded-none border-none' : 'rounded-[var(--radius-box)]'}">
  <TitleBar {isMaximized} />

  <div class="flex-1 flex overflow-hidden">
    <!-- Optimized Sidebar -->
    <aside class="w-72 bg-base-100 border-r border-base-200 flex flex-col z-20 shadow-md">
      <div class="p-6 flex items-center gap-3 border-b border-base-200 bg-base-200/20">
        <div class="bg-primary text-primary-content p-2 rounded-xl shadow-sm"><Server size={20} /></div>
        <div>
          <h1 class="text-sm font-black tracking-tight leading-none uppercase italic">Roam MC</h1>
          <p class="text-[9px] uppercase font-bold opacity-50 tracking-[0.2em] mt-1 text-primary">Management Hub</p>
        </div>
      </div>

      <div class="flex-1 flex flex-col min-h-0">
        <div class="flex-1 overflow-y-auto custom-scrollbar">
          <div class="p-4 space-y-6">
            <ServerList />

            {#if serverStore.config}
              <div class="space-y-4 pt-4 border-t border-base-200">
                <h3 class="text-[10px] font-black uppercase tracking-[0.2em] opacity-60 px-2">Control Panel</h3>
                <div class="grid grid-cols-1 relative">
                  <!-- Precise Sliding Marker using integer percentages -->
                  <div 
                    class="absolute left-0 right-0 h-[42px] bg-primary rounded-xl transition-all duration-300 ease-out z-0 pointer-events-none shadow-lg shadow-primary/20"
                    style="transform: translateY({
                      activeSubPage === 'dashboard' ? '0%' : 
                      activeSubPage === 'players' ? '100%' : 
                      activeSubPage === 'worlds' ? '200%' : 
                      activeSubPage === 'mods' ? '300%' : '400%'
                    });"
                  ></div>

                  <button 
                    class="relative h-[42px] z-10 flex items-center gap-3 w-full px-4 py-2.5 rounded-xl transition-colors duration-300 {activeSubPage === 'dashboard' ? 'text-primary-content font-bold' : 'hover:bg-base-200 opacity-80'}"
                    onclick={() => activeSubPage = 'dashboard'}
                  >
                    <LayoutDashboard size={16} /> <span class="text-xs">Dashboard</span>
                  </button>
                  <button 
                    class="relative h-[42px] z-10 flex items-center gap-3 w-full px-4 py-2.5 rounded-xl transition-colors duration-300 {activeSubPage === 'players' ? 'text-primary-content font-bold' : 'hover:bg-base-200 opacity-80'}"
                    onclick={() => { activeSubPage = 'players'; serverStore.refreshPlayers(); }}
                  >
                    <Users size={16} /> <span class="text-xs">Players</span>
                  </button>
                  <button 
                    class="relative h-[42px] z-10 flex items-center gap-3 w-full px-4 py-2.5 rounded-xl transition-colors duration-300 {activeSubPage === 'worlds' ? 'text-primary-content font-bold' : 'hover:bg-base-200 opacity-80'}"
                    onclick={() => { activeSubPage = 'worlds'; serverStore.refreshWorlds(); }}
                  >
                    <Globe2 size={16} /> <span class="text-xs">Worlds</span>
                  </button>
                  <button 
                    class="relative h-[42px] z-10 flex items-center gap-3 w-full px-4 py-2.5 rounded-xl transition-colors duration-300 {activeSubPage === 'mods' ? 'text-primary-content font-bold' : 'hover:bg-base-200 opacity-80'}"
                    onclick={() => activeSubPage = 'mods'}
                  >
                    <Box size={16} /> <span class="text-xs">Mods & Plugins</span>
                  </button>
                  <button 
                    class="relative h-[42px] z-10 flex items-center gap-3 w-full px-4 py-2.5 rounded-xl transition-colors duration-300 {activeSubPage === 'settings' ? 'text-primary-content font-bold' : 'hover:bg-base-200 opacity-80'}"
                    onclick={() => activeSubPage = 'settings'}
                  >
                    <Settings2 size={16} /> <span class="text-xs">Server Settings</span>
                  </button>
                </div>
              </div>
            {/if}
          </div>
        </div>
      </div>

      {#if serverStore.config}
        <div class="p-4 border-t border-base-200 bg-base-200/10">
          <div class="flex items-center justify-between text-[10px] font-bold uppercase tracking-widest opacity-60 mb-2 px-1">
            <span>{serverStore.stats.status}</span>
            <span>{serverStore.stats.player_count}/{maxPlayers}</span>
          </div>
          <progress class="progress {getStatusColor(serverStore.stats.status).replace('badge-', 'progress-')} w-full h-1" value={serverStore.stats.status === 'Running' ? 100 : (serverStore.stats.status === 'Offline' ? 0 : 50)} max="100"></progress>
        </div>
      {/if}
    </aside>

    <!-- Main Content Area: Simplified Rendering -->
    <main class="flex-1 flex flex-col min-w-0 relative bg-base-300">
      {#if !serverStore.config}
        <div class="h-full flex flex-col items-center justify-center opacity-30 gap-4">
          <Server size={64} />
          <p class="text-xl font-black uppercase tracking-[0.2em]">Select a Server Instance</p>
        </div>
      {:else}
        <header class="h-16 bg-base-100 border-b border-base-200 px-8 flex items-center justify-between shrink-0 z-10">
          <div class="flex items-center gap-4">
            <div class="badge {getStatusColor(serverStore.stats.status)} badge-sm"></div>
            <h2 class="font-black text-lg tracking-tight uppercase italic">{serverStore.config.name}</h2>
            <div class="divider divider-horizontal mx-0 h-4 opacity-20"></div>
            <span class="text-[10px] font-bold uppercase opacity-60 tracking-widest">{activeSubPage}</span>
          </div>
          
          <div class="flex items-center gap-4">
            <div class="text-[9px] font-mono opacity-60 bg-base-200 px-3 py-1.5 rounded-lg border border-base-200 hidden md:block truncate max-w-[200px]">
              {serverStore.config.path}
            </div>
            <button class="btn btn-xs btn-ghost gap-2 opacity-60 hover:opacity-100 transition-none" onclick={openServerFolder}>
              <ExternalLink size={12}/> View Files
            </button>
          </div>
        </header>

        <!-- Page Switcher: Using hidden visibility for instant switching -->
        <div class="flex-1 relative overflow-hidden">
          
          <div class="absolute inset-0 p-8 flex flex-col gap-6 overflow-hidden {activeSubPage === 'dashboard' ? 'visible' : 'hidden'}">
            <div class="grid grid-cols-2 xl:grid-cols-5 gap-6 shrink-0">
              <div class="card bg-base-100 shadow-md border border-base-200 p-6 col-span-2 xl:col-span-2">
                <div class="flex items-center justify-between gap-6">
                  <div class="flex items-center gap-4">
                    <div class="p-4 {serverStore.stats.status === 'Running' ? 'bg-success/10 text-success' : 'bg-error/10 text-error'} rounded-2xl">
                      <Server size={32} />
                    </div>
                    <div>
                      <p class="text-[10px] font-black uppercase opacity-60 tracking-widest">Engine</p>
                      <p class="text-xl font-black">{serverStore.stats.status}</p>
                    </div>
                  </div>
                  <button 
                    class="btn btn-lg {serverStore.stats.status === 'Running' || serverStore.stats.status === 'Starting' ? 'btn-error' : 'btn-primary'} gap-3 px-8 transition-none shadow-sm"
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
                      <Play size={20} fill="currentColor" /> START
                    {/if}
                  </button>
                </div>
              </div>

                <!-- Processor Card -->
                <div class="stats shadow-md bg-base-100 border border-base-200">
                  <div class="stat p-4">
                    <div class="stat-label text-[10px] font-black uppercase opacity-60 mb-1 text-primary">Processor</div>
                    <div class="flex items-end gap-2">
                      <div class="stat-value text-2xl font-mono text-primary">{normalizedCpu.toFixed(1)}%</div>
                      <progress class="progress progress-primary w-full h-1 mb-2 opacity-40" value={normalizedCpu} max="100"></progress>
                    </div>
                  </div>
                </div>

              <div class="stats shadow-md bg-base-100 border border-base-200">
                <div class="stat p-4">
                  <div class="stat-label text-[10px] font-black uppercase opacity-60 mb-1 text-secondary">Memory (RSS)</div>
                  <div class="flex items-end gap-2">
                    <div class="stat-value text-2xl font-mono text-secondary">{(serverStore.stats.memory / 1024 / 1024).toFixed(0)}<span class="text-xs font-normal">M</span></div>
                    <progress class="progress progress-secondary w-full h-1 mb-2 opacity-40" value={serverStore.stats.memory / 1024 / 1024} max={4096}></progress>
                  </div>
                </div>
              </div>

              <div class="stats shadow-md bg-base-100 border border-base-200 col-span-2 md:col-span-1">
                <div class="stat p-4">
                  <div class="stat-label text-[10px] font-black uppercase opacity-60 mb-1 text-info">Active Players</div>
                  <div class="flex items-end gap-3">
                    <div class="stat-value text-2xl font-mono text-info">{serverStore.stats.player_count}<span class="text-sm opacity-50 font-bold">/{maxPlayers}</span></div>
                    <div class="pb-1 text-info opacity-50"><TrendingUp size={16} /></div>
                  </div>
                </div>
              </div>
            </div>

            <div class="flex-1 bg-neutral rounded-3xl shadow-lg overflow-hidden flex flex-col border border-white/5 relative group text-neutral-content">
              <div class="bg-base-100/20 p-4 px-8 flex justify-between items-center border-b border-white/5">
                <div class="flex items-center gap-4">
                  <div class="flex gap-1.5">
                    <div class="w-2 h-2 rounded-full bg-error/60"></div>
                    <div class="w-2 h-2 rounded-full bg-warning/60"></div>
                    <div class="w-2 h-2 rounded-full bg-success/60"></div>
                  </div>
                  <span class="text-[10px] font-black tracking-[0.3em] opacity-50 uppercase ml-2">Console</span>
                </div>
                <button class="btn btn-ghost btn-xs opacity-40 hover:opacity-100 hover:bg-white/5 rounded-lg px-4 transition-none text-[10px] font-bold" onclick={() => serverStore.logs = []}>CLEAR BUFFER</button>
              </div>
              <div class="flex-1 p-8 overflow-y-auto font-mono text-[11px] flex flex-col-reverse gap-2 select-text custom-scrollbar will-change-scroll">
                {#each serverStore.logs.slice().reverse() as log}
                  <div class="flex gap-6 group/line transition-none -mx-8 px-8 py-0.5 border-l-2 border-transparent hover:border-primary/40">
                    <span class="opacity-30 select-none shrink-0 font-bold w-20">{new Date().toLocaleTimeString([], {hour12: false})}</span>
                    <span class="text-neutral-content leading-relaxed whitespace-pre-wrap">{log}</span>
                  </div>
                {/each}
              </div>
              <div class="bg-base-100/10 border-t border-white/5 p-4 px-8 flex items-center gap-4 group-focus-within:bg-base-100/20 transition-colors">
                <span class="text-primary font-black opacity-80">&gt;</span>
                <input 
                  type="text" 
                  placeholder={serverStore.stats.status === 'Running' ? "Send command to server..." : "Server must be running to send commands"}
                  class="bg-transparent border-none outline-none flex-1 font-mono text-xs text-neutral-content placeholder:opacity-40 disabled:cursor-not-allowed"
                  bind:value={commandInput}
                  onkeydown={handleCommand}
                  disabled={serverStore.stats.status !== 'Running'}
                />
                <div class="flex gap-2 opacity-40 group-focus-within:opacity-80 transition-opacity"><kbd class="kbd kbd-xs bg-neutral-900 border-none">ENTER</kbd></div>
              </div>
            </div>
          </div>

          <div class="absolute inset-0 p-8 overflow-hidden {activeSubPage === 'settings' ? 'visible' : 'hidden'}">
            <div class="max-w-5xl mx-auto h-full overflow-y-auto custom-scrollbar pr-4"><ServerSettings /></div>
          </div>

          <div class="absolute inset-0 p-8 overflow-hidden {activeSubPage === 'players' ? 'visible' : 'hidden'}">
            <div class="max-w-6xl mx-auto h-full overflow-y-auto custom-scrollbar pr-4"><PlayerManager /></div>
          </div>

          <div class="absolute inset-0 p-8 overflow-hidden {activeSubPage === 'worlds' ? 'visible' : 'hidden'}">
            <div class="max-w-6xl mx-auto h-full overflow-y-auto custom-scrollbar pr-4"><WorldManager /></div>
          </div>

          <div class="absolute inset-0 p-8 flex flex-col items-center justify-center opacity-30 gap-6 {activeSubPage === 'mods' ? 'visible' : 'hidden'}">
            <Box size={80} /><p class="text-2xl font-black uppercase tracking-[0.3em]">Mods & Plugins Module</p>
            <div class="badge badge-outline badge-lg p-4 font-bold opacity-50 italic text-primary">Development In Progress</div>
          </div>

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
    background: rgba(255, 255, 255, 0.1);
    border-radius: 10px;
  }
  .custom-scrollbar::-webkit-scrollbar-thumb:hover {
    background: rgba(255, 255, 255, 0.2);
  }
  .will-change-scroll {
    will-change: scroll-position;
  }
</style>
