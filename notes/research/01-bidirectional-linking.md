# Bidirectional Linking Research

**Date:** 2026-04-05  
**Topic:** Implementing `[[Note]]` wikilinks with backlinks  
**Goal:** Design linking system for Synote (like Obsidian)

## Wikilink Syntax

### Basic Format
```markdown
[[Note Title]]           - Link to note by title
[[Note Title|Alias]]     - Link with display alias
[[Note Title#Heading]]   - Link to specific heading
```

### Obsidian Features (Reference)
1. **Wikilinks**: `[[Page Name]]`
2. **Markdown links**: `[text](Page%20Name.md)` (fallback)
3. **Aliases**: `[[Page Name|display text]]`
4. **Headings**: `[[Page Name#Heading]]`
5. **Blocks**: `[[Page Name#^block-id]]`

## Implementation Strategy

### Architecture Options

#### Option 1: Parse-on-Read (Selected)
- Parse wikilinks when loading note content
- Build backlinks index in real-time
- Store links in memory/cache

**Pros:**
- ✓ Always accurate (no stale data)
- ✓ Simple implementation
- ✓ Works with any storage backend

**Cons:**
- ✗ Slower for large vaults (parse every note)
- ✗ No pre-computed graph

#### Option 2: Build Index on Startup
- Scan all notes at startup
- Build SQLite/JSON index of links
- Update incrementally on changes

**Pros:**
- ✓ Fast queries
- ✓ Enables graph visualization
- ✓ Backlinks instant

**Cons:**
- ✗ Startup time for large vaults
- ✗ Index maintenance complexity

## Decision: Hybrid Approach

**Phase 1 (Now):** Parse-on-read with in-memory cache
- When loading a note, extract its outgoing links
- Build backlinks map dynamically
- Cache per-session

**Phase 2 (Future):** Persistent index
- SQLite table: `links(id, source_id, target_id, target_title)`
- Background reindex on changes
- Graph view queries from DB

## Parsing Regex

```rust
// Match [[...]] with optional |alias
// Captures: 1=target (may contain #heading), 2=optional alias
let wikilink_re = Regex::new(r"\[\[([^\]|]+)(?:\|([^\]]+))?\]\]").unwrap();
```

## Data Model

```rust
struct Link {
    source_id: Uuid,      // Note containing the link
    target_title: String, // Raw [[Target Title]] text
    target_id: Option<Uuid>, // Resolved ID (None if note doesn't exist yet)
    alias: Option<String>, // Display text after |
    heading: Option<String>, // From #Heading (if present)
}

struct NoteLinks {
    outgoing: Vec<Link>, // Links from this note
    incoming: Vec<Link>, // Backlinks to this note
}
```

## Backlinks Resolution

**Problem:** When viewing Note A, need to find all notes linking TO Note A.

**Solution:**
1. Store all notes' outgoing links in a global map
2. For backlinks, filter: `outgoing_links.filter(|l| l.target_id == current_note.id)`
3. Or: Build reverse index `backlinks: HashMap<Uuid, Vec<Link>>`

## Edge Cases

1. **Broken links**: Target note doesn't exist (show as "create?")
2. **Ambiguous titles**: Multiple notes with same title (pick first or show disambiguation)
3. **Circular links**: A → B → A (no issue, just graph cycles)
4. **Special chars in titles**: Handle `[]|` characters properly

## Frontend Integration

1. Render `[[Note]]` as `<a href="#/note/uuid">Note</a>`
2. Clicking broken link → "Create note 'Title'?" prompt
3. Backlinks panel in sidebar
4. Hover preview (like Obsidian)

## Implementation Plan

### Backend
- [ ] Add `links` module with parser
- [ ] Extract wikilinks from note content
- [ ] Build backlinks map
- [ ] API: `GET /api/notes/:id/links` → {outgoing, incoming}
- [ ] Auto-create notes for broken links (optional)

### Frontend  
- [ ] Render wikilinks as clickable links
- [ ] Backlinks panel component
- [ ] Link hover preview

## References
- Obsidian Help: https://help.obsidian.md/Linking+notes
- Wikilink spec: Community convention, not formal standard
- Regex testing: https://regex101.com

---
**Next:** Implement parser, add links API, build backlinks panel