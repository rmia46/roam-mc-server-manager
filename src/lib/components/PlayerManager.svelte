<script lang="ts">
  import { serverStore } from "../server-store.svelte";
  import { onMount } from "svelte";
  import { Users, Clock, Footprints, RefreshCw, Trophy, UserCircle2 } from "lucide-svelte";

  async function refresh() {
    await serverStore.refreshPlayers();
  }

  onMount(() => {
    refresh();
  });
</script>

<div class="h-full flex flex-col gap-6">
  <!-- Header -->
  <div class="flex justify-between items-center bg-base-100 p-4 rounded-2xl border border-base-200 shadow-sm">
    <div class="flex items-center gap-3">
      <div class="p-2 bg-info/10 rounded-lg text-info"><Users size={18} /></div>
      <div>
        <h3 class="font-black text-sm uppercase tracking-widest">Player Analytics</h3>
        <p class="text-[10px] opacity-40">Lifetime statistics from server records</p>
      </div>
    </div>
    <button class="btn btn-sm btn-ghost gap-2 opacity-40 hover:opacity-100" onclick={refresh}>
      <RefreshCw size={14} /> Refresh Records
    </button>
  </div>

  <!-- Player Grid/List -->
  <div class="flex-1 overflow-y-auto pr-2 custom-scrollbar">
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      {#each serverStore.players as player}
        <div class="card bg-base-100 border border-base-200 shadow-md hover:border-primary/30 transition-colors group">
          <div class="card-body p-5">
            <div class="flex items-start justify-between">
              <div class="flex items-center gap-4">
                <!-- Avatar (Using Minotar) -->
                <div class="avatar shadow-inner bg-base-200 p-1 rounded-xl">
                  <div class="w-12 h-12 rounded-lg">
                    <img 
                      src="https://minotar.net/helm/{player.name}/48.png" 
                      alt={player.name}
                      onerror={(e) => e.currentTarget.src = "https://minotar.net/helm/Steve/48.png"}
                    />
                  </div>
                </div>
                <div>
                  <h4 class="font-black text-lg tracking-tight group-hover:text-primary transition-colors">{player.name}</h4>
                  <p class="text-[9px] font-mono opacity-20 uppercase truncate w-32">{player.uuid.split('-')[0]}...</p>
                </div>
              </div>
              <div class="p-2 bg-warning/5 rounded-lg text-warning/40"><Trophy size={16} /></div>
            </div>

            <div class="grid grid-cols-2 gap-4 mt-6">
              <div class="flex flex-col gap-1">
                <div class="flex items-center gap-2 opacity-40">
                  <Clock size={12} />
                  <span class="text-[9px] font-bold uppercase tracking-widest">Time Played</span>
                </div>
                <p class="font-mono font-bold text-sm text-info">{player.time_played.toFixed(1)} <span class="text-[10px] font-normal opacity-50">Hrs</span></p>
              </div>
              <div class="flex flex-col gap-1">
                <div class="flex items-center gap-2 opacity-40">
                  <Footprints size={12} />
                  <span class="text-[9px] font-bold uppercase tracking-widest">Distance</span>
                </div>
                <p class="font-mono font-bold text-sm text-success">{player.steps.toLocaleString()} <span class="text-[10px] font-normal opacity-50">Steps</span></p>
              </div>
            </div>
          </div>
        </div>
      {/each}

      {#if serverStore.players.length === 0}
        <div class="col-span-full py-20 flex flex-col items-center justify-center opacity-10 gap-4 border-2 border-dashed border-base-200 rounded-3xl">
          <UserCircle2 size={64} />
          <p class="text-xl font-black uppercase tracking-[0.2em]">No Player Records Found</p>
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
