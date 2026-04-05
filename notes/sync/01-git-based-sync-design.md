# Local-First Architecture Research

**Date:** 2026-04-05  
**Topic:** Multi-device sync for Synote  
**Goal:** Design git-based sync that respects local-first principles

## Problem Statement

Synote stores notes as local markdown files. User wants:
1. ✓ Notes always available locally (no network required)
2. ✓ Sync across devices (phone, laptop, tablet)
3. ✓ No cloud lock-in (self-hosted)
4. ✓ Conflict resolution when editing on multiple devices

## Candidate Solutions

### 1. Git-Based Sync (Selected)
**How it works:**
- Notes directory is a git repository
- Synote auto-commits on changes
- User sets up their own git remote (GitHub, Gitea, self-hosted)
- Background sync pulls/pushes

**Pros:**
- ✓ Industry-standard conflict resolution (merge/rebase)
- ✓ Full history (can restore old versions)
- ✓ Works with any git host
- ✓ No new infrastructure needed
- ✓ Users control their data

**Cons:**
- ✗ Merge conflicts need UI (educational burden)
- ✗ Binary files (images) bloat git history
- ✗ Requires some git knowledge for setup

**Implementation approach:**
- Use `git2` crate (libgit2 bindings for Rust)
- Auto-commit with debounce (e.g., 30 seconds after last edit)
- Sync interval: every 5 minutes or on manual trigger
- Conflict UI: show both versions, let user pick/merge

### 2. Syncthing Protocol
**How it works:**
- Peer-to-peer block-level sync
- No central server
- Handles conflicts with versioning

**Pros:**
- ✓ No server needed at all
- ✓ Handles conflicts automatically (keeps both versions)
- ✓ Works on local network

**Cons:**
- ✗ Requires Syncthing running on all devices
- ✗ No structured conflict resolution for text
- ✗ Not integrated into app

**Verdict:** Too decoupled from the app. User would manage Syncthing separately.

### 3. Custom Sync Protocol
**How it works:**
- Build our own CRDT or OT-based sync
- Synote server acts as sync coordinator
- Client-server architecture

**Pros:**
- ✓ Can optimize for notes specifically
- ✓ Seamless UX (invisible to user)

**Cons:**
- ✗ Massive engineering effort
- ✗ Complex conflict resolution
- ✗ Requires always-on server

**Verdict:** Overkill for this project. Git already exists and works.

## Decision

**Use Git-based sync.**

**Rationale:**
1. Synote already uses file-based storage (perfect for git)
2. Markdown is text (git's sweet spot)
3. Rust has excellent git bindings (`git2` crate)
4. Users who self-host likely know git
5. Conflict resolution is solved problem (3-way merge)

## Implementation Plan

### Phase 1: Basic Git Integration
- [ ] Initialize git repo in data directory on first run
- [ ] Auto-commit on note changes (debounced)
- [ ] Config for git remote URL
- [ ] Manual sync button in UI

### Phase 2: Background Sync
- [ ] Periodic pull/push (configurable interval)
- [ ] Sync status indicator in UI
- [ ] Handle network errors gracefully

### Phase 3: Conflict Resolution
- [ ] Detect merge conflicts
- [ ] UI to show "local vs remote" versions
- [ ] Allow picking one or manual merge

### Phase 4: Mobile Considerations
- [ ] iOS: Working Copy app integration
- [ ] Android: Git client or sync via Syncthing bridge
- [ ] Web: Can't do git directly, need server proxy

## Research Links
- [git2 crate docs](https://docs.rs/git2/latest/git2/)
- [libgit2 examples](https://libgit2.org/docs/guides/101-samples/)
- [Automerge CRDT](https://automerge.org/) (alternative, more complex)
- [Git sync in Obsidian](https://forum.obsidian.md/t/mobile-git-sync-solutions/13049)

---
**Next:** Check if git2 crate is already in dependencies, prototype auto-commit