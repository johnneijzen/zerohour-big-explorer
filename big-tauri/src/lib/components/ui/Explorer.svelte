<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { writable } from "svelte/store";

  const archive = writable("");
  const entries = writable<Array<any>>([]);

  // Use plugin-dialog open as requested (dynamic import keeps bundler happy)
  async function chooseArchive() {
    try {
      const { open } = await import('@tauri-apps/plugin-dialog');
      const sel = await open({ multiple: false, directory: false, filters: [{ name: 'BIG', extensions: ['big'] }], title: 'Select .BIG archive' });
      const path = Array.isArray(sel) ? sel[0] : sel;
      console.log('plugin-dialog selected', path);
      if (path) {
        archive.set(path as string);
        await listArchive(path as string);
      }
    } catch (err) {
      console.error('chooseArchive error', err);
      alert('Could not open file dialog: ' + String(err));
    }
  }

  async function listArchive(path: string) {
    try {
      // Use dynamic import for the runtime invoke to avoid SSR bundling problems
      const tauriPath = ['@tauri-apps','/api/tauri'].join('');
      const { invoke } = await import(tauriPath);
      const result = await invoke('list_archive', { archive_path: path, filter: null });
      entries.set(result as any[]);
    } catch (err) {
      console.error('listArchive error', err);
      entries.set([]);
      alert('Failed to list archive: ' + String(err));
    }
  }
</script>

<style>
  .entry { display:flex; align-items:center; gap:0.75rem; padding:0.25rem 0.5rem; border-bottom:1px solid rgba(0,0,0,0.04); }
  .muted { color: rgba(0,0,0,0.5); font-size:0.9rem; }
</style>

<div class="p-4">
  <h2 class="text-lg font-semibold mb-2">Explorer</h2>

  <div class="grid w-full max-w-sm items-center gap-1.5 mb-4">
    <Label for="archive">BIG Archive</Label>
    <div class="flex gap-2 w-full">
      <Input id="archive" type="text" bind:value={$archive} placeholder="Select .BIG archive" />
      <Button on:click={chooseArchive}>Browse</Button>
    </div>
  </div>

  <div>
    {#if $entries.length === 0}
      <div class="muted">No entries loaded</div>
    {/if}

    {#each $entries as e}
      {#if e?.name}
        <div class="entry" style="padding-left: {(e.name.split('/').length - 1) * 12}px">
          <div>
            <div class="font-medium">{e.name}</div>
            <div class="muted">{e.length} bytes {e.compressed ? '(compressed)' : ''}</div>
          </div>
        </div>
      {/if}
    {/each}
  </div>
</div>
