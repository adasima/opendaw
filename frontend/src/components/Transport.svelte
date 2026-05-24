<script>
  let { timeString, onPlay, onStop, onToggleLoop, onMasterVolume } = $props();
</script>

<footer class="bottom-panel glass-panel">
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
  <div class="master-fader">
    <span>Master</span>
    <input
      type="range"
      min="0"
      max="100"
      value="80"
      class="fader"
      oninput={onMasterVolume}
    />
  </div>
</footer>
