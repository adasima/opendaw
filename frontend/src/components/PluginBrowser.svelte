<script>
  let plugins = $state([
    { id: 1, name: "Synth Wave", type: "VST3", vendor: "OpenDAW" },
    { id: 2, name: "Fat Bass", type: "CLAP", vendor: "OpenDAW" },
    { id: 3, name: "Spacy Reverb", type: "VST3", vendor: "FXAudio" },
    { id: 4, name: "Crunch Dist", type: "CLAP", vendor: "FXAudio" },
    { id: 5, name: "Classic EQ", type: "VST3", vendor: "OpenDAW" }
  ]);

  let searchQuery = $state("");

  let filteredPlugins = $derived(
    plugins.filter(p => p.name.toLowerCase().includes(searchQuery.toLowerCase()) || p.vendor.toLowerCase().includes(searchQuery.toLowerCase()))
  );
</script>

<div class="plugin-browser-container glass-panel">
  <div class="browser-header">
    <h3>Plugin Browser</h3>
    <input
      type="text"
      placeholder="Search plugins..."
      bind:value={searchQuery}
      class="search-input"
    />
  </div>

  <div class="plugin-list">
    {#each filteredPlugins as plugin}
      <div class="plugin-item">
        <div class="plugin-icon">
          {plugin.type === 'VST3' ? '🎹' : '🎛️'}
        </div>
        <div class="plugin-info">
          <span class="plugin-name">{plugin.name}</span>
          <span class="plugin-meta">{plugin.vendor} • {plugin.type}</span>
        </div>
      </div>
    {/each}
    {#if filteredPlugins.length === 0}
      <div class="no-results">No plugins found.</div>
    {/if}
  </div>
</div>

<style>
  .plugin-browser-container {
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

  .browser-header {
    padding: 12px 16px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .browser-header h3 {
    margin: 0;
    font-size: 14px;
    color: var(--on-surface);
  }

  .search-input {
    width: 100%;
    padding: 6px 8px;
    background: rgba(0, 0, 0, 0.2);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 4px;
    color: var(--on-surface);
    font-size: 12px;
  }

  .search-input:focus {
    outline: none;
    border-color: var(--primary);
  }

  .plugin-list {
    flex-grow: 1;
    overflow-y: auto;
    padding: 8px;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .plugin-item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 8px;
    border-radius: 6px;
    background: rgba(255, 255, 255, 0.03);
    cursor: pointer;
    border: 1px solid transparent;
    transition: all 0.2s;
  }

  .plugin-item:hover {
    background: rgba(255, 255, 255, 0.08);
    border-color: rgba(255, 255, 255, 0.1);
  }

  .plugin-icon {
    font-size: 16px;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 24px;
    height: 24px;
    background: rgba(0, 0, 0, 0.2);
    border-radius: 4px;
  }

  .plugin-info {
    display: flex;
    flex-direction: column;
  }

  .plugin-name {
    font-size: 13px;
    color: var(--on-surface);
    font-weight: 500;
  }

  .plugin-meta {
    font-size: 11px;
    color: var(--on-surface-variant);
  }

  .no-results {
    padding: 16px;
    text-align: center;
    color: var(--on-surface-variant);
    font-size: 12px;
  }
</style>
