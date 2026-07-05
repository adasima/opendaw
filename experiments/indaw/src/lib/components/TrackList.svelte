<script lang="ts">
    import { audioContext } from "../audio/AudioContext.svelte";
    import { meterStore } from "../stores/meter";
    import LevelMeter from "./LevelMeter.svelte";
    // import { slide } from "svelte/transition"; // Unused

    function handleSelect(id: number) {
        audioContext.selectTrack(id);
    }

    function handleMute(id: number, e: MouseEvent) {
        e.stopPropagation();
        audioContext.toggleTrackMute(id);
    }

    function handleSolo(id: number, e: MouseEvent) {
        e.stopPropagation();
        audioContext.toggleTrackSolo(id);
    }

    function handleVolume(id: number, e: Event) {
        const val = parseFloat((e.target as HTMLInputElement).value);
        audioContext.setTrackVolume(id, val);
    }
</script>

<div class="w-full h-full flex flex-col bg-theme-wall-light overflow-y-auto">
    <div
        class="p-2 text-xs font-bold text-theme-hair uppercase tracking-widest border-b border-theme-wall-shadow"
    >
        Tracks
    </div>

    {#each audioContext.tracks as track}
        <div
            class="flex flex-col border-b border-theme-wall-shadow p-2 cursor-pointer transition-colors
               {track.id === audioContext.selectedTrackId
                ? 'bg-theme-wall/50 border-l-4 border-l-theme-skin'
                : 'bg-transparent hover:bg-theme-wall/20 border-l-4 border-l-transparent'}"
            on:click={() => handleSelect(track.id)}
            on:keydown={(e) => {
                if (e.key === "Enter") handleSelect(track.id);
            }}
            role="button"
            tabindex="0"
        >
            <div class="flex justify-between items-center mb-2">
                <span class="text-sm font-medium text-theme-hair truncate"
                    >{track.name}</span
                >
                <span
                    class="text-xs text-theme-hair/50 px-1 border border-theme-hair/30 rounded"
                    >{track.kind}</span
                >
            </div>

            <div class="flex gap-2 mb-2">
                <button
                    class="px-2 py-0.5 text-xs rounded border
                       {track.muted
                        ? 'bg-red-500/20 text-red-500 border-red-500'
                        : 'bg-theme-wall text-theme-hair/70 border-theme-wall-shadow hover:bg-theme-wall'}"
                    on:click={(e) => handleMute(track.id, e)}>M</button
                >
                <button
                    class="px-2 py-0.5 text-xs rounded border
                       {track.soloed
                        ? 'bg-yellow-500/20 text-yellow-500 border-yellow-500'
                        : 'bg-theme-wall text-theme-hair/70 border-theme-wall-shadow hover:bg-theme-wall'}"
                    on:click={(e) => handleSolo(track.id, e)}>S</button
                >
            </div>

            <div class="flex items-center gap-2">
                <!-- Meters: Still using legacy meterStore for now -->
                <div class="h-8 w-4 flex gap-[1px] shrink-0 bg-theme-hair/10 p-[1px] rounded overflow-hidden">
                     <LevelMeter value={$meterStore.track_peaks[track.id]?.[0] || 0} vertical={true} />
                     <LevelMeter value={$meterStore.track_peaks[track.id]?.[1] || 0} vertical={true} />
                </div>

                <span class="text-[10px] text-theme-hair/60 w-4">Vol</span>
                <input
                    type="range"
                    min="0"
                    max="1.5"
                    step="0.01"
                    value={track.volume}
                    on:input={(e) => handleVolume(track.id, e)}
                    on:click|stopPropagation
                    class="flex-1 h-1 appearance-none bg-theme-wall rounded-full accent-theme-skin"
                />
            </div>
        </div>
    {/each}

    {#if audioContext.tracks.length === 0}
        <div class="p-4 text-xs text-center text-theme-hair/50">
            No tracks. Add one!
        </div>
    {/if}
</div>
