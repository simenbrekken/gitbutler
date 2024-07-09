use anyhow::{Context, Result};
use gitbutler_branch::branch::BranchId;
use gitbutler_command_context::ProjectRepository;
use gitbutler_project as projects;
use gitbutler_project::ProjectId;
use gitbutler_reference::RemoteRefname;
use gitbutler_repo::{credentials::Helper, RepoActions, RepositoryExt};
use gitbutler_virtual::conflicts;

#[derive(Clone)]
pub struct App {
    projects: projects::Controller,
}

impl App {
    pub fn new(projects: projects::Controller) -> Self {
        Self { projects }
    }

    pub fn mark_resolved(&self, project_id: ProjectId, path: &str) -> Result<()> {
        let project = self.projects.get(project_id)?;
        let project_repository = ProjectRepository::open(&project)?;
        // mark file as resolved
        conflicts::resolve(&project_repository, path)?;
        Ok(())
    }

    pub fn git_remote_branches(&self, project_id: ProjectId) -> Result<Vec<RemoteRefname>> {
        let project = self.projects.get(project_id)?;
        let project_repository = ProjectRepository::open(&project)?;
        project_repository.repo().remote_branches()
    }

    pub fn git_test_push(
        &self,
        project_id: ProjectId,
        remote_name: &str,
        branch_name: &str,
        credentials: &Helper,
        askpass: Option<Option<BranchId>>,
    ) -> Result<()> {
        let project = self.projects.get(project_id)?;
        let project_repository = ProjectRepository::open(&project)?;
        project_repository.git_test_push(credentials, remote_name, branch_name, askpass)
    }

    pub fn git_test_fetch(
        &self,
        project_id: ProjectId,
        remote_name: &str,
        credentials: &Helper,
        askpass: Option<String>,
    ) -> Result<()> {
        let project = self.projects.get(project_id)?;
        let project_repository = ProjectRepository::open(&project)?;
        project_repository.fetch(remote_name, credentials, askpass)
    }

    pub fn git_index_size(&self, project_id: ProjectId) -> Result<usize> {
        let project = self.projects.get(project_id)?;
        let project_repository = ProjectRepository::open(&project)?;
        let size = project_repository
            .repo()
            .index()
            .context("failed to get index size")?
            .len();
        Ok(size)
    }

    pub fn git_head(&self, project_id: ProjectId) -> Result<String> {
        let project = self.projects.get(project_id)?;
        let project_repository = ProjectRepository::open(&project)?;
        let head = project_repository
            .repo()
            .head()
            .context("failed to get repository head")?;
        Ok(head.name().unwrap().to_string())
    }

    pub fn git_set_global_config(key: &str, value: &str) -> Result<String> {
        let mut config = git2::Config::open_default()?;
        config.set_str(key, value)?;
        Ok(value.to_string())
    }

    pub fn git_remove_global_config(key: &str) -> Result<()> {
        let mut config = git2::Config::open_default()?;
        Ok(config.remove(key)?)
    }

    pub fn git_get_global_config(key: &str) -> Result<Option<String>> {
        let config = git2::Config::open_default()?;
        let value = config.get_string(key);
        match value {
            Ok(value) => Ok(Some(value)),
            Err(e) => {
                if e.code() == git2::ErrorCode::NotFound {
                    Ok(None)
                } else {
                    Err(e.into())
                }
            }
        }
    }

    pub async fn delete_all_data(&self) -> Result<()> {
        for project in self.projects.list().context("failed to list projects")? {
            self.projects
                .delete(project.id)
                .await
                .map_err(|err| err.context("failed to delete project"))?;
        }
        Ok(())
    }
}
