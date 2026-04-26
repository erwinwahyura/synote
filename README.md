# Synote

A self-hosted personal note-taking app inspired by Obsidian, built with Rust. Notes are stored as plain markdown files — no database, no lock-in.

**Live instance:** [research.erwarx.com](https://research.erwarx.com)

## Features

### Multi-Pane Workspace
- Open multiple notes side-by-side (up to 5 panes)
- Drag the handle between panes to resize
- Minimize panes to a vertical strip, restore with a click
- Per-pane back/forward navigation history
- Click a note in the sidebar to open it; if already open, scrolls to it

### Editor
- Click a note to preview it; click **Edit** to open the split editor modal
- Live markdown preview (GitHub Flavored Markdown)
- Syntax highlighting for code blocks
- Drag-and-drop or paste images directly into the editor
- Resizable editor/preview split
- `⌘S` to save

### Linking & Organization
- **Wikilinks** — `[[Note Title]]` syntax; click to navigate within the same pane
- **Hover previews** — hover a wikilink to see a content excerpt
- **Broken link creation** — click a `[[broken link]]` to create the note instantly
- **Backlinks panel** — every pane shows incoming and outgoing links
- **Tags** — write `#tagname` anywhere; click a tag to filter notes
- **Graph view** — D3 force graph of all wikilink connections; click nodes to open notes
- **Table of contents** — auto-generated per pane for notes with 3+ headings

### Sidebar
- Real-time title search with match highlighting
- **Pinned notes** — star any note (☆ in the pane topbar) to float it to the top
- **Recently viewed** section
- Sort by: last updated, title A→Z, created, or last viewed
- Resizable sidebar

### View & Navigation
- **Dark mode** — `⌘D` or click the moon icon
- **Zen / focus mode** — `⌘.` hides the sidebar; hover a pane to reveal controls
- **Command palette** — `⌘K` to jump to any note; shows recently viewed when empty
- **Keyboard shortcuts cheatsheet** — press `?`
- **Word count + reading time** — shown in the footer of every pane
- **Inline rename** — double-click a pane title to rename the note

### Export
- Export any pane as **Markdown** (`.md` download)
- **Print / PDF** via browser print dialog

### Production Ready
- Docker deployment with health checks
- Automatic HTTPS via Let's Encrypt (Caddy)
- Security headers & CSP
- Token-based authentication
- Automated daily backups
- Persistent volume storage (Hetzner)

## Quick Start

### Prerequisites

- Rust 1.70+ — [rustup.rs](https://rustup.rs)
- A modern web browser

### Run locally

**1. Start the backend:**
```bash
cd backend
cargo run
```
The API starts on `http://localhost:8080`.

**2. Serve the frontend:**
```bash
cd frontend/public
python3 -m http.server 3001
```

**3. Open** `http://localhost:3001`

## Production Deployment

### Option 1: Docker (Recommended)

**Development:**
```bash
cd docker
docker-compose up --build
```

**Production with HTTPS:**
```bash
git clone https://github.com/erwinwahyura/synote.git
cd synote/docker

export DOMAIN=notes.yourdomain.com
export SYNOTE_AUTH_TOKEN=$(openssl rand -hex 32)
echo "Token: $SYNOTE_AUTH_TOKEN"

docker-compose -f docker-compose.prod.yml up -d
```

HTTPS is configured automatically via Let's Encrypt. Visit `https://notes.yourdomain.com`.

| Feature | Description |
|---------|-------------|
| 🔒 Automatic HTTPS | Let's Encrypt certificates (auto-renew) |
| 🛡️ Security Headers | CSP, HSTS, X-Frame-Options |
| ♻️ Auto-restart | Container restarts on crash |
| 🏥 Health Checks | Automatic monitoring & recovery |
| 💾 Daily Backups | Automated backups to persistent volume |
| 🚀 Gzip Compression | Faster content delivery |

**Environment variables:**
| Variable | Description | Default |
|----------|-------------|---------|
| `DOMAIN` | Your domain for HTTPS | `localhost` |
| `SYNOTE_AUTH_TOKEN` | API authentication token | `changeme` |
| `RUST_LOG` | Logging level | `info` |

### Option 2: Hetzner (Current Setup)

**Server:** `46.224.127.221` (hetzner-cx23)  
**Data:** `/mnt/apps-data/synote/` (persistent volume)  
**Proxy:** Caddy with automatic HTTPS

```bash
cd /home/deploy/synote
docker compose pull && docker compose up -d
```

## Project Structure

```
synote/
├── backend/          # Rust/Axum API server
│   └── src/
│       ├── api/      # REST endpoints (notes, tags, links, graph)
│       ├── models/   # Data models
│       ├── storage/  # File system operations
│       ├── links/    # Wikilink parsing and index
│       ├── tags/     # Tag extraction and index
│       └── main.rs
├── frontend/
│   └── public/
│       └── index.html   # Entire frontend (single file, vanilla JS)
├── docker/
│   ├── docker-compose.yml
│   ├── docker-compose.prod.yml
│   └── Caddyfile
├── data/
│   └── notes/        # Markdown note files
└── config.toml
```

## Configuration

```toml
[server]
host = "127.0.0.1"
port = 8080

[storage]
notes_dir = "./data/notes"

[auth]
enabled = true
token = "your-secure-token-here"  # or set SYNOTE_AUTH_TOKEN env var
```

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/notes` | List all notes |
| POST | `/api/notes` | Create a note |
| GET | `/api/notes/:id` | Get a note |
| PUT | `/api/notes/:id` | Update a note |
| DELETE | `/api/notes/:id` | Delete a note |
| GET | `/api/search?q=query` | Search notes by title |
| GET | `/api/tags` | List all tags with counts |
| GET | `/api/tags/:tag/notes` | Notes with a specific tag |
| GET | `/api/notes/:id/links` | Wikilinks for a note |
| GET | `/api/graph` | Graph data (nodes + edges) |

**Authentication:** include `Authorization: Bearer <token>` on all requests when auth is enabled.

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `⌘K` | Command palette |
| `⌘S` | Save note (in editor) |
| `⌘D` | Toggle dark mode |
| `⌘.` | Toggle zen mode |
| `?` | Show all shortcuts |
| `Esc` | Close overlay / exit zen mode |
| `dbl-click title` | Rename note inline |

## Backup & Restore

Notes are plain `.md` files — just copy the data directory.

```bash
# Manual backup from Hetzner
scp -r deploy@46.224.127.221:/mnt/apps-data/synote ./backup

# Restore
docker compose down
cp -r ./backup /mnt/apps-data/synote
docker compose up -d
```

Automated daily backups run to `/mnt/apps-data/synote-backups/` in production.

## Security

- Change `SYNOTE_AUTH_TOKEN` before deploying
- HTTPS is handled automatically in the Docker production setup
- Keep Rust dependencies updated (`cargo update`)

## Tech Stack

- **Backend:** [Axum](https://github.com/tokio-rs/axum) (Rust)
- **Frontend:** Vanilla JS, single HTML file
- **Markdown:** [marked.js](https://marked.js.org)
- **Syntax highlighting:** [highlight.js](https://highlightjs.org)
- **Graph:** [D3.js](https://d3js.org)
- **Proxy:** [Caddy](https://caddyserver.com)

## License

See [LICENSE](LICENSE) for details.
