<script>
  import "../app.css";
  import { onMount } from "svelte";

  onMount(() => {
    // Disable right-click globally
    const handleContextMenu = (e) => {
      e.preventDefault();
    };
    
    document.addEventListener("contextmenu", handleContextMenu);
    return () => document.removeEventListener("contextmenu", handleContextMenu);
  });
</script>

<slot />
