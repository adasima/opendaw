<script>
  import './app.css';
  import './theme.css';
  import { onMount } from 'svelte';
  import { init as i18nInit, addMessages, _ } from 'svelte-i18n';
  import TitleBar from './lib/TitleBar.svelte';
  import en from './locales/en.json';
  import ja from './locales/ja.json';

  addMessages('en', en);
  addMessages('ja', ja);

  i18nInit({
    fallbackLocale: 'en',
    initialLocale: 'ja',
  });

  let wasmReady = $state(false);
  let wasmError = $state('');
  let wasmModule;

  onMount(async () => {
    try {
      wasmModule = await import('../../opendaw-wasm/pkg/opendaw_wasm.js');
      await wasmModule.default();
      console.log("Wasm module initialized.");
      wasmModule.start("egui_canvas");
      wasmReady = true;
      console.log("egui canvas started.");
    } catch (e) {
      console.error("Failed to load Wasm:", e);
      wasmError = String(e);
    }
  });

  function handlePlay() {
    if (wasmModule && wasmModule.play) wasmModule.play();
  }

  function handleStop() {
    if (wasmModule && wasmModule.stop) wasmModule.stop();
  }

  function handleToggleLoop() {
    if (wasmModule && wasmModule.toggle_loop) wasmModule.toggle_loop();
  }

  function handleMasterVolume(event) {
    if (wasmModule && wasmModule.set_master_volume) {
      wasmModule.set_master_volume(parseFloat(event.target.value) / 100.0);
    }
  }
</script>

<TitleBar />

