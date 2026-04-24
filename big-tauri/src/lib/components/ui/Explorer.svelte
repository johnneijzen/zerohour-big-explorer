<script lang="ts">
  import { invoke } from '@tauri-apps/api/core'
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { writable } from "svelte/store";

  const archive = writable("");
  const entries = writable<Array<any>>([]);

  // Robust runtime-aware chooseArchive: prefer global __TAURI__ when available,
  // fall back to dynamic plugin import to keep bundlers/SSR happy.
  async function chooseArchive() {
    try {
      const globalOpen = (globalThis as any).__TAURI__?.dialog?.open;
      const dialogOpen = globalOpen ?? (await import('@tauri-apps/plugin-dialog')).open;

      const sel = await dialogOpen({
        multiple: false,
        filters: [{ 
          name: 'BIG', 
          extensions: ['big'] 
        }],
        title: 'Select .BIG archive'
      });

      const path = Array.isArray(sel) ? sel[0] : sel;
      console.log('dialog selected', path);
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
      // Prefer runtime global invoke when available (Tauri runtime),
      // otherwise use a literal dynamic import so the bundler can resolve it.
      const globalInvoke = (globalThis as any).__TAURI__?.invoke;
      if (typeof globalInvoke === 'function') {
        const result = await globalInvoke('list_archive', { archivePath: path, filter: null });
        entries.set(result as any[]);
        return;
      }
      
      const result = await invoke('list_archive', { archivePath: path, filter: null });
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
      <!-- <Button on:click={chooseArchive}>Browse</Button> -->
      <button onclick={chooseArchive}>Open File</button>
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
