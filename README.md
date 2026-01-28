# Synote

A self-hosted personal note-taking application inspired by Obsidian and Notion, built with Rust.

## Features

### Core Features
- ✅ Create, edit, and delete markdown notes
- ✅ File-based storage (notes stored as .md files)
- ✅ **Split-view editor** with live markdown preview
- ✅ **Syntax highlighting** for code blocks
- ✅ **Real-time search** across all notes
- ✅ **Auto-save** functionality
- ✅ Self-hosted and privacy-focused

### Editor Features
- ✅ Live markdown preview (GitHub Flavored Markdown)
- ✅ Syntax highlighting for code blocks
- ✅ Auto-save (2 seconds after last edit)
- ✅ Keyboard shortcuts (Cmd+S to save, Cmd+K to search)

### Coming Soon
- 🔲 Bidirectional linking (`[[Note]]` syntax)
- 🔲 Tags and filtering
- 🔲 Graph view of note connections
- 🔲 Advanced search with Tantivy
- 🔲 Folder organization

## Quick Start

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- A modern web browser

### Running the Backend

1. Navigate to the backend directory:
```bash
cd backend
```

2. Build and run the server:
```bash
cargo run
```

The server will start on `http://localhost:8080`

### Using the Frontend

1. Open `frontend/public/index.html` in your web browser, or
2. Use a simple HTTP server:
```bash
cd frontend/public
python3 -m http.server 3000
```

Then visit `http://localhost:3000`

## Project Structure

```
synote/
├── backend/          # Rust backend (Axum web server)
│   ├── src/
│   │   ├── api/      # REST API endpoints
│   │   ├── models/   # Data models
│   │   ├── storage/  # File system operations
│   │   └── main.rs
│   └── Cargo.toml
├── frontend/         # Web frontend
│   └── public/
│       └── index.html
├── data/
│   └── notes/        # Your notes are stored here
└── config.toml       # Configuration file
```

## Configuration

Edit `config.toml` to customize:

```toml
[server]
host = "127.0.0.1"
port = 8080

[storage]
notes_dir = "./data/notes"
```

## API Endpoints

- `GET /api/notes` - List all notes
- `GET /api/notes/:id` - Get a specific note
- `POST /api/notes` - Create a new note
- `PUT /api/notes/:id` - Update a note
- `DELETE /api/notes/:id` - Delete a note

## Development

See [project.md](project.md) for the full roadmap and development plan.

### Running in Development Mode

```bash
cd backend
cargo watch -x run  # Auto-reload on changes (requires cargo-watch)
```

## License

See [LICENSE](LICENSE) file for details.
