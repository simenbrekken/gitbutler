<script lang="ts">
	import ContextMenu from '$lib/components/contextmenu/ContextMenu.svelte';
	import ContextMenuItem from '$lib/components/contextmenu/ContextMenuItem.svelte';
	import ContextMenuSection from '$lib/components/contextmenu/ContextMenuSection.svelte';
	import Button from '@gitbutler/ui/Button.svelte';
	import Checkbox from '@gitbutler/ui/Checkbox.svelte';
	import Toggle from '@gitbutler/ui/Toggle.svelte';
	import type { Writable, Readable } from 'svelte/store';

	export let filtersActive: Readable<boolean>;
	export let showPrCheckbox: boolean;

	export let includePrs: Writable<boolean | undefined>;
	export let includeRemote: Writable<boolean | undefined>;
	export let hideBots: Writable<boolean | undefined>;
	export let hideInactive: Writable<boolean | undefined>;

	let target: HTMLElement;
	let contextMenu: ReturnType<typeof ContextMenu>;

	export function onFilterClick() {
		contextMenu.toggle();
	}
</script>

<div class="header__filter-btn">
	<Button
		bind:el={target}
		style="ghost"
		outline
		icon={$filtersActive ? 'filter-applied-small' : 'filter-small'}
		onmousedown={onFilterClick}
	>
		Filter
	</Button>
	<ContextMenu bind:this={contextMenu} {target}>
		<ContextMenuSection>
			{#if showPrCheckbox}
				<ContextMenuItem label="Pull requests" onclick={() => ($includePrs = !$includePrs)}>
					{#snippet control()}
						<Checkbox small bind:checked={$includePrs} />
					{/snippet}
				</ContextMenuItem>
			{/if}
			<ContextMenuItem label="Branches" onclick={() => ($includeRemote = !$includeRemote)}>
				{#snippet control()}
					<Checkbox small bind:checked={$includeRemote} />
				{/snippet}
			</ContextMenuItem>
		</ContextMenuSection>

		<ContextMenuSection>
			<ContextMenuItem label="Hide bots" onclick={() => ($hideBots = !$hideBots)}>
				{#snippet control()}
					<Toggle small bind:checked={$hideBots} />
				{/snippet}
			</ContextMenuItem>
			<ContextMenuItem label="Hide inactive" onclick={() => ($hideInactive = !$hideInactive)}>
				{#snippet control()}
					<Toggle small bind:checked={$hideInactive} />
				{/snippet}
			</ContextMenuItem>
		</ContextMenuSection>
	</ContextMenu>
</div>
