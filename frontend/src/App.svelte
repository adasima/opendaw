<script>
  let wasmModule = $state();
  import PluginBrowser from "./components/PluginBrowser.svelte";

  import "./app.css";
  import "./theme.css";
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { init as i18nInit, addMessages, _ } from "svelte-i18n";
  import TitleBar from "./lib/TitleBar.svelte";
  import TimelineCanvas from "./components/TimelineCanvas.svelte";
  import Tracks from "./components/Tracks.svelte";
  import Mixer from "./components/Mixer.svelte";
  import SessionView from "./components/SessionView.svelte";
  import Transport from "./components/Transport.svelte";
  import TrackDetails from "./components/TrackDetails.svelte";
  import en from "./locales/en.json";
  import ja from "./locales/ja.json";

  addMessages("en", en);
  addMessages("ja", ja);

  i18nInit({
    fallbackLocale: "en",
    initialLocale: "ja",
  });
  let wasmReady = $state(false);
  let wasmError = $state(null);
  let playheadPos = $state(0.0);
  let timeString = $state("00:00:00.000");
  let bpm = $state(120.0);
  let activeTrackId = $state(null);
  let tracks = $state([
    { id: 1, name: "Audio 1 (Select to Open PR)", color: "var(--primary)" },
    { id: 2, name: "VocalSynth 1 (ARA)", color: "#4ade80" }
  ]);
  let aiPanelOpen = $state(false);
  let showTrackDetails = $state(false);
  let showSessionView = $state(false);
  let showPluginBrowser = $state(false);
  let animationFrameId;

  $effect(() => {
    invoke("set_bpm", { bpm }).catch(console.error);
  });

  function updateTime() {
    if (wasmModule && wasmModule.get_playhead_pos) {
      playheadPos = wasmModule.get_playhead_pos();
      // 時間文字列のフォーマット (例: playheadPos が秒の場合)
      const date = new Date(playheadPos * 1000);
      const m = String(date.getUTCMinutes()).padStart(2, "0");
      const s = String(date.getUTCSeconds()).padStart(2, "0");
      const ms = String(date.getUTCMilliseconds()).padStart(3, "0");
      timeString = `00:${m}:${s}.${ms}`;
      
      if (wasmModule.get_tracks_json) {
        try {
          const jsonStr = wasmModule.get_tracks_json();
          if (jsonStr && jsonStr !== "[]") {
            const parsed = JSON.parse(jsonStr);
            tracks = parsed.map(t => ({
              id: t.id,
              name: t.name,
              color: t.id === 1 ? "var(--primary)" : "#4ade80",
              isMuted: t.is_muted,
              isSolo: t.is_solo,
              isRecordArmed: t.is_record_armed
            }));
          }
        } catch (e) {
          console.error("Failed to parse tracks JSON:", e);
        }
      }
    }
    animationFrameId = requestAnimationFrame(updateTime);
  }

  onMount(async () => {
    try {
      wasmModule = await import("../../opendaw-wasm/pkg/opendaw.js");
      await wasmModule.default();
      console.log("Wasm module initialized.");
      wasmModule.start("egui_canvas");
      wasmReady = true;
      console.log("egui canvas started.");
      updateTime();
    } catch (e) {
      console.error("Failed to load Wasm:", e);
      wasmError = String(e);
    }
  });

  // タイトルバーからAIパネルトグルを受け取るためのイベントリッスン
  // windowにカスタムイベントを生やしてTitleBarから叩かせる等でもよいが
  // 今回は単純にTitleBarにbindさせるか、windowに露出させておく
  window.toggleAiPanel = () => {
    aiPanelOpen = !aiPanelOpen;
  };

  function handleTrackSelect(id) {
    activeTrackId = id;
    showTrackDetails = true;
    if (wasmModule && wasmModule.select_track) {
      wasmModule.select_track(id);
    }
  }

  function handlePlay() {
    if (wasmModule && wasmModule.play) wasmModule.play();
    invoke("play").catch(console.error);
  }

  function handleStop() {
    if (wasmModule && wasmModule.stop) wasmModule.stop();
    invoke("stop").catch(console.error);
  }

  function handleToggleLoop() {
    if (wasmModule && wasmModule.toggle_loop) wasmModule.toggle_loop();
  }

  function handleMasterVolume(event) {
    const vol = parseFloat(event.target.value) / 100.0;
    if (wasmModule && wasmModule.set_master_volume) {
      wasmModule.set_master_volume(vol);
    }
    invoke("set_master_volume", { volume: vol }).catch(console.error);
  }

  function handleAddTrack() {
    invoke("add_track", { name: "New Track" }).catch(console.error);
  }
