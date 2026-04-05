feat: Git-based multi-device sync foundation

- Add git2 crate (libgit2 bindings) for git operations
- Create sync module with GitSync struct:
  - Auto-initialize git repo at notes directory
  - Auto-commit with 30s debounce on note changes
  - Background task for non-blocking commits
  - Configurable git remote for push/pull
- Integrate sync into NoteStorage:
  - Storage notifies sync on create/update/delete
  - No-op if sync disabled/unavailable
- Add sync config section to Config:
  - enabled: bool (defaults to true if SYNOTE_GIT_REMOTE set)
  - git_remote: Option<String> (env override supported)
- Update config.toml template with auth and sync sections

This enables local-first multi-device sync:
1. Notes always available locally (markdown files)
2. Auto-commits create version history
3. Manual/periodic push to user-controlled git remote
4. Pull on other devices to sync

Next: Push/pull implementation, sync status API, UI sync indicator