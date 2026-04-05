//! Git-based sync for multi-device support
//! 
//! Design: Local-first with git as sync mechanism
//! - Notes directory is a git repository
//! - Auto-commit on changes (30s debounce)
//! - Manual/periodic push/pull to remote

pub mod git;

pub use git::{GitSync, SyncStatus};
