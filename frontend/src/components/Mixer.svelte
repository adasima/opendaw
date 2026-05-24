<script>
  let { onMasterVolume = () => {} } = $props();
</script>

<div class="mixer-container">
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
</div>

<style>
  .mixer-container {
    margin-left: auto;
    display: flex;
    align-items: center;
  }

  .master-fader {
    display: flex;
    align-items: center;
    gap: 12px;
    font-size: 12px;
    color: var(--on-surface-variant);
  }

  .fader {
    width: 120px;
    accent-color: var(--primary);
  }
</style>
