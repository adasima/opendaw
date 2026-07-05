<script lang="ts">
  // Piano Roll with Grid, Playhead, Auto-Scroll, Timeline Ruler, Piano Keys, Drag/Drop, Selection, Copy/Paste, Note Creation/Resize
  import { audioContext } from "../audio/AudioContext.svelte";
  import type { NoteEvent } from "../bindings";
  import { onDestroy, onMount } from "svelte";
  import TimelineRuler from "./TimelineRuler.svelte";

  let scrollContainer = $state<HTMLDivElement>();
  let rulerContainer = $state<HTMLDivElement>();
  let keysContainer = $state<HTMLDivElement>();

  // Dimensions
  const PIXELS_PER_SECOND = 100;
  const NOTE_HEIGHT = 24;
  const TOTAL_NOTES = 128;
  const GRID_SNAP_SEC = 0.25;

  // State
  let startScrollLeft = $state(0);
  let wasPlaying = $state(false);
  let selectedIndices = $state(new Set<number>());

  // Derived Sequence
  // Get notes from Selected Track
  const selectedTrack = $derived(audioContext.tracks.find(
    (t) => t.id === audioContext.selectedTrackId,
  ));
  
  const currentNotes = $derived(
    selectedTrack &&
    selectedTrack.kind === "Midi" &&
    selectedTrack.content &&
    "Midi" in selectedTrack.content
      ? selectedTrack.content.Midi.notes
      : []
  );

  // Drag/Resize State
  let draggingIdx = $state<number | null>(null);
  let resizingIdx = $state<number | null>(null);
  let dragStartX = $state(0);
  let dragStartY = $state(0);
  let dragOriginalStartTime = $state(0);
  let dragOriginalDuration = $state(0);
  let dragOriginalNoteNum = $state(0);
  let dragPreviewNote = $state<NoteEvent | null>(null);

  // Clipboard
  let clipboard: NoteEvent[] = []; // No need for reactivity if only internal logic uses it? Maybe. Let's keep strict.
  // actually clipboard doesn't affect UI rendering directly usually, but let's make it state to be safe.
  // Wait, Svelte 5 $active is deep? No. 
  // let clipboard = $state<NoteEvent[]>([]); 

  // Reactivity
  const playheadX = $derived(audioContext.playheadPosition * PIXELS_PER_SECOND);

  // -------------------------
  // Handlers
  // -------------------------

  // Create Note on Double Click
  function handleBgDblClick(e: MouseEvent) {
    if (!audioContext.tracks.length) return;

    // Get time and note from click position
    // safe check for scrollContainer in case it's not bound yet.
    if (!scrollContainer) return;

    const containerRect = scrollContainer.getBoundingClientRect();
    const x = e.clientX - containerRect.left + scrollContainer.scrollLeft;
    const y = e.clientY - containerRect.top + scrollContainer.scrollTop;

    const rawTime = x / PIXELS_PER_SECOND;
    const snappedTime = Math.round(rawTime / GRID_SNAP_SEC) * GRID_SNAP_SEC;

    // Y is from top, so 0 is C9 (note 127)
    const noteIndex = Math.floor(y / NOTE_HEIGHT);
    const noteNum = 127 - noteIndex;

    if (noteNum < 0 || noteNum > 127) return;

    // Create new note
    const newNote: NoteEvent = {
      note: noteNum,
      start_time: snappedTime,
      duration: GRID_SNAP_SEC,
      velocity: 100,
    };

    const newSequence = [...currentNotes, newNote];
    audioContext.updateSequence(newSequence);
  }

  function handleNoteMouseDown(e: MouseEvent, index: number, note: NoteEvent) {
    e.stopPropagation(); // Prevent background deselect

    // Check if clicking resize handle (right edge)
    const target = e.target as HTMLElement;
    if (target.classList.contains("resize-handle")) {
      resizingIdx = index;
      dragStartX = e.clientX;
      dragOriginalDuration = note.duration;
      dragPreviewNote = { ...note };
      return;
    }

    // Selection Logic
    if (e.ctrlKey) {
      if (selectedIndices.has(index)) selectedIndices.delete(index);
      else selectedIndices.add(index);
      selectedIndices = new Set(selectedIndices); // trigger reactivity for Set
      return; // Don't drag on multi-select toggle
    } else {
      if (!selectedIndices.has(index)) {
        selectedIndices = new Set([index]);
      }
    }

    // Start Drag
    draggingIdx = index;
    dragStartX = e.clientX;
    dragStartY = e.clientY;
    dragOriginalStartTime = note.start_time;
    dragOriginalNoteNum = note.note;
    dragPreviewNote = { ...note };
  }

  function handleNoteContextMenu(e: MouseEvent, index: number) {
    e.preventDefault();
    // Remove note
    const newSequence = currentNotes.filter((_, i) => i !== index);
    audioContext.updateSequence(newSequence);
    selectedIndices.delete(index);
    selectedIndices = new Set(selectedIndices);
  }

  function handleWindowMouseMove(e: MouseEvent) {
    // Handling Dragging (Move)
    if (draggingIdx !== null && dragPreviewNote) {
      const deltaX = e.clientX - dragStartX;
      const deltaY = e.clientY - dragStartY;

      let rawTime = dragOriginalStartTime + deltaX / PIXELS_PER_SECOND;
      let snappedTime = Math.round(rawTime / GRID_SNAP_SEC) * GRID_SNAP_SEC;
      if (snappedTime < 0) snappedTime = 0;

      let noteDelta = Math.round(-deltaY / NOTE_HEIGHT);
      let newNote = dragOriginalNoteNum + noteDelta;
      if (newNote > 127) newNote = 127;
      if (newNote < 0) newNote = 0;

      dragPreviewNote = {
        ...dragPreviewNote,
        start_time: snappedTime,
        note: newNote,
      };
    }

    // Handling Resizing
    if (resizingIdx !== null && dragPreviewNote) {
      const deltaX = e.clientX - dragStartX;
      let rawDuration = dragOriginalDuration + deltaX / PIXELS_PER_SECOND;

      let snappedDuration =
        Math.round(rawDuration / GRID_SNAP_SEC) * GRID_SNAP_SEC;
      if (snappedDuration < GRID_SNAP_SEC) snappedDuration = GRID_SNAP_SEC;

      dragPreviewNote = {
        ...dragPreviewNote,
        duration: snappedDuration,
      };
    }
  }

  function handleWindowMouseUp() {
    // Commit Move
    if (draggingIdx !== null && dragPreviewNote) {
      const newSequence = [...currentNotes];
      newSequence[draggingIdx] = dragPreviewNote;
      audioContext.updateSequence(newSequence);

      draggingIdx = null;
      dragPreviewNote = null;
    }

    // Commit Resize
    if (resizingIdx !== null && dragPreviewNote) {
      const newSequence = [...currentNotes];
      newSequence[resizingIdx] = dragPreviewNote;
      audioContext.updateSequence(newSequence);

      resizingIdx = null;
      dragPreviewNote = null;
    }
  }

  function handleKeyDown(e: KeyboardEvent) {
    // Delete
    if (e.key === "Delete" || e.key === "Backspace") {
      if (selectedIndices.size > 0) {
        const newNotes = currentNotes.filter(
          (_: NoteEvent, i: number) => !selectedIndices.has(i),
        );
        audioContext.updateSequence(newNotes);
        selectedIndices = new Set();
      }
    }
    // Copy (Ctrl+C)
    if (e.ctrlKey && e.key === "c") {
      if (selectedIndices.size > 0) {
        clipboard = currentNotes.filter((_, i) => selectedIndices.has(i));
      }
    }
    // Paste (Ctrl+V)
    if (e.ctrlKey && e.key === "v") {
      if (clipboard.length > 0) {
        const playhead = audioContext.playheadPosition;
        const minStart = Math.min(...clipboard.map((n) => n.start_time));
        const offset = playhead - minStart;

        const newNotes = clipboard.map((n) => ({
          ...n,
          start_time: n.start_time + offset,
        }));

        const updated = [...currentNotes, ...newNotes];
        audioContext.updateSequence(updated);

        // Select pasted notes
        const startIdx = currentNotes.length;
        const newIndices = new Set<number>();
        for (let i = 0; i < newNotes.length; i++) newIndices.add(startIdx + i);
        selectedIndices = newIndices;
      }
    }
  }

  // Handle Play/Stop state changes
  $effect(() => {
    if (audioContext.isPlaying !== wasPlaying) {
        if (audioContext.isPlaying) {
            if (scrollContainer) startScrollLeft = scrollContainer.scrollLeft;
        } else {
            if (scrollContainer && audioContext.autoScroll) {
                scrollContainer.scrollLeft = startScrollLeft;
            }
        }
        wasPlaying = audioContext.isPlaying;
    }
  });

  // Auto-Scroll Logic
  $effect(() => {
    if (audioContext.isPlaying && audioContext.autoScroll && scrollContainer) {
        // Dependency on playheadX
        const px = playheadX;
        const containerWidth = scrollContainer.clientWidth;
        if (px > scrollContainer.scrollLeft + containerWidth * 0.9) {
            scrollContainer.scrollLeft = px - containerWidth * 0.1;
        } else if (px < scrollContainer.scrollLeft) {
            scrollContainer.scrollLeft = px;
        }
    }
  });

  // Sync Scrolls
  function handleScroll() {
    if (scrollContainer) {
      if (rulerContainer)
        rulerContainer.scrollLeft = scrollContainer.scrollLeft;
      if (keysContainer) keysContainer.scrollTop = scrollContainer.scrollTop;
    }
  }

  // Sync Ruler Programmatically
  $effect(() => {
    if (scrollContainer && rulerContainer) {
        // Just ensures they are synced if something else changes scroll
        // rulerContainer.scrollLeft = scrollContainer.scrollLeft;
    }
  });
