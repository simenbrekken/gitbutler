use gitbutler_branch::BranchCreateRequest;

use super::*;

#[test]
fn detect_upstream_commits() {
    let Test {
        repository,
        project,
        controller,
        ..
    } = &Test::default();

    controller
        .set_base_branch(project, &"refs/remotes/origin/master".parse().unwrap())
        .unwrap();

    let branch1_id = controller
        .create_virtual_branch(project, &BranchCreateRequest::default())
        .unwrap();

    let oid1 = {
        // create first commit
        fs::write(repository.path().join("file.txt"), "content").unwrap();
        controller
            .create_commit(project, branch1_id, "commit", None, false)
            .unwrap()
    };

    let oid2 = {
        // create second commit
        fs::write(repository.path().join("file.txt"), "content2").unwrap();
        controller
            .create_commit(project, branch1_id, "commit", None, false)
            .unwrap()
    };

    // push
    controller
        .push_virtual_branch(project, branch1_id, false, None)
        .unwrap();

    let oid3 = {
        // create third commit
        fs::write(repository.path().join("file.txt"), "content3").unwrap();
        controller
            .create_commit(project, branch1_id, "commit", None, false)
            .unwrap()
    };

    {
        // should correctly detect pushed commits
        let (branches, _) = controller.list_virtual_branches(project).unwrap();
        assert_eq!(branches.len(), 1);
        assert_eq!(branches[0].id, branch1_id);
        assert_eq!(branches[0].commits.len(), 3);
        assert_eq!(branches[0].commits[0].id, oid3);
        assert!(!branches[0].commits[0].is_remote);
        assert_eq!(branches[0].commits[1].id, oid2);
        assert!(branches[0].commits[1].is_remote);
        assert_eq!(branches[0].commits[2].id, oid1);
        assert!(branches[0].commits[2].is_remote);
    }
}

#[test]
fn detect_integrated_commits() {
    let Test {
        repository,
        project,
        controller,
        ..
    } = &Test::default();

    controller
        .set_base_branch(project, &"refs/remotes/origin/master".parse().unwrap())
        .unwrap();

    let branch1_id = controller
        .create_virtual_branch(project, &BranchCreateRequest::default())
        .unwrap();

    let oid1 = {
        // create first commit
        fs::write(repository.path().join("file.txt"), "content").unwrap();
        controller
            .create_commit(project, branch1_id, "commit", None, false)
            .unwrap()
    };

    let oid2 = {
        // create second commit
        fs::write(repository.path().join("file.txt"), "content2").unwrap();
        controller
            .create_commit(project, branch1_id, "commit", None, false)
            .unwrap()
    };

    // push
    controller
        .push_virtual_branch(project, branch1_id, false, None)
        .unwrap();

    {
        // merge branch upstream
        let branch = controller
            .list_virtual_branches(project)
            .unwrap()
            .0
            .into_iter()
            .find(|b| b.id == branch1_id)
            .unwrap();
        repository.merge(&branch.upstream.as_ref().unwrap().name);
        repository.fetch();
    }

    let oid3 = {
        // create third commit
        fs::write(repository.path().join("file.txt"), "content3").unwrap();
        controller
            .create_commit(project, branch1_id, "commit", None, false)
            .unwrap()
    };

    {
        // should correctly detect pushed commits
        let (branches, _) = controller.list_virtual_branches(project).unwrap();
        assert_eq!(branches.len(), 1);
        assert_eq!(branches[0].id, branch1_id);
        assert_eq!(branches[0].commits.len(), 3);
        assert_eq!(branches[0].commits[0].id, oid3);
        assert!(!branches[0].commits[0].is_integrated);
        assert_eq!(branches[0].commits[1].id, oid2);
        assert!(branches[0].commits[1].is_integrated);
        assert_eq!(branches[0].commits[2].id, oid1);
        assert!(branches[0].commits[2].is_integrated);
    }
}
