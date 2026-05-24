<script>
  let { tracks = [], activeTrackId = null, onSelectTrack = () => {} } = $props();
</script>

{#each tracks as track}
  <div
    class="track-item"
    class:active={track.id === activeTrackId}
    role="button"
    tabindex="0"
    onclick={() => onSelectTrack(track.id)}
    onkeydown={(e) => e.key === "Enter" && onSelectTrack(track.id)}
  >
    <div class="track-color" style="background: {track.color};"></div>
    <div class="track-info">
      <span class="track-name">{track.name}</span>
      <div class="track-controls">
        <button class="ctrl-btn mute">M</button>
        <button class="ctrl-btn solo">S</button>
        <button class="ctrl-btn record">R</button>
      </div>
    </div>
  </div>
{/each}

<style>
  .track-item {
    display: flex;
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid transparent;
    border-radius: 6px;
    overflow: hidden;
    cursor: pointer;
    transition: all 0.2s;
    margin-bottom: 4px;
  }

  .track-item:hover {
    background: rgba(255, 255, 255, 0.06);
  }

  .track-item.active {
    background: rgba(255, 255, 255, 0.08);
    border-color: var(--outline-variant);
  }

  .track-color {
    width: 6px;
  }

  .track-info {
    padding: 8px 12px;
    flex-grow: 1;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .track-name {
    font-size: 13px;
    font-weight: 500;
  }

  .track-controls {
    display: flex;
    gap: 4px;
  }

  .ctrl-btn {
    width: 22px;
    height: 22px;
    border-radius: 4px;
    border: 1px solid var(--outline-variant);
    background: transparent;
    color: var(--on-surface-variant);
    font-size: 10px;
    font-weight: bold;
    cursor: pointer;
    transition: all 0.15s;
  }

  .ctrl-btn:hover {
    background: rgba(255, 255, 255, 0.1);
  }

  .ctrl-btn.mute:hover {
    color: #f87171;
    border-color: #f87171;
  }
  .ctrl-btn.solo:hover {
    color: #fbbf24;
    border-color: #fbbf24;
  }
  .ctrl-btn.record:hover {
    color: #ef4444;
    border-color: #ef4444;
  }
</style>
