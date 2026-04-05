use crate::models::Note;
use regex::Regex;
use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;

/// Tag extracted from note (inline #tag or frontmatter)
#[derive(Debug, Clone)]
pub struct Tag {
    pub name: String,
    pub source: TagSource, // inline or frontmatter
}

#[derive(Debug, Clone)]
pub enum TagSource {
    Inline,      // #tag in content
    Frontmatter, // tags: [...] in YAML
}

/// Global tag index - maps tag names to note IDs
pub struct TagIndex {
    // Map from tag name to set of note IDs
    tag_to_notes: RwLock<HashMap<String, Vec<Uuid>>>,
    // Map from note ID to its tags (for quick removal)
    note_tags: RwLock<HashMap<Uuid, Vec<String>>>,
}

impl TagIndex {
    pub fn new() -> Self {
        Self {
            tag_to_notes: RwLock::new(HashMap::new()),
            note_tags: RwLock::new(HashMap::new()),
        }
    }
    
    /// Extract tags from a note and update index
    pub fn index_note(&self, note: &Note) {
        let tags = extract_tags(note);
        
        // Remove old tags for this note
        if let Ok(note_tags_guard) = self.note_tags.read() {
            if let Some(old_tags) = note_tags_guard.get(&note.id) {
                if let Ok(mut tag_notes_guard) = self.tag_to_notes.write() {
                    for tag in old_tags {
                        if let Some(notes) = tag_notes_guard.get_mut(tag) {
                            notes.retain(|&id| id != note.id);
                        }
                    }
                }
            }
        }
        
        // Add new tags
        let tag_names: Vec<String> = tags.iter().map(|t| t.name.clone()).collect();
        
        if let Ok(mut tag_notes_guard) = self.tag_to_notes.write() {
            for tag in &tag_names {
                tag_notes_guard
                    .entry(tag.clone())
                    .or_insert_with(Vec::new)
                    .push(note.id);
            }
        }
        
        if let Ok(mut note_tags_guard) = self.note_tags.write() {
            note_tags_guard.insert(note.id, tag_names);
        }
    }
    
    /// Remove note from index
    pub fn remove_note(&self, note_id: &Uuid) {
        if let Ok(note_tags_guard) = self.note_tags.read() {
            if let Some(tags) = note_tags_guard.get(note_id) {
                if let Ok(mut tag_notes_guard) = self.tag_to_notes.write() {
                    for tag in tags {
                        if let Some(notes) = tag_notes_guard.get_mut(tag) {
                            notes.retain(|&id| id != *note_id);
                        }
                    }
                }
            }
        }
        
        if let Ok(mut note_tags_guard) = self.note_tags.write() {
            note_tags_guard.remove(note_id);
        }
    }
    
    /// Get all notes with a specific tag
    pub fn get_notes_with_tag(&self, tag: &str) -> Vec<Uuid> {
        if let Ok(guard) = self.tag_to_notes.read() {
            guard.get(tag).cloned().unwrap_or_default()
        } else {
            vec![]
        }
    }
    
    /// Get all tags in the vault
    pub fn get_all_tags(&self) -> Vec<(String, usize)> {
        if let Ok(guard) = self.tag_to_notes.read() {
            let mut tags: Vec<(String, usize)> = guard
                .iter()
                .map(|(tag, notes)| (tag.clone(), notes.len()))
                .collect();
            tags.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by count desc
            tags
        } else {
            vec![]
        }
    }
    
    /// Get tags for a specific note
    pub fn get_note_tags(&self, note_id: &Uuid) -> Vec<String> {
        if let Ok(guard) = self.note_tags.read() {
            guard.get(note_id).cloned().unwrap_or_default()
        } else {
            vec![]
        }
    }
}

/// Extract all tags from a note (inline + frontmatter)
fn extract_tags(note: &Note) -> Vec<Tag> {
    let mut tags = vec![];
    
    // Parse inline tags from content
    tags.extend(parse_inline_tags(&note.content));
    
    // Parse frontmatter tags (from the raw content since Note struct doesn't store it separately)
    // For now, we re-parse the file content to get frontmatter
    // TODO: Optimize by storing frontmatter in Note struct
    
    tags
}

/// Parse inline hashtags: #tag-name or #multi-word-tag
/// Rules:
/// - Starts with #
/// - Contains letters, numbers, hyphens, underscores
/// - Ends at whitespace or punctuation (.,!?) 
fn parse_inline_tags(content: &str) -> Vec<Tag> {
    lazy_static::lazy_static! {
        // Match #tag where tag contains word chars, hyphens, underscores, forward slashes (nested)
        // Doesn't match hex colors (#fff, #123456) 
        static ref TAG_RE: Regex = Regex::new(
            r"(?<![a-zA-Z0-9])#([a-zA-Z0-9_\-/]+)"
        ).unwrap();
        
        // Exclude hex colors (6 hex chars after #)
        static ref HEX_COLOR_RE: Regex = Regex::new(
            r"^#[a-fA-F0-9]{6}$|^#[a-fA-F0-9]{3}$"
        ).unwrap();
    }
    
    TAG_RE
        .captures_iter(content)
        .filter_map(|cap| {
            let tag_name = cap.get(1)?.as_str();
            
            // Skip hex colors
            if HEX_COLOR_RE.is_match(&format!("#{}", tag_name)) {
                return None;
            }
            
            Some(Tag {
                name: tag_name.to_lowercase(),
                source: TagSource::Inline,
            })
        })
        .collect()
}

/// Parse frontmatter tags from YAML
/// Supports:
///   tags: [tag1, tag2, tag3]
///   tags:
///     - tag1
///     - tag2
pub fn parse_frontmatter_tags(content: &str) -> Vec<Tag> {
    // Extract frontmatter section
    if !content.starts_with("---") {
        return vec![];
    }
    
    let parts: Vec<&str> = content.splitn(3, "---").collect();
    if parts.len() < 3 {
        return vec![];
    }
    
    let frontmatter = parts[1];
    let mut tags = vec![];
    
    // Simple line-by-line parsing for tags field
    // This is a basic parser - for production use a proper YAML library
    for line in frontmatter.lines() {
        let line = line.trim();
        
        // Array format: tags: [tag1, tag2]
        if line.starts_with("tags:") {
            // Check inline array
            if let Some(start) = line.find('[') {
                if let Some(end) = line.find(']') {
                    let array_content = &line[start+1..end];
                    for item in array_content.split(',') {
                        let tag = item.trim().trim_matches('"').trim_matches('\'');
                        if !tag.is_empty() {
                            tags.push(Tag {
                                name: tag.to_lowercase(),
                                source: TagSource::Frontmatter,
                            });
                        }
                    }
                }
            }
        }
        
        // List format continuation (simplified)
        else if line.starts_with("- ") && !tags.is_empty() {
            // This is simplistic - assumes previous line was tags:
            let tag = line[2..].trim().trim_matches('"').trim_matches('\'');
            if !tag.is_empty() {
                tags.push(Tag {
                    name: tag.to_lowercase(),
                    source: TagSource::Frontmatter,
                });
            }
        }
    }
    
    tags
}

/// Check if content contains a tag (for highlighting)
pub fn contains_tag(content: &str, tag: &str) -> bool {
    let pattern = format!(r"#{}(?![a-zA-Z0-9])", regex::escape(tag));
    Regex::new(&pattern).map(|re| re.is_match(content)).unwrap_or(false)
}
