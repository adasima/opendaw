<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import { audioContext } from "../audio/AudioContext.svelte";

    export let pixelsPerSecond = 100;
    export let totalSeconds = 200;

    const dispatch = createEventDispatcher();

    let container: HTMLDivElement;
    let isDragging = false;
    let dragStartX = 0;
    let dragStartTime = 0;
    let selectionStart = 0;
    let selectionEnd = 0;

    // Context menu state
    let contextMenuOpen = false;
    let contextMenuX = 0;
    let contextMenuY = 0;
    let contextMenuTime = 0; // Time position where right-clicked

    // Reactivity
    $: width = totalSeconds * pixelsPerSecond;

    // Format time (mm:ss)
    function formatTime(s: number) {
        const m = Math.floor(s / 60);
        const sec = Math.floor(s % 60);
        return `${m}:${sec.toString().padStart(2, "0")}`;
    }

    function handleMouseDown(e: MouseEvent) {
        if (e.button !== 0) return; // Only left click for drag

        const rect = container.getBoundingClientRect();
        const x = e.clientX - rect.left;
        const t = Math.max(0, x / pixelsPerSecond);

        dragStartX = x;
        dragStartTime = t;
        isDragging = false;

        window.addEventListener("mousemove", handleMouseMove);
        window.addEventListener("mouseup", handleMouseUp);
    }

    function handleMouseMove(e: MouseEvent) {
        if (!isDragging) {
            const rect = container.getBoundingClientRect();
            const x = e.clientX - rect.left;
            if (Math.abs(x - dragStartX) > 5) {
                isDragging = true;
                selectionStart = dragStartTime;
                selectionEnd = dragStartTime;
            }
        }

        if (isDragging) {
            const rect = container.getBoundingClientRect();
            const x = e.clientX - rect.left;
            const t = Math.max(0, x / pixelsPerSecond);

            if (t < dragStartTime) {
                selectionStart = t;
                selectionEnd = dragStartTime;
            } else {
                selectionStart = dragStartTime;
                selectionEnd = t;
            }
        }
    }

    function handleMouseUp(e: MouseEvent) {
        window.removeEventListener("mousemove", handleMouseMove);
        window.removeEventListener("mouseup", handleMouseUp);

        const rect = container.getBoundingClientRect();
        const x = e.clientX - rect.left;
        const t = Math.max(0, x / pixelsPerSecond);

        if (!isDragging) {
            audioContext.seek(t);
        } else {
            audioContext.setLoopRegion(selectionStart, selectionEnd);
        }
        isDragging = false;
    }

    // Right-click context menu
    function handleContextMenu(e: MouseEvent) {
        e.preventDefault();
        const rect = container.getBoundingClientRect();
        contextMenuX = e.clientX - rect.left;
        contextMenuY = e.clientY - rect.top;
        contextMenuTime = Math.max(0, contextMenuX / pixelsPerSecond);
        contextMenuOpen = true;
    }

    function closeContextMenu() {
        contextMenuOpen = false;
    }

    // Context menu actions
    function resetSelection() {
        audioContext.setLoopRegion(0, 0);
        closeContextMenu();
    }

    function resetPlayhead() {
        audioContext.seek(0);
        closeContextMenu();
    }

    function goToPlayhead() {
        // Dispatch event to scroll to playhead
        dispatch("scrollToPlayhead");
        closeContextMenu();
    }

    function toggleLoopSelection() {
        audioContext.toggleLoop();
        closeContextMenu();
    }

    function setPlayheadHere() {
        audioContext.seek(contextMenuTime);
        closeContextMenu();
    }

    function setLoopStartHere() {
        const currentEnd =
            audioContext.loopEnd > contextMenuTime
                ? audioContext.loopEnd
                : contextMenuTime + 4;
        audioContext.setLoopRegion(contextMenuTime, currentEnd);
        closeContextMenu();
    }

    function setLoopEndHere() {
        const currentStart =
            audioContext.loopStart < contextMenuTime
                ? audioContext.loopStart
                : 0;
        audioContext.setLoopRegion(currentStart, contextMenuTime);
        closeContextMenu();
    }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
    bind:this={container}
    class="h-6 bg-zinc-800 border-b border-zinc-700 relative cursor-pointer select-none overflow-visible"
    style="width: {width}px;"
    on:mousedown={handleMouseDown}
    on:contextmenu={handleContextMenu}
    on:click={closeContextMenu}