</script>

<TitleBar />

<main class="daw-container">
  <!-- グラスモーフィズムのサイドバー (Svelte側で実装) -->
  <aside class="sidebar glass-panel">

    <div class="sidebar-header">
      <h2>{$_("tracks.title")}</h2>
      <div style="display: flex; gap: 8px;">
        <button class="icon-btn" onclick={() => { showSessionView = !showSessionView; showPluginBrowser = false; }} title="Toggle Session View" style="width: auto; padding: 0 8px; font-size: 11px;">
          {showSessionView ? 'Timeline' : 'Session'}
        </button>
        <button class="icon-btn" onclick={() => { showPluginBrowser = !showPluginBrowser; showSessionView = false; }} title="Toggle Plugin Browser" style="width: auto; padding: 0 8px; font-size: 11px;">
          {showPluginBrowser ? 'Hide Plugins' : 'Plugins'}
        </button>
        <button class="icon-btn" aria-label="Add Track" onclick={handleAddTrack}>+</button>
      </div>
    </div>

    <div class="track-list">
      <Tracks
        tracks={tracks}
        activeTrackId={activeTrackId}
        onSelectTrack={handleTrackSelect}
      />
    </div>
  </aside>

  <div class="main-content">
    <!-- 中央のメインキャンバス (ここにegui Wasmがはまる) -->
    <div class="canvas-wrapper">
      <div style="display: {showSessionView ? 'block' : 'none'}; height: 100%;">
        <SessionView />
      </div>
      <div style="display: {showPluginBrowser ? 'block' : 'none'}; height: 100%;">
        <PluginBrowser />
      </div>
      <div style="display: {(!showSessionView && !showPluginBrowser) ? 'block' : 'none'}; height: 100%;">
        <TimelineCanvas id="egui_canvas" {wasmModule} />
      </div>
      {#if !wasmReady && !wasmError && !showSessionView && !showPluginBrowser}
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
    <div class="bottom-panel glass-panel">
      <Transport
        {timeString}
        bind:bpm={bpm}
        onPlay={handlePlay}
        onStop={handleStop}
        onToggleLoop={handleToggleLoop}
      />
      <Mixer onMasterVolume={handleMasterVolume} />
    </div>
  </div>


  {#if showTrackDetails && activeTrackId !== null}
    <aside class="ai-panel glass-panel" style="width: 300px;">
      <div class="sidebar-header">
        <h2>Track Details</h2>
        <button class="icon-btn" onclick={() => (showTrackDetails = false)}>✕</button>
      </div>
      <div class="ai-content">
        <div style="padding: 12px;">
          <TrackDetails trackId={activeTrackId} />
        </div>
      </div>
    </aside>
  {/if}

  {#if aiPanelOpen}

    <aside class="ai-panel glass-panel">
      <div class="sidebar-header">
        <h2>AI Agent & CLI</h2>
        <button class="icon-btn" onclick={() => (aiPanelOpen = false)}>✕</button
        >
      </div>
      <div class="ai-content">
        <div class="chat-message system">Agent is ready.</div>
      </div>
      <div class="ai-input">
        <input type="text" placeholder="Ask AI to automate..." />
      </div>
    </aside>
  {/if}
</main>

<div class="orb-container">
  <div class="orb orb-1"></div>
  <div class="orb orb-2"></div>
  <div class="orb orb-3"></div>
  <div class="orb orb-4"></div>
</div>

<style>
  /* 透過背景用に全体レイアウト設定 */
  .daw-container {
    display: flex;
    height: 100vh;
    padding-top: var(--titlebar-height, 32px);
    box-sizing: border-box;
    background: transparent;
    position: relative;
    z-index: 10;
    gap: 12px;
    padding: calc(var(--titlebar-height, 32px) + 12px) 12px 12px 12px;
  }

  .glass-panel {
    background: rgba(20, 20, 25, 0.4);
    backdrop-filter: blur(20px);
    border: 1px solid rgba(255, 255, 255, 0.08);
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
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



  /* メインコンテンツエリア */
  .main-content {
    flex-grow: 1;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .canvas-wrapper {
    flex-grow: 1;
    position: relative;
    border-radius: 12px;
    overflow: hidden;
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

  /* ユーティリティ */
  .loading-overlay,
  .error-overlay {
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
    to {
      transform: rotate(360deg);
    }
  }

  .error-msg {
    color: #ef4444;
    background: rgba(239, 68, 68, 0.1);
    padding: 12px 24px;
    border-radius: 8px;
    border: 1px solid rgba(239, 68, 68, 0.3);
  }

  /* AI Panel */
  .ai-panel {
    width: 300px;
    display: flex;
    flex-direction: column;
    border-radius: 12px;
    overflow: hidden;
    animation: slide-in 0.2s ease-out;
  }

  @keyframes slide-in {
    from {
      transform: translateX(100%);
      opacity: 0;
    }
    to {
      transform: translateX(0);
      opacity: 1;
    }
  }

  .ai-content {
    flex-grow: 1;
    padding: 12px;
    overflow-y: auto;
  }

  .chat-message {
    font-size: 13px;
    color: var(--on-surface-variant);
    padding: 8px 12px;
    background: rgba(255, 255, 255, 0.05);
    border-radius: 8px;
  }

  .ai-input {
    padding: 12px;
    border-top: 1px solid var(--outline-variant);
  }

  .ai-input input {
    width: 100%;
    padding: 8px 12px;
    background: rgba(0, 0, 0, 0.2);
    border: 1px solid var(--outline);
    border-radius: 6px;
    color: white;
    font-family: var(--font-label);
  }

  /* Orbs Background */
  .orb-container {
    position: fixed;
    inset: 0;
    z-index: 0;
    pointer-events: none;
    background: #0d0b14;
    overflow: hidden;
  }

  .orb {
    position: absolute;
    border-radius: 50%;
    filter: blur(100px);
    opacity: 0.6;
    animation: float 20s infinite ease-in-out alternate;
  }

  .orb-1 {
    width: 50vw;
    height: 50vw;
    background: #6b21a8;
    top: -10%;
    left: -10%;
    animation-delay: 0s;
  }
  .orb-2 {
    width: 60vw;
    height: 60vw;
    background: #1e3a8a;
    bottom: -20%;
    right: -10%;
    animation-delay: -5s;
  }
  .orb-3 {
    width: 40vw;
    height: 40vw;
    background: #9d174d;
    top: 40%;
    left: 30%;
    animation-delay: -10s;
  }
  .orb-4 {
    width: 45vw;
    height: 45vw;
    background: #047857;
    top: 10%;
    right: 20%;
    animation-delay: -15s;
  }

  @keyframes float {
    0% {
      transform: translate(0, 0) scale(1);
    }
    33% {
      transform: translate(5%, 10%) scale(1.1);
    }
    66% {
      transform: translate(-10%, 5%) scale(0.9);
    }
    100% {
      transform: translate(0, -5%) scale(1.05);
    }
  }
</style>
