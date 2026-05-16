<script lang="ts">
    import { onMount } from "svelte";
    import { get } from "svelte/store";
    import { audioContext } from "../audio/AudioContext.svelte";
    import { invoke, open, save, ask, getWindowApi } from "../api";
    import { t, locale } from "../stores/i18n";
    import { isSettingsOpen } from "../stores/settings";
    import MidiImportDialog from "./MidiImportDialog.svelte";
    import type { ImportOptions } from "../types";

    // Window API (lazy loaded or mocked)
    let appWindow: any = null;

    onMount(async () => {
        try {
            const mod = await getWindowApi();
            appWindow = mod.getCurrentWindow();
        } catch (e) {
            console.warn("Tauri window API not available:", e);
        }
    });

    async function minimizeWindow() {
        if (appWindow) await appWindow.minimize();
    }

    async function toggleMaximize() {
        if (!appWindow) return;
        const isMaximized = await appWindow.isMaximized();
        if (isMaximized) {
            await appWindow.unmaximize();
        } else {
            await appWindow.maximize();
        }
    }

    async function closeWindow() {
        if (appWindow) await appWindow.close();
    }

    // Debug mode
    let debugMode = false;
    let fileMenuOpen = false;

    // Import Dialog State
    let showImportDialog = false;
    let importPath = "";
    let importMode: "open" | "import" = "import";

    function toggleDebug() {
        debugMode = !debugMode;
    }

    function toggleFileMenu() {
        fileMenuOpen = !fileMenuOpen;
    }

    function closeMenus() {
        fileMenuOpen = false;
    }

    function openSettings() {
        closeMenus();
        isSettingsOpen.set(true);
    }

    function debugRefresh() {
        audioContext.stopSync();
        audioContext.startSync();
    }

    function debugLogState() {
        console.log("=== INDAW Debug State ===");
        console.log("AudioState:", audioContext);
    }

    async function debugReset() {
        const _t = get(t);
        if (!confirm(_t("dialog.confirm_reset_body"))) return;
        try {
            await audioContext.newProject();
            console.log("=== State Reset ===");
            alert(_t("dialog.reset_complete"));
        } catch (e) {
            console.error("Reset failed:", e);
        }
    }

    // === FILE OPERATIONS ===

    // Open MIDI Project (Reset + Import)
    async function handleOpenMidi() {
        closeMenus();
        try {
            // Confirm reset if tracks exist
            const _t = get(t);
            if (audioContext.tracks.length > 0) {
                const confirmed = await ask(_t("dialog.confirm_reset_body"), {
                    title: _t("dialog.confirm_reset_title"),
                    kind: "warning",
                    okLabel: "OK",
                    cancelLabel: _t("common.cancel"),
                });
                if (!confirmed) return;
            }

            const selected = await open({
                multiple: false,
                filters: [
                    {
                        name: "MIDI Files",
                        extensions: ["mid", "midi"],
                    },
                ],
            });
            if (selected) {
                importPath = selected as string;
                importMode = "open";
                showImportDialog = true;
            }
        } catch (e) {
            console.error("Open failed:", e);
            alert("Open failed: " + e);
        }
    }

    // Import MIDI Track (Add to current project)
    async function handleImportMidiTrack() {
        closeMenus();
        try {
            const selected = await open({
                multiple: false,
                filters: [
                    {
                        name: "MIDI Files",
                        extensions: ["mid", "midi"],
                    },
                ],
            });
            if (selected) {
                importPath = selected as string;
                importMode = "import";
                showImportDialog = true;
            }
        } catch (e) {
            console.error("Import failed:", e);
            alert("Import failed: " + e);
        }
    }

    async function onImportConfirm(e: CustomEvent<ImportOptions>) {
        showImportDialog = false;
        const options = e.detail;

        try {
            if (importMode === "open") {
                await audioContext.newProject();
            }
            await audioContext.importMidi(importPath, options);

            // Create project name (simple)
            const parts = importPath.split(/[/\\]/);
            if (importMode === "open") {
                projectName = parts[parts.length - 1].replace(
                    /\.(mid|midi)$/i,
                    "",
                );
            }
        } catch (err) {
            console.error("Import execution failed:", err);
            alert("Import failed: " + err);
        }
    }

    // Export MIDI
    async function handleExportMidi() {
        closeMenus();
        try {
            const path = await save({
                filters: [
                    {
                        name: "MIDI File",
                        extensions: ["mid"],
                    },
                ],
                defaultPath: `${projectName}.mid`,
            });
            if (path) {
                await audioContext.exportMidi(audioContext.selectedTrackId, path);
                const _t = get(t);
                alert(_t("dialog.midi_export_success"));
            }
        } catch (e) {
            console.error("Export MIDI failed:", e);
            alert("Failed to export MIDI: " + e);
        }
    }

    // Export WAV
    async function handleExportWav() {
        closeMenus();
        try {
            const path = await save({
                filters: [
                    {
                        name: "WAV Audio",
                        extensions: ["wav"],
                    },
                ],
                defaultPath: `${projectName}.wav`,
            });
            if (path) {
                await invoke("export_project", { path });
                const _t = get(t);
                alert(_t("dialog.wav_export_success"));
            }
        } catch (e) {
            console.error("Export WAV failed:", e);
            alert("Failed to export WAV: " + e);
        }
    }

    // Project name (placeholder)
    let projectName = "Untitled";

    // Language Menu
    let languageMenuOpen = false;
    function toggleLanguageMenu() {
        languageMenuOpen = !languageMenuOpen;
    }
    async function setLocale(l: string) {
        if (l === "custom") {
            await locale.loadCustom();
        }
        locale.set(l);
        languageMenuOpen = false;
    }

    function openCustomFolder() {
        locale.openFolder();
        languageMenuOpen = false;
    }
    // Menu Timers
    let fileMenuTimer: ReturnType<typeof setTimeout>;
    let langMenuTimer: ReturnType<typeof setTimeout>;

    function openFileMenu() {
        clearTimeout(fileMenuTimer);
        fileMenuOpen = true;
    }

    function closeFileMenu() {
        fileMenuTimer = setTimeout(() => {
            fileMenuOpen = false;
        }, 300);
    }

    function openLangMenu() {
        clearTimeout(langMenuTimer);
        languageMenuOpen = true;
    }

    function closeLangMenu() {
        langMenuTimer = setTimeout(() => {
            languageMenuOpen = false;
        }, 300);
    }
