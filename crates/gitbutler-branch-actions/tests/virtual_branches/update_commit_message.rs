use gitbutler_branch::{BranchCreateRequest, BranchUpdateRequest};
use gitbutler_commit::commit_ext::CommitExt;

use super::*;

#[test]
fn head() {
    let Test {
        repository,
        project,
        controller,
        ..
    } = &Test::default();

    controller
        .set_base_branch(project, &"refs/remotes/origin/master".parse().unwrap())
        .unwrap();

    let branch_id = controller
        .create_virtual_branch(project, &BranchCreateRequest::default())
        .unwrap();

    {
        fs::write(repository.path().join("file one.txt"), "").unwrap();
        controller
            .create_commit(project, branch_id, "commit one", None, false)
            .unwrap()
    };

    {
        fs::write(repository.path().join("file two.txt"), "").unwrap();
        controller
            .create_commit(project, branch_id, "commit two", None, false)
            .unwrap()
    };

    let commit_three_oid = {
        fs::write(repository.path().join("file three.txt"), "").unwrap();
        controller
            .create_commit(project, branch_id, "commit three", None, false)
            .unwrap()
    };
    let commit_three = repository.find_commit(commit_three_oid).unwrap();
    let before_change_id = &commit_three.change_id();

    controller
        .update_commit_message(project, branch_id, commit_three_oid, "commit three updated")
        .unwrap();

    let branch = controller
        .list_virtual_branches(project)
        .unwrap()
        .0
        .into_iter()
        .find(|b| b.id == branch_id)
        .unwrap();

    let descriptions = branch
        .commits
        .iter()
        .map(|c| c.description.clone())
        .collect::<Vec<_>>();

    // get the last commit
    let commit = repository.find_commit(branch.head).unwrap();

    // make sure the SHA changed, but the change ID did not
    assert_ne!(&commit_three.id(), &commit.id());
    assert_eq!(before_change_id, &commit.change_id());

    assert_eq!(
        descriptions,
        vec!["commit three updated", "commit two", "commit one"]
    );
}

#[test]
fn middle() {
    let Test {
        repository,
        project,
        controller,
        ..
    } = &Test::default();

    controller
        .set_base_branch(project, &"refs/remotes/origin/master".parse().unwrap())
        .unwrap();

    let branch_id = controller
        .create_virtual_branch(project, &BranchCreateRequest::default())
        .unwrap();

    {
        fs::write(repository.path().join("file one.txt"), "").unwrap();
        controller
            .create_commit(project, branch_id, "commit one", None, false)
            .unwrap()
    };

    let commit_two_oid = {
        fs::write(repository.path().join("file two.txt"), "").unwrap();
        controller
            .create_commit(project, branch_id, "commit two", None, false)
            .unwrap()
    };

    {
        fs::write(repository.path().join("file three.txt"), "").unwrap();
        controller
            .create_commit(project, branch_id, "commit three", None, false)
            .unwrap()
    };

    controller
        .update_commit_message(project, branch_id, commit_two_oid, "commit two updated")
        .unwrap();

    let branch = controller
        .list_virtual_branches(project)
        .unwrap()
        .0
        .into_iter()
        .find(|b| b.id == branch_id)
        .unwrap();

    let descriptions = branch
        .commits
        .iter()
        .map(|c| c.description.clone())
        .collect::<Vec<_>>();
    assert_eq!(
        descriptions,
        vec!["commit three", "commit two updated", "commit one"]
    );
}

