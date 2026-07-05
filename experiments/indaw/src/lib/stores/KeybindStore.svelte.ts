/**
 * Keybind Store - Manages keyboard shortcut bindings with persistence
 */

// Action definitions
export type ActionId =
    | 'playback.toggle'
    | 'playback.stop'
    | 'recording.toggle'
    | 'track.solo'
    | 'track.mute'
    | 'view.arrange'
    | 'view.mixer'
    | 'scroll.auto';

export interface ActionDefinition {
    id: ActionId;
    label: string;
    category: 'playback' | 'track' | 'view';
}

export const ACTION_DEFINITIONS: ActionDefinition[] = [
    { id: 'playback.toggle', label: '再生/停止', category: 'playback' },
    { id: 'playback.stop', label: '停止 (先頭へ)', category: 'playback' },
    { id: 'recording.toggle', label: '録音', category: 'playback' },
    { id: 'track.solo', label: 'ソロ切替', category: 'track' },
    { id: 'track.mute', label: 'ミュート切替', category: 'track' },
    { id: 'view.arrange', label: 'アレンジビュー', category: 'view' },
    { id: 'view.mixer', label: 'ミキサービュー', category: 'view' },
    { id: 'scroll.auto', label: 'オートスクロール', category: 'view' },
];

// Default keybindings
const DEFAULT_BINDINGS: Record<ActionId, string> = {
    'playback.toggle': 'Space',
    'playback.stop': 'Enter',
    'recording.toggle': 'KeyR',
    'track.solo': 'KeyS',
    'track.mute': 'KeyM',
    'view.arrange': 'Digit1',
    'view.mixer': 'Digit2',
    'scroll.auto': 'KeyF',
};

const STORAGE_KEY = 'indaw_keybindings';

class KeybindStore {
    bindings = $state<Record<ActionId, string>>({ ...DEFAULT_BINDINGS });

    constructor() {
        this.load();
    }

    load() {
        if (typeof localStorage === 'undefined') return;
        try {
            const saved = localStorage.getItem(STORAGE_KEY);
            if (saved) {
                const parsed = JSON.parse(saved);
                this.bindings = { ...DEFAULT_BINDINGS, ...parsed };
            }
        } catch (e) {
            console.error('Failed to load keybindings:', e);
        }
    }

    save() {
        if (typeof localStorage === 'undefined') return;
        try {
            localStorage.setItem(STORAGE_KEY, JSON.stringify(this.bindings));
        } catch (e) {
            console.error('Failed to save keybindings:', e);
        }
    }

    getKey(actionId: ActionId): string {
        return this.bindings[actionId] || '';
    }

    setKey(actionId: ActionId, key: string) {
        // Remove duplicate binding if exists
        for (const [action, k] of Object.entries(this.bindings)) {
            if (k === key && action !== actionId) {
                this.bindings[action as ActionId] = '';
            }
        }
        this.bindings[actionId] = key;
        this.save();
    }

    getActionForKey(key: string): ActionId | null {
        for (const [action, k] of Object.entries(this.bindings)) {
            if (k === key) return action as ActionId;
        }
        return null;
    }

    resetToDefaults() {
        this.bindings = { ...DEFAULT_BINDINGS };
        this.save();
    }

    // Get human-readable key name
    static keyCodeToLabel(code: string): string {
        const mapping: Record<string, string> = {
            'Space': 'Space',
            'Enter': 'Enter',
            'Escape': 'Esc',
            'Backspace': '⌫',
            'Delete': 'Del',
            'ArrowUp': '↑',
            'ArrowDown': '↓',
            'ArrowLeft': '←',
            'ArrowRight': '→',
        };
        if (mapping[code]) return mapping[code];
        if (code.startsWith('Key')) return code.slice(3);
        if (code.startsWith('Digit')) return code.slice(5);
        return code;
    }
}

export const keybindStore = new KeybindStore();
