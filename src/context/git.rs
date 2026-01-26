//! Git Status Integration
//!
//! Provides git status tracking and caching for sidebar display.

use git2::{Repository, StatusOptions, Status, StatusShow};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

/// Git status for a single file
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileGitStatus {
    Clean,
    Modified,
    Staged,
    StagedModified,
    Untracked,
    Deleted,
    Renamed,
    Conflicted,
    Ignored,
}

impl FileGitStatus {
    /// Get display character for this status
    pub fn indicator(&self) -> &'static str {
        match self {
            FileGitStatus::Clean => " ",
            FileGitStatus::Modified => "M",
            FileGitStatus::Staged => "A",
            FileGitStatus::StagedModified => "M",
            FileGitStatus::Untracked => "U",
            FileGitStatus::Deleted => "D",
            FileGitStatus::Renamed => "R",
            FileGitStatus::Conflicted => "!",
            FileGitStatus::Ignored => " ",
        }
    }

    /// Get color key for theme integration
    pub fn color_key(&self) -> &'static str {
        match self {
            FileGitStatus::Clean => "text_dim",
            FileGitStatus::Modified => "yellow",
            FileGitStatus::Staged => "green",
            FileGitStatus::StagedModified => "yellow",
            FileGitStatus::Untracked => "secondary",
            FileGitStatus::Deleted => "red",
            FileGitStatus::Renamed => "cyan",
            FileGitStatus::Conflicted => "red",
            FileGitStatus::Ignored => "text_dim",
        }
    }

    /// Convert from git2 Status flags
    fn from_git2_status(status: Status) -> Self {
        let staged_new = status.contains(Status::INDEX_NEW);
        let staged_modified = status.contains(Status::INDEX_MODIFIED);
        let staged_deleted = status.contains(Status::INDEX_DELETED);
        let staged_renamed = status.contains(Status::INDEX_RENAMED);

        let wt_new = status.contains(Status::WT_NEW);
        let wt_modified = status.contains(Status::WT_MODIFIED);
        let wt_deleted = status.contains(Status::WT_DELETED);

        let conflicted = status.contains(Status::CONFLICTED);
        let ignored = status.contains(Status::IGNORED);

        if conflicted {
            FileGitStatus::Conflicted
        } else if ignored {
            FileGitStatus::Ignored
        } else if staged_renamed {
            FileGitStatus::Renamed
        } else if (staged_new || staged_modified) && wt_modified {
            FileGitStatus::StagedModified
        } else if staged_new || staged_modified {
            FileGitStatus::Staged
        } else if staged_deleted || wt_deleted {
            FileGitStatus::Deleted
        } else if wt_new {
            FileGitStatus::Untracked
        } else if wt_modified {
            FileGitStatus::Modified
        } else {
            FileGitStatus::Clean
        }
    }
}

/// Repository status summary
#[derive(Debug, Clone, Default)]
pub struct RepoStatus {
    pub root: PathBuf,
    pub branch: String,
    pub modified_count: usize,
    pub staged_count: usize,
    pub untracked_count: usize,
    pub is_dirty: bool,
    pub ahead: usize,
    pub behind: usize,
}

/// Cache for git status
pub struct GitStatusCache {
    repo: Option<Repository>,
    repo_root: Option<PathBuf>,
    file_statuses: HashMap<PathBuf, FileGitStatus>,
    repo_status: Option<RepoStatus>,
    last_refresh: Instant,
    refresh_interval: Duration,
    dirty: bool,
}

impl GitStatusCache {
    pub fn new(refresh_interval: Duration) -> Self {
        Self {
            repo: None,
            repo_root: None,
            file_statuses: HashMap::new(),
            repo_status: None,
            last_refresh: Instant::now() - refresh_interval,
            refresh_interval,
            dirty: true,
        }
    }

