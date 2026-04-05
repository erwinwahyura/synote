feat: Bidirectional linking (wikilinks) foundation

- Add regex, lazy_static, urlencoding dependencies for parsing
- Create links module (backend/src/links/):
  - Wikilink parser: `[[Target|Alias]]` with heading support
  - LinksIndex: in-memory index for outgoing/incoming links
  - Link resolution: resolve titles to note UUIDs
  - Backlinks computation: find all notes linking to a target
- Add links API endpoint: `GET /api/notes/:id/links`
  - Returns {outgoing, incoming} links
  - On-demand index update when called
- Integrate links into main application
- Add research notes on bidirectional linking patterns

**Wikilink features:**
- `[[Note Title]]` - basic link
- `[[Note Title|Display Text]]` - aliased link
- `[[Note Title#Heading]]` - heading reference
- Broken links tracked (target_id: null, exists: false)

**Next:**
- Frontend: render wikilinks as clickable links
- Frontend: backlinks panel in note view
- Auto-create notes for broken links (optional)
- Graph visualization of note connections