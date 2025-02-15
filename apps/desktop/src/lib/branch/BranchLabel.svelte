<script lang="ts">
	import { autoSelectBranchNameFeature } from '$lib/config/uiFeatureFlags';
	import { resizeObserver } from '@gitbutler/ui/utils/resizeObserver';

	interface Props {
		name: string;
		disabled?: boolean;
		readonly?: boolean;
		onChange?: (value: string) => void;
		onDblClick?: () => void;
	}

	let { name, disabled = false, readonly = false, onChange, onDblClick }: Props = $props();

	let inputEl: HTMLInputElement;
	let initialName = name;
	let inputWidth = $state('');
</script>

<span
	use:resizeObserver={(e) => {
		inputWidth = `${Math.round(e.frame.width)}px`;
	}}
	class="branch-name-measure-el text-14 text-bold"
>
	{name}
</span>
<input
	type="text"
	{disabled}
	{readonly}
	bind:this={inputEl}
	bind:value={name}
	onchange={(e) => onChange?.(e.currentTarget.value.trim())}
	title={name}
	class="branch-name-input text-14 text-bold"
	ondblclick={(e) => {
		e.stopPropagation();
		if (readonly) {
			onDblClick?.();
		}
	}}
	oncontextmenu={(e) => {
		e.stopPropagation();
	}}
	onclick={(e) => {
		e.stopPropagation();
		inputEl.focus();
		if ($autoSelectBranchNameFeature) {
			inputEl.select();
		}
	}}
	onblur={() => {
		if (name === '') name = initialName;
	}}
	onfocus={() => {
		initialName = name;
	}}
	onkeydown={(e) => {
		if (e.key === 'Enter' || e.key === 'Escape') {
			inputEl.blur();
		}
	}}
	autocomplete="off"
	autocorrect="off"
	spellcheck="false"
	style:width={inputWidth}
/>

<style lang="postcss">
	.branch-name-measure-el,
	.branch-name-input {
		min-width: 44px;
		height: 20px;
		padding: 2px 3px;
		border: 1px solid transparent;
	}
	.branch-name-measure-el {
		pointer-events: none;
		visibility: hidden;
		border: 2px solid transparent;
		color: black;
		position: fixed;
		display: inline-block;
		visibility: hidden;
		white-space: pre;
		width: fit-content;
	}
	.branch-name-input {
		text-overflow: ellipsis;
		white-space: nowrap;
		overflow: hidden;

		max-width: 100%;
		width: 100%;
		border-radius: var(--radius-s);
		color: var(--clr-scale-ntrl-0);
		background-color: var(--clr-bg-1);
		outline: none;

		/* not readonly */
		&:not([readonly]):not([disabled]):not(:focus):hover {
			background-color: var(--clr-bg-1-muted);
		}

		&:not([readonly]):not([disabled]):focus {
			outline: none;
			background-color: var(--clr-bg-1-muted);
			border: 1px solid var(--clr-border-2);
		}
	}
	.branch-name-input[readonly] {
		pointer: normal;
		user-select: none;
	}
</style>
