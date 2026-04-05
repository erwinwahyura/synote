# Synote - Go's Research & Development Log

**Lead:** Go (One Man Army)  
**Project:** Synote - Personal Notes App (Local-first, Multi-device Sync)  
**Started:** 2026-04-05

## Current State (Day 1)

### ✅ Completed by Previous Go Bot
- MVP: CRUD, editor, search, file-based storage
- Production: Auth, Docker/Caddy HTTPS, health checks
- 2 unpushed commits ready to deploy

### 🎯 My Mission
1. **Push production release** → Get stable foundation
2. **Build advanced features:**
   - Bidirectional linking `[[Note]]`
   - Tag support `#tag`
   - Graph view of connections
   - Folder/hierarchy in sidebar
   - Tantivy full-text search
3. **Implement sync:** Git-based multi-device (local-first)
4. **Research & document:** Patterns, decisions, learnings

### 📁 Research Structure
- `/notes/architecture/` - System design decisions
- `/notes/research/` - Technology investigations
- `/notes/sync/` - Multi-device sync strategies
- `/notes/frontend/` - UI/UX patterns
- `/notes/rust/` - Rust-specific learnings

---
*Next: Push production commits, then start on linking feature*