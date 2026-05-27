<script>
  import { invoke } from "@tauri-apps/api/core";
  import { save, open } from "@tauri-apps/plugin-dialog";
  let { timeString, onPlay, onStop, onToggleLoop, bpm = $bindable(120.0) } = $props();

  function handlePause() {
    invoke("pause").catch(console.error);
  }

  /**
   * プロジェクトの状態をJSONファイルとして保存します。
   */
  async function handleSaveProject() {
    try {
      const path = await save({
        filters: [{
          name: 'OpenDAW Project',
          extensions: ['json']
        }]
      });
      if (path) {
        await invoke("save_project", { path });
      }
    } catch (e) {
      console.error(e);
    }
  }

  /**
   * JSONファイルからプロジェクトの状態を読み込みます。
   */
  async function handleLoadProject() {
    try {
      const path = await open({
        multiple: false,
        filters: [{
          name: 'OpenDAW Project',
          extensions: ['json']
        }]
      });
      if (path) {
        await invoke("load_project", { path });
      }
    } catch (e) {
      console.error(e);
    }
  }

  function handleUndo() {
    invoke("undo").catch(console.error);
  }

  function handleRedo() {
    invoke("redo").catch(console.error);
  }

  let isGridEnabled = $state(true);
  let gridResolution = $state(4); // デフォルトは1/4音符

  /**
   * グリッド設定をバックエンドに送信します。
   */
  async function handleGridSettingsChange() {
    try {
      await invoke("set_grid_settings", {
        isEnabled: isGridEnabled,
        resolution: parseInt(gridResolution.toString(), 10)
      });
    } catch (e) {
      console.error(e);
    }
  }

  function toggleGrid() {
    isGridEnabled = !isGridEnabled;
    handleGridSettingsChange();
  }
</script>

<div class="transport-container">
  <div class="transport-controls">
    <button class="transport-btn" onclick={handleSaveProject} title="Save Project">💾</button>
    <button class="transport-btn" onclick={handleLoadProject} title="Load Project">📂</button>
    <button class="transport-btn" onclick={handleUndo} title="Undo">↩️</button>
    <button class="transport-btn" onclick={handleRedo} title="Redo">↪️</button>
    <button class="transport-btn" onclick={onToggleLoop} title="Toggle Loop"
      >🔁</button
    >
    <button class="transport-btn play" onclick={onPlay} title="Play">▶</button>
    <button class="transport-btn pause" onclick={handlePause} title="Pause">⏸</button>
    <button class="transport-btn" onclick={onStop} title="Stop">⏹</button>
    <button class="transport-btn record" title="Record">⏺</button>
  </div>
  <div class="time-display">
    <span>{timeString}</span>
  </div>
  <div class="bpm-control">
    <span class="bpm-label">BPM:</span>
    <input type="number" class="bpm-number" bind:value={bpm} min="20" max="300" step="1" />
    <input type="range" class="bpm-slider" bind:value={bpm} min="20" max="300" step="1" />
  </div>
  <div class="grid-control">
    <button
      class="transport-btn grid-btn {isGridEnabled ? 'active' : ''}"
      onclick={toggleGrid}
      title="Toggle Grid Snap"
    >
      🧲
    </button>
    <select
      class="grid-resolution-select"
      bind:value={gridResolution}
      onchange={handleGridSettingsChange}
      disabled={!isGridEnabled}
    >
      <option value="1">1/1</option>
      <option value="2">1/2</option>
      <option value="4">1/4</option>
      <option value="8">1/8</option>
      <option value="16">1/16</option>
      <option value="32">1/32</option>
    </select>
  </div>
</div>

<style>
  .transport-container {
    display: flex;
    align-items: center;
    gap: 32px;
  }

  .transport-controls {
    display: flex;
    gap: 8px;
  }

  .transport-btn {
    width: 36px;
    height: 36px;
    border-radius: 18px;
    border: 1px solid var(--outline-variant);
    background: rgba(255, 255, 255, 0.05);
    color: var(--on-surface);
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all 0.2s;
  }

  .transport-btn:hover {
    background: rgba(255, 255, 255, 0.15);
  }

  .transport-btn.play {
    width: 48px;
    height: 48px;
    border-radius: 24px;
    font-size: 20px;
    background: var(--primary);
    color: var(--on-primary);
    border: none;
    box-shadow: 0 0 15px rgba(255, 115, 0, 0.4);
  }

  .transport-btn.play:hover {
    transform: scale(1.05);
    box-shadow: 0 0 20px rgba(255, 115, 0, 0.6);
  }

  .transport-btn.record {
    color: #ef4444;
  }

  .time-display {
    font-family: "Courier New", Courier, monospace;
    font-size: 24px;
    color: var(--primary);
    font-weight: bold;
    text-shadow: 0 0 10px rgba(255, 115, 0, 0.3);
    padding: 4px 16px;
    background: rgba(0, 0, 0, 0.3);
    border-radius: 6px;
    border: 1px inset rgba(255, 255, 255, 0.1);
  }

  .bpm-control {
    display: flex;
    align-items: center;
    gap: 8px;
    background: rgba(0, 0, 0, 0.3);
    padding: 4px 12px;
    border-radius: 6px;
    border: 1px inset rgba(255, 255, 255, 0.1);
  }

  .bpm-label {
    font-size: 14px;
    color: var(--on-surface);
    font-weight: bold;
  }

  .bpm-number {
    width: 60px;
    background: rgba(255, 255, 255, 0.1);
    border: 1px solid var(--outline);
    color: white;
    border-radius: 4px;
    padding: 2px 4px;
    text-align: right;
  }

  .bpm-slider {
    width: 100px;
  }

  .grid-control {
    display: flex;
    align-items: center;
    gap: 8px;
    background: rgba(0, 0, 0, 0.3);
    padding: 4px 12px;
    border-radius: 6px;
    border: 1px inset rgba(255, 255, 255, 0.1);
  }

  .grid-btn {
    width: 28px;
    height: 28px;
    font-size: 14px;
    opacity: 0.5;
  }

  .grid-btn.active {
    opacity: 1;
    background: rgba(255, 255, 255, 0.2);
    border-color: var(--primary);
  }

  .grid-resolution-select {
    background: rgba(255, 255, 255, 0.1);
    border: 1px solid var(--outline);
    color: white;
    border-radius: 4px;
    padding: 2px 4px;
    outline: none;
    cursor: pointer;
  }

  .grid-resolution-select:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .grid-resolution-select option {
    background: #2a2a2a;
    color: white;
  }
</style>
