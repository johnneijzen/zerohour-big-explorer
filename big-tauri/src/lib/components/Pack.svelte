<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher();

  let inputDir = '';
  let outputFile = '';

  function pickDir() {
    dispatch('pick-dir');
  }

  function pack() {
    dispatch('pack', { inputDir, outputFile });
  }
</script>

<style>
  .pack-controls {
    display: flex;
    gap: 8px;
    align-items: center;
  }
</style>

<div class="pack-controls">
  <input placeholder="Input directory" bind:value={inputDir} />
  <input placeholder="Output .big file" bind:value={outputFile} />
  <button on:click={pickDir}>Browse</button>
  <button on:click={pack} disabled={!inputDir || !outputFile}>Pack</button>
</div>
