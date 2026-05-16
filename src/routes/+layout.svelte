<script>
  import "../app.css";
  import { onMount } from "svelte";
  import { initShortcuts, destroyShortcuts } from "$lib/utils/ShortcutManager";

  onMount(() => {
    initShortcuts();
    return () => destroyShortcuts();
  });
</script>

<slot />
