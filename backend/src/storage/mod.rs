use crate::models::Note;
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;
use walkdir::WalkDir;

pub struct NoteStorage {
    notes_dir: PathBuf,
}

impl NoteStorage {
    pub fn new(notes_dir: PathBuf) -> Result<Self> {
        // Create notes directory if it doesn't exist
        fs::create_dir_all(&notes_dir)
            .context("Failed to create notes directory")?;

        Ok(Self { notes_dir })
    }

    fn parse_note_file(&self, path: &Path) -> Result<Note> {
        let content = fs::read_to_string(path)?;

        // Extract metadata from frontmatter (basic implementation)
        // Format:
        // ---
        // id: uuid
        // title: Note Title
        // created_at: timestamp
        // updated_at: timestamp
        // ---
        // Note content here

        let parts: Vec<&str> = content.splitn(3, "---").collect();

        if parts.len() < 3 {
            anyhow::bail!("Invalid note format: missing frontmatter");
        }

        let frontmatter = parts[1].trim();
        let note_content = parts[2].trim();

        // Parse frontmatter (simple key-value parsing)
        let mut id = None;
        let mut title = String::new();
        let mut created_at = None;
        let mut updated_at = None;

        for line in frontmatter.lines() {
            if let Some((key, value)) = line.split_once(':') {
                let key = key.trim();
                let value = value.trim();

                match key {
                    "id" => id = Some(Uuid::parse_str(value)?),
                    "title" => title = value.to_string(),
                    "created_at" => created_at = Some(value.parse()?),
                    "updated_at" => updated_at = Some(value.parse()?),
                    _ => {}
                }
            }
        }

        let relative_path = path.strip_prefix(&self.notes_dir)?
            .to_string_lossy()
            .to_string();

        Ok(Note {
            id: id.context("Missing id in frontmatter")?,
            title,
            content: note_content.to_string(),
            created_at: created_at.context("Missing created_at")?,
            updated_at: updated_at.context("Missing updated_at")?,
            path: relative_path,
        })
    }

    fn write_note_file(&self, note: &Note) -> Result<()> {
        let path = self.notes_dir.join(&note.path);

        // Create parent directories if needed
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = format!(
            "---\nid: {}\ntitle: {}\ncreated_at: {}\nupdated_at: {}\n---\n\n{}",
            note.id,
            note.title,
            note.created_at.to_rfc3339(),
            note.updated_at.to_rfc3339(),
            note.content
        );

        fs::write(path, content)?;
        Ok(())
    }

    pub fn create(&self, note: Note) -> Result<Note> {
        self.write_note_file(&note)?;
        Ok(note)
    }

    pub fn get(&self, id: &Uuid) -> Result<Note> {
        // Find the note file by searching through all markdown files
        for entry in WalkDir::new(&self.notes_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file()
                && entry.path().extension().and_then(|s| s.to_str()) == Some("md")
            {
                if let Ok(note) = self.parse_note_file(entry.path()) {
                    if note.id == *id {
                        return Ok(note);
                    }
                }
            }
        }

        anyhow::bail!("Note not found: {}", id)
    }

    pub fn list(&self) -> Result<Vec<Note>> {
        let mut notes = Vec::new();

        for entry in WalkDir::new(&self.notes_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file()
                && entry.path().extension().and_then(|s| s.to_str()) == Some("md")
            {
                if let Ok(note) = self.parse_note_file(entry.path()) {
                    notes.push(note);
                }
            }
        }

        // Sort by updated_at descending
        notes.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

        Ok(notes)
    }

    pub fn update(&self, id: &Uuid, mut note: Note) -> Result<Note> {
        // Get the old note to preserve its path
        let old_note = self.get(id)?;
        note.path = old_note.path;

        self.write_note_file(&note)?;
        Ok(note)
    }

    pub fn delete(&self, id: &Uuid) -> Result<()> {
        let note = self.get(id)?;
        let path = self.notes_dir.join(&note.path);
        fs::remove_file(path)?;
        Ok(())
    }

    pub fn search(&self, query: &str) -> Result<Vec<Note>> {
        let all_notes = self.list()?;
        let query_lower = query.to_lowercase();

        let results: Vec<Note> = all_notes
            .into_iter()
            .filter(|note| {
                note.title.to_lowercase().contains(&query_lower)
                    || note.content.to_lowercase().contains(&query_lower)
            })
            .collect();

        Ok(results)
    }
}
