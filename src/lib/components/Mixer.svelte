<script lang="ts">
    import { audioContext } from "../audio/AudioContext.svelte";
    import ChannelStrip from "./ChannelStrip.svelte";
</script>

<div class="flex h-full w-full bg-theme-panel-bg p-4 gap-2 overflow-x-auto items-start">
    <!-- Tracks -->
    {#each audioContext.tracks as track (track.id)}
        <ChannelStrip {track} />
    {/each}

    <!-- Spacer / Divider -->
    <div class="w-4 h-full border-l border-theme-skin-shadow/30 mx-2"></div>

    <!-- Master -->
    <ChannelStrip isMaster={true} />
</div>