</script>

<svelte:window
  on:keydown={handleKeyDown}
  on:mousemove={handleWindowMouseMove}
  on:mouseup={handleWindowMouseUp}
/>

<div
  class="w-full h-full bg-theme-wall-light flex flex-col border border-theme-wall-shadow relative select-none"
>
  <!-- Header -->
  <div
    class="h-8 bg-theme-wall-light border-b border-theme-wall-shadow flex items-center px-2 text-xs text-theme-hair justify-between shrink-0"
  >
    <!-- Show Track Name -->
    <span class="font-bold"
      >{selectedTrack ? selectedTrack.name : "No Track Selected"}</span
    >
    <div class="flex gap-4">
      <span>Piano Roll - ({audioContext.autoScroll ? "Follow" : "Free"})</span>
      <span>Loop: {audioContext.isLooping ? "ON" : "OFF"}</span>
    </div>
  </div>

  <!-- Main Content Row -->
  <div class="flex-1 flex overflow-hidden relative">
    <!-- Left Sidebar: Piano Keys -->
    <div
      bind:this={keysContainer}
      class="w-12 bg-theme-wall-light border-r border-theme-wall-shadow overflow-hidden relative shrink-0 scrollbar-hide"
    >
      <div
        class="relative w-full"
        style="height: {TOTAL_NOTES * NOTE_HEIGHT}px;"
      >
        {#each Array(TOTAL_NOTES) as _, i}
          {@const noteNum = 127 - i}
          {@const isBlack = [1, 3, 6, 8, 10].includes(noteNum % 12)}
          {@const isC = noteNum % 12 === 0}

          <div
            class="absolute left-0 w-full flex items-center justify-end pr-1 text-[9px] border-b border-theme-wall-shadow/30 box-border
                               {isBlack
              ? 'bg-theme-hair text-theme-wall-light'
              : 'bg-theme-wall-light text-theme-hair'}
                               {isC ? 'border-b-theme-wall-shadow' : ''}"
            style="top: {i * NOTE_HEIGHT}px; height: {NOTE_HEIGHT}px;"
          >
            {#if isC}
              <span class="font-bold opacity-50"
                >C{Math.floor(noteNum / 12) - 1}</span
              >
            {/if}
          </div>
        {/each}
      </div>
    </div>

    <!-- Right Column: Timeline + Grid -->
    <div class="flex-1 flex flex-col overflow-hidden relative">
      <!-- Timeline Ruler -->
      <div
        bind:this={rulerContainer}
        class="h-6 w-full overflow-hidden bg-theme-wall-light relative border-b border-theme-wall-shadow shrink-0"
      >
        <TimelineRuler pixelsPerSecond={PIXELS_PER_SECOND} totalSeconds={200} />
      </div>

      <!-- Grid Container -->
      <div
        bind:this={scrollContainer}
        class="flex-1 overflow-auto relative bg-theme-wall-light"
        style="contain: strict;"
        on:scroll={handleScroll}
      >
        <div
          class="relative"
          style="width: 20000px; height: {TOTAL_NOTES * NOTE_HEIGHT}px;"
        >
          <!-- Grid Background -->
          <div
            class="absolute inset-0 pointer-events-none"
            style="background-image: linear-gradient(to bottom, #A0A09B 1px, transparent 1px);
                                 background-size: 100% {NOTE_HEIGHT}px; opacity: 0.3;"
          ></div>
          <div
            class="absolute inset-0 pointer-events-none"
            style="background-image: linear-gradient(to right, #A0A09B 1px, transparent 1px);
                                 background-size: {PIXELS_PER_SECOND}px 100%; opacity: 0.3;"
          ></div>

          <!-- Click Handler for Background Deselect & Create Note -->
          <div
            class="absolute inset-0 z-0"
            role="button"
            tabindex="0"
            on:mousedown|self={(e) => {
              selectedIndices = new Set();
            }}
            on:dblclick={handleBgDblClick}
          ></div>

          <!-- Loop Region Overlay -->
          {#if audioContext.loopEnd > audioContext.loopStart}
            <div
              class="absolute top-0 bottom-0 bg-theme-skin/10 border-l border-r border-theme-skin/30 pointer-events-none"
              style="left: {audioContext.loopStart *
                PIXELS_PER_SECOND}px; width: {(audioContext.loopEnd -
                audioContext.loopStart) *
                PIXELS_PER_SECOND}px;"
            ></div>
          {/if}

          <!-- Playhead -->
          <div
            class="absolute top-0 bottom-0 w-0.5 bg-theme-skin z-10 pointer-events-none"
            style="transform: translateX({playheadX}px);"
          >
            <div
              class="w-2 h-2 -ml-[3px] bg-theme-skin rounded-full shadow-sm"
            ></div>
          </div>

          <!-- Real Notes from Sequence -->
          {#each currentNotes as note, i}
            {@const isDragging = draggingIdx === i}
            {@const isResizing = resizingIdx === i}
            {@const displayNote =
              (isDragging || isResizing) && dragPreviewNote
                ? dragPreviewNote
                : note}
            {@const isSelected = selectedIndices.has(i)}

            <div
              class="absolute border rounded-sm hover:opacity-90 cursor-grab overflow-visible text-[9px] text-theme-hair px-1 whitespace-nowrap z-20 select-none group
                                   {isSelected
                ? 'bg-theme-skin border-theme-hair ring-1 ring-theme-hair'
                : 'bg-theme-skin/80 border-theme-skin-shadow'}
                                   {isDragging || isResizing
                ? 'opacity-80 shadow-lg z-30'
                : ''}"
              style="
                                top: {(127 - displayNote.note) *
                NOTE_HEIGHT}px; 
                                left: {displayNote.start_time *
                PIXELS_PER_SECOND}px; 
                                width: {displayNote.duration *
                PIXELS_PER_SECOND}px; 
                                height: {NOTE_HEIGHT - 2}px;
                            "
              role="button"
              tabindex="0"
              title="Note: {displayNote.note}, Vel: {displayNote.velocity}"
              on:mousedown={(e) => handleNoteMouseDown(e, i, note)}
              on:contextmenu={(e) => handleNoteContextMenu(e, i)}
            >
              <span class="pointer-events-none block truncate w-full"
                >{displayNote.note}</span
              >

              <!-- Resize Handle -->
              <div
                class="resize-handle absolute right-0 top-0 bottom-0 w-2 cursor-e-resize hover:bg-white/20"
              ></div>
            </div>
          {/each}
        </div>
      </div>
    </div>
  </div>
</div>
