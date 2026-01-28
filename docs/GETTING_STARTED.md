# Getting Started with Synote

## Installation

### Install Rust

If you don't have Rust installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Restart your terminal and verify:
```bash
rustc --version
cargo --version
```

## Running Synote

### Option 1: Development Mode

1. Clone the repository and navigate to it:
```bash
cd synote/backend
```

2. Run the backend server:
```bash
cargo run
```

You should see:
```
Synote server listening on 127.0.0.1:8080
```

3. Open the frontend in your browser:
- Simply open `frontend/public/index.html` in your browser
- Or use a local server: `cd frontend/public && python3 -m http.server 3000`

### Option 2: Production Build

1. Build the release binary:
```bash
cd backend
cargo build --release
```

2. Run the binary:
```bash
./target/release/synote
```

### Option 3: Docker

1. Build and run with Docker:
```bash
cd docker
docker-compose up --build
```

## First Steps

1. Click "New Note" to create your first note
2. Edit the title and content
3. Click "Save" to persist your note
4. Your note is stored as a markdown file in `data/notes/`

## Understanding the Code Structure

### Backend (Rust)

- `main.rs` - Entry point, sets up the web server
- `config.rs` - Configuration loading
- `models/note.rs` - Note data structure
- `storage/mod.rs` - File system operations
- `api/notes.rs` - HTTP API endpoints

### Key Rust Concepts Used

1. **Ownership**: Each note file is owned by the storage system
2. **Error Handling**: Using `Result<T, E>` for operations that can fail
3. **Async/Await**: Using Tokio for async I/O operations
4. **Traits**: Using `Serialize`/`Deserialize` for JSON conversion

### Frontend

Simple vanilla JavaScript with fetch API to communicate with the backend.

## Next Steps

1. Try creating multiple notes
2. Explore the code in `backend/src/`
3. Read the Rust code to understand how it works
4. Check out `project.md` for the full roadmap
5. Implement new features from the roadmap!

## Learning Resources

### Rust Basics
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

### Web Development in Rust
- [Axum Documentation](https://docs.rs/axum/latest/axum/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)

### Next Features to Implement

1. Add search functionality
2. Implement tags
3. Add bidirectional linking
4. Create a better editor with markdown preview

## Troubleshooting

### Port already in use
Change the port in `config.toml`:
```toml
[server]
port = 8081
```

### Notes not persisting
Check that `data/notes/` directory exists and has write permissions.

### CORS errors
Make sure the backend is running and the frontend is accessing the correct URL.
