<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  export let archive: string;

  async function unpack() {
    try {
      const dialog = await import('@tauri-apps/plugin-dialog');
      const dest = await dialog.save({ directory: true, title: 'Select output folder' });
      if (!dest) return;
      await invoke('unpack_all', { archivePath: archive, outputDir: dest });
      alert('Unpack completed');
    } catch (err) {
      alert('Unpack failed: ' + String(err));
    }
  }
</script>

<button on:click={unpack}>Unpack All</button>

<style>
button { padding:8px; }
</style>
