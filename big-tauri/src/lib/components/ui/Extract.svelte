<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  export let archive: string;
  export let name: string;

  async function extract() {
    try {
      // Prefer native save dialog + backend write to disk when available
      const globalSave = (globalThis as any).__TAURI__?.dialog?.save;
      const dialogSave = globalSave ?? (await import('@tauri-apps/plugin-dialog')).save;

      // Ensure we use only the base filename. Some archives use backslashes
      // (Windows-style). Normalize separators to forward slashes first.
      function basename(p: string) {
        if (!p) return 'file.bin';
        const normalized = p.replace(/\\/g, '/');
        const parts = normalized.split('/');
        return parts[parts.length - 1] || 'file.bin';
      }

      const defaultName = basename(name);
      // Use only the base filename as the suggested default so the dialog
      // shows a simple filename instead of a full archive-relative path.
      const dest = await dialogSave({ defaultPath: defaultName, title: defaultName });

      if (dest) {
        const args = { archivePath: archive, entryName: name, output: dest };
        console.debug('invoke extract_file_to_disk args:', args);
        await invoke('extract_file_to_disk', args);
        return;
      }

      // If no dest chosen (or dialog not available), fall back to returning bytes and downloading in-browser
      const args = { archivePath: archive, entryName: name };
      console.debug('invoke extract_file_bytes args:', args);
      const bytes: number[] = await invoke('extract_file_bytes', args);
      const u8 = new Uint8Array(bytes);
      const blob = new Blob([u8]);
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = defaultName;
      document.body.appendChild(a);
      a.click();
      a.remove();
      URL.revokeObjectURL(url);
    } catch (err) {
      alert('Extract failed: ' + String(err));
    }
  }
</script>

<button on:click={extract}>Extract</button>
