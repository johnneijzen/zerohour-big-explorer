<script lang="ts">
	import './layout.css';
	import { onMount } from 'svelte';

	onMount(() => {
		document.documentElement.classList.add('dark');
	});

	const { children } = $props();
</script>

{@render children()}