<main class="daw-container">
  <!-- グラスモーフィズムのサイドバー (Svelte側で実装) -->
  <aside class="sidebar glass-panel">
    <div class="sidebar-header">
      <h2>{$_('tracks.title')}</h2>
      <button class="icon-btn" aria-label="Add Track">+</button>
    </div>
    
    <div class="track-list">
      <!-- トラックのモックアップ -->
      <div class="track-item active">
        <div class="track-color" style="background: var(--primary);"></div>
        <div class="track-info">
          <span class="track-name">Audio 1</span>
          <div class="track-controls">
            <button class="ctrl-btn mute">M</button>
            <button class="ctrl-btn solo">S</button>
            <button class="ctrl-btn record">R</button>
          </div>
        </div>
      </div>
      <div class="track-item">
        <div class="track-color" style="background: #4ade80;"></div>
        <div class="track-info">
          <span class="track-name">MIDI 1</span>
          <div class="track-controls">
            <button class="ctrl-btn mute">M</button>
            <button class="ctrl-btn solo">S</button>
            <button class="ctrl-btn record">R</button>
          </div>
        </div>
      </div>
    </div>
  </aside>

  <div class="main-content">
    <!-- 中央のメインキャンバス (ここにegui Wasmがはまる) -->
    <div class="canvas-container glass-panel">
      <canvas id="egui_canvas"></canvas>
      {#if !wasmReady && !wasmError}
        <div class="loading-overlay">
          <div class="spinner"></div>
          <span>Loading Engine...</span>
        </div>
      {/if}
      {#if wasmError}
        <div class="error-overlay">
          <p class="error-msg">Engine Error: {wasmError}</p>
        </div>
      {/if}
    </div>

    <!-- 下部ミキサー・トランスポートパネルのモック -->
    <footer class="bottom-panel glass-panel">
      <div class="transport-controls">
        <button class="transport-btn" onclick={handleToggleLoop} title="Toggle Loop">🔁</button>
        <button class="transport-btn play" onclick={handlePlay} title="Play">▶</button>
        <button class="transport-btn" onclick={handleStop} title="Stop">⏹</button>
        <button class="transport-btn record" title="Record">⏺</button>
      </div>
      <div class="time-display">
        <span>00:01:23.456</span>
      </div>
      <div class="master-fader">
        <span>Master</span>
        <input type="range" min="0" max="100" value="80" class="fader" oninput={handleMasterVolume}>
      </div>
    </footer>
  </div>
</main>

<style>
  .daw-container {
    display: flex;
    height: 100vh;
    padding-top: var(--titlebar-height, 32px);
    box-sizing: border-box;
    background: var(--background);
    gap: 12px;
    padding: calc(var(--titlebar-height, 32px) + 12px) 12px 12px 12px;
  }

  /* サイドバー */
  .sidebar {
    width: 280px;
    display: flex;
    flex-direction: column;
    border-radius: 12px;
    overflow: hidden;
  }

  .sidebar-header {
    padding: 16px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid var(--outline-variant);
  }

  .sidebar-header h2 {
    font-size: 14px;
    margin: 0;
    color: var(--on-surface);
    font-weight: 600;
  }

  .icon-btn {
    background: transparent;
    border: 1px solid var(--outline);
    color: var(--on-surface);
    width: 24px;
    height: 24px;
    border-radius: 4px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s;
  }

  .icon-btn:hover {
    background: var(--primary);
    color: var(--on-primary);
    border-color: var(--primary);
  }

  .track-list {
    flex-grow: 1;
    overflow-y: auto;
    padding: 8px;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .track-item {
    display: flex;
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid transparent;
    border-radius: 6px;
    overflow: hidden;
    cursor: pointer;
    transition: all 0.2s;
  }

  .track-item:hover {
    background: rgba(255, 255, 255, 0.06);
  }

  .track-item.active {
    background: rgba(255, 255, 255, 0.08);
    border-color: var(--outline-variant);
  }

  .track-color {
    width: 6px;
  }

  .track-info {
    padding: 8px 12px;
    flex-grow: 1;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .track-name {
    font-size: 13px;
    font-weight: 500;
  }

  .track-controls {
    display: flex;
    gap: 4px;
  }

  .ctrl-btn {
    width: 22px;
    height: 22px;
    border-radius: 4px;
    border: 1px solid var(--outline-variant);
    background: transparent;
    color: var(--on-surface-variant);
    font-size: 10px;
    font-weight: bold;
    cursor: pointer;
    transition: all 0.15s;
  }

  .ctrl-btn:hover {
    background: rgba(255, 255, 255, 0.1);
  }
  
  .ctrl-btn.mute:hover { color: #f87171; border-color: #f87171; }
  .ctrl-btn.solo:hover { color: #fbbf24; border-color: #fbbf24; }
  .ctrl-btn.record:hover { color: #ef4444; border-color: #ef4444; }

  /* メインコンテンツエリア */
  .main-content {
    flex-grow: 1;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .canvas-container {
    flex-grow: 1;
    border-radius: 12px;
    position: relative;
    overflow: hidden;
  }

  #egui_canvas {
    width: 100%;
    height: 100%;
    display: block;
    outline: none;
  }

  /* 下部パネル */
  .bottom-panel {
    height: 60px;
    border-radius: 12px;
    display: flex;
    align-items: center;
    padding: 0 24px;
    gap: 32px;
  }

  .transport-controls {
    display: flex;
    gap: 8px;
  }

  .transport-btn {
    width: 36px;
    height: 36px;
    border-radius: 18px;
    border: 1px solid var(--outline-variant);
    background: rgba(255, 255, 255, 0.05);
    color: var(--on-surface);
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all 0.2s;
  }

  .transport-btn:hover {
    background: rgba(255, 255, 255, 0.15);
  }

  .transport-btn.play {
    width: 48px;
    height: 48px;
    border-radius: 24px;
    font-size: 20px;
    background: var(--primary);
    color: var(--on-primary);
    border: none;
    box-shadow: 0 0 15px rgba(255, 115, 0, 0.4);
  }

  .transport-btn.play:hover {
    transform: scale(1.05);
    box-shadow: 0 0 20px rgba(255, 115, 0, 0.6);
  }

  .transport-btn.record {
    color: #ef4444;
  }

  .time-display {
    font-family: 'Courier New', Courier, monospace;
    font-size: 24px;
    color: var(--primary);
    font-weight: bold;
    text-shadow: 0 0 10px rgba(255, 115, 0, 0.3);
    padding: 4px 16px;
    background: rgba(0, 0, 0, 0.3);
    border-radius: 6px;
    border: 1px inset rgba(255, 255, 255, 0.1);
  }

  .master-fader {
    margin-left: auto;
    display: flex;
    align-items: center;
    gap: 12px;
    font-size: 12px;
    color: var(--on-surface-variant);
  }

  .fader {
    width: 120px;
    accent-color: var(--primary);
  }

  /* ユーティリティ */
  .loading-overlay, .error-overlay {
    position: absolute;
    inset: 0;
    display: flex;
    flex-direction: column;
    gap: 16px;
    align-items: center;
    justify-content: center;
    background: rgba(0, 0, 0, 0.6);
    backdrop-filter: blur(4px);
    color: var(--on-surface-variant);
    font-family: var(--font-label);
  }

  .spinner {
    width: 32px;
    height: 32px;
    border: 3px solid rgba(255, 255, 255, 0.1);
    border-top-color: var(--primary);
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .error-msg {
    color: #ef4444;
    background: rgba(239, 68, 68, 0.1);
    padding: 12px 24px;
    border-radius: 8px;
    border: 1px solid rgba(239, 68, 68, 0.3);
  }
</style>
