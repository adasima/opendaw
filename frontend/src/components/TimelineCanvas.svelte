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
</script>

<div
  class="canvas-container glass-panel"
  ondragover={(e) => {
    e.preventDefault();
    if (wasmModule && wasmModule.set_drag_hover_y) {
      let rect = e.currentTarget.getBoundingClientRect();
      wasmModule.set_drag_hover_y(e.clientY - rect.top);
    } else if (window.wasmBindings && window.wasmBindings.set_drag_hover_y) {
      let rect = e.currentTarget.getBoundingClientRect();
      window.wasmBindings.set_drag_hover_y(e.clientY - rect.top);
    }
  }}
  ondragleave={(e) => {
    if (wasmModule && wasmModule.clear_drag_hover) {
      wasmModule.clear_drag_hover();
    } else if (window.wasmBindings && window.wasmBindings.clear_drag_hover) {
      window.wasmBindings.clear_drag_hover();
    }
  }}
  ondrop={async (e) => {
    e.preventDefault();
    if (wasmModule && wasmModule.clear_drag_hover) {
      wasmModule.clear_drag_hover();
    } else if (window.wasmBindings && window.wasmBindings.clear_drag_hover) {
      window.wasmBindings.clear_drag_hover();
    }

    let pluginName = e.dataTransfer.getData("application/vnd.opendaw.plugin");
    if (pluginName) {
      let rect = e.currentTarget.getBoundingClientRect();
      let y = e.clientY - rect.top;

      // Calculate track index based on TRACK_HEIGHT = 80.0
      // Also need to account for automation lanes if they are open, but assuming simple 80px tracks for now.
      // Svelte side tracks are known via polling.
      try {
        let state = await invoke("get_project_state");
        let tracks = state.tracks;

        let trackIndex = Math.floor(y / 80.0);

        if (trackIndex >= 0 && trackIndex < tracks.length) {
          let trackId = tracks[trackIndex].id;
          invoke("load_plugin_to_track", { track_id: trackId, plugin_id: pluginName }).catch(console.error);
        }
      } catch (err) {
        console.error("Failed to drop plugin:", err);
      }
    }
  }}
>
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
