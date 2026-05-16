/**
 * Shortcut Manager - Listens for keyboard events and triggers actions
 */
import { keybindStore, type ActionId } from '../stores/KeybindStore.svelte';
import { audioContext } from '../audio/AudioContext.svelte';
import { uiState } from '../stores/UIState.svelte';

// Actions registry
const actionHandlers: Record<ActionId, () => void> = {
    'playback.toggle': () => audioContext.togglePlayback(),
    'playback.stop': () => audioContext.stop(),
    'recording.toggle': () => audioContext.toggleRecording(),
    'track.solo': () => {
        const id = audioContext.selectedTrackId;
        if (id >= 0) audioContext.toggleTrackSolo(id);
    },
    'track.mute': () => {
        const id = audioContext.selectedTrackId;
        if (id >= 0) audioContext.toggleTrackMute(id);
    },
    'view.arrange': () => uiState.viewMode = 'arrange',
    'view.mixer': () => uiState.viewMode = 'mixer',
    'scroll.auto': () => audioContext.toggleAutoScroll(),
};

function isInputFocused(): boolean {
    const el = document.activeElement;
    if (!el) return false;
    const tag = el.tagName.toLowerCase();
    if (tag === 'input' || tag === 'textarea' || tag === 'select') return true;
    if (el.getAttribute('contenteditable') === 'true') return true;
    return false;
}

function handleKeydown(e: KeyboardEvent) {
    // Skip if focus is on input
    if (isInputFocused()) return;

    const code = e.code;
    const actionId = keybindStore.getActionForKey(code);

    if (actionId) {
        e.preventDefault();
        const handler = actionHandlers[actionId];
        if (handler) handler();
    }
}

let isInitialized = false;

export function initShortcuts() {
    if (isInitialized) return;
    if (typeof window === 'undefined') return;

    window.addEventListener('keydown', handleKeydown);
    isInitialized = true;
}

export function destroyShortcuts() {
    if (!isInitialized) return;
    window.removeEventListener('keydown', handleKeydown);
    isInitialized = false;
}
