<script lang="ts">
  import { 
    Plus, Trash2, Server, Play, ChevronRight, X, AlertCircle, 
    CheckCircle2, Info, Timer, Cpu, HardDrive, Users, Settings 
  } from "lucide-svelte";
</script>

<div class="h-full space-y-12 pb-20">
  <!-- Brand Header -->
  <div class="flex flex-col gap-2">
    <h2 class="text-4xl font-black tracking-tighter">Theme Laboratory</h2>
    <p class="opacity-50 text-sm font-medium uppercase tracking-widest">Visual Style & Component Testing</p>
  </div>

  <!-- Buttons & Actions -->
  <section class="space-y-6">
    <h3 class="text-xs font-bold uppercase tracking-widest opacity-30 border-b border-base-200 pb-2">Action Elements</h3>
    <div class="flex flex-wrap gap-4">
      <button class="btn btn-primary">Primary Button</button>
      <button class="btn btn-secondary">Secondary</button>
      <button class="btn btn-accent">Accent</button>
      <button class="btn btn-info">Info</button>
      <button class="btn btn-success">Success</button>
      <button class="btn btn-warning">Warning</button>
      <button class="btn btn-error">Error</button>
      <button class="btn btn-ghost">Ghost</button>
      <button class="btn btn-outline">Outline</button>
    </div>
    <div class="flex flex-wrap gap-4">
      <button class="btn btn-primary btn-sm">Small</button>
      <button class="btn btn-primary btn-xs">Extra Small</button>
      <button class="btn btn-primary btn-lg">Large Action</button>
    </div>
  </section>

  <!-- Badges & Indicators -->
  <section class="space-y-6">
    <h3 class="text-xs font-bold uppercase tracking-widest opacity-30 border-b border-base-200 pb-2">Badges & Tags</h3>
    <div class="flex flex-wrap gap-3">
      <div class="badge badge-primary p-3">Primary</div>
      <div class="badge badge-secondary p-3">Secondary</div>
      <div class="badge badge-success badge-outline">Running</div>
      <div class="badge badge-error badge-outline">Stopped</div>
      <div class="badge badge-warning gap-2">
        <Timer size={12} /> Pending
      </div>
    </div>
  </section>

  <!-- Cards & Stats -->
  <section class="space-y-6">
    <h3 class="text-xs font-bold uppercase tracking-widest opacity-30 border-b border-base-200 pb-2">Cards & Data</h3>
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <div class="card bg-base-100 shadow-xl border border-base-200">
        <div class="card-body">
          <h2 class="card-title">Theme Card</h2>
          <p class="text-sm opacity-70">This card uses base-100 and respects the radius-box variable.</p>
          <div class="card-actions justify-end mt-4">
            <button class="btn btn-primary btn-sm">Action</button>
          </div>
        </div>
      </div>

      <div class="stats shadow bg-primary text-primary-content">
        <div class="stat">
          <div class="stat-title text-primary-content opacity-70">Total Players</div>
          <div class="stat-value">1,240</div>
          <div class="stat-desc text-primary-content opacity-50">21% more than last month</div>
        </div>
      </div>

      <div class="card bg-neutral text-neutral-content">
        <div class="card-body items-center text-center">
          <h2 class="card-title">Neutral Style</h2>
          <p class="text-xs">Testing high contrast neutral components.</p>
        </div>
      </div>
    </div>
  </section>

  <!-- Progress & Status -->
  <section class="space-y-6">
    <h3 class="text-xs font-bold uppercase tracking-widest opacity-30 border-b border-base-200 pb-2">Feedback & Loading</h3>
    <div class="space-y-4 max-w-xl">
      <progress class="progress progress-primary w-full h-3" value="70" max="100"></progress>
      <progress class="progress progress-secondary w-full" value="40" max="100"></progress>
      <div class="flex gap-4">
        <span class="loading loading-spinner text-primary"></span>
        <span class="loading loading-dots text-secondary"></span>
        <span class="loading loading-ring text-accent"></span>
      </div>
    </div>
  </section>

  <!-- Inputs -->
  <section class="space-y-6">
    <h3 class="text-xs font-bold uppercase tracking-widest opacity-30 border-b border-base-200 pb-2">Form Elements</h3>
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
      <div class="form-control w-full">
        <label class="label"><span class="label-text">Theme Input</span></label>
        <input type="text" placeholder="Type here" class="input input-bordered w-full" />
      </div>
      <div class="form-control w-full">
        <label class="label"><span class="label-text">Range Selector</span></label>
        <input type="range" min="0" max="100" class="range range-primary" />
      </div>
      <div class="form-control">
        <label class="label cursor-pointer justify-start gap-4">
          <input type="checkbox" checked class="checkbox checkbox-primary" />
          <span class="label-text">Checkbox Test</span>
        </label>
      </div>
      <div class="form-control">
        <label class="label cursor-pointer justify-start gap-4">
          <input type="radio" name="radio-1" class="radio radio-secondary" checked />
          <span class="label-text">Radio Test</span>
        </label>
      </div>
    </div>
  </section>

  <!-- Alerts -->
  <section class="space-y-6">
    <h3 class="text-xs font-bold uppercase tracking-widest opacity-30 border-b border-base-200 pb-2">Alert Notifications</h3>
    <div class="space-y-3">
      <div class="alert alert-info shadow-lg">
        <Info size={20} />
        <span>New server version available.</span>
      </div>
      <div class="alert alert-success shadow-lg">
        <CheckCircle2 size={20} />
        <span>Server started successfully.</span>
      </div>
      <div class="alert alert-warning shadow-lg">
        <AlertCircle size={20} />
        <span>High RAM usage detected!</span>
      </div>
    </div>
  </section>

  <!-- Dashboard Mirror -->
  <section class="space-y-6 pb-10">
    <h3 class="text-xs font-bold uppercase tracking-widest opacity-30 border-b border-base-200 pb-2">Dashboard Mirror (Mockup)</h3>
    <div class="p-6 bg-base-200 rounded-3xl border border-base-100 shadow-inner space-y-6">
      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div class="stats shadow bg-base-100 border border-base-200">
          <div class="stat">
            <div class="stat-figure text-info opacity-30"><Users size={24} /></div>
            <div class="stat-label text-[10px] font-bold uppercase opacity-50 mb-1">Players</div>
            <div class="stat-value text-2xl font-mono">12<span class="text-xs opacity-30">/50</span></div>
          </div>
        </div>
        <div class="stats shadow bg-base-100 border border-base-200">
          <div class="stat">
            <div class="stat-figure text-primary opacity-30"><Cpu size={24} /></div>
            <div class="stat-label text-[10px] font-bold uppercase opacity-50 mb-1">CPU</div>
            <div class="stat-value text-2xl font-mono text-primary">45%</div>
          </div>
        </div>
        <div class="stats shadow bg-base-100 border border-base-200">
          <div class="stat">
            <div class="stat-figure text-secondary opacity-30"><HardDrive size={24} /></div>
            <div class="stat-label text-[10px] font-bold uppercase opacity-50 mb-1">RAM</div>
            <div class="stat-value text-2xl font-mono text-secondary">1.2<span class="text-xs font-normal opacity-50 ml-1">GB</span></div>
          </div>
        </div>
      </div>

      <div class="h-48 bg-neutral rounded-2xl overflow-hidden flex flex-col border border-white/5">
        <div class="bg-base-100/10 p-2 px-4 flex justify-between items-center border-b border-white/5">
          <span class="text-[10px] font-black tracking-widest opacity-30">MOCK_CONSOLE</span>
          <div class="badge badge-success badge-xs animate-pulse"></div>
        </div>
        <div class="p-4 font-mono text-[10px] space-y-1 opacity-60">
          <div class="text-success">[12:00:01] Server started on port 25565</div>
          <div>[12:00:05] Preparing level "world"</div>
          <div>[12:00:10] Done (9.452s)! For help, type "help"</div>
          <div class="text-info">[12:05:22] Player "Steve" joined the game</div>
        </div>
      </div>
    </div>
  </section>
</div>
