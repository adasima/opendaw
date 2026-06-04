<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  let { trackId = null } = $props();

  let devices = $state([]);
  let selectedDevice = $state("");
  let selectedChannel = $state("1");
  let channels = Array.from({length: 16}, (_, i) => (i + 1).toString());

  let volume = $state(0.8);
  let pan = $state(0.0);
  let plugins = $state([]);

  let allTracks = $state([]);
  let outputRouting = $state("master"); // "master" or track id
  let sends = $state([]);
  let newSendTarget = $state("");

  onMount(async () => {
    try {
      let state = await invoke("get_project_state");
      allTracks = state.tracks;
      if (trackId !== null) {
        let track = state.tracks.find(t => t.id === trackId);
        if (track) {
          plugins = track.plugins || [];
          volume = track.volume;
          pan = track.pan;
          outputRouting = track.output_routing !== null ? track.output_routing.toString() : "master";
          sends = track.sends || [];
        }
      }
      devices = await invoke("get_midi_devices");
      if (devices.length > 0) {
        selectedDevice = devices[0];
      }
    } catch (e) {
      console.error("Failed to load track data", e);
    }
  });

  function applyRouting() {
    if (trackId !== null && selectedDevice) {
      invoke("set_track_midi_routing", {
        trackId: parseInt(trackId),
        device: selectedDevice,
        channel: parseInt(selectedChannel)
      }).catch(console.error);
    }
  }

  function applyVolume() {
    if (trackId !== null) {
      invoke("set_track_volume", {
        trackId: parseInt(trackId),
        volume: parseFloat(volume)
      }).catch(console.error);
    }
  }

  function applyPan() {
    if (trackId !== null) {
      invoke("set_track_pan", {
        trackId: parseInt(trackId),
        pan: parseFloat(pan)
      }).catch(console.error);
    }
  }

  function applyOutputRouting() {
    if (trackId !== null) {
      let target = outputRouting === "master" ? null : parseInt(outputRouting);
      invoke("set_track_output_routing", {
        trackId: parseInt(trackId),
        target: target
      }).catch(console.error);
    }
  }

  function applySendAmount(targetTrackId, amount) {
    if (trackId !== null) {
      invoke("set_track_send_amount", {
        trackId: parseInt(trackId),
        targetTrackId: targetTrackId,
        amount: parseFloat(amount)
      }).catch(console.error);
    }
  }

  function addNewSend() {
    if (trackId !== null && newSendTarget) {
      let targetId = parseInt(newSendTarget);
      if (!sends.some(s => s.target_track_id === targetId)) {
        let amount = 0.5; // Default amount
        invoke("add_track_send", {
          trackId: parseInt(trackId),
          targetTrackId: targetId,
          amount: amount
        }).then(() => {
          sends = [...sends, { target_track_id: targetId, amount: amount }];
          newSendTarget = "";
        }).catch(console.error);
      }
    }
  }
</script>

