<script lang="ts">
    import { audioContext } from "../audio/AudioContext.svelte";
    import type { TrackSnapshot } from "../bindings";
    import { meterStore } from "../stores/meter";
    import LevelMeter from "./LevelMeter.svelte";
    import MasterMeter from "./MasterMeter.svelte";

    export let track: TrackSnapshot | null = null;
    export let isMaster = false;

    // Local getters/setters for binding
    // Volume logic: 0.0 to 1.0 (or logarithmic later)
    
    function handleVolume(e: any) {
        const val = parseFloat(e.target.value);

        if (isMaster) {
            // Master volume not implemented in AudioEngine per track logic?
            // Usually trackId 0 or special handling.
            // For now, ignore master volume or implement global gain command.
            // Let's assume Master is just a visual aggregator or we implement set_master_volume
        } else if (track) {
            audioContext.setTrackVolume(track.id, val);
        }
    }

    function handlePan(e: any) {
        const val = parseFloat(e.target.value);
        if (track && !isMaster) {
            audioContext.setTrackPan(track.id, val);
        }
    }

    function toggleMute() {
        if (track && !isMaster) audioContext.toggleTrackMute(track.id);
    }
    
    function toggleSolo() {
        if (track && !isMaster) audioContext.toggleTrackSolo(track.id);
    }
    
    // Meter value
    let peakL = 0;
    let peakR = 0;
    
    // Reactive meter update
    $: if (isMaster) {
        peakL = $meterStore.master_peak[0];
        peakR = $meterStore.master_peak[1];
    } else if (track) {
        const p = $meterStore.track_peaks[track.id];
        if (p) {
            peakL = p[0];
            peakR = p[1];
        } else {
            peakL = 0;
            peakR = 0;
        }
    }
</script>

<div class="w-24 h-full bg-theme-wall border-r border-theme-skin-shadow flex flex-col p-2 gap-2 shadow-xl shrink-0">
    <!-- Meter bridge / Peak value -->
    <div class="text-[10px] text-theme-wall-shadow font-mono text-center h-4">
        {Math.max(peakL, peakR).toFixed(2)}
    </div>

    <!-- Meter + Fader Container -->
    <div class="flex-1 flex gap-2 justify-center relative bg-black/20 rounded p-1">
         {#if isMaster}
             <!-- Master Meter + Fader -->
             <div class="h-full flex gap-1 items-center justify-center">
                 <MasterMeter left={peakL} right={peakR} />
                 <!-- Master Fader -->
                 <div class="relative w-6 h-full"> 
                    <input 
                        type="range"
                        min="0" max="1" step="0.01"
                        value={audioContext.masterVolume}
                        oninput={(e) => audioContext.setMasterVolume(parseFloat(e.currentTarget.value))}
                        class="track-fader"
                        title="Master Volume"
                    />
                 </div>
             </div>
         {:else}
             <!-- Stereo Meter -->
             <div class="h-full flex gap-[1px]">
                 <LevelMeter value={peakL} vertical={true} />
                 <LevelMeter value={peakR} vertical={true} />
             </div>
             
             <!-- Fader -->
             <div class="relative w-8 h-full"> 
                <input 
                    type="range"
                    min="0" max="1" step="0.01"
                    value={track ? track.volume : 1.0}
                    oninput={handleVolume}
                    class="track-fader"
                />
             </div>
         {/if}
    </div>

    <!-- Pan -->
    {#if !isMaster && track}
        <div class="h-8 w-full flex items-center justify-center">
             <input 
                type="range"
                min="-1" max="1" step="0.05"
                value={track.pan}
                oninput={handlePan}
                class="w-full h-1 bg-theme-skin-shadow rounded-full appearance-none [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:w-3 [&::-webkit-slider-thumb]:h-3 [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-theme-wall-light"
                title="Pan"
             />
        </div>
    {/if}

    <!-- Buttons -->
    {#if !isMaster && track}
        <div class="flex gap-1 h-6">
            <button 
                class="flex-1 text-[10px] font-bold rounded border border-theme-skin-shadow transition-colors {track.muted ? 'bg-red-500 text-white border-red-600' : 'text-theme-wall-shadow hover:text-theme-wall-light bg-theme-shirt'}"
                onclick={toggleMute}
            >
                M
            </button>
            <button 
                class="flex-1 text-[10px] font-bold rounded border border-theme-skin-shadow transition-colors {track.soloed ? 'bg-yellow-500 text-black border-yellow-600' : 'text-theme-wall-shadow hover:text-theme-wall-light bg-theme-shirt'}"
                onclick={toggleSolo}
            >
                S
            </button>
        </div>
    {/if}
    
    <!-- Name -->
    <div class="h-6 flex items-center justify-center bg-black/20 rounded border border-theme-skin-shadow/30">
        <span class="text-xs truncate px-1 font-bold {isMaster ? 'text-theme-skin' : 'text-theme-wall-light'}">
            {isMaster ? "MASTER" : (track ? track.name : "")}
        </span>
    </div>
</div>

<style>
    /* Custom vertical slider */
    .track-fader {
        -webkit-appearance: none; /* Override default look */
        appearance: none;
        width: 100%; /* Height becomes width due to rotation if done via CSS, but normally input[type=range][orient=vertical] is not standard across browsers. */
        height: 100%;
        background: transparent;
        writing-mode: bt-lr; /* IE/Edge */
        -webkit-appearance: slider-vertical; /* WebKit */
        outline: none;
    }
</style>
