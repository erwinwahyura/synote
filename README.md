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

### Linking & Organization
- ✅ **Bidirectional linking** (`[[Note]]` syntax) - Click to navigate, create missing notes
- ✅ **Tags and filtering** - Sidebar tag list, click to filter notes by tag
- ✅ **Graph view** - Visualize note connections via wikilinks
- ✅ **Tag-based graph connections** - Notes sharing the same tag are connected (green dashed lines)
- ✅ **Backlinks panel** - See all notes linking to current note

### Production Ready
- ✅ Docker deployment with health checks
- ✅ Automatic HTTPS with Let's Encrypt (Caddy)
- ✅ Security headers & CSP
- ✅ Authentication support
- ✅ Automated backups
- ✅ **Persistent volume storage** - Data saved on Hetzner persistent volume

### In Progress
- 🔄 **Advanced search with Tantivy** - Backend module ready, needs API integration
- 🔄 **Folder organization** - Backend structure ready, needs UI implementation

### Coming Soon
- 🔲 Real-time sync across devices (git-based or CRDT)
- 🔲 Mobile-optimized UI
- 🔲 Plugin/extension system

## Data Storage

Your notes are stored as **plain markdown files** on a **persistent Hetzner volume**:

| Location | Path | Purpose |
|----------|------|---------|
| **Production data** | `/mnt/apps-data/synote/` | Live notes storage |
| **Daily backups** | `/mnt/apps-data/synote-backups/` | Automated backups |
| **Live URL** | `https://research.erwarx.com` | Production instance |

**Benefits:**
- ✅ Human-readable `.md` files (not locked in database)
- ✅ Easy backup/restore (just copy files)
- ✅ Git-compatible for version control
- ✅ Portable across systems

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

## Production Deployment

### Option 1: Docker (Recommended)

#### Quick Start (Development)
```bash
cd docker
docker-compose up --build
```

#### Production with HTTPS

1. **Clone and enter the directory:**
   ```bash
   git clone https://github.com/erwinwahyura/synote.git
   cd synote/docker
   ```

2. **Set your domain:**
   ```bash
   export DOMAIN=notes.yourdomain.com
   ```

3. **Set a secure authentication token:**
   ```bash
   export SYNOTE_AUTH_TOKEN=$(openssl rand -hex 32)
   echo "Token: $SYNOTE_AUTH_TOKEN"
   ```

4. **Start the production stack:**
   ```bash
   docker-compose -f docker-compose.prod.yml up -d
   ```

5. **Access your instance:**
   - HTTPS will be automatically configured via Let's Encrypt
   - Visit `https://notes.yourdomain.com`

#### Production Features
| Feature | Description |
|---------|-------------|
| 🔒 Automatic HTTPS | Let's Encrypt certificates (auto-renew) |
| 🛡️ Security Headers | CSP, HSTS, X-Frame-Options, etc. |
| ♻️ Auto-restart | Container restarts on crash |
| 🏥 Health Checks | Automatic monitoring & recovery |
| 💾 Daily Backups | Automated note backups to persistent volume |
| 🚀 Gzip Compression | Faster content delivery |

#### Environment Variables
| Variable | Description | Default |
|----------|-------------|---------|
| `DOMAIN` | Your domain for HTTPS | `localhost` |
| `SYNOTE_AUTH_TOKEN` | API authentication token | `changeme` |
| `RUST_LOG` | Logging level (error/warn/info/debug) | `info` |

### Option 2: Hetzner Deployment (Current Setup)

**Server:** `46.224.127.221` (hetzner-cx23)
**Data Volume:** `/mnt/apps-data/synote/` (persistent)
**Caddy:** Reverse proxy with automatic HTTPS

```bash
# On Hetzner server
cd /home/deploy/synote
docker compose pull
docker compose up -d
```

## Project Structure

