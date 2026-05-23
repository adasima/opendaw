<script>
  import { Window } from '@tauri-apps/api/window';
  import { locale, _ } from 'svelte-i18n';

  const appWindow = new Window('main');

  let settingsOpen = $state(false);

  function minimize() { appWindow.minimize(); }
  function toggleMaximize() { appWindow.toggleMaximize(); }
  function close() { appWindow.close(); }

  function toggleSettings() { settingsOpen = !settingsOpen; }

  function setLocale(lang) {
    $locale = lang;
    settingsOpen = false;
  }

  // Close dropdown on outside click
  function handleWindowClick(e) {
    if (settingsOpen && !e.target.closest('.settings-dropdown') && !e.target.closest('.settings-btn')) {
      settingsOpen = false;
    }
  }
</script>

<svelte:window onclick={handleWindowClick} />

<div data-tauri-drag-region class="titlebar">
  <div class="titlebar-drag-region" data-tauri-drag-region>
    <span class="title">OpenDAW</span>
    <span class="version">v0.1.0</span>
  </div>

  <div class="titlebar-actions">
    <!-- Settings Gear -->
    <div class="settings-wrapper">
      <button class="titlebar-action-btn settings-btn" onclick={toggleSettings} aria-label="Settings"
        class:active={settingsOpen}>
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"
          stroke-linecap="round" stroke-linejoin="round">
          <circle cx="12" cy="12" r="3"></circle>
          <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path>
        </svg>
      </button>

      {#if settingsOpen}
        <div class="settings-dropdown glass-panel">
          <div class="dropdown-section">
            <span class="dropdown-label">{$_('ui.switch_lang')}</span>
            <div class="locale-buttons">
              <button class="locale-btn" class:selected={$locale === 'en'} onclick={() => setLocale('en')}>EN</button>
              <button class="locale-btn" class:selected={$locale === 'ja'} onclick={() => setLocale('ja')}>JA</button>
            </div>
          </div>
          <div class="dropdown-section">
            <span class="dropdown-label">{$_('ui.theme')}</span>
            <div class="locale-buttons">
              <button class="locale-btn selected">Dark</button>
              <button class="locale-btn" disabled>Light</button>
            </div>
          </div>
        </div>
      {/if}
    </div>
  </div>

  <!-- Windows Native-style Buttons -->
  <div class="titlebar-buttons">
    <button class="titlebar-button" onclick={minimize} aria-label="Minimize">
      <svg width="10" height="10" viewBox="0 0 10 10" stroke="currentColor" stroke-width="1.2">
        <line x1="0" y1="5" x2="10" y2="5"></line>
      </svg>
    </button>
    <button class="titlebar-button" onclick={toggleMaximize} aria-label="Maximize">
      <svg width="10" height="10" viewBox="0 0 10 10" stroke="currentColor" stroke-width="1.2" fill="none">
        <rect x="0.5" y="0.5" width="9" height="9"></rect>
      </svg>
    </button>
    <button class="titlebar-button close-btn" onclick={close} aria-label="Close">
      <svg width="10" height="10" viewBox="0 0 10 10" stroke="currentColor" stroke-width="1.2">
        <line x1="1" y1="1" x2="9" y2="9"></line>
        <line x1="9" y1="1" x2="1" y2="9"></line>
      </svg>
    </button>
  </div>
</div>

<style>
  .titlebar {
    height: var(--titlebar-height, 32px);
    background: var(--surface-container-highest);
    display: flex;
    align-items: center;
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    z-index: 9999;
    border-bottom: 1px solid var(--outline-variant);
  }

  .titlebar-drag-region {
    flex-grow: 1;
    height: 100%;
    display: flex;
    align-items: center;
    padding-left: 12px;
    gap: 8px;
  }

  .title {
    font-size: 12px;
    font-family: var(--font-label);
    color: var(--on-surface);
    font-weight: 600;
    pointer-events: none;
  }

  .version {
    font-size: 10px;
    font-family: var(--font-label);
    color: var(--outline);
    pointer-events: none;
  }

  .titlebar-actions {
    display: flex;
    align-items: center;
    height: 100%;
    margin-right: 4px;
  }

  .settings-wrapper {
    position: relative;
  }

  .titlebar-action-btn {
    display: inline-flex;
    justify-content: center;
    align-items: center;
    width: 32px;
    height: 26px;
    background: transparent;
    border: none;
    border-radius: 4px;
    color: var(--on-surface-variant);
    cursor: pointer;
    transition: all 0.15s;
  }

  .titlebar-action-btn:hover, .titlebar-action-btn.active {
    background: rgba(255, 255, 255, 0.08);
    color: var(--on-surface);
  }

  .settings-dropdown {
    position: absolute;
    top: calc(100% + 8px);
    right: 0;
    min-width: 180px;
    padding: 8px;
    display: flex;
    flex-direction: column;
    gap: 8px;
    z-index: 10000;
    animation: dropdown-in 0.15s ease-out;
  }

  @keyframes dropdown-in {
    from { opacity: 0; transform: translateY(-4px); }
    to { opacity: 1; transform: translateY(0); }
  }

  .dropdown-section {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .dropdown-label {
    font-size: 10px;
    font-family: var(--font-label);
    color: var(--outline);
    text-transform: uppercase;
    letter-spacing: 0.5px;
    padding: 0 4px;
  }

  .locale-buttons {
    display: flex;
    gap: 4px;
  }

  .locale-btn {
    flex: 1;
    padding: 4px 8px;
    font-size: 11px;
    font-family: var(--font-label);
    font-weight: 600;
    border: 1px solid var(--outline-variant);
    border-radius: 4px;
    background: transparent;
    color: var(--on-surface-variant);
    cursor: pointer;
    transition: all 0.15s;
  }

  .locale-btn:hover:not(:disabled) {
    background: rgba(255, 255, 255, 0.06);
    color: var(--on-surface);
  }

  .locale-btn.selected {
    background: var(--primary);
    color: var(--on-primary);
    border-color: var(--primary);
  }

  .locale-btn:disabled {
    opacity: 0.3;
    cursor: not-allowed;
  }

  .titlebar-buttons {
    display: flex;
    height: 100%;
  }

  .titlebar-button {
    display: inline-flex;
    justify-content: center;
    align-items: center;
    width: 46px;
    height: 100%;
    background: transparent;
    border: none;
    color: var(--on-surface-variant);
    cursor: pointer;
    transition: background-color 0.1s;
  }

  .titlebar-button:hover {
    background: rgba(255, 255, 255, 0.1);
    color: #fff;
  }

  .titlebar-button.close-btn:hover {
    background: #e81123;
    color: #fff;
  }
</style>
