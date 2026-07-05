<script lang="ts">
    import { uiState } from '../stores/UIState.svelte';

    export let left = 0;
    export let right = 0;

    // dB Constants
    const MIN_DB = -60;
    const MAX_DB = 0; // 0dBFS ceiling

    // Utils
    function toDb(amp: number): number {
        if (amp <= 0.001) return MIN_DB;
        let db = 20 * Math.log10(amp);
        return Math.max(MIN_DB, Math.min(MAX_DB + 2, db)); // Allow +2 for clip detection
    }

    function dbToHeight(db: number): number {
        const h = ((db - MIN_DB) / (MAX_DB - MIN_DB)) * 100;
        return Math.max(0, Math.min(100, h));
    }

    // Reactive Levels
    let dbL = 0;
    let dbR = 0;
    $: dbL = toDb(left);
    $: dbR = toDb(right);

    // Peak Hold Logic
    let peakL = MIN_DB;
    let peakR = MIN_DB;
    let peakTimerL: number;
    let peakTimerR: number;

    $: if (dbL > peakL) {
        peakL = dbL;
        clearTimeout(peakTimerL);
        peakTimerL = setTimeout(() => peakL = dbL, 2000) as unknown as number; // Fallback after 2s? Or decay?
        // Simple fallback logic: reset to current after 2s
    }
    $: if (dbR > peakR) {
        peakR = dbR;
        clearTimeout(peakTimerR);
        peakTimerR = setTimeout(() => peakR = dbR, 2000) as unknown as number;
    }

    // Clip Indicator
    let clippedL = false;
    let clippedR = false;
    $: if (dbL > 0) clippedL = true;
    $: if (dbR > 0) clippedR = true;

    function resetClip() {
        clippedL = false;
        clippedR = false;
        peakL = MIN_DB;
        peakR = MIN_DB;
    }

    // Mode Settings
    $: colors = uiState.meterMode === 'broadcast' 
        ? { // Strict: -18(G), -9(Y), -2(R)
            greenLimit: -18,
            yellowLimit: -9,
            redLimit: -2
          }
        : { // Standard: -12(G), -3(Y), 0(R)
            greenLimit: -12, // Below -12 is green? No, usually -inf to -12 Green? 
            // Standard: Signal usually sits around -12 to -6.
            // Let's interpret: Green up to -12, Yellow -12 to -3, Red > -3.
            transition1: -12,
            transition2: -3
          };

    // Gradient stops calculation is simpler if we use CSS segments or a linear-gradient with hard stops.
    // But dynamic stops based on mode is tricky with Tailwind.
    // I'll use inline style for background gradient.
    
    function getGradient(mode: 'standard' | 'broadcast') {
        // Map dB to %
        // height 0% = -60dB, 100% = 0dB
        const t1 = dbToHeight(mode === 'broadcast' ? -18 : -12); // Green end / Yellow start
        const t2 = dbToHeight(mode === 'broadcast' ? -9 : -3);   // Yellow end / Red start
        
        // Colors
        // Green: #22c55e, Yellow: #eab308, Red: #ef4444
        return `linear-gradient(to top, 
            #22c55e 0%, 
            #22c55e ${t1}%, 
            #eab308 ${t1}%, 
            #eab308 ${t2}%, 
            #ef4444 ${t2}%, 
            #ef4444 100%)`;
    }

    $: bgStyle = getGradient(uiState.meterMode);

</script>

<div class="flex flex-row h-full w-14 gap-1 select-none bg-black p-1 rounded border border-theme-wall-shadow">
    <!-- RULER (Left) -->
    <div class="relative w-4 h-full text-[9px] text-gray-400 font-mono">
        <!-- Ticks -->
        <div class="absolute top-0 right-0 w-full border-t border-gray-600"></div> <span class="absolute -top-1.5 right-1">0</span>
        <div class="absolute top-[10%] right-0 w-1/2 border-t border-gray-700"></div> <!-- -6dB approx? 0dB is top. -60dB bottom. 
             If Linear dB:
             0dB = 100%
             -6dB = 90%
             -12dB = 80%
             ...
             Wait, linear dB means uniform spacing.
             Range 60dB. 
             0 at top (0px or 100% height? 100% height in CSS usually grows up if flex-col reverse or using bottom positioning).
             Let's use percent from BOTTOM.
        -->
        
        {#each [0, -6, -12, -24, -48] as tick}
             {@const bottomPct = ((tick - MIN_DB) / (MAX_DB - MIN_DB)) * 100}
             <div class="absolute right-0 w-1.5 border-t border-gray-500" style:bottom="{bottomPct}%"></div>
             <span class="absolute right-2 text-[8px] leading-none" style:bottom="{bottomPct - 2}%">{Math.abs(tick)}</span>
        {/each}
    </div>

    <!-- METERS Container -->
    <div class="flex-1 flex gap-0.5 bg-gray-900 relative" on:click={resetClip}>
        
        <!-- Clip Indicators -->
        <div class="absolute -top-1 left-0 w-full flex gap-0.5">
             <div class="flex-1 h-3 transition-colors duration-100 {clippedL ? 'bg-red-500 shadow-[0_0_5px_red]' : 'bg-gray-800/50'}"></div>
             <div class="flex-1 h-3 transition-colors duration-100 {clippedR ? 'bg-red-500 shadow-[0_0_5px_red]' : 'bg-gray-800/50'}"></div>
        </div>

        <!-- Left Bar -->
        <div class="flex-1 relative h-full mt-2.5 overflow-hidden bg-gray-800">
             <div class="w-full absolute bottom-0 transition-height duration-75 ease-out"
                  style:height="{dbToHeight(dbL)}%"
                  style:background={bgStyle}>
             </div>
             <!-- Peak Line -->
             <div class="w-full h-[1px] bg-white absolute transition-bottom duration-300"
                  style:bottom="{dbToHeight(peakL)}%">
             </div>
        </div>

        <!-- Right Bar -->
        <div class="flex-1 relative h-full mt-2.5 overflow-hidden bg-gray-800">
             <div class="w-full absolute bottom-0 transition-height duration-75 ease-out"
                  style:height="{dbToHeight(dbR)}%"
                  style:background={bgStyle}>
             </div>
             <!-- Peak Line -->
             <div class="w-full h-[1px] bg-white absolute transition-bottom duration-300"
                  style:bottom="{dbToHeight(peakR)}%">
             </div>
        </div>
    </div>
</div>
