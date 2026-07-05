<script lang="ts">
    import { audioContext } from "../audio/AudioContext.svelte";
    import { onMount } from "svelte";

    // --- PRESET MANAGEMENT ---
    interface SynthPreset {
        name: string;
        params: {
            attack: number;
            decay: number;
            sustain: number;
            release: number;
            oscillator_type?: string; 
        };
    }

    const DEFAULT_PRESETS: SynthPreset[] = [
        {
            name: "Default Basic",
            params: { attack: 0.05, decay: 0.1, sustain: 0.7, release: 0.2 },
        },
        {
            name: "Pluck (Short)",
            params: { attack: 0.01, decay: 0.2, sustain: 0.0, release: 0.1 },
        },
        {
            name: "Soft Pad",
            params: { attack: 0.8, decay: 0.5, sustain: 0.8, release: 1.0 },
        },
        {
            name: "Hard Lead",
            params: { attack: 0.01, decay: 0.1, sustain: 1.0, release: 0.1 },
        },
        {
            name: "Slow Swell",
            params: { attack: 1.5, decay: 0.1, sustain: 1.0, release: 1.5 },
        },
    ];

    let presets: SynthPreset[] = [...DEFAULT_PRESETS];
    let selectedPresetName = "";
    let newPresetName = "";

    onMount(() => {
        const saved = localStorage.getItem("userPresets");
        if (saved) {
            try {
                const userPresets = JSON.parse(saved);
                presets = [...DEFAULT_PRESETS, ...userPresets];
            } catch (e) {
                console.error("Failed to load presets", e);
            }
        }
    });

    function saveUserPresets() {
        const userPresets = presets.filter(
            (p) => !DEFAULT_PRESETS.find((dp) => dp.name === p.name),
        );
        localStorage.setItem("userPresets", JSON.stringify(userPresets));
    }

    function loadPreset(name: string) {
        const p = presets.find((x) => x.name === name);
        if (p) {
            attack = p.params.attack;
            decay = p.params.decay;
            sustain = p.params.sustain;
            release = p.params.release;
            updateAdsr();
            selectedPresetName = name;
        }
    }

    function savePreset() {
        if (!newPresetName) return;
        if (presets.find((p) => p.name === newPresetName)) {
            if (!confirm("Overwrite existing preset?")) return;
            presets = presets.filter((p) => p.name !== newPresetName);
        }
        presets = [
            ...presets,
            {
                name: newPresetName,
                params: { attack, decay, sustain, release },
            },
        ];
        saveUserPresets();
        selectedPresetName = newPresetName;
        newPresetName = "";
    }

    function deletePreset() {
        if (!selectedPresetName) return;
        if (DEFAULT_PRESETS.find((p) => p.name === selectedPresetName)) {
            alert("Cannot delete default presets.");
            return;
        }
        if (confirm(`Delete preset '${selectedPresetName}'?`)) {
            presets = presets.filter((p) => p.name !== selectedPresetName);
            saveUserPresets();
            selectedPresetName = "";
        }
    }

    // --- ADSR BINDING ---
    // Initialize from store safely
    let attack = 0.05,
        decay = 0.1,
        sustain = 0.7,
        release = 0.2;

    // Subscribe to store updates (one-way sync from store -> local, if store changes externally)
    $: if (audioContext.synthParams) {
        // Only update local if drastically different (to avoid loop/jitter) or if not user interacting?
        // Simple sync for now
    }

    // Sync On Mount
    onMount(() => {
        if (audioContext.synthParams) {
            attack = audioContext.synthParams.attack;
            decay = audioContext.synthParams.decay;
            sustain = audioContext.synthParams.sustain;
            release = audioContext.synthParams.release;
        }
    });

    function updateAdsr() {
        audioContext.setAdsr(
            Number(attack),
            Number(decay),
            Number(sustain),
            Number(release),
        );
    }
</script>

<div
    class="w-full h-full bg-theme-hair border-l border-theme-skin-shadow flex flex-col select-none"
