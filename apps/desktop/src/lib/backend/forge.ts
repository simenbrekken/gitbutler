import { invoke } from './ipc';

export type ForgeName = 'github' | 'gitlab' | 'bitbucket' | 'azure';

export class ForgeService {
	constructor(private projectId: string) {}

	async getAvailableReviewTemplates(forgeName: string): Promise<string[]> {
		return await invoke<string[]>('get_available_review_templates', {
			projectId: this.projectId,
			forge: { name: forgeName }
		});
	}

	async getReviewTemplateContent(forgeName: string, templatePath: string): Promise<string> {
		return await invoke('get_review_template_contents', {
			relativePath: templatePath,
			projectId: this.projectId,
			forge: { name: forgeName }
		});
	}
}
