<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher();
  let srcDir = '';
  let outFile = '';

  function onPack() {
    dispatch('pack', { srcDir, outFile });
  }
</script>

<div class="pack">
  <label>Source Directory</label>
  <input bind:value={srcDir} placeholder="Select folder to pack" />
  <label>Output .BIG file</label>
  <input bind:value={outFile} placeholder="out.big" />
  <button on:click={onPack}>Pack</button>
</div>

<style>
.pack { display:flex; flex-direction:column; gap:8px; }
input { padding:6px; }
button { padding:8px; }
</style>
