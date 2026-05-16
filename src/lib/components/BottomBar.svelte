<script lang="ts">
  // Bottom Bar: Transport & Audio Control
  import { audioContext } from "../audio/AudioContext.svelte";
  import { meterStore } from "../stores/meter";
  import LevelMeter from "./LevelMeter.svelte";
  import { onMount } from "svelte";
  import { uiState } from '../stores/UIState.svelte';

  // Start syncing when mounted (can be moved to layout later)
  onMount(() => {
    audioContext.startSync();
    meterStore.init();
    return () => audioContext.stopSync();
  });

  function togglePlay() {
    audioContext.togglePlayback();
  }

  function stop() {
    audioContext.stop();
  }

  function toggleLoop() {
    audioContext.toggleLoop();
  }
  
  // Wait, I need 'invoke' import if I use it.
  import { invoke } from "../api";

  // --- Preview Mode Logic ---
  // PREVIEW: Plays the current track's notes from the beginning
  let isPreviewMode = $state(false);

  // Derived current notes from selected track
  const selectedTrack = $derived(audioContext.tracks.find(
    (t) => t.id === audioContext.selectedTrackId,
  ));
  
  const currentNotes = $derived(
    selectedTrack &&
    selectedTrack.kind === "Midi" &&
    selectedTrack.content &&
    "Midi" in selectedTrack.content
      ? selectedTrack.content.Midi.notes
      : []
  );

  async function togglePreview() {
    if (!isPreviewMode) {
      // Enter Preview Mode - seek to start (user presses Play separately)
      if (currentNotes.length === 0) {
        alert("プレビューするノートがありません。");
        return;
      }

      // Just seek to start, don't auto-play
      await audioContext.seek(0);
      isPreviewMode = true;
    } else {
      // Exit Preview Mode
      isPreviewMode = false;
    }
  }
</script>

<div
  class="h-16 bg-theme-shirt border-t border-theme-skin-shadow flex items-center select-none z-50 relative px-4"
>
  <!-- Left Section: Preview Mode -->
  <div class="flex-shrink-0 w-32">
    <button
      onclick={togglePreview}
      class="px-3 py-1 rounded text-xs font-bold transition-colors {isPreviewMode
        ? 'bg-green-500 text-white animate-pulse'
        : 'bg-theme-skin-shadow text-theme-wall-light hover:bg-theme-skin/80'}"
    >
      {isPreviewMode ? "▶ PLAYING" : "🎵 PREVIEW"}
    </button>
  </div>

  <!-- Center Section: Transport Controls -->
  <div class="flex-1 flex items-center justify-center gap-3">
    <!-- Record Button -->
    <button
      onclick={() => audioContext.toggleRecording()}
      class="w-10 h-10 rounded-full bg-theme-skin-shadow hover:bg-theme-skin/80 flex items-center justify-center transition-all active:scale-95 {audioContext.isRecording ? 'ring-2 ring-red-500' : ''}"
      title={audioContext.isRecording ? "Stop Recording" : "Record"}
    >
      <span class="block w-4 h-4 rounded-full {audioContext.isRecording ? 'bg-red-500 animate-pulse' : 'bg-red-900'}"></span>
    </button>

    <button
      onclick={stop}
      class="w-10 h-10 rounded-full bg-theme-skin-shadow hover:bg-theme-skin/80 flex items-center justify-center text-theme-wall-light transition-all active:scale-95"
      aria-label="Stop"
    >
      <span class="block w-3 h-3 bg-theme-wall-light rounded-sm"></span>
    </button>
    <button
      onclick={togglePlay}
      class="w-12 h-12 rounded-full bg-theme-skin hover:bg-theme-skin/90 flex items-center justify-center text-theme-hair transition-all active:scale-95 shadow-md"
      aria-label={audioContext.isPlaying ? "Pause" : "Play"}
    >
      {#if audioContext.isPlaying}
        <span class="flex gap-0.5">
          <span class="block h-4 w-1.5 bg-theme-hair"></span>
          <span class="block h-4 w-1.5 bg-theme-hair"></span>
        </span>
      {:else}
        <span
          class="block w-0 h-0 border-t-[8px] border-t-transparent border-l-[14px] border-l-theme-hair border-b-[8px] border-b-transparent ml-1"
        ></span>
      {/if}
    </button>
  </div>

  <!-- Right Section: Loop, Scroll, Status -->
  <div class="flex-shrink-0 flex items-center gap-2">
    <!-- View Switcher -->
    <div class="flex bg-black/30 rounded p-0.5 mr-2">
        <button 
           class="px-2 py-0.5 text-xs rounded transition-colors {uiState.viewMode === 'arrange' ? 'bg-theme-skin text-theme-hair font-bold' : 'text-theme-wall-shadow hover:text-white'}"
           onclick={() => uiState.viewMode = 'arrange'}
           title="Arrangement View"
        >ARR</button>
        <button 
           class="px-2 py-0.5 text-xs rounded transition-colors {uiState.viewMode === 'mixer' ? 'bg-theme-skin text-theme-hair font-bold' : 'text-theme-wall-shadow hover:text-white'}"
           onclick={() => uiState.viewMode = 'mixer'}
           title="Mixer View"
        >MIX</button>
    </div>

    <button
      class="px-2 py-1 rounded font-bold text-xs transition-colors {audioContext.isLooping
        ? 'bg-theme-skin text-theme-hair'
        : 'text-theme-wall-shadow hover:text-theme-skin'}"
      onclick={toggleLoop}
      title="Toggle Loop"
    >
      LOOP
    </button>

    <button
      class="p-1 rounded transition-colors {audioContext.autoScroll
        ? 'text-theme-skin'
        : 'text-theme-wall-shadow hover:text-theme-skin'}"
      onclick={() => audioContext.toggleAutoScroll()}
      title="Auto Scroll"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="16"
        height="16"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
        ><path d="M21 12H3" /><path d="M21 6H3" /><path d="M21 18H3" /><path
          d="M18 6L22 12L18 18"
        /></svg
      >
    </button>

    <div class="hidden sm:flex items-center gap-2">
      <span class="text-xs text-theme-wall-shadow">Master</span>
      <div class="w-24 h-4 flex flex-col gap-[2px]">
        <LevelMeter value={$meterStore.master_peak[0] || 0} vertical={false} />
        <LevelMeter value={$meterStore.master_peak[1] || 0} vertical={false} />
      </div>
    </div>

    <span class="text-xs text-theme-wall-shadow w-14 text-right font-mono"
      >{audioContext.playheadPosition.toFixed(2)}s</span
    >
  </div>
</div>
