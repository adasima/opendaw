<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { audioContext } from "../audio/AudioContext.svelte";
    import { meterStore } from "../stores/meter";

    const { mockEnabled } = meterStore; // meterStore still uses stores

    // Debug Mode Toggle
    let debugMode = false;

    async function handleExport() {
        const path = window.prompt(
            "Enter output path for WAV export (e.g., C:/output.wav):",
        );
        if (path) {
            try {
                await invoke("export_project", { path });
                alert("Export complete: " + path);
            } catch (e) {
                alert("Export failed: " + e);
            }
        }
    }

    // Debug: Force refresh state
    async function debugRefresh() {
        // Trigger a re-sync
        audioContext.stopSync();
        audioContext.startSync();
    }

    // Debug: Log current state to console
    function debugLogState() {
        console.log("=== INDAW Debug State ===");
        console.log("AudioContext:", audioContext);
        console.log("Tracks:", audioContext.tracks); 
        console.log("Selected Track ID:", audioContext.selectedTrackId);
    }
</script>

<div
    class="h-10 bg-theme-shirt border-b border-theme-skin-shadow flex items-center px-4 justify-between shrink-0 select-none"
>
    <div class="flex items-center space-x-4">
        <span class="font-bold text-theme-skin text-sm tracking-wide"
            >INDAW</span
        >
        <div class="h-4 w-[1px] bg-theme-skin-shadow/50"></div>
        <!-- Menu Items (Disabled Placeholders) -->
        <div class="flex space-x-1 text-xs">
            <span
                class="px-2 py-1 text-theme-wall-shadow/60 cursor-not-allowed"
                title="Coming soon">File</span
            >
            <span
                class="px-2 py-1 text-theme-wall-shadow/60 cursor-not-allowed"
                title="Coming soon">Edit</span
            >
            <span
                class="px-2 py-1 text-theme-wall-shadow/60 cursor-not-allowed"
                title="Coming soon">View</span
            >
        </div>
    </div>
    <div class="flex items-center gap-2">
        <!-- Debug Toggle -->
        <button
            class="px-2 py-1 text-xs rounded border transition-all duration-150 shadow-sm
                   {debugMode
                ? 'bg-yellow-500/30 text-yellow-400 border-yellow-500 shadow-yellow-500/20'
                : 'bg-theme-wall/80 text-theme-hair/60 border-theme-wall-shadow hover:bg-theme-wall hover:text-yellow-400 hover:border-yellow-500/50 active:scale-95'}"
            on:click={() => (debugMode = !debugMode)}
            title="Toggle Debug Panel"
        >
            🐛
        </button>

        <button
            class="px-3 py-1 text-xs rounded border transition-all duration-150 shadow-sm
                   bg-theme-skin/20 hover:bg-theme-skin/40 active:bg-theme-skin/60 active:scale-95
                   text-theme-skin border-theme-skin/50 hover:border-theme-skin"
            on:click={handleExport}
        >
            Export WAV
        </button>
    </div>
