<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { X, Minus, Square } from "lucide-svelte";

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
  class="titlebar" 
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
    background: #111;
    display: flex;
    justify-content: space-between;
    align-items: center;
    user-select: none;
    -webkit-user-select: none;
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
    cursor: default;
    z-index: 1000;
  }

  .left-section {
    display: flex;
    align-items: center;
    gap: 10px;
    padding-left: 12px;
    height: 100%;
    flex: 1; /* Makes the title section fill available space for better drag area */
  }

  .logo {
    width: 14px;
    height: 14px;
    background: #646cff;
    border-radius: 3px;
  }

  .title {
    font-size: 10px;
    font-weight: 800;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    opacity: 0.5;
    color: #fff;
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
    color: #888;
    transition: all 0.15s ease;
    outline: none;
  }

  .win-btn:hover {
    background: rgba(255, 255, 255, 0.1);
    color: #fff;
  }

  .win-btn.close:hover {
    background: #e81123;
    color: #fff;
  }

  /* Ensure text selection doesn't trigger during drag attempts */
  :global(::selection) {
    background: transparent;
  }
</style>
