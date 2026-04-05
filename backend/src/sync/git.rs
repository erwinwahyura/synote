use std::path::Path;
use std::time::Duration;
use git2::{Repository, Signature, IndexAddOption};
use anyhow::{Result, Context};
use tracing::{info, warn, error, debug};
use tokio::time::interval;
use tokio::sync::mpsc;

pub struct GitSync {
    repo: Repository,
    notes_path: String,
    remote_url: Option<String>,
    debounce_tx: mpsc::Sender<()>,
}

impl GitSync {
    /// Initialize or open a git repository at the notes directory
    pub fn init(notes_path: &str, remote_url: Option<String>) -> Result<Self> {
        let path = Path::new(notes_path);
        
        // Check if already a git repo
        let repo = if path.join(".git").exists() {
            info!("Opening existing git repository at {}", notes_path);
            Repository::open(path)?
        } else {
            info!("Initializing new git repository at {}", notes_path);
            Repository::init(path)?
        };
        
        // Configure git user if not set (required for commits)
        let mut config = repo.config()?;
        if config.get_string("user.name").is_err() {
            config.set_str("user.name", "Synote")?;
        }
        if config.get_string("user.email").is_err() {
            config.set_str("user.email", "synote@local")?;
        }
        
        let (debounce_tx, debounce_rx) = mpsc::channel(1);
        
        let sync = Self {
            repo,
            notes_path: notes_path.to_string(),
            remote_url,
            debounce_tx,
        };
        
        // Start background auto-commit task
        sync.spawn_auto_commit_task(debounce_rx);
        
        Ok(sync)
    }
    
    /// Spawn background task that auto-commits with debounce
    fn spawn_auto_commit_task(&self, rx: mpsc::Receiver<()>) {
        let repo_path = self.notes_path.clone();
        
        tokio::spawn(async move {
            let mut pending = false;
            let mut debounce_timer = interval(Duration::from_secs(30)); // 30s debounce
            
            loop {
                tokio::select! {
                    _ = rx.recv() => {
                        pending = true;
                        debug!("Change detected, waiting for debounce...");
                    }
                    _ = debounce_timer.tick() => {
                        if pending {
                            if let Err(e) = Self::perform_commit(&repo_path).await {
                                error!("Auto-commit failed: {}", e);
                            }
                            pending = false;
                        }
                    }
                }
            }
        });
    }
    
    /// Perform a git commit of all changes
    async fn perform_commit(repo_path: &str) -> Result<()> {
        // Use tokio::task::spawn_blocking for git operations (CPU/blocking)
        let path = repo_path.to_string();
        tokio::task::spawn_blocking(move || {
            let repo = Repository::open(&path)?;
            
            let mut index = repo.index()?;
            index.add_all(["*"], IndexAddOption::DEFAULT, None)?;
            index.write()?;
            
            let oid = index.write_tree()?;
            let tree = repo.find_tree(oid)?;
            let sig = repo.signature()?;
            
            // Check if there are changes to commit
            let head = repo.head().ok();
            let parent = head.as_ref().and_then(|h| h.target()).and_then(|oid| repo.find_commit(oid).ok());
            
            let parents = parent.as_ref().map(|p| vec![p]).unwrap_or_default();
            let parent_refs: Vec<&git2::Commit> = parents.iter().collect();
            
            // Check if there's anything to commit
            let diff = if let Some(ref parent) = parent {
                repo.diff_tree_to_tree(Some(&parent.tree()?), Some(&tree), None)?
            } else {
                repo.diff_tree_to_tree(None, Some(&tree), None)?
            };
            
            if diff.deltas().count() == 0 {
                debug!("No changes to commit");
                return Ok(());
            }
            
            let msg = format!("Auto-commit: {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"));
            
            repo.commit(
                Some("HEAD"),
                &sig,
                &sig,
                &msg,
                &tree,
                &parent_refs
            )?;
            
            info!("Auto-committed: {}", msg);
            Ok(())
        })
        .await
        .context("Commit task panicked")?
        .context("Git commit failed")
    }
    
    /// Trigger a commit (called when notes change)
    pub fn notify_change(&self) {
        let _ = self.debounce_tx.try_send(());
    }
    
    /// Push to remote (if configured)
    pub async fn push(&self) -> Result<()> {
        if let Some(ref remote_url) = self.remote_url {
            // Implementation for push will go here
            info!("Push to {} not yet implemented", remote_url);
        }
        Ok(())
    }
    
    /// Pull from remote (if configured)  
    pub async fn pull(&self) -> Result<()> {
        if self.remote_url.is_some() {
            info!("Pull not yet implemented");
        }
        Ok(())
    }
    
    /// Get sync status for UI
    pub fn status(&self) -> SyncStatus {
        // Simplified status for now
        SyncStatus {
            commits_ahead: 0,
            commits_behind: 0,
            has_remote: self.remote_url.is_some(),
            last_sync: None,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SyncStatus {
    pub commits_ahead: u32,
    pub commits_behind: u32,
    pub has_remote: bool,
    pub last_sync: Option<String>,
}
