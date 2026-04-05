use crate::models::Note;
use regex::Regex;
use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;

/// A parsed wikilink [[Target|Alias#Heading]]
#[derive(Debug, Clone)]
pub struct Link {
    pub target_title: String,  // Raw target text (may include #heading)
    pub display_text: String,  // What to show (alias or title without heading)
    pub heading: Option<String>, // #Heading reference
    pub target_id: Option<Uuid>, // Resolved ID (None if target doesn't exist)
}

/// All links for a note
#[derive(Debug, Clone, Default)]
pub struct NoteLinks {
    pub outgoing: Vec<Link>, // Links FROM this note
    pub incoming: Vec<Link>, // Links TO this note (backlinks)
}

/// Global links index - updated as notes are loaded/modified
pub struct LinksIndex {
    // Map from note ID to its outgoing links
    outgoing: RwLock<HashMap<Uuid, Vec<Link>>>,
}

impl LinksIndex {
    pub fn new() -> Self {
        Self {
            outgoing: RwLock::new(HashMap::new()),
        }
    }
    
    /// Parse wikilinks from note content and store
    pub fn update_note(&self, note: &Note, all_notes: &[Note]) {
        let outgoing = parse_wikilinks(&note.content, all_notes);
        
        if let Ok(mut guard) = self.outgoing.write() {
            guard.insert(note.id, outgoing);
        }
    }
    
    /// Remove note from index
    pub fn remove_note(&self, note_id: &Uuid) {
        if let Ok(mut guard) = self.outgoing.write() {
            guard.remove(note_id);
        }
    }
    
    /// Get all links for a note (outgoing + backlinks)
    pub fn get_note_links(&self, note_id: Uuid, all_notes: &[Note]) -> NoteLinks {
        let outgoing = if let Ok(guard) = self.outgoing.read() {
            guard.get(&note_id).cloned().unwrap_or_default()
        } else {
            vec![]
        };
        
        // Compute backlinks by scanning all notes
        let incoming = self.compute_backlinks(note_id, all_notes);
        
        NoteLinks { outgoing, incoming }
    }
    
    /// Find all notes linking TO this note
    fn compute_backlinks(&self, target_id: Uuid, all_notes: &[Note]) -> Vec<Link> {
        let mut incoming = vec![];
        
        if let Ok(guard) = self.outgoing.read() {
            for (source_id, links) in guard.iter() {
                for link in links {
                    if link.target_id == Some(target_id) {
                        // Find source note title for display
                        let source_title = all_notes
                            .iter()
                            .find(|n| n.id == *source_id)
                            .map(|n| n.title.clone())
                            .unwrap_or_else(|| "Unknown".to_string());
                        
                        incoming.push(Link {
                            target_title: source_title, // Backlink shows source
                            display_text: source_title.clone(),
                            heading: None,
                            target_id: Some(*source_id),
                        });
                    }
                }
            }
        }
        
        incoming
    }
}

/// Parse wikilinks [[...]] from markdown content
fn parse_wikilinks(content: &str, all_notes: &[Note]) -> Vec<Link> {
    lazy_static::lazy_static! {
        // Match [[target|alias]] or [[target#heading|alias]] or [[target]]
        // Groups: 1 = target (with optional #heading), 2 = optional alias
        static ref WIKILINK_RE: Regex = Regex::new(
            r"\[\[([^\]|]+?)(?:\|([^\]]+?))?\]\]"
        ).unwrap();
    }
    
    WIKILINK_RE
        .captures_iter(content)
        .filter_map(|cap| {
            let full_target = cap.get(1)?.as_str().trim();
            let alias = cap.get(2).map(|m| m.as_str().trim().to_string());
            
            // Split target and heading: "Note Title#Heading"
            let (target_title, heading) = if let Some(pos) = full_target.find('#') {
                let (title, rest) = full_target.split_at(pos);
                (title.trim(), Some(rest[1..].trim().to_string()))
            } else {
                (full_target, None)
            };
            
            // Resolve to ID
            let target_id = all_notes
                .iter()
                .find(|n| n.title.to_lowercase() == target_title.to_lowercase())
                .map(|n| n.id);
            
            // Display text: alias if provided, otherwise title (without heading)
            let display_text = alias.unwrap_or_else(|| target_title.to_string());
            
            Some(Link {
                target_title: target_title.to_string(),
                display_text,
                heading,
                target_id,
            })
        })
        .collect()
}

/// Render markdown with wikilinks replaced as HTML links
pub fn render_wikilinks(content: &str, links: &[Link]) -> String {
    let mut result = content.to_string();
    
    // Simple string replacement (in order of appearance)
    for link in links {
        let pattern = if link.target_id.is_some() {
            // Existing note - link to it
            format!(
                r#"<a href="#/note/{}" class="wikilink">{}</a>"#,
                link.target_id.unwrap(),
                link.display_text
            )
        } else {
            // Broken link - show with 'create' indicator
            format!(
                r#"<a href="#/create?title={}" class="wikilink broken" data-title="{}">{}</a>"#,
                urlencoding::encode(&link.target_title),
                link.target_title,
                link.display_text
            )
        };
        
        // Replace first occurrence of [[...]]
        // This is naive - proper implementation needs better handling
        let wikilink = if link.display_text != link.target_title {
            format!("[[{}|{}]]", link.target_title, link.display_text)
        } else {
            format!("[[{}]]", link.target_title)
        };
        
        result = result.replacen(&wikilink, &pattern, 1);
    }
    
    result
}

/// Find all note titles mentioned in content (for auto-complete, graph)
pub fn extract_linked_titles(content: &str) -> Vec<String> {
    lazy_static::lazy_static! {
        static ref WIKILINK_RE: Regex = Regex::new(
            r"\[\[([^\]|]+?)(?:\|[^\]]+?)?\]\]"
        ).unwrap();
    }
    
    WIKILINK_RE
        .captures_iter(content)
        .map(|cap| {
            let full = cap.get(1).unwrap().as_str();
            full.split('#').next().unwrap().trim().to_string()
        })
        .collect()
}
