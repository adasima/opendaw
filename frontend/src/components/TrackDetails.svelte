<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  let { trackId = null } = $props();

  let devices = $state([]);
  let selectedDevice = $state("");
  let selectedChannel = $state("1");
  let channels = Array.from({length: 16}, (_, i) => (i + 1).toString());

  onMount(async () => {
    try {
      devices = await invoke("get_midi_devices");
      if (devices.length > 0) {
        selectedDevice = devices[0];
      }
    } catch (e) {
      console.error("Failed to load MIDI devices", e);
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
</script>

<div class="track-details">
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

  select {
    background: rgba(0, 0, 0, 0.3);
    border: 1px solid var(--outline-variant);
    border-radius: 4px;
    color: white;
    padding: 4px 8px;
    font-size: 12px;
  }
</style>
