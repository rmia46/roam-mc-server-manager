<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { X, Minus, Square } from "lucide-svelte";

  let { isMaximized } = $props();
  const appWindow = getCurrentWindow();

  async function handleMouseDown(e: MouseEvent) {
    if (e.button === 0 && !(e.target as HTMLElement).closest('button')) {
      try {
        await appWindow.startDragging();
      } catch (err) {
        console.error("Dragging not supported or failed:", err);
      }
    }
  }

  async function handleDoubleClick(e: MouseEvent) {
    if (!(e.target as HTMLElement).closest('button')) {
      try {
        await invoke("maximize_window");
      } catch (err) {
        console.error("Toggle maximize failed:", err);
      }
    }
  }
</script>

<div 
  role="presentation"
  class="titlebar {isMaximized ? 'sharp' : 'rounded'}" 
  onmousedown={handleMouseDown}
  ondblclick={handleDoubleClick}
  data-tauri-drag-region
>
  <div class="left-section" data-tauri-drag-region>
    <div class="logo"></div>
    <span class="title">Roam MC Manager</span>
  </div>

  <div class="window-controls">
    <button class="win-btn minimize" onclick={() => invoke("minimize_window")} title="Minimize">
      <Minus size={14} />
    </button>
    <button class="win-btn maximize" onclick={() => invoke("maximize_window")} title="Maximize">
      <Square size={10} />
    </button>
    <button class="win-btn close" onclick={() => invoke("close_window")} title="Close">
      <X size={14} />
    </button>
  </div>
</div>

<style>
  .titlebar {
    height: 32px;
    background: var(--color-base-100);
    display: flex;
    justify-content: space-between;
    align-items: center;
    user-select: none;
    -webkit-user-select: none;
    border-bottom: 1px solid var(--color-base-200);
    cursor: default;
    z-index: 1000;
    transition: border-radius 0.1s;
  }

  .titlebar.rounded {
    border-top-left-radius: var(--radius-box);
    border-top-right-radius: var(--radius-box);
  }

  .titlebar.sharp {
    border-top-left-radius: 0;
    border-top-right-radius: 0;
  }

  .left-section {
    display: flex;
    align-items: center;
    gap: 10px;
    padding-left: 12px;
    height: 100%;
    flex: 1;
  }

  .logo {
    width: 14px;
    height: 14px;
    background: var(--color-primary);
    border-radius: var(--radius-selector);
  }

  .title {
    font-size: 10px;
    font-weight: 800;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: var(--color-base-content);
    opacity: 0.5;
  }

  .window-controls {
    display: flex;
    height: 100%;
  }

  .win-btn {
    width: 46px;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    background: transparent;
    border: none;
    color: var(--color-base-content);
    opacity: 0.6;
    transition: all 0.15s ease;
    outline: none;
  }

  .win-btn:hover {
    background: var(--color-base-200);
    opacity: 1;
  }

  .win-btn.close:hover {
    background: var(--color-error);
    color: var(--color-error-content);
    opacity: 1;
  }

  :global(::selection) {
    background: transparent;
  }
</style>