<div class="track-details">
  <div class="routing-section">
    <h3>Mixer</h3>
    <div class="control-group">
      <label for="track-volume">Volume ({Math.round(volume * 100)}%)</label>
      <input type="range" id="track-volume" min="0" max="1" step="0.01" bind:value={volume} onchange={applyVolume} />
    </div>
    <div class="control-group">
      <label for="track-pan">Pan ({Math.round(pan * 100)}%)</label>
      <input type="range" id="track-pan" min="-1" max="1" step="0.01" bind:value={pan} onchange={applyPan} />
    </div>
  </div>

  <div class="routing-section">
    <h3>Audio Routing</h3>
    <div class="control-group">
      <label for="output-routing">Output</label>
      <select id="output-routing" bind:value={outputRouting} onchange={applyOutputRouting}>
        <option value="master">Master</option>
        {#each allTracks as track}
          {#if track.id !== trackId}
            <option value={track.id.toString()}>{track.name}</option>
          {/if}
        {/each}
      </select>
    </div>

    <div class="control-group sends-group">
      <label>Sends</label>
      {#each sends as send, index}
        <div class="send-item">
          <span>{allTracks.find(t => t.id === send.target_track_id)?.name || 'Unknown'}</span>
          <input type="range" min="0" max="1" step="0.01" bind:value={sends[index].amount} onchange={() => applySendAmount(send.target_track_id, sends[index].amount)} />
        </div>
      {/each}
      <div class="add-send">
        <select bind:value={newSendTarget}>
          <option value="" disabled>Select track...</option>
          {#each allTracks as track}
            {#if track.id !== trackId && !sends.some(s => s.target_track_id === track.id)}
              <option value={track.id.toString()}>{track.name}</option>
            {/if}
          {/each}
        </select>
        <button onclick={addNewSend} disabled={!newSendTarget}>Add Send</button>
      </div>
    </div>
  </div>

  <div class="routing-section">
    <h3>MIDI Routing</h3>

    <div class="control-group">
      <label for="midi-device">Input Device</label>
      <select id="midi-device" bind:value={selectedDevice} onchange={applyRouting}>
        {#each devices as device}
          <option value={device}>{device}</option>
        {/each}
        {#if devices.length === 0}
          <option value="">No devices found</option>
        {/if}
      </select>
    </div>

    <div class="control-group">
      <label for="midi-channel">Channel</label>
      <select id="midi-channel" bind:value={selectedChannel} onchange={applyRouting}>
        <option value="0">All</option>
        {#each channels as ch}
          <option value={ch}>{ch}</option>
        {/each}
      </select>
    </div>
  </div>

  <div class="routing-section">
    <h3>Plugins</h3>
    {#if plugins.length > 0}
      {#each plugins as plugin}
        <div class="plugin-list-item">
          <span class="plugin-name">{plugin}</span>
          <button class="open-gui-btn">Open GUI</button>
        </div>
      {/each}
    {:else}
      <div class="no-plugins">No plugins loaded</div>
    {/if}
  </div>
</div>

<style>
  .track-details {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .routing-section {
    background: rgba(0, 0, 0, 0.2);
    border: 1px solid var(--outline);
    border-radius: 8px;
    padding: 12px;
  }

  h3 {
    margin: 0 0 12px 0;
    font-size: 13px;
    color: var(--on-surface);
  }

  .control-group {
    display: flex;
    flex-direction: column;
    gap: 4px;
    margin-bottom: 8px;
  }

  label {
    font-size: 11px;
    color: var(--on-surface-variant);
  }

  select, input[type="range"] {
    background: rgba(0, 0, 0, 0.3);
    border: 1px solid var(--outline-variant);
    border-radius: 4px;
    color: white;
    padding: 4px 8px;
    font-size: 12px;
    width: 100%;
    box-sizing: border-box;
  }

  .plugin-list-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    background: rgba(0, 0, 0, 0.2);
    border: 1px solid var(--outline-variant);
    padding: 6px 8px;
    border-radius: 4px;
    margin-bottom: 4px;
  }

  .plugin-name {
    font-size: 12px;
    color: var(--on-surface);
  }

  .open-gui-btn {
    background: var(--primary);
    color: white;
    border: none;
    border-radius: 4px;
    padding: 4px 8px;
    font-size: 10px;
    cursor: pointer;
  }

  .no-plugins {
    font-size: 11px;
    color: var(--on-surface-variant);
  }

  .sends-group {
    margin-top: 12px;
  }

  .send-item {
    display: flex;
    flex-direction: column;
    gap: 4px;
    margin-bottom: 8px;
    padding: 6px;
    background: rgba(0,0,0,0.1);
    border-radius: 4px;
  }

  .send-item span {
    font-size: 12px;
    color: var(--on-surface);
  }

  .add-send {
    display: flex;
    gap: 8px;
    margin-top: 8px;
  }

  .add-send button {
    background: var(--primary);
    color: white;
    border: none;
    border-radius: 4px;
    padding: 4px 8px;
    font-size: 10px;
    cursor: pointer;
    white-space: nowrap;
  }

  .add-send button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