</script>

{#if showImportDialog}
    <MidiImportDialog
        path={importPath}
        mode={importMode}
        on:confirm={onImportConfirm}
        on:cancel={() => (showImportDialog = false)}
    />
{/if}

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div on:click={closeMenus}>
    <!-- Custom Title Bar -->
    <div
        class="h-8 bg-theme-hair border-b border-theme-skin-shadow/50 flex items-center select-none shrink-0"
    >
        <!-- Left: Logo + Menus -->
        <div class="flex items-center gap-0.5 h-full px-2 shrink-0">
            <!-- Logo -->
            <span class="text-theme-skin font-bold text-sm px-2">INDAW</span>

            <div class="h-4 w-px bg-theme-skin-shadow/30 mx-1"></div>

            <!-- File Menu -->
            <div
                class="relative"
                on:mouseenter={openFileMenu}
                on:mouseleave={closeFileMenu}
                role="button"
                tabindex="-1"
            >
                <button
                    class="px-2 py-1 text-xs text-theme-wall-light/70 hover:bg-theme-shirt/50 hover:text-theme-wall-light rounded transition-colors"
                >
                    {$t("menu.file")}
                </button>

                {#if fileMenuOpen}
                    <div
                        class="absolute top-full left-0 mt-1 w-48 bg-theme-shirt border border-theme-skin-shadow rounded shadow-xl z-50"
                        on:mouseenter={openFileMenu}
                        on:mouseleave={closeFileMenu}
                        role="group"
                    >
                        <button
                            class="w-full px-3 py-2 text-xs text-left text-theme-wall-light hover:bg-theme-skin/20 transition-colors"
                            on:click={handleOpenMidi}
                        >
                            📂 {$t("menu.open_midi")}
                        </button>
                        <button
                            class="w-full px-3 py-2 text-xs text-left text-theme-wall-light hover:bg-theme-skin/20 transition-colors"
                            on:click={handleImportMidiTrack}
                        >
                            ➕ {$t("menu.import_midi")}
                        </button>
                        <div class="border-t border-theme-skin-shadow/30"></div>
                        <button
                            class="w-full px-3 py-2 text-xs text-left text-theme-wall-light hover:bg-theme-skin/20 transition-colors"
                            on:click={handleExportMidi}
                        >
                            📤 {$t("menu.export_midi")}
                        </button>
                        <button
                            class="w-full px-3 py-2 text-xs text-left text-theme-wall-light hover:bg-theme-skin/20 transition-colors"
                            on:click={handleExportWav}
                        >
                            🎵 {$t("menu.export_wav")}
                        </button>
                        <div class="border-t border-theme-skin-shadow/30"></div>
                        <button
                            class="w-full px-3 py-2 text-xs text-left text-theme-wall-light hover:bg-theme-skin/20 transition-colors"
                            on:click={openSettings}
                        >
                            ⚙️ Settings
                        </button>
                    </div>
                {/if}
            </div>

            <button
                class="px-2 py-1 text-xs text-theme-wall-light/50 hover:bg-theme-shirt/50 hover:text-theme-wall-light rounded transition-colors cursor-not-allowed"
                disabled
                title="Coming soon"
            >
                {$t("menu.edit")}
            </button>
            <button
                class="px-2 py-1 text-xs text-theme-wall-light/50 hover:bg-theme-shirt/50 hover:text-theme-wall-light rounded transition-colors cursor-not-allowed"
                disabled
                title="Coming soon"
            >
                {$t("menu.view")}
            </button>
        </div>

        <!-- Center: Drag Region + Title -->
        <div
            data-tauri-drag-region
            class="flex-1 h-full flex items-center justify-center"
        >
            <span class="text-xs text-theme-wall-shadow pointer-events-none"
                >INDAW - {projectName}</span
            >
        </div>

        <!-- Right: Debug + Language + Window Controls -->
        <div class="flex items-center gap-0.5 h-full shrink-0">
            <!-- Language Menu -->
            <div
                class="relative"
                on:mouseenter={openLangMenu}
                on:mouseleave={closeLangMenu}
                role="button"
                tabindex="-1"
            >
                <button
                    class="px-2 py-1 text-xs text-theme-wall-light/50 hover:bg-theme-shirt/50 hover:text-theme-wall-light rounded transition-colors"
                    title="Change Language"
                >
                    🌐
                </button>
                {#if languageMenuOpen}
                    <div
                        class="absolute top-full right-0 mt-1 w-32 bg-theme-shirt border border-theme-skin-shadow rounded shadow-xl z-50"
                        on:mouseenter={openLangMenu}
                        on:mouseleave={closeLangMenu}
                        role="group"
                    >
                        <button
                            class="w-full px-3 py-2 text-xs text-left text-theme-wall-light hover:bg-theme-skin/20 transition-colors {$locale ===
                            'en'
                                ? 'font-bold text-theme-skin'
                                : ''}"
                            on:click={() => setLocale("en")}
                        >
                            English
                        </button>
                        <button
                            class="w-full px-3 py-2 text-xs text-left text-theme-wall-light hover:bg-theme-skin/20 transition-colors {$locale ===
                            'ja'
                                ? 'font-bold text-theme-skin'
                                : ''}"
                            on:click={() => setLocale("ja")}
                        >
                            日本語
                        </button>
                        <button
                            class="w-full px-3 py-2 text-xs text-left text-theme-wall-light hover:bg-theme-skin/20 transition-colors {$locale ===
                            'custom'
                                ? 'font-bold text-theme-skin'
                                : ''}"
                            on:click={() => setLocale("custom")}
                        >
                            Custom
                        </button>
                        <div class="border-t border-theme-skin-shadow/30"></div>
                        <button
                            class="w-full px-3 py-2 text-xs text-left text-theme-wall-light/70 hover:bg-theme-skin/20 transition-colors"
                            on:click={openCustomFolder}
                        >
                            📂 {$t("menu.open_folder")}
                        </button>
                    </div>
                {/if}
            </div>

            <div class="h-4 w-px bg-theme-skin-shadow/30 mx-1"></div>

            <!-- Debug Toggle -->
            <button
                class="px-2 py-1 text-xs rounded transition-colors {debugMode
                    ? 'bg-yellow-500/30 text-yellow-400'
                    : 'text-theme-wall-light/50 hover:text-yellow-400 hover:bg-theme-shirt/50'}"
                on:click={toggleDebug}
                title="Toggle Debug Panel"
            >
                🐛
            </button>

            <div class="h-4 w-px bg-theme-skin-shadow/30 mx-1"></div>

            <!-- Window Controls -->
            <button
                class="w-10 h-8 flex items-center justify-center text-theme-wall-light/60 hover:bg-theme-shirt transition-colors"
                on:click={minimizeWindow}
                title="Minimize"
            >
                <svg width="10" height="10" viewBox="0 0 10 10" fill="none">
                    <rect y="4" width="10" height="2" fill="currentColor" />
                </svg>
            </button>

            <button
                class="w-10 h-8 flex items-center justify-center text-theme-wall-light/60 hover:bg-theme-shirt transition-colors"
                on:click={toggleMaximize}
                title="Maximize"
            >
                <svg width="10" height="10" viewBox="0 0 10 10" fill="none">
                    <rect
                        x="0.5"
                        y="0.5"
                        width="9"
                        height="9"
                        stroke="currentColor"
                        stroke-width="1"
                        fill="none"
                    />
                </svg>
            </button>

            <button
                class="w-10 h-8 flex items-center justify-center text-theme-wall-light/60 hover:bg-red-600 hover:text-white transition-colors"
                on:click={closeWindow}
                title="Close"
            >
                <svg width="10" height="10" viewBox="0 0 10 10" fill="none">
                    <path
                        d="M1 1L9 9M9 1L1 9"
                        stroke="currentColor"
                        stroke-width="1.5"
                        stroke-linecap="round"
                    />
                </svg>
            </button>
        </div>
    </div>
