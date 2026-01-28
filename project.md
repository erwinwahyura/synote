# Synote - Personal Note-Taking Application

## Overview
Synote is a self-hosted, personal note-taking application inspired by Obsidian and Notion. Built with Rust to provide high performance, reliability, and as a learning opportunity for Rust development.

## Project Goals
1. Create a powerful, self-hosted alternative to Obsidian/Notion for personal use
2. Learn Rust through practical application development
3. Build a deployable solution that can run on personal servers/VPS
4. Focus on markdown-based notes with bidirectional linking
5. Provide a clean, fast web interface

## Core Features

### Phase 1: MVP (Minimum Viable Product)
- **Note Management**
  - Create, read, update, delete notes
  - Markdown support with live preview
  - File-based storage (markdown files on disk)
  - Folder/hierarchy organization

- **Basic Search**
  - Full-text search across all notes
  - Search by title and content

- **Web Interface**
  - Clean, responsive UI
  - Note editor with markdown preview
  - File tree/sidebar navigation

- **Self-Hosting**
  - Single binary deployment
  - Minimal dependencies
  - Simple configuration

### Phase 2: Enhanced Features
- **Bidirectional Linking**
  - Wiki-style `[[note]]` links
  - Backlinks panel showing incoming links
  - Graph view of note connections

- **Tags & Metadata**
  - Tag support (`#tag` syntax)
  - Front matter metadata (YAML)
  - Filter and search by tags

- **Rich Editor**
  - Syntax highlighting for code blocks
  - Image embedding support
  - Tables, checkboxes, etc.

### Phase 3: Advanced Features
- **Multi-device Sync** (optional)
  - Git-based synchronization
  - Conflict resolution

- **Advanced Search**
  - Fuzzy search
  - Advanced filters (by date, tags, etc.)

- **Extensibility**
  - Plugin system
  - Custom themes
  - Export options (PDF, HTML)

## Technology Stack

### Backend
- **Language**: Rust
- **Web Framework**: Axum (modern, ergonomic web framework)
- **Database**:
  - File system for note storage (markdown files)
  - SQLite for metadata/search index (optional, for performance)
- **Authentication**: Simple token-based auth (single user)
- **Search**: Tantivy (full-text search library in Rust)

### Frontend
- **Framework**: HTMX + Alpine.js (lightweight, server-rendered)
  - Alternative: SvelteKit or React (if you want to learn frontend too)
- **Styling**: TailwindCSS
- **Markdown**: markdown-it or marked.js
- **Editor**: CodeMirror 6 or Monaco Editor

### Deployment
- **Containerization**: Docker
- **Reverse Proxy**: Nginx/Caddy (for HTTPS)
- **Platform**: Any Linux VPS, Raspberry Pi, or home server

## Project Structure
```
synote/
├── backend/
│   ├── src/
│   │   ├── main.rs
│   │   ├── api/           # REST API endpoints
│   │   ├── models/        # Data models
│   │   ├── services/      # Business logic
│   │   ├── storage/       # File system operations
│   │   ├── search/        # Search functionality
│   │   └── config.rs      # Configuration
│   ├── Cargo.toml
│   └── tests/
├── frontend/
│   ├── src/
│   ├── public/
│   └── package.json
├── docker/
│   ├── Dockerfile
│   └── docker-compose.yml
├── docs/
│   └── api.md
├── data/                  # Default notes directory
└── README.md
```

## Development Roadmap

### Milestone 1: Basic Note CRUD (Week 1-2) ✅ COMPLETED
- [x] Set up Rust project with Axum
- [x] Implement file system note storage
- [x] Create REST API for CRUD operations
- [x] Basic error handling and validation

### Milestone 2: Web Interface (Week 3-4) ✅ COMPLETED
- [x] Set up frontend project
- [x] Create note list/tree view
- [x] Implement markdown editor
- [x] Connect frontend to backend API

### Milestone 2.5: Enhanced Editor ✅ COMPLETED
- [x] Add markdown preview panel
- [x] Implement split-view editor
- [x] Add syntax highlighting for code blocks
- [x] Improve UI/UX with better styling
- [x] Auto-save functionality (2 second debounce)
- [x] Keyboard shortcuts (Cmd+S/Ctrl+S to save)
- [x] Toast notifications for user feedback

### Milestone 3: Search & Organization (Week 5-6) - IN PROGRESS
- [x] Create search UI with search bar
- [x] Implement basic search (title and content)
- [x] Add keyboard shortcuts (Ctrl+K to search, Escape to clear)
- [x] Add search debouncing and clear button
- [ ] Upgrade to full-text search with Tantivy (advanced)
- [ ] Add folder/hierarchy support
- [ ] Add note filtering by tags/date

### Milestone 4: Advanced Features (Week 7-8)
- [ ] Implement bidirectional linking
- [ ] Add tag support
- [ ] Create backlinks panel
- [ ] Build basic graph view

### Milestone 5: Deployment (Week 9-10)
- [ ] Create Docker configuration
- [ ] Add authentication
- [ ] Write deployment documentation
- [ ] Create backup/restore functionality

## API Design (REST)

### Notes
- `GET /api/notes` - List all notes
- `GET /api/notes/:id` - Get specific note
- `POST /api/notes` - Create new note
- `PUT /api/notes/:id` - Update note
- `DELETE /api/notes/:id` - Delete note

### Search
- `GET /api/search?q=query` - Search notes
- `GET /api/tags` - List all tags

### Links
- `GET /api/notes/:id/backlinks` - Get backlinks for a note
- `GET /api/graph` - Get graph data for visualization

## Configuration
```toml
# config.toml
[server]
host = "0.0.0.0"
port = 8080

[storage]
notes_dir = "./data/notes"

[search]
index_dir = "./data/index"

[auth]
enabled = true
token = "your-secure-token"
```

## Learning Resources for Rust

### Books
- The Rust Programming Language (official book)
- Rust by Example
- Zero To Production In Rust (web development focused)

### Key Concepts to Learn
1. Ownership, borrowing, and lifetimes
2. Error handling with Result and Option
3. Async/await with Tokio
4. Traits and generics
5. Pattern matching
6. Testing in Rust

## Success Criteria
- [ ] Can create, edit, and delete notes through web interface
- [ ] Fast full-text search across all notes
- [ ] Notes stored as plain markdown files
- [ ] Can be deployed with single Docker command
- [ ] Responsive UI works on desktop and mobile
- [ ] Bidirectional linking works reliably
- [ ] Learning Rust fundamentals through implementation

## Future Possibilities
- Mobile app (using Tauri or native)
- End-to-end encryption
- Collaboration features (shared notes)
- API for third-party integrations
- CLI tool for note management
- Vim/Emacs plugin integration

## Getting Started

1. **Initialize Rust backend**
   ```bash
   cd backend
   cargo init
   cargo add axum tokio serde
   ```

2. **Set up development environment**
   - Install Rust toolchain
   - Install Node.js for frontend
   - Set up your code editor (VS Code with rust-analyzer)

3. **Start with MVP**
   - Focus on basic CRUD first
   - Get something working end-to-end
   - Iterate and add features

---

**Note**: This is a living document. Update as the project evolves and requirements change.