```
synote/
├── backend/          # Rust backend (Axum web server)
│   ├── src/
│   │   ├── api/      # REST API endpoints (notes, tags, links, graph)
│   │   ├── models/   # Data models
│   │   ├── storage/  # File system operations
│   │   ├── links/    # Wikilink parsing and index
│   │   ├── tags/     # Tag extraction and index
│   │   └── main.rs
│   └── Cargo.toml
├── frontend/         # Web frontend (vanilla JS + D3.js for graph)
│   └── public/
│       └── index.html
├── docker/           # Docker configurations
│   ├── docker-compose.yml
│   ├── docker-compose.prod.yml
│   └── Caddyfile
├── data/
│   └── notes/        # Your notes are stored here (markdown files)
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

[auth]
enabled = true
token = "your-secure-token-here"  # Set via SYNOTE_AUTH_TOKEN env var
```

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/notes` | List all notes |
| POST | `/api/notes` | Create a new note |
| GET | `/api/notes/:id` | Get a specific note |
| PUT | `/api/notes/:id` | Update a note |
| DELETE | `/api/notes/:id` | Delete a note |
| GET | `/api/search?q=query` | Search notes |
| GET | `/api/tags` | List all tags with counts |
| GET | `/api/tags/:tag/notes` | Get notes with specific tag |
| GET | `/api/notes/:id/tags` | Get tags for a note |
| GET | `/api/notes/:id/links` | Get wikilinks for a note |
| GET | `/api/graph?include_tags=true` | Get graph data (nodes + edges) |

### Authentication
When auth is enabled, include the token in the `Authorization` header:
```
Authorization: Bearer your-token-here
```

## Using the App

### Creating Notes
1. Click **"New Note"** button
2. Add title and content
3. Auto-saves after 2 seconds of inactivity

### Adding Tags
Type `#tagname` anywhere in note content. Tags appear in sidebar.

### Creating Links
Type `[[Note Title]]` to link to another note. Creates bidirectional connection.
- If note doesn't exist, click to create it
- Graph view shows all connections

### Viewing Graph
Click **🕸️ Graph** button to see:
- **Solid lines**: Wikilink connections
- **Green dashed lines**: Tag-based connections (notes sharing same tag)
- Click any node to open that note

### Filtering by Tag
Click any tag in the sidebar to show only notes with that tag.

## Development

See [project.md](project.md) for the full roadmap and development plan.

### Running in Development Mode

```bash
cd backend
cargo watch -x run  # Auto-reload on changes (requires cargo-watch)
```

### Running Tests

```bash
cd backend
cargo test
```

## Backup & Restore

### Automated Backup (Production)
Your notes are automatically backed up daily to `/mnt/apps-data/synote-backups/`.

### Manual Backup
Your notes are plain markdown files. Simply copy the data directory:
```bash
# From Hetzner server
sudo cp -r /mnt/apps-data/synote /path/to/backup

# Or download via scp
scp -r deploy@46.224.127.221:/mnt/apps-data/synote ./backup
```

### Restore
1. Stop the server: `docker compose down`
2. Copy your backup to `/mnt/apps-data/synote/`
3. Start the server: `docker compose up -d`

## Security Considerations

- Change the default `SYNOTE_AUTH_TOKEN` in production
- Use HTTPS (handled automatically in Docker production setup)
- Keep your server and Rust dependencies updated
- Back up your notes regularly
- Run behind a firewall/VPN for additional protection

## License

See [LICENSE](LICENSE) file for details.

## Contributing

Found a bug or have a feature request? Open an issue on GitHub!

## Acknowledgments

- Built with [Axum](https://github.com/tokio-rs/axum) web framework
- Inspired by [Obsidian](https://obsidian.md) and [Notion](https://notion.so)
- Markdown rendering by [marked](https://marked.js.org)
- Syntax highlighting by [highlight.js](https://highlightjs.org)
- Graph visualization by [D3.js](https://d3js.org)
