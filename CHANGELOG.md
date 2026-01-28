# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0] - 2026-01-28

### Added
- **Split-view markdown editor** with live preview panel
- **Syntax highlighting** for code blocks using highlight.js
- **Auto-save functionality** with 2-second debounce
- **Search functionality** across note titles and content
- **Keyboard shortcuts**:
  - `Cmd+S` / `Ctrl+S` - Save current note
  - `Cmd+K` / `Ctrl+K` - Focus search box
  - `Escape` - Clear search and unfocus search box
- **Toast notifications** for user feedback
- **Search debouncing** (300ms) for better performance
- **Search clear button** with visual feedback
- **Search results counter** showing number of matches
- REST API endpoint: `GET /api/search?q=query`

### Improved
- Enhanced UI with better spacing and layout
- Better editor experience with split-pane view
- Real-time markdown preview using marked.js
- GitHub Flavored Markdown support
- Improved error handling and user feedback

### Technical
- Added `marked.js` for markdown rendering
- Added `highlight.js` for code syntax highlighting
- Implemented search in storage layer
- Added query parameter extraction in API layer

## [0.1.0] - 2026-01-28

### Initial Release
- Basic note CRUD operations (Create, Read, Update, Delete)
- File-based storage with markdown files
- YAML frontmatter for metadata
- REST API with Axum framework
- Simple web interface
- Note list with timestamps
- Basic markdown editor
- Automatic note timestamps
- UUID-based note identification
- Docker deployment support

### Backend
- Axum web server
- File system-based storage
- CORS support for development
- Structured logging with tracing
- Configuration file support (config.toml)

### Frontend
- Single-page application
- Responsive sidebar layout
- Note creation and deletion
- Live note list updates

---

## Version Guidelines

- **Major version** (X.0.0) - Breaking API changes or major feature overhaul
- **Minor version** (0.X.0) - New features, backwards compatible
- **Patch version** (0.0.X) - Bug fixes and minor improvements
