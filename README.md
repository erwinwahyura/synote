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

### Production Ready
- ✅ Docker deployment with health checks
- ✅ Automatic HTTPS with Let's Encrypt (Caddy)
- ✅ Security headers & CSP
- ✅ Authentication support
- ✅ Automated backups

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
| 💾 Daily Backups | Automated note backups to `./backups/` |
| 🚀 Gzip Compression | Faster content delivery |

#### Environment Variables
| Variable | Description | Default |
|----------|-------------|---------|
| `DOMAIN` | Your domain for HTTPS | `localhost` |
| `SYNOTE_AUTH_TOKEN` | API authentication token | `changeme` |
| `RUST_LOG` | Logging level (error/warn/info/debug) | `info` |

### Option 2: Manual Deployment

Build the release binary:
```bash
cd backend
cargo build --release
```

Run with custom config:
```bash
./target/release/synote
```

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
├── docker/           # Docker configurations
│   ├── docker-compose.yml
│   ├── docker-compose.prod.yml
│   └── Caddyfile
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

### Authentication
When auth is enabled, include the token in the `Authorization` header:
```
Authorization: Bearer your-token-here
```

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

### Manual Backup
Your notes are plain markdown files. Simply copy the `data/notes` directory:
```bash
cp -r data/notes /path/to/backup
```

### Automated Backup (Production)
The production Docker setup includes automatic daily backups to `./backups/`.

### Restore
1. Stop the server: `docker-compose down`
2. Copy your backup to `data/notes/`
3. Start the server: `docker-compose up -d`

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
