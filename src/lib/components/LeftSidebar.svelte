<script lang="ts">
    // Left Bar: Asset Browser / Plugins
</script>

<div
    class="w-full h-full bg-theme-hair border-r border-theme-skin-shadow flex flex-col select-none shrink-0"
>
    <!-- Plugins Section -->
    <div
        class="p-2 border-b border-theme-skin-shadow/50 font-semibold text-xs text-theme-bg uppercase tracking-wider"
    >
        Plugins
    </div>
    <div class="flex-1 overflow-y-auto p-2 space-y-1">
        {#each Array(5) as _, i}
            <div
                class="p-2 bg-theme-shirt/30 hover:bg-theme-shirt/50 rounded cursor-pointer transition-colors border border-theme-shirt/20"
            >
                <div class="text-sm font-medium text-theme-wall-light">
                    Synthesizer {i + 1}
                </div>
                <div class="text-xs text-theme-wall-shadow">Instrument</div>
            </div>
        {/each}
    </div>

    <!-- Presets Section -->
    <div
        class="p-2 border-t border-theme-skin-shadow/50 font-semibold text-xs text-theme-bg uppercase tracking-wider"
    >
        Presets
    </div>
    <div class="h-1/3 overflow-y-auto p-2 space-y-1">
        {#each Array(5) as _, i}
            <div
                class="px-2 py-1 hover:bg-theme-shirt/40 rounded cursor-pointer text-xs text-theme-wall-light transition-colors"
            >
                Bass Preset {i + 1}
            </div>
        {/each}
    </div>
</div>
