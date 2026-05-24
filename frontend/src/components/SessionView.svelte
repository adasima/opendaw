<script>
  import ClipLauncher from "./ClipLauncher.svelte";
</script>

<div class="session-view-container glass-panel">
  <div class="session-header">
    <h2>Session View</h2>
  </div>
  <div class="session-content">
    <ClipLauncher />
  </div>
</div>

<style>
  .session-view-container {
    display: flex;
    flex-direction: column;
    height: 100%;
    border-radius: 12px;
    overflow: hidden;
  }

  .glass-panel {
    background: rgba(20, 20, 25, 0.4);
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 255, 255, 0.08);
  }

  .session-header {
    padding: 12px 16px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  }

  .session-header h2 {
    margin: 0;
    font-size: 16px;
    color: var(--on-surface);
  }

  .session-content {
    flex-grow: 1;
    overflow: auto;
    padding: 8px;
  }
</style>
