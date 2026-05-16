<script lang="ts">
    import { keybindStore, ACTION_DEFINITIONS, type ActionId } from '../stores/KeybindStore.svelte';

    // Keyboard layout (US QWERTY simplified)
    const KEYBOARD_ROWS = [
        ['Escape', 'F1', 'F2', 'F3', 'F4', 'F5', 'F6', 'F7', 'F8', 'F9', 'F10', 'F11', 'F12'],
        ['Backquote', 'Digit1', 'Digit2', 'Digit3', 'Digit4', 'Digit5', 'Digit6', 'Digit7', 'Digit8', 'Digit9', 'Digit0', 'Minus', 'Equal', 'Backspace'],
        ['Tab', 'KeyQ', 'KeyW', 'KeyE', 'KeyR', 'KeyT', 'KeyY', 'KeyU', 'KeyI', 'KeyO', 'KeyP', 'BracketLeft', 'BracketRight', 'Backslash'],
        ['CapsLock', 'KeyA', 'KeyS', 'KeyD', 'KeyF', 'KeyG', 'KeyH', 'KeyJ', 'KeyK', 'KeyL', 'Semicolon', 'Quote', 'Enter'],
        ['ShiftLeft', 'KeyZ', 'KeyX', 'KeyC', 'KeyV', 'KeyB', 'KeyN', 'KeyM', 'Comma', 'Period', 'Slash', 'ShiftRight'],
        ['ControlLeft', 'MetaLeft', 'AltLeft', 'Space', 'AltRight', 'MetaRight', 'ContextMenu', 'ControlRight'],
    ];

    // Key display labels
    function getKeyLabel(code: string): string {
        const labels: Record<string, string> = {
            'Backquote': '`', 'Minus': '-', 'Equal': '=', 'Backspace': '⌫',
            'BracketLeft': '[', 'BracketRight': ']', 'Backslash': '\\',
            'Semicolon': ';', 'Quote': "'", 'Comma': ',', 'Period': '.', 'Slash': '/',
            'CapsLock': 'Caps', 'Tab': 'Tab', 'Enter': '↵', 'Escape': 'Esc',
            'ShiftLeft': 'Shift', 'ShiftRight': 'Shift',
            'ControlLeft': 'Ctrl', 'ControlRight': 'Ctrl',
            'AltLeft': 'Alt', 'AltRight': 'Alt',
            'MetaLeft': '⌘', 'MetaRight': '⌘', 'ContextMenu': '☰',
            'Space': 'Space',
        };
        if (labels[code]) return labels[code];
        if (code.startsWith('Key')) return code.slice(3);
        if (code.startsWith('Digit')) return code.slice(5);
        if (code.startsWith('F') && code.length <= 3) return code;
        return code;
    }

    // Key widths (relative units)
    function getKeyWidth(code: string): string {
        const widths: Record<string, string> = {
            'Backspace': '2', 'Tab': '1.5', 'Backslash': '1.5',
            'CapsLock': '1.75', 'Enter': '2.25',
            'ShiftLeft': '2.25', 'ShiftRight': '2.75',
            'ControlLeft': '1.25', 'MetaLeft': '1.25', 'AltLeft': '1.25',
            'Space': '6.25',
            'AltRight': '1.25', 'MetaRight': '1.25', 'ContextMenu': '1.25', 'ControlRight': '1.25',
        };
        return widths[code] || '1';
    }

    // Get action bound to this key
    function getAction(keyCode: string): ActionId | null {
        return keybindStore.getActionForKey(keyCode);
    }

    function getActionLabel(actionId: ActionId): string {
        const def = ACTION_DEFINITIONS.find(a => a.id === actionId);
        return def ? def.label : actionId;
    }

    // Edit mode
    let editingKey: string | null = $state(null);
    let listeningForKey = $state(false);

    function startEdit(keyCode: string) {
        editingKey = keyCode;
        listeningForKey = true;
    }

    function handleKeyCapture(e: KeyboardEvent) {
        if (!listeningForKey || !editingKey) return;
        e.preventDefault();
        e.stopPropagation();

        const newCode = e.code;
        const currentAction = getAction(editingKey);
        
        if (currentAction) {
            // Rebind action to new key
            keybindStore.setKey(currentAction, newCode);
        }
        
        listeningForKey = false;
        editingKey = null;
    }

    function clearBinding(keyCode: string) {
        const action = getAction(keyCode);
        if (action) {
            keybindStore.setKey(action, '');
        }
    }
</script>

<svelte:window on:keydown={handleKeyCapture} />

<div class="keyboard-visualizer select-none">
    {#each KEYBOARD_ROWS as row, rowIdx}
        <div class="keyboard-row">
            {#each row as keyCode}
                {@const action = getAction(keyCode)}
                {@const isEditing = editingKey === keyCode}
                <button
                    class="key"
                    class:has-action={action !== null}
                    class:editing={isEditing}
                    style:flex={getKeyWidth(keyCode)}
                    onclick={() => startEdit(keyCode)}
                    oncontextmenu={(e) => { e.preventDefault(); clearBinding(keyCode); }}
                    title={action ? `${getActionLabel(action)} (右クリックで解除)` : 'クリックして割り当て'}
                >
                    <span class="key-label">{getKeyLabel(keyCode)}</span>
                    {#if action}
                        <span class="action-label">{getActionLabel(action)}</span>
                    {/if}
                    {#if isEditing && listeningForKey}
                        <span class="listening">キー入力待ち...</span>
                    {/if}
                </button>
            {/each}
        </div>
    {/each}
</div>

<style>
    .keyboard-visualizer {
        display: flex;
        flex-direction: column;
        gap: 4px;
        padding: 16px;
        background: #1a1a1a;
        border-radius: 12px;
        max-width: 900px;
    }

    .keyboard-row {
        display: flex;
        gap: 4px;
    }

    .key {
        position: relative;
        min-width: 40px;
        height: 44px;
        background: linear-gradient(180deg, #3a3a3a 0%, #2a2a2a 100%);
        border: 1px solid #444;
        border-radius: 6px;
        color: #aaa;
        font-size: 11px;
        font-weight: 500;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        cursor: pointer;
        transition: all 0.15s ease;
        padding: 2px;
    }

    .key:hover {
        background: linear-gradient(180deg, #4a4a4a 0%, #3a3a3a 100%);
        border-color: #666;
    }

    .key.has-action {
        background: linear-gradient(180deg, #4a3a2a 0%, #3a2a1a 100%);
        border-color: #c9a060;
        color: #fff;
    }

    .key.editing {
        background: linear-gradient(180deg, #2a4a3a 0%, #1a3a2a 100%);
        border-color: #60c9a0;
        animation: pulse 1s infinite;
    }

    @keyframes pulse {
        0%, 100% { box-shadow: 0 0 0 0 rgba(96, 201, 160, 0.4); }
        50% { box-shadow: 0 0 0 4px rgba(96, 201, 160, 0.2); }
    }

    .key-label {
        font-size: 10px;
        opacity: 0.7;
    }

    .action-label {
        font-size: 8px;
        color: #c9a060;
        font-weight: 600;
        text-transform: uppercase;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
        max-width: 100%;
    }

    .listening {
        position: absolute;
        inset: 0;
        display: flex;
        align-items: center;
        justify-content: center;
        background: rgba(0, 0, 0, 0.85);
        font-size: 8px;
        color: #60c9a0;
        border-radius: 5px;
    }
</style>
