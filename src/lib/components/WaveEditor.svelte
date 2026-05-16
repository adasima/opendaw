<script lang="ts">
  // Waveform Editor Placeholder
</script>

<div class="w-full h-full bg-zinc-925 flex flex-col items-center justify-center border border-zinc-800">
     <h3 class="text-zinc-500 font-bold mb-2">Waveform</h3>
     <div class="flex items-center space-x-0.5 h-16">
        {#each Array(20) as _, i}
            <div class="w-1 bg-blue-500/80 rounded-full" style="height: {Math.sin(i)*100}%;"></div>
        {/each}
     </div>
</div>