</div>

    <!-- Debug Panel (Overlay - Right side, below top bar) -->
    {#if debugMode}
    <div
        class="fixed top-14 right-2 w-80 max-h-[85vh] bg-zinc-900/98 border border-yellow-500/50 rounded-lg z-[100] overflow-y-auto text-xs text-yellow-100 shadow-xl backdrop-blur-sm flex flex-col"
    >
        <div
            class="p-2 border-b border-yellow-500/30 flex justify-between items-center sticky top-0 bg-zinc-900"
        >
            <span class="font-bold text-yellow-400">🐛 Debug Panel</span>
            <div class="flex gap-1">
                <button
                    class="px-2 py-0.5 bg-yellow-500/20 hover:bg-yellow-500/40 rounded"
                    on:click={debugRefresh}>Refresh</button
                >
                <button
                    class="px-2 py-0.5 bg-yellow-500/20 hover:bg-yellow-500/40 rounded"
                    on:click={debugLogState}>Log</button
                >
            </div>
        </div>

        <div class="p-2 space-y-2">
            <!-- State Overview -->
            {#if import.meta.env.DEV}
            <div class="border border-yellow-500/20 rounded p-2">
                 <div class="font-bold text-yellow-400 mb-1">Testing</div>
                 <label class="flex items-center gap-2 cursor-pointer text-yellow-100 hover:text-yellow-400 transition-colors">
                     <input type="checkbox" bind:checked={$mockEnabled} class="accent-yellow-500">
                     Simulate Meter Data (Mock)
                 </label>
            </div>
            {/if}
            
            <div class="border border-yellow-500/20 rounded p-2">
                <div class="font-bold text-yellow-400 mb-1">State</div>
                <div>
                    Playing: <span
                        class={audioContext.isPlaying
                            ? "text-green-400"
                            : "text-red-400"}>{audioContext.isPlaying}</span
                    >
                </div>
                <div>Playhead: {audioContext.playheadPosition.toFixed(3)}s</div>
                <div>Tempo: {audioContext.tempo} BPM</div>
                <div>
                    Looping: {audioContext.isLooping
                        ? `${audioContext.loopStart.toFixed(1)}-${audioContext.loopEnd.toFixed(1)}s`
                        : "OFF"}
                </div>
                <div>AutoScroll: {audioContext.autoScroll}</div>
            </div>

            <!-- Synth Params -->
            <div class="border border-yellow-500/20 rounded p-2">
                <div class="font-bold text-yellow-400 mb-1">Synth</div>
                <div>
                    A:{audioContext.synthParams.attack.toFixed(2)} D:{audioContext.synthParams.decay.toFixed(
                        2,
                    )} S:{audioContext.synthParams.sustain.toFixed(2)} R:{audioContext.synthParams.release.toFixed(
                        2,
                    )}
                </div>
            </div>

            <!-- Tracks -->
            <div class="border border-yellow-500/20 rounded p-2">
                <div class="font-bold text-yellow-400 mb-1">
                    Tracks ({audioContext.tracks.length})
                </div>
                {#each audioContext.tracks as track}
                    <div
                        class="pl-2 border-l border-yellow-500/20 mb-1 {track.id ===
                        audioContext.selectedTrackId
                            ? 'border-l-yellow-400'
                            : ''}"
                    >
                        <div class="font-medium">
                            [{track.id}] {track.name} ({track.kind})
                        </div>
                        <div class="text-yellow-100/60">
                            Vol:{track.volume.toFixed(2)} Pan:{track.pan.toFixed(
                                2,
                            )}
                            {track.muted ? "🔇" : ""}{track.soloed ? "🔊" : ""}
                        </div>
                        {#if track.kind === "Midi" && track.content && "Midi" in track.content}
                            <div class="text-yellow-100/40">
                                Notes: {track.content.Midi.notes.length}
                            </div>
                        {/if}
                    </div>
                {/each}
            </div>

            <!-- Meter Store Debug -->
            <div class="border border-yellow-500/20 rounded p-2">
                <div class="font-bold text-yellow-400 mb-1">Meter State</div>
                <div>Master: L:{$meterStore.master_peak[0].toFixed(3)} R:{$meterStore.master_peak[1].toFixed(3)}</div>
                <div>Track IDs: {Object.keys($meterStore.track_peaks).join(', ')}</div>
            </div>

            <!-- Raw JSON (Audio) -->
            <details class="border border-yellow-500/20 rounded">
                <summary class="p-2 cursor-pointer text-yellow-400 font-bold"
                    >Raw JSON (Audio)</summary
                >
                <pre
                    class="p-2 text-[10px] overflow-x-auto max-h-40 overflow-y-auto bg-black/50">{JSON.stringify(
                        audioContext, // Note: Circular structures in Runes? Probably fine but might need snapshot
                        null,
                        2,
                    )}</pre>
            </details>
             <details class="border border-yellow-500/20 rounded">
                <summary class="p-2 cursor-pointer text-yellow-400 font-bold"
                    >Raw JSON (Meter)</summary
                >
                <pre
                    class="p-2 text-[10px] overflow-x-auto max-h-40 overflow-y-auto bg-black/50">{JSON.stringify(
                        $meterStore,
                        null,
                        2,
                    )}</pre>
            </details>
        </div>
    </div>
{/if}
