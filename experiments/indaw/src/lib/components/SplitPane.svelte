<script lang="ts">
  export let type: "horizontal" | "vertical" = "horizontal";
  export let primary: "first" | "second" = "first";
  export let minSize = 50;
  export let initialSize = 250;

  let size = initialSize;
  let container: HTMLDivElement;
  let isDragging = false;

  function startDrag(e: MouseEvent) {
    e.preventDefault();
    isDragging = true;
    document.body.style.cursor =
      type === "horizontal" ? "col-resize" : "row-resize";
    window.addEventListener("mousemove", onDrag);
    window.addEventListener("mouseup", stopDrag);
  }

  function onDrag(e: MouseEvent) {
    if (!container) return;
    const rect = container.getBoundingClientRect();

    if (type === "horizontal") {
      const total = rect.width;
      if (primary === "first") {
        const newSize = e.clientX - rect.left;
        size = Math.max(minSize, Math.min(newSize, total - minSize));
      } else {
        const newSize = rect.right - e.clientX;
        size = Math.max(minSize, Math.min(newSize, total - minSize));
      }
    } else {
      const total = rect.height;
      if (primary === "first") {
        const newSize = e.clientY - rect.top;
        size = Math.max(minSize, Math.min(newSize, total - minSize));
      } else {
        const newSize = rect.bottom - e.clientY;
        size = Math.max(minSize, Math.min(newSize, total - minSize));
      }
    }
  }

  function stopDrag() {
    isDragging = false;
    document.body.style.cursor = "";
    window.removeEventListener("mousemove", onDrag);
    window.removeEventListener("mouseup", stopDrag);
  }
</script>

<div
  bind:this={container}
  class="flex w-full h-full overflow-hidden {type === 'horizontal'
    ? 'flex-row'
    : 'flex-col'}"
>
  {#if primary === "first"}
    <div
      style="{type === 'horizontal' ? `width:${size}px` : `height:${size}px`};"
      class="shrink-0 relative"
    >
      <slot name="first" />
    </div>

    <!-- Handle -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="{type === 'horizontal'
        ? 'w-1 cursor-col-resize border-x border-theme-skin-shadow/50'
        : 'h-1 cursor-row-resize border-y border-theme-skin-shadow/50'} 
             bg-theme-wall-shadow/30 hover:bg-theme-skin transition-colors z-10 shrink-0 select-none {isDragging
        ? 'bg-theme-skin'
        : ''}"
      on:mousedown={startDrag}
    ></div>

    <div class="flex-1 relative overflow-hidden min-w-0 min-h-0">
      <slot name="second" />
    </div>
  {:else}
    <!-- Primary is second -->
    <div class="flex-1 relative overflow-hidden min-w-0 min-h-0">
      <slot name="first" />
    </div>

    <!-- Handle -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="{type === 'horizontal'
        ? 'w-1 cursor-col-resize border-x border-theme-skin-shadow/50'
        : 'h-1 cursor-row-resize border-y border-theme-skin-shadow/50'} 
             bg-theme-wall-shadow/30 hover:bg-theme-skin transition-colors z-10 shrink-0 select-none {isDragging
        ? 'bg-theme-skin'
        : ''}"
      on:mousedown={startDrag}
    ></div>

    <div
      style="{type === 'horizontal' ? `width:${size}px` : `height:${size}px`};"
      class="shrink-0 relative"
    >
      <slot name="second" />
    </div>
  {/if}

  <!-- Overlay when dragging -->
  {#if isDragging}
    <div
      class="fixed inset-0 z-[9999] cursor-{type === 'horizontal'
        ? 'col-resize'
        : 'row-resize'}"
    ></div>
  {/if}
</div>