</div>

<!-- Debug Panel (Overlay) -->
{#if debugMode}
    <div
        class="fixed top-10 right-2 w-72 max-h-[50vh] bg-zinc-900/98 border border-yellow-500/50 rounded-lg z-[100] overflow-auto text-xs text-yellow-100 shadow-xl backdrop-blur-sm"
    >
        <div
            class="p-2 border-b border-yellow-500/30 flex justify-between items-center sticky top-0 bg-zinc-900 rounded-t-lg"
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
                <button
                    class="px-2 py-0.5 bg-red-500/30 hover:bg-red-500/50 text-red-300 rounded"
                    on:click={debugReset}>Reset</button
                >
            </div>
        </div>

        <div class="p-2 space-y-2">
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
                <div>Looping: {audioContext.isLooping ? "ON" : "OFF"}</div>
            </div>

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
                        [{track.id}] {track.name} ({track.kind})
                    </div>
                {/each}
            </div>

            <details class="border border-yellow-500/20 rounded">
                <summary class="p-2 cursor-pointer text-yellow-400 font-bold"
                    >Raw JSON</summary
                >
                <pre
                    class="p-2 text-[10px] overflow-x-auto max-h-40 overflow-y-auto bg-black/50">{JSON.stringify(
                        audioContext,
                        null,
                        2,
                    )}</pre>
            </details>
        </div>
    </div>
{/if}
