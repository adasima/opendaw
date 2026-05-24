<script>
  let { timeString, onPlay, onStop, onToggleLoop } = $props();
</script>

<div class="transport-container">
  <div class="transport-controls">
    <button class="transport-btn" onclick={onToggleLoop} title="Toggle Loop"
      >🔁</button
    >
    <button class="transport-btn play" onclick={onPlay} title="Play">▶</button>
    <button class="transport-btn" onclick={onStop} title="Stop">⏹</button>
    <button class="transport-btn record" title="Record">⏺</button>
  </div>
  <div class="time-display">
    <span>{timeString}</span>
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
</style>