>
    <div
        class="p-2 border-b border-theme-skin-shadow font-semibold text-xs text-theme-bg uppercase tracking-wider"
    >
        Inspector
    </div>
    <div class="p-4 space-y-6">
        <!-- PRESETS -->
        <div class="space-y-3 pb-4 border-b border-theme-skin-shadow/30">
            <h3 class="text-xs font-bold text-theme-wall-shadow uppercase">
                Presets
            </h3>
            <select
                class="w-full bg-theme-shirt text-theme-wall-light text-xs p-1 rounded border border-theme-skin-shadow outline-none"
                bind:value={selectedPresetName}
                on:change={() => loadPreset(selectedPresetName)}
            >
                <option value="" disabled>Select Preset...</option>
                {#each presets as p}
                    <option value={p.name}>{p.name}</option>
                {/each}
            </select>

            <div class="flex space-x-2">
                <input
                    type="text"
                    placeholder="New Preset Name"
                    bind:value={newPresetName}
                    class="flex-1 bg-theme-shirt text-theme-wall-light text-xs p-1 rounded border border-theme-skin-shadow outline-none"
                />
                <button
                    on:click={savePreset}
                    disabled={!newPresetName}
                    class="px-2 py-1 bg-theme-skin text-theme-hair text-xs rounded font-bold hover:bg-theme-skin/80 disabled:opacity-50"
                    >Save</button
                >
                <button
                    on:click={deletePreset}
                    disabled={!selectedPresetName}
                    class="px-2 py-1 bg-theme-skin-shadow text-theme-wall-light text-xs rounded hover:bg-red-900 disabled:opacity-50"
                    >Del</button
                >
            </div>
        </div>

        <!-- Global Mix -->
        <div class="space-y-3">
            <h3 class="text-xs font-bold text-theme-wall-shadow uppercase">
                Global Mix
            </h3>
                <label class="text-xs text-theme-wall-shadow flex flex-col space-y-1">
                    <span>Volume</span>
                    <input type="range" class="w-full accent-theme-skin" />
                </label>
        </div>

        <!-- Oscillator Type -->
        <div class="space-y-3">
            <h3 class="text-xs font-bold text-theme-wall-shadow uppercase">
                Oscillator
            </h3>
            <select
                class="w-full bg-theme-shirt text-theme-wall-light text-xs p-1 rounded border border-theme-skin-shadow outline-none"
                value={audioContext.synthParams.oscillator_type}
                on:change={(e) => audioContext.setOscillatorType(e.currentTarget.value as any)}
            >
                <option value="Sine">Sine</option>
                <option value="Square">Square</option>
                <option value="Sawtooth">Sawtooth</option>
                <option value="Triangle">Triangle</option>
            </select>
        </div>

        <!-- Synth ADSR -->
        <div class="space-y-3">
            <h3 class="text-xs font-bold text-theme-wall-shadow uppercase">
                Synth Envelope
            </h3>

            <div class="space-y-1">
                <div
                    class="flex justify-between text-xs text-theme-wall-shadow"
                >
                    <span>Attack</span>
                    <span>{attack.toFixed(2)}s</span>
                </div>
                <input
                    type="range"
                    min="0.01"
                    max="2.0"
                    step="0.01"
                    bind:value={attack}
                    on:input={updateAdsr}
                    class="w-full accent-theme-skin"
                />
            </div>

            <div class="space-y-1">
                <div
                    class="flex justify-between text-xs text-theme-wall-shadow"
                >
                    <span>Decay</span>
                    <span>{decay.toFixed(2)}s</span>
                </div>
                <input
                    type="range"
                    min="0.01"
                    max="2.0"
                    step="0.01"
                    bind:value={decay}
                    on:input={updateAdsr}
                    class="w-full accent-theme-skin"
                />
            </div>

            <div class="space-y-1">
                <div
                    class="flex justify-between text-xs text-theme-wall-shadow"
                >
                    <span>Sustain</span>
                    <span>{sustain.toFixed(2)}</span>
                </div>
                <input
                    type="range"
                    min="0.0"
                    max="1.0"
                    step="0.01"
                    bind:value={sustain}
                    on:input={updateAdsr}
                    class="w-full accent-theme-skin"
                />
            </div>

            <div class="space-y-1">
                <div
                    class="flex justify-between text-xs text-theme-wall-shadow"
                >
                    <span>Release</span>
                    <span>{release.toFixed(2)}s</span>
                </div>
                <input
                    type="range"
                    min="0.01"
                    max="5.0"
                    step="0.01"
                    bind:value={release}
                    on:input={updateAdsr}
                    class="w-full accent-theme-skin"
                />
            </div>
        </div>
    </div>
</div>
