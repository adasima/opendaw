<script lang="ts">
    import { onMount } from 'svelte';
    import { isSettingsOpen } from '../stores/settings';
    import { t, locale } from '../stores/i18n';
    import { audioContext } from '../audio/AudioContext.svelte';
    import { meterStore } from '../stores/meter';
    import { uiState } from '../stores/UIState.svelte';
    import LevelMeter from './LevelMeter.svelte';
    import KeyboardVisualizer from './KeyboardVisualizer.svelte';
    import { keybindStore } from '../stores/KeybindStore.svelte';
    
    let activeTab = 'general';
    let loadingDevices = false;
    let deviceError = '';
    
    onMount(async () => {
        loadingDevices = true;
        try {
            await audioContext.loadOutputDevices();
        } catch (e) {
            deviceError = String(e);
        } finally {
            loadingDevices = false;
        }
    });
    
    function close() {
        isSettingsOpen.set(false);
    }

    function changeLanguage(event: Event) {
        const target = event.target as HTMLSelectElement;
        locale.set(target.value);
    }

    function changeDevice(event: Event) {
        const target = event.target as HTMLSelectElement;
        audioContext.setOutputDevice(target.value);
    }

    function handleKeydown(event: KeyboardEvent) {
        if (event.key === 'Escape') close();
    }
</script>

<svelte:window onkeydown={handleKeydown} />

<!-- Backdrop -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div 
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm animate-in fade-in duration-200"
    role="dialog"
    aria-modal="true"
    onclick={close}
