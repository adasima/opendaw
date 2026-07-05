<script lang="ts">
  export let value: number = 0; // 0.0 to 1.0
  export let vertical: boolean = false;
  
  // Clamping
  $: level = Math.max(0, Math.min(1, value));
  
  // Color calculation for single gradient bar
  // Green (low) -> Yellow (mid) -> Red (high)
</script>

<div 
  class="bg-theme-hair/20 rounded overflow-hidden relative border border-theme-wall-shadow/30 w-full h-full"
>
  <!-- Meter Fill -->
  <div 
    class="absolute bottom-0 left-0 transition-all duration-75 ease-out bg-green-500"
    style:width={vertical ? '100%' : `${level * 100}%`}
    style:height={vertical ? `${level * 100}%` : '100%'}
    style:background={level > 0.8 ? '#ef4444' : level > 0.5 ? '#eab308' : '#22c55e'}
    style:opacity={level > 0.001 ? 1 : 0}
  ></div>
</div>
