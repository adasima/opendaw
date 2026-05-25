<script>
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/core";

  let { id = "egui_canvas" } = $props();
  let pollInterval;

  async function pollProjectState() {
    try {
      // 1. Tauri側からJSONで状態を取得する (Rust側にget_project_stateを実装済みの想定)
      const stateJson = await invoke("get_project_state");

      // 2. opendaw-wasmのエクスポート関数を呼び出して状態を渡す
      if (window.wasmBindings && window.wasmBindings.set_tracks_json) {
        window.wasmBindings.set_tracks_json(stateJson);
        window.wasmBindings.request_repaint();
      }
    } catch (err) {
      // コマンド未実装時やエラー時はログを残しつつスキップ
      // console.debug("Failed to poll project state:", err);
    }
  }

  onMount(() => {
    // 10FPS (100ms) 程度で定期的にTauriからプロジェクト状態を取得しWASMに渡す
    pollInterval = setInterval(pollProjectState, 100);
  });

  onDestroy(() => {
    if (pollInterval) {
      clearInterval(pollInterval);
    }
  });
</script>

<div class="canvas-container glass-panel">
  <canvas {id}></canvas>
</div>

<style>
  .canvas-container {
    width: 100%;
    height: 100%;
    border-radius: 12px;
    position: relative;
    overflow: hidden;
    background: transparent;
  }

  canvas {
    width: 100%;
    height: 100%;
    display: block;
    outline: none;
    background: transparent;
  }
</style>
