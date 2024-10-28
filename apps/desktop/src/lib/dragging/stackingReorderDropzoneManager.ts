import { DraggableCommit } from '$lib/dragging/draggables';
import type { BranchController } from '$lib/vbranches/branchController';
import type { VirtualBranch, PatchSeries } from '$lib/vbranches/types';

// Exported for type access only
export class StackingReorderDropzone {
	constructor(
		private branchId: string,
		private branchController: BranchController,
		private currentSeries: PatchSeries,
		private series: PatchSeries[],
		private commitId: string
	) {}

	accepts(data: any) {
		if (!(data instanceof DraggableCommit)) return false;
		if (data.branchId !== this.branchId) return false;
		if (distanceBetweenCommits(this.series, data.commit.id, this.commitId) === 0) return false;

		return true;
	}

	onDrop(data: any) {
		if (!(data instanceof DraggableCommit)) return;
		if (data.branchId !== this.branchId) return;

		const stackOrder = getTargetStackOrder(
			this.series,
			this.currentSeries,
			data.commit.id,
			this.commitId
		);

		console.log('onDrop.stackOrder.series', { series: stackOrder });
		if (stackOrder) {
			this.branchController.reorderStackCommit(data.branchId, { series: stackOrder });
		}
	}
}

export class StackingReorderDropzoneManager {
	public series: Map<string, PatchSeries>;

	constructor(
		private branchController: BranchController,
		private branch: VirtualBranch
	) {
		const seriesMap = new Map();
		this.branch.series.forEach((series) => {
			seriesMap.set(series.name, series);
		});
		this.series = seriesMap;
	}

	topDropzone(seriesName: string) {
		const currentSeries = this.series.get(seriesName);
		if (!currentSeries) {
			throw new Error('Series not found');
		}

		return new StackingReorderDropzone(
			this.branch.id,
			this.branchController,
			currentSeries,
			this.branch.series,
			'top'
		);
	}

	dropzoneBelowCommit(seriesName: string, commitId: string) {
		const currentSeries = this.series.get(seriesName);
		if (!currentSeries) {
			throw new Error('Series not found');
		}

		return new StackingReorderDropzone(
			this.branch.id,
			this.branchController,
			currentSeries,
			this.branch.series,
			commitId
		);
	}
}

export class StackingReorderDropzoneManagerFactory {
	constructor(private branchController: BranchController) {}

	build(branch: VirtualBranch) {
		return new StackingReorderDropzoneManager(this.branchController, branch);
	}
}

function getTargetStackOrder(
	allSeries: PatchSeries[],
	currentSeries: PatchSeries,
	actorCommitId: string,
	targetCommitId: string
) {
	const allSeriesCommits = allSeries.flatMap((s) => ({
		name: s.name,
		commitIds: s.patches.flatMap((p) => p.id)
	}));
	const flatCommits = allSeriesCommits.flatMap((s) => s.commitIds);

	if (
		targetCommitId !== 'top' &&
		(!flatCommits.includes(actorCommitId) || !flatCommits.includes(targetCommitId))
	) {
		throw new Error('Commit not found in series');
	}

	const stackOrderCurrentSeries = allSeriesCommits.find((s) => s.name === currentSeries.name);

	// Move actorCommitId after targetCommitId in stackOrderCurrentSeries.commitIds
	if (stackOrderCurrentSeries) {
		// Remove from old position
		allSeriesCommits.forEach((s) => {
			if (s.commitIds.includes(actorCommitId)) {
				s.commitIds.splice(s.commitIds.indexOf(actorCommitId), 1);
			}
		});

		if (targetCommitId === 'top') {
			// Insert targetCommitId on top
			stackOrderCurrentSeries?.commitIds.unshift(actorCommitId);
		} else {
			// Insert at new position
			stackOrderCurrentSeries?.commitIds.splice(
				stackOrderCurrentSeries?.commitIds.indexOf(targetCommitId) + 1,
				0,
				actorCommitId
			);
		}

		// Replace current series in `allSeries` list with our new series
		allSeriesCommits.splice(
			allSeriesCommits.findIndex((s) => s.name === currentSeries.name),
			1,
			stackOrderCurrentSeries
		);

		return allSeriesCommits;
	}
}

function distanceBetweenCommits(
	allSeries: PatchSeries[],
	actorCommitId: string,
	targetCommitId: string
) {
	const allSeriesCommitsFlat = allSeries.flatMap((s) => s.patches.flatMap((p) => p.id));
	// console.log('allSeries', { allSeriesCommitsFlat, actorCommitId, targetCommitId });
	// if (
	// 	!allSeriesCommitsFlat.includes(actorCommitId) ||
	// 	!allSeriesCommitsFlat.includes(targetCommitId)
	// ) {
	// 	throw new Error('Commits not found in series');
	// }

	const actorIndex = allSeriesCommitsFlat.indexOf(actorCommitId);
	const targetIndex = allSeriesCommitsFlat.indexOf(targetCommitId);
	return actorIndex - targetIndex;
}