>
    <!-- ticks -->
    {#each Array(Math.ceil(totalSeconds)) as _, i}
        <div
            class="absolute top-0 bottom-0 border-l border-zinc-600"
            style="left: {i * pixelsPerSecond}px;"
        >
            <span class="text-[10px] text-zinc-400 pl-1">{formatTime(i)}</span>
        </div>
    {/each}

    <!-- Loop Region Overlay -->
    {#if audioContext.loopEnd > audioContext.loopStart}
        <div
            class="absolute top-0 bottom-0 bg-theme-skin/20 border-l border-r border-theme-skin/50 pointer-events-none"
            style="left: {audioContext.loopStart *
                pixelsPerSecond}px; width: {(audioContext.loopEnd -
                audioContext.loopStart) *
                pixelsPerSecond}px;"
        >
            <div class="text-[9px] text-theme-skin px-1 truncate">Loop</div>
        </div>
    {/if}

    <!-- Selection Drag Overlay -->
    {#if isDragging}
        <div
            class="absolute top-0 bottom-0 bg-white/10 border-l border-r border-white/30 pointer-events-none"
            style="left: {selectionStart *
                pixelsPerSecond}px; width: {(selectionEnd - selectionStart) *
                pixelsPerSecond}px;"
        ></div>
    {/if}

    <!-- Playhead Indicator on Ruler -->
    <div
        class="absolute top-0 bottom-0 w-0.5 bg-red-500 z-10 pointer-events-none"
        style="transform: translateX({audioContext.playheadPosition *
            pixelsPerSecond}px);"
    >
        <div class="w-2 h-2 -ml-[3px] bg-red-500 rounded-full"></div>
    </div>

    <!-- Context Menu -->
    {#if contextMenuOpen}
        <div
            class="absolute bg-theme-shirt border border-theme-skin-shadow rounded shadow-xl z-50 py-1 min-w-40"
            style="left: {contextMenuX}px; top: {contextMenuY + 5}px;"
            on:click|stopPropagation
        >
            <button
                class="w-full px-3 py-1.5 text-xs text-left text-theme-wall-light hover:bg-theme-skin/20 transition-colors flex items-center gap-2"
                on:click={setPlayheadHere}
            >
                <span class="text-red-400">▶</span> ここに再生位置を移動
            </button>

            <div class="border-t border-theme-skin-shadow/30 my-1"></div>

            <button
                class="w-full px-3 py-1.5 text-xs text-left text-theme-wall-light hover:bg-theme-skin/20 transition-colors flex items-center gap-2"
                on:click={setLoopStartHere}
            >
                <span class="text-theme-skin">[</span> ループ開始位置に設定
            </button>
            <button
                class="w-full px-3 py-1.5 text-xs text-left text-theme-wall-light hover:bg-theme-skin/20 transition-colors flex items-center gap-2"
                on:click={setLoopEndHere}
            >
                <span class="text-theme-skin">]</span> ループ終了位置に設定
            </button>

            <div class="border-t border-theme-skin-shadow/30 my-1"></div>

            <button
                class="w-full px-3 py-1.5 text-xs text-left text-theme-wall-light hover:bg-theme-skin/20 transition-colors flex items-center gap-2"
                on:click={resetSelection}
            >
                <span class="text-zinc-400">✕</span> 選択範囲をリセット
            </button>
            <button
                class="w-full px-3 py-1.5 text-xs text-left text-theme-wall-light hover:bg-theme-skin/20 transition-colors flex items-center gap-2"
                on:click={resetPlayhead}
            >
                <span class="text-zinc-400">⏮</span> 再生位置をリセット
            </button>
            <button
                class="w-full px-3 py-1.5 text-xs text-left text-theme-wall-light hover:bg-theme-skin/20 transition-colors flex items-center gap-2"
                on:click={goToPlayhead}
            >
                <span class="text-zinc-400">👁</span> 再生位置へスクロール
            </button>

            <div class="border-t border-theme-skin-shadow/30 my-1"></div>

            <button
                class="w-full px-3 py-1.5 text-xs text-left text-theme-wall-light hover:bg-theme-skin/20 transition-colors flex items-center gap-2"
                on:click={toggleLoopSelection}
            >
                {#if audioContext.isLooping}
                    <span class="text-green-400">🔁</span> ループ OFF
                {:else}
                    <span class="text-zinc-400">🔁</span> ループ ON
                {/if}
            </button>
        </div>
    {/if}
</div>

<!-- Click outside to close -->
<svelte:window on:click={closeContextMenu} />