#[test]
fn forcepush_allowed() {
    let Test {
        repository,
        project_id,
        project,
        controller,
        projects,
        ..
    } = &Test::default();

    controller
        .set_base_branch(project, &"refs/remotes/origin/master".parse().unwrap())
        .unwrap();

    projects
        .update(&projects::UpdateRequest {
            id: *project_id,
            ..Default::default()
        })
        .unwrap();

    let branch_id = controller
        .create_virtual_branch(project, &BranchCreateRequest::default())
        .unwrap();

    let commit_one_oid = {
        fs::write(repository.path().join("file one.txt"), "").unwrap();
        controller
            .create_commit(project, branch_id, "commit one", None, false)
            .unwrap()
    };

    controller
        .push_virtual_branch(project, branch_id, false, None)
        .unwrap();

    controller
        .update_commit_message(project, branch_id, commit_one_oid, "commit one updated")
        .unwrap();

    let branch = controller
        .list_virtual_branches(project)
        .unwrap()
        .0
        .into_iter()
        .find(|b| b.id == branch_id)
        .unwrap();

    let descriptions = branch
        .commits
        .iter()
        .map(|c| c.description.clone())
        .collect::<Vec<_>>();
    assert_eq!(descriptions, vec!["commit one updated"]);
    assert!(branch.requires_force);
}

#[test]
fn forcepush_forbidden() {
    let Test {
        repository,
        project,
        controller,
        ..
    } = &Test::default();

    controller
        .set_base_branch(project, &"refs/remotes/origin/master".parse().unwrap())
        .unwrap();

    let branch_id = controller
        .create_virtual_branch(project, &BranchCreateRequest::default())
        .unwrap();

    controller
        .update_virtual_branch(
            project,
            BranchUpdateRequest {
                id: branch_id,
                allow_rebasing: Some(false),
                ..Default::default()
            },
        )
        .unwrap();

    let commit_one_oid = {
        fs::write(repository.path().join("file one.txt"), "").unwrap();
        controller
            .create_commit(project, branch_id, "commit one", None, false)
            .unwrap()
    };

    controller
        .push_virtual_branch(project, branch_id, false, None)
        .unwrap();

    assert_eq!(
        controller
            .update_commit_message(project, branch_id, commit_one_oid, "commit one updated",)
            .unwrap_err()
            .to_string(),
        "force push not allowed"
    );
}

#[test]
fn root() {
    let Test {
        repository,
        project,
        controller,
        ..
    } = &Test::default();

    controller
        .set_base_branch(project, &"refs/remotes/origin/master".parse().unwrap())
        .unwrap();

    let branch_id = controller
        .create_virtual_branch(project, &BranchCreateRequest::default())
        .unwrap();

    let commit_one_oid = {
        fs::write(repository.path().join("file one.txt"), "").unwrap();
        controller
            .create_commit(project, branch_id, "commit one", None, false)
            .unwrap()
    };

    {
        fs::write(repository.path().join("file two.txt"), "").unwrap();
        controller
            .create_commit(project, branch_id, "commit two", None, false)
            .unwrap()
    };

    {
        fs::write(repository.path().join("file three.txt"), "").unwrap();
        controller
            .create_commit(project, branch_id, "commit three", None, false)
            .unwrap()
    };

    controller
        .update_commit_message(project, branch_id, commit_one_oid, "commit one updated")
        .unwrap();

    let branch = controller
        .list_virtual_branches(project)
        .unwrap()
        .0
        .into_iter()
        .find(|b| b.id == branch_id)
        .unwrap();

    let descriptions = branch
        .commits
        .iter()
        .map(|c| c.description.clone())
        .collect::<Vec<_>>();
    assert_eq!(
        descriptions,
        vec!["commit three", "commit two", "commit one updated"]
    );
}

#[test]
fn empty() {
    let Test {
        repository,
        project,
        controller,
        ..
    } = &Test::default();

    controller
        .set_base_branch(project, &"refs/remotes/origin/master".parse().unwrap())
        .unwrap();

    let branch_id = controller
        .create_virtual_branch(project, &BranchCreateRequest::default())
        .unwrap();

    let commit_one_oid = {
        fs::write(repository.path().join("file one.txt"), "").unwrap();
        controller
            .create_commit(project, branch_id, "commit one", None, false)
            .unwrap()
    };

    assert_eq!(
        controller
            .update_commit_message(project, branch_id, commit_one_oid, "",)
            .unwrap_err()
            .to_string(),
        "commit message can not be empty"
    );
}
