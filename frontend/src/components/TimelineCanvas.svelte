<script>
  let { id = "egui_canvas" } = $props();
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
