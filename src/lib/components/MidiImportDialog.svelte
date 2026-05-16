<script lang="ts">
    import { onMount, createEventDispatcher } from "svelte";
    import { get } from "svelte/store";
    import { t } from "../stores/i18n";
    import { invoke } from "../api";
    import type { MidiMetadata, ImportOptions } from "../types";
    import { audioContext } from "../audio/AudioContext.svelte";

    export let path: string;
    export let mode: "open" | "import" = "import";

    const dispatch = createEventDispatcher();

    let loading = true;
    let error: string | null = null;
    let metadata: MidiMetadata | null = null;

    // Options
    let bakeSustain = true;
    let bpmMode: "project" | "scale" | "absolute" =
        mode === "open" ? "project" : "scale";

    onMount(async () => {
        try {
            metadata = (await invoke("get_midi_info", {
                path,
            })) as MidiMetadata;
            loading = false;
        } catch (e: any) {
            error = e.toString();
            loading = false;
        }
    });

    function handleImport() {
        const options: ImportOptions = {
            bake_sustain: bakeSustain,
            import_tempo: bpmMode === "project",
            scale_to_bpm: bpmMode === "scale" ? audioContext.tempo : undefined,
        };
        dispatch("confirm", options);
    }

    function handleCancel() {
        dispatch("cancel");
    }
</script>

<div
    class="fixed inset-0 z-[100] flex items-center justify-center bg-black/50 backdrop-blur-sm"
>
    <div
        class="w-96 bg-zinc-800 border border-zinc-700 rounded-lg shadow-2xl overflow-hidden text-sm text-zinc-200"
    >
        <!-- Header -->
        <div
            class="px-4 py-2 bg-zinc-900 border-b border-zinc-700 flex justify-between items-center"
        >
            <span class="font-bold text-zinc-100"
                >{$t("dialog.midi_import_title")}</span
            >
            <button
                class="text-zinc-500 hover:text-zinc-300"
                on:click={handleCancel}>✕</button
            >
        </div>

        <!-- Body -->
        <div class="p-4 space-y-4">
            {#if loading}
                <div
                    class="flex items-center justify-center py-4 text-zinc-400"
                >
                    <span>{$t("dialog.loading_info")}</span>
                </div>
            {:else if error}
                <div
                    class="p-2 bg-red-900/30 border border-red-800 text-red-200 rounded"
                >
                    {$t("dialog.error_title")}: {error}
                </div>
            {:else if metadata}
                <!-- File Info -->
                <div class="space-y-1">
                    <div class="font-bold text-zinc-400 text-xs">
                        {$t("dialog.file_info")}
                    </div>
                    <div class="grid grid-cols-2 gap-2 text-xs">
                        <div class="p-2 bg-zinc-900/50 rounded">
                            <span class="text-zinc-500">File:</span>
                            <span class="text-zinc-300"
                                >{metadata.file_name}</span
                            >
                        </div>
                        <div class="p-2 bg-zinc-900/50 rounded">
                            <span class="text-zinc-500">BPM:</span>
                            <span class="text-zinc-300"
                                >{metadata.initial_bpm.toFixed(1)}</span
                            >
                        </div>
                        <div class="p-2 bg-zinc-900/50 rounded">
                            <span class="text-zinc-500"
                                >{$t("dialog.track_count")}:</span
                            >
                            <span class="text-zinc-300"
                                >{metadata.track_count}</span
                            >
                        </div>
                        <div class="p-2 bg-zinc-900/50 rounded">
                            <span class="text-zinc-500"
                                >{$t("dialog.duration")}:</span
                            >
                            <span class="text-zinc-300"
                                >{metadata.duration.toFixed(1)}s</span
                            >
                        </div>
                    </div>
                </div>

                <!-- Options -->
                <div class="space-y-2 pt-2 border-t border-zinc-700/50">
                    <div class="font-bold text-zinc-400 text-xs">
                        {$t("dialog.options")}
                    </div>

                    <!-- Sustain -->
                    <label
                        class="flex items-center space-x-2 cursor-pointer select-none"
                    >
                        <input
                            type="checkbox"
                            bind:checked={bakeSustain}
                            class="rounded border-zinc-600 bg-zinc-700 text-theme-skin focus:ring-1 focus:ring-theme-skin"
                        />
                        <span>{$t("dialog.bake_sustain")}</span>
                    </label>

                    <!-- BPM Mode -->
                    <div class="space-y-1 mt-2">
                        <div class="text-xs text-zinc-500 mb-1">
                            {$t("dialog.import_mode")}
                        </div>

                        {#if mode === "open"}
                            <label
                                class="flex items-center space-x-2 cursor-pointer"
                            >
                                <input
                                    type="radio"
                                    group={bpmMode}
                                    value="project"
                                    class="text-theme-skin focus:ring-1"
                                    checked
                                    disabled
                                />
                                <span class="text-zinc-400"
                                    >{$t("dialog.mode_project")}</span
                                >
                            </label>
                            <div class="text-[10px] text-yellow-500/70 ml-6">
                                * Project BPM will be set to {metadata.initial_bpm.toFixed(
                                    1,
                                )}
                            </div>
                        {:else}
                            <label
                                class="flex items-center space-x-2 cursor-pointer"
                            >
                                <input
                                    type="radio"
                                    bind:group={bpmMode}
                                    value="scale"
                                    class="text-theme-skin focus:ring-1"
                                />
                                <span>{$t("dialog.mode_scale")}</span>
                            </label>
                            <label
                                class="flex items-center space-x-2 cursor-pointer"
                            >
                                <input
                                    type="radio"
                                    bind:group={bpmMode}
                                    value="absolute"
                                    class="text-theme-skin focus:ring-1"
                                />
                                <span>{$t("dialog.mode_absolute")}</span>
                            </label>
                            <label
                                class="flex items-center space-x-2 cursor-pointer"
                            >
                                <input
                                    type="radio"
                                    bind:group={bpmMode}
                                    value="project"
                                    class="text-theme-skin focus:ring-1"
                                />
                                <span>{$t("dialog.mode_project")}</span>
                            </label>
                        {/if}
                    </div>
                </div>
            {/if}
        </div>

        <!-- Footer -->
        <div
            class="px-4 py-3 bg-zinc-900 border-t border-zinc-700 flex justify-end gap-2"
        >
            <button
                class="px-3 py-1.5 text-xs text-zinc-400 hover:text-zinc-200 transition-colors"
                on:click={handleCancel}
            >
                {$t("dialog.cancel_btn")}
            </button>
            <button
                class="px-4 py-1.5 text-xs bg-theme-skin text-white font-bold rounded hover:bg-theme-skin/90 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
                disabled={loading || !!error}
                on:click={handleImport}
            >
                {$t("dialog.import_btn")}
            </button>
        </div>
    </div>
</div>
