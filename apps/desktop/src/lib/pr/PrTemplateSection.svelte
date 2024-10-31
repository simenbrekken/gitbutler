<script lang="ts">
	import { ForgeService } from '$lib/backend/forge';
	import { Project } from '$lib/backend/projects';
	import { getForge } from '$lib/forge/interface/forge';
	import Select from '$lib/select/Select.svelte';
	import SelectItem from '$lib/select/SelectItem.svelte';
	import { getContext } from '@gitbutler/shared/context';
	import { persisted } from '@gitbutler/shared/persisted';
	import { onMount } from 'svelte';

	interface Props {
		availableTemplates: string[];
		onselected: (body: string) => void;
	}

	let { availableTemplates, onselected }: Props = $props();

	const forge = getForge();
	// TODO: Rename or refactor this service.
	const forgeService = getContext(ForgeService);

	const project = getContext(Project);
	const lastUsedTemplate = persisted<string | undefined>(undefined, `last-template-${project.id}`);

	async function setPullRequestTemplatePath(path: string) {
		if ($forge) {
			lastUsedTemplate.set(path);
			loadAndEmit(path);
		}
	}

	async function loadAndEmit(path: string) {
		if (path && $forge) {
			const template = await forgeService.getReviewTemplateContent($forge.name, path);
			if (template) {
				onselected(template);
			}
		}
	}

	onMount(() => {
		if ($lastUsedTemplate && availableTemplates.includes($lastUsedTemplate)) {
			loadAndEmit($lastUsedTemplate);
		} else if (availableTemplates.length === 1) {
			const path = availableTemplates.at(0);
			if (path) {
				loadAndEmit(path);
				lastUsedTemplate.set(path);
			}
		}
	});
</script>

<div class="pr-template__wrap">
	<Select
		value={$lastUsedTemplate}
		options={availableTemplates.map((value) => ({ label: value, value }))}
		placeholder={availableTemplates.length > 0
			? 'Choose template'
			: 'No PR templates found ¯_(ツ)_/¯'}
		flex="1"
		searchable
		disabled={availableTemplates.length === 0}
		onselect={setPullRequestTemplatePath}
	>
		{#snippet itemSnippet({ item, highlighted })}
			<SelectItem selected={item.value === $lastUsedTemplate} {highlighted}>
				{item.label}
			</SelectItem>
		{/snippet}
	</Select>
</div>

<style lang="postcss">
	.pr-template__wrap {
		display: flex;
		gap: 6px;
	}
</style>
