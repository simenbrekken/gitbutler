// use anyhow::Result;
use bstr::BStr;

/// Extension trait for `git2::Commit`.
///
/// For now, it collects useful methods from `gitbutler-core::git::Commit`
pub trait CommitExt {
    /// Obtain the commit-message as bytes, but without assuming any encoding.
    fn message_bstr(&self) -> &BStr;
    fn change_id(&self) -> Option<String>;
    fn is_signed(&self) -> bool;
    fn is_conflicted(&self) -> bool;
    fn conflicted_files(&self) -> Option<u32>;
    fn is_merge(&self) -> bool;
}

impl<'repo> CommitExt for git2::Commit<'repo> {
    fn message_bstr(&self) -> &BStr {
        self.message_bytes().as_ref()
    }
    fn change_id(&self) -> Option<String> {
        let cid = self.header_field_bytes("change-id").ok()?;
        if cid.is_empty() {
            None
        } else {
            String::from_utf8(cid.to_owned()).ok()
        }
    }
    fn is_signed(&self) -> bool {
        self.header_field_bytes("gpgsig").is_ok()
    }

    fn is_conflicted(&self) -> bool {
        let cid = self.header_field_bytes("conflicted").ok();
        cid.is_some()
    }

    fn conflicted_files(&self) -> Option<u32> {
        let cid = self.header_field_bytes("conflicted").ok();
        if let Some(cid) = cid {
            let cid = std::str::from_utf8(&cid).ok()?;
            let cid = cid.parse::<u32>().ok()?;
            Some(cid)
        } else {
            None
        }
    }

    fn is_merge(&self) -> bool {
        self.parent_count() > 1
    }
}