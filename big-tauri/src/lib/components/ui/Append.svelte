<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  export let archive: string;

  let targetPath = '';
  let force = false;

  async function chooseSource() {
    try {
      // call tauri-open dialog
      const sel: string | string[] = await invoke('open_dialog');
      const path = Array.isArray(sel) ? sel[0] : sel;
      if (path) {
        // prompt for target path inside archive
        const name = prompt('Enter target archive path (e.g. Data/Audio/file.wav)', path.split('/').pop() || 'file.bin');
        if (name) {
          targetPath = name;
          // Check if target exists in archive and confirm overwrite if necessary
          try {
            const args = { archivePath: archive, filter: null };
            console.debug('invoke list_archive args (append):', args);
            const entries: any[] = await invoke('list_archive', args);
            const exists = entries.some((e) => e.name === targetPath);
            let doForce = force;
            if (exists && !doForce) {
              const ok = confirm(`Target path ${targetPath} already exists in archive. Overwrite?`);
              if (!ok) {
                alert('Append cancelled');
                return;
              }
              doForce = true;
            }

            const aargs = { archivePath: archive, source: path, target_archive_path: targetPath, force: doForce };
            console.debug('invoke append_file args:', aargs);
            const res = await invoke('append_file', aargs);
            alert('Append result: ' + JSON.stringify(res));
          } catch (err) {
            alert('Append failed: ' + String(err));
          }
        }
      }
    } catch (err) {
      alert('Append failed: ' + String(err));
    }
  }
</script>

<div class="append">
  <label><input type="checkbox" bind:checked={force} /> Force overwrite</label>
  <button on:click={chooseSource}>Append File</button>
</div>
