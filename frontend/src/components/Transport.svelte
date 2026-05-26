<script>
  import { invoke } from "@tauri-apps/api/core";
  import { save, open } from "@tauri-apps/plugin-dialog";
  let { timeString, onPlay, onStop, onToggleLoop, bpm = $bindable(120.0) } = $props();

  function handlePause() {
    invoke("pause").catch(console.error);
  }

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
</script>

<div class="transport-container">
  <div class="transport-controls">
    <button class="transport-btn" onclick={handleSaveProject} title="Save Project">💾</button>
    <button class="transport-btn" onclick={handleLoadProject} title="Load Project">📂</button>
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
</style>