    pub fn set_root(&mut self, path: &Path) {
        match Repository::discover(path) {
            Ok(repo) => {
                let root = repo.workdir()
                    .map(|p| p.to_path_buf())
                    .unwrap_or_else(|| path.to_path_buf());

                if self.repo_root.as_ref() != Some(&root) {
                    log::info!("Git repository found at: {:?}", root);
                    self.repo_root = Some(root);
                    self.repo = Some(repo);
                    self.dirty = true;
                }
            }
            Err(e) => {
                if self.repo.is_some() {
                    log::debug!("No git repository at {:?}: {}", path, e);
                }
                self.repo = None;
                self.repo_root = None;
                self.file_statuses.clear();
                self.repo_status = None;
            }
        }
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn needs_refresh(&self) -> bool {
        self.dirty || self.last_refresh.elapsed() >= self.refresh_interval
    }

    pub fn refresh_if_needed(&mut self) -> bool {
        if !self.needs_refresh() {
            return false;
        }
        self.refresh();
        true
    }

    pub fn refresh(&mut self) {
        let Some(repo) = &self.repo else { return };

        self.file_statuses.clear();
        self.dirty = false;
        self.last_refresh = Instant::now();

        let mut opts = StatusOptions::new();
        opts.show(StatusShow::IndexAndWorkdir)
            .include_untracked(true)
            .recurse_untracked_dirs(true)
            .include_ignored(false)
            .exclude_submodules(true);

        match repo.statuses(Some(&mut opts)) {
            Ok(statuses) => {
                let mut modified_count = 0;
                let mut staged_count = 0;
                let mut untracked_count = 0;

                for entry in statuses.iter() {
                    if let Some(path) = entry.path() {
                        let status = FileGitStatus::from_git2_status(entry.status());
                        let path_buf = PathBuf::from(path);

                        match status {
                            FileGitStatus::Modified | FileGitStatus::StagedModified => {
                                modified_count += 1;
                            }
                            FileGitStatus::Staged => {
                                staged_count += 1;
                            }
                            FileGitStatus::Untracked => {
                                untracked_count += 1;
                            }
                            _ => {}
                        }

                        self.file_statuses.insert(path_buf, status);
                    }
                }

                let branch = Self::get_branch_name(repo);
                let (ahead, behind) = Self::get_ahead_behind(repo);
                let is_dirty = modified_count > 0 || staged_count > 0;

                self.repo_status = Some(RepoStatus {
                    root: self.repo_root.clone().unwrap_or_default(),
                    branch,
                    modified_count,
                    staged_count,
                    untracked_count,
                    is_dirty,
                    ahead,
                    behind,
                });
            }
            Err(e) => {
                log::warn!("Failed to get git status: {}", e);
            }
        }
    }

    fn get_branch_name(repo: &Repository) -> String {
        match repo.head() {
            Ok(head) => {
                if head.is_branch() {
                    head.shorthand()
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| "HEAD".to_string())
                } else {
                    head.target()
                        .map(|oid| format!("{:.7}", oid))
                        .unwrap_or_else(|| "HEAD".to_string())
                }
            }
            Err(_) => "HEAD".to_string(),
        }
    }

    fn get_ahead_behind(repo: &Repository) -> (usize, usize) {
        let head = match repo.head() {
            Ok(h) => h,
            Err(_) => return (0, 0),
        };

        let local_oid = match head.target() {
            Some(oid) => oid,
            None => return (0, 0),
        };

        let branch_name = match head.shorthand() {
            Some(name) => name,
            None => return (0, 0),
        };

        let upstream_name = format!("refs/remotes/origin/{}", branch_name);
        let upstream_ref = match repo.find_reference(&upstream_name) {
            Ok(r) => r,
            Err(_) => return (0, 0),
        };

        let upstream_oid = match upstream_ref.target() {
            Some(oid) => oid,
            None => return (0, 0),
        };

        match repo.graph_ahead_behind(local_oid, upstream_oid) {
            Ok((ahead, behind)) => (ahead, behind),
            Err(_) => (0, 0),
        }
    }

    pub fn get_file_status(&self, relative_path: &Path) -> FileGitStatus {
        self.file_statuses
            .get(relative_path)
            .copied()
            .unwrap_or(FileGitStatus::Clean)
    }

    pub fn get_status_for_absolute(&self, path: &Path) -> FileGitStatus {
        if let Some(root) = &self.repo_root {
            if let Ok(relative) = path.strip_prefix(root) {
                return self.get_file_status(relative);
            }
        }
        FileGitStatus::Clean
    }

    pub fn repo_status(&self) -> Option<&RepoStatus> {
        self.repo_status.as_ref()
    }

    pub fn is_in_repo(&self) -> bool {
        self.repo.is_some()
    }

    pub fn repo_root(&self) -> Option<&Path> {
        self.repo_root.as_deref()
    }
}

impl Default for GitStatusCache {
    fn default() -> Self {
        Self::new(Duration::from_secs(5))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_git_status_indicators() {
        assert_eq!(FileGitStatus::Modified.indicator(), "M");
        assert_eq!(FileGitStatus::Staged.indicator(), "A");
        assert_eq!(FileGitStatus::Untracked.indicator(), "U");
        assert_eq!(FileGitStatus::Deleted.indicator(), "D");
        assert_eq!(FileGitStatus::Renamed.indicator(), "R");
        assert_eq!(FileGitStatus::Conflicted.indicator(), "!");
    }

    #[test]
    fn test_cache_creation() {
        let cache = GitStatusCache::new(Duration::from_secs(5));
        assert!(!cache.is_in_repo());
        assert!(cache.repo_status().is_none());
    }

    #[test]
    fn test_status_color_keys() {
        assert_eq!(FileGitStatus::Modified.color_key(), "yellow");
        assert_eq!(FileGitStatus::Staged.color_key(), "green");
        assert_eq!(FileGitStatus::Deleted.color_key(), "red");
    }
}