>
    <!-- Modal Window -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div 
        class="w-[800px] h-[600px] bg-theme-shirt rounded-xl shadow-2xl flex border border-theme-skin-shadow overflow-hidden relative animate-in zoom-in-95 duration-200"
        onclick={(e) => e.stopPropagation()}
    >
        <!-- Sidebar -->
        <div class="w-48 bg-theme-hair/40 border-r border-theme-skin-shadow flex flex-col p-4 space-y-2 select-none">
            <h2 class="text-theme-skin font-bold text-lg mb-6 pl-2 tracking-wide flex items-center gap-2">
                <span>⚙️</span> Settings
            </h2>
            
            <button 
                class="px-3 py-2 text-left rounded-lg text-sm font-medium transition-all duration-200 
                       {activeTab === 'general' ? 'bg-theme-skin text-theme-hair shadow-md scale-105' : 'text-theme-wall-shadow hover:bg-theme-skin/10 hover:text-theme-wall-light'}" 
                onclick={() => activeTab = 'general'}
            >
                {$t("menu.view") || "General"}
            </button>
            <button 
                class="px-3 py-2 text-left rounded-lg text-sm font-medium transition-all duration-200 
                       {activeTab === 'audio' ? 'bg-theme-skin text-theme-hair shadow-md scale-105' : 'text-theme-wall-shadow hover:bg-theme-skin/10 hover:text-theme-wall-light'}" 
                onclick={() => activeTab = 'audio'}
            >
                Audio
            </button>
            <button 
                class="px-3 py-2 text-left rounded-lg text-sm font-medium transition-all duration-200 
                       {activeTab === 'theme' ? 'bg-theme-skin text-theme-hair shadow-md scale-105' : 'text-theme-wall-shadow hover:bg-theme-skin/10 hover:text-theme-wall-light'}" 
                onclick={() => activeTab = 'theme'}
            >
                Theme
            </button>
            <button 
                class="px-3 py-2 text-left rounded-lg text-sm font-medium transition-all duration-200 
                       {activeTab === 'shortcuts' ? 'bg-theme-skin text-theme-hair shadow-md scale-105' : 'text-theme-wall-shadow hover:bg-theme-skin/10 hover:text-theme-wall-light'}" 
                onclick={() => activeTab = 'shortcuts'}
            >
                ⌨️ Shortcuts
            </button>
        </div>
        
        <!-- Content Area -->
        <div class="flex-1 p-8 overflow-y-auto bg-theme-shirt/90 scrollbar-thin scrollbar-thumb-theme-skin-shadow scrollbar-track-transparent">
            
            <!-- Close Button (Absolute) -->
            <button 
                class="absolute top-4 right-4 w-8 h-8 flex items-center justify-center rounded-full text-theme-wall-shadow hover:text-theme-hair hover:bg-theme-skin transition-colors"
                onclick={close}
                title="Close"
            >
                ✕
            </button>

            {#if activeTab === 'general'}
                <div class="space-y-8 animate-in slide-in-from-right-4 duration-300">
                    <div>
                        <h3 class="text-2xl font-bold text-theme-wall-light mb-1">General</h3>
                        <p class="text-theme-wall-shadow text-xs">Application global preferences.</p>
                    </div>
                    <div class="h-px bg-theme-skin-shadow/30 w-full"></div>
                    
                    <div class="space-y-4">
                        <div class="flex flex-col space-y-2">
                            <label class="text-sm font-medium text-theme-wall-light">Language</label>
                            <select 
                                class="w-64 p-2 bg-theme-hair text-theme-wall-light rounded border border-theme-skin-shadow focus:border-theme-skin outline-none transition-colors"
                                value={$locale}
                                onchange={changeLanguage}
                            >
                                <option value="ja">🇯🇵 Japanese (日本語)</option>
                                <option value="en">🇺🇸 English</option>
                            </select>
                            <p class="text-xs text-theme-wall-shadow opacity-70">Changes are applied immediately.</p>
                        </div>
                    </div>
                </div>
            {/if}
            
            {#if activeTab === 'audio'}
                <div class="space-y-8 animate-in slide-in-from-right-4 duration-300">
                     <div>
                        <h3 class="text-2xl font-bold text-theme-wall-light mb-1">Audio</h3>
                        <p class="text-theme-wall-shadow text-xs">Device and latency settings.</p>
                    </div>
                    <div class="h-px bg-theme-skin-shadow/30 w-full"></div>

                    <div class="space-y-4">
                        <div class="flex flex-col space-y-2">
                            <label class="text-sm font-medium text-theme-wall-light">Output Device</label>
                            {#if loadingDevices}
                                <p class="text-xs text-theme-wall-shadow">Loading devices...</p>
                            {:else if deviceError}
                                <p class="text-xs text-red-400">{deviceError}</p>
                            {:else}
                                <select 
                                    class="w-full p-2 bg-theme-hair text-theme-wall-light rounded border border-theme-skin-shadow focus:border-theme-skin outline-none transition-colors"
                                    value={audioContext.selectedDeviceName}
                                    onchange={changeDevice}
                                >
                                    {#each audioContext.outputDevices as device}
                                        <option value={device.name}>
                                            {device.name} {device.is_default ? '(Default)' : ''}
                                        </option>
                                    {/each}
                                </select>
                                <p class="text-xs text-theme-wall-shadow opacity-70">
                                    Selecting a device will restart the audio engine.
                                </p>
                            {/if}
                        </div>
                        
                        <!-- Meter Bridge Test -->
                         <div class="flex flex-col space-y-2 pt-4">
                            <label class="text-sm font-medium text-theme-wall-light">Signal Test (Master)</label>
                            <div class="w-full h-8 flex flex-col gap-[2px] bg-theme-hair/50 p-1 rounded border border-theme-skin-shadow/30">
                                <LevelMeter value={$meterStore.master_peak[0] || 0} vertical={false} />
                                <LevelMeter value={$meterStore.master_peak[1] || 0} vertical={false} />
                            </div>
                            <p class="text-[10px] text-theme-wall-shadow font-mono">
                                Peak L: {$meterStore.master_peak[0]?.toFixed(3) ?? '0.000'} | R: {$meterStore.master_peak[1]?.toFixed(3) ?? '0.000'}
                            </p>
                        </div>
                    </div>

                    <div class="p-4 bg-theme-hair/30 rounded border border-theme-skin-shadow/20 border-l-4 border-l-theme-skin">
                        <h4 class="text-theme-skin font-bold text-sm mb-1">Buffer Size & Latency</h4>
                        <p class="text-xs text-theme-wall-shadow">
                            Buffer size configuration is managed automatically by CPAL default config.
                        </p>
                    </div>

                    <!-- Meter Settings -->
                    <div class="space-y-4 pt-4 border-t border-theme-skin-shadow/30">
                        <h4 class="text-lg font-bold text-theme-wall-light">Metering</h4>
                        <div class="flex flex-col space-y-2">
                             <label class="text-sm font-medium text-theme-wall-light">Meter Mode</label>
                             <select
                                class="w-full p-2 bg-theme-hair text-theme-wall-light rounded border border-theme-skin-shadow focus:border-theme-skin outline-none"
                                value={uiState.meterMode}
                                onchange={(e) => uiState.meterMode = e.currentTarget.value as any}
                             >
                                <option value="standard">DTM Standard (0dB = Clip)</option>
                                <option value="broadcast">Broadcast Strict (-2dB = Red)</option>
                             </select>
                             <p class="text-xs text-theme-wall-shadow opacity-70">
                                Changes the color scale and warning levels on the Master Meter.
                             </p>
                        </div>
                    </div>
                </div>
            {/if}
            
             {#if activeTab === 'theme'}
                <div class="space-y-8 animate-in slide-in-from-right-4 duration-300">
                     <div>
                        <h3 class="text-2xl font-bold text-theme-wall-light mb-1">Theme & Layout</h3>
                        <p class="text-theme-wall-shadow text-xs">Customize the look and feel.</p>
                    </div>
                    <div class="h-px bg-theme-skin-shadow/30 w-full"></div>

                    <div class="p-4 bg-theme-hair/30 rounded border border-theme-skin-shadow/20 border-l-4 border-l-theme-skin">
                         <h4 class="text-theme-skin font-bold text-sm mb-1">Coming Soon</h4>
                         <p class="text-xs text-theme-wall-shadow">
                             Advanced layout editor and theme switching will be available here.
                         </p>
                    </div>

                    <!-- Placeholder for Layout Reset -->
                    <div class="pt-4">
                        <button class="px-4 py-2 bg-theme-skin-shadow text-theme-wall-light text-sm rounded hover:bg-theme-skin hover:text-theme-hair transition-colors shadow">
                            Reset Layout
                        </button>
                    </div>
                </div>
            {/if}

            {#if activeTab === 'shortcuts'}
                <div class="space-y-6 animate-in slide-in-from-right-4 duration-300">
                     <div>
                        <h3 class="text-2xl font-bold text-theme-wall-light mb-1">Keyboard Shortcuts</h3>
                        <p class="text-theme-wall-shadow text-xs">キーをクリックして割り当てを変更。右クリックで解除。</p>
                    </div>
                    <div class="h-px bg-theme-skin-shadow/30 w-full"></div>

                    <div class="overflow-x-auto">
                        <KeyboardVisualizer />
                    </div>

                    <div class="flex gap-2">
                        <button 
                            class="px-4 py-2 bg-theme-skin-shadow text-theme-wall-light text-sm rounded hover:bg-theme-skin hover:text-theme-hair transition-colors shadow"
                            onclick={() => keybindStore.resetToDefaults()}
                        >
                            デフォルトに戻す
                        </button>
                    </div>
                </div>
            {/if}
        </div>
    </div>
</div>

<style>
    .backdrop-blur-sm {
        backdrop-filter: blur(4px);
    }
</style>
