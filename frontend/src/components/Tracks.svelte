<script>
  import { invoke } from "@tauri-apps/api/core";
  let { tracks = [], activeTrackId = null, onSelectTrack = () => {} } = $props();

  function toggleAutomation(trackId, currentVisible, currentSelected) {
    let newVisible = !currentVisible;
    let newSelected = newVisible ? (currentSelected || "Volume") : null;
    invoke("set_automation_visibility", {
      trackId: trackId,
      visible: newVisible,
      selectedParam: newSelected
    }).catch(console.error);
  }

  function changeAutomationParam(trackId, event) {
    invoke("set_automation_visibility", {
      trackId: trackId,
      visible: true,
      selectedParam: event.target.value
    }).catch(console.error);
  }
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
      <button class="delete-btn" onclick={(e) => { e.stopPropagation(); invoke("remove_track", { trackId: track.id }).catch(console.error); }} title="Delete Track">✕</button>
      <div class="track-controls">
        <button class="ctrl-btn mute">M</button>
        <button class="ctrl-btn solo">S</button>
        <button class="ctrl-btn record">R</button>
        <button
          class="ctrl-btn auto {track.automation_visible ? 'active' : ''}"
          onclick={(e) => { e.stopPropagation(); toggleAutomation(track.id, track.automation_visible, track.selected_automation); }}
          title="Toggle Automation"
        >A</button>
      </div>
      {#if track.automation_visible}
        <div class="automation-controls">
          <select
            value={track.selected_automation || "Volume"}
            onclick={(e) => e.stopPropagation()}
            onchange={(e) => changeAutomationParam(track.id, e)}
          >
            <option value="Volume">Volume</option>
            <option value="Pan">Pan</option>
          </select>
        </div>
      {/if}
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
  .ctrl-btn.auto.active {
    background: rgba(114, 137, 218, 0.3);
    color: #7289da;
    border-color: #7289da;
  }
  .automation-controls {
    margin-top: 4px;
  }
  .automation-controls select {
    background: rgba(0, 0, 0, 0.3);
    border: 1px solid var(--outline-variant);
    border-radius: 4px;
    color: white;
    padding: 2px 4px;
    font-size: 10px;
    width: 100%;
  }

  .track-info {
    position: relative;
  }
  .delete-btn {
    position: absolute;
    top: 4px;
    right: 4px;
    width: 16px;
    height: 16px;
    background: transparent;
    border: none;
    color: var(--on-surface-variant);
    font-size: 10px;
    cursor: pointer;
    border-radius: 50%;
    display: none;
    align-items: center;
    justify-content: center;
  }
  .track-item:hover .delete-btn {
    display: flex;
  }
  .delete-btn:hover {
    background: rgba(255, 0, 0, 0.2);
    color: #f87171;
  }

</style>
