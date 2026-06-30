<script>
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/core";

  let { id = "egui_canvas", wasmModule = null } = $props();
  let pollInterval;

  async function pollProjectState() {
    try {
      // 1. Tauri側からJSONで状態を取得する (Rust側にget_project_stateを実装済みの想定)
      const stateJson = await invoke("get_project_state");

      // 2. opendaw-wasmのエクスポート関数を呼び出して状態を渡す
      if (wasmModule && wasmModule.set_tracks_json) {
        wasmModule.set_tracks_json(stateJson);
        if (wasmModule.request_repaint) {
            wasmModule.request_repaint();
        }
      } else if (window.wasmBindings && window.wasmBindings.set_tracks_json) {
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

  function handleDragOver(e) {
    e.preventDefault();
    e.dataTransfer.dropEffect = "copy";
  }

  function handleDrop(e) {
    e.preventDefault();
    try {
      const dataStr = e.dataTransfer.getData("application/json");
      if (!dataStr) return;
      const data = JSON.parse(dataStr);

      const rect = e.target.getBoundingClientRect();
      const y = e.clientY - rect.top;

      let trackId = -1;
      if (wasmModule && wasmModule.get_track_id_at_y) {
        trackId = wasmModule.get_track_id_at_y(y);
      } else if (window.wasmBindings && window.wasmBindings.get_track_id_at_y) {
        trackId = window.wasmBindings.get_track_id_at_y(y);
      }

      if (trackId >= 0 && data.type === "plugin") {
        invoke("load_plugin_to_track", { track_id: trackId, plugin_id: data.id }).catch(console.error);
      }
    } catch (err) {
      console.error("Drop handling failed:", err);
    }
  }
</script>

<div class="canvas-container glass-panel" role="region" ondragover={handleDragOver} ondrop={handleDrop}>
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
