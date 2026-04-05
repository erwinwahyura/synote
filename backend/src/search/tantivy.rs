use tantivy::{
    collector::TopDocs,
    query::QueryParser,
    schema::{Field, Schema, STORED, TEXT},
    Index, IndexReader, ReloadPolicy,
};
use std::path::Path;
use std::sync::{Arc, Mutex};
use anyhow::{Result, Context};
use crate::models::Note;

// Re-export Document for our usage
type Document = tantivy::TantivyDocument;

/// Full-text search index using Tantivy
pub struct SearchIndex {
    index: Index,
    schema: Schema,
    fields: SearchFields,
    writer: Arc<Mutex<tantivy::IndexWriter>>,
    reader: IndexReader,
}

struct SearchFields {
    id: Field,
    title: Field,
    content: Field,
}

impl SearchIndex {
    /// Create or open a Tantivy index at the given path
    pub fn open(index_dir: &Path) -> Result<Self> {
        std::fs::create_dir_all(index_dir)?;
        
        // Define schema
        let mut schema_builder = Schema::builder();
        let id_field = schema_builder.add_text_field("id", TEXT | STORED);
        let title_field = schema_builder.add_text_field("title", TEXT | STORED);
        let content_field = schema_builder.add_text_field("content", TEXT | STORED);
        let schema = schema_builder.build();
        
        let fields = SearchFields {
            id: id_field,
            title: title_field,
            content: content_field,
        };
        
        // Open or create index (Tantivy 0.21 API)
        let index = if index_dir.exists() {
            Index::open_in_dir(index_dir).context("Failed to open existing Tantivy index")?
        } else {
            std::fs::create_dir_all(index_dir)?;
            Index::create_in_dir(index_dir, schema.clone()).context("Failed to create Tantivy index")?
        };
        
        // Create writer
        let writer = index.writer(50_000_000)?; // 50MB heap
        
        // Create reader
        let reader = index.reader_builder()
            .reload_policy(ReloadPolicy::OnCommit)
            .try_into()?;
        
        Ok(Self {
            index,
            schema,
            fields,
            writer: Arc::new(RwLock::new(writer)),
            reader,
        })
    }
    
    /// Add or update a note in the index
    pub fn index_note(&self, note: &Note) -> Result<()> {
        let writer = self.writer.lock().map_err(|_| anyhow::anyhow!("Lock poisoned"))?;
        
        // Delete existing document with same ID
        let id_term = tantivy::Term::from_field_text(self.fields.id, &note.id.to_string());
        writer.delete_term(id_term);
        
        // Add new document
        let mut doc = tantivy::TantivyDocument::default();
        doc.add_text(self.fields.id, &note.id.to_string());
        doc.add_text(self.fields.title, &note.title);
        doc.add_text(self.fields.content, &note.content);
        writer.add_document(doc)?;
        
        Ok(())
    }
    
    /// Remove a note from the index
    pub fn remove_note(&self, note_id: &uuid::Uuid) -> Result<()> {
        let writer = self.writer.lock().map_err(|_| anyhow::anyhow!("Lock poisoned"))?;
        let id_term = tantivy::Term::from_field_text(self.fields.id, &note_id.to_string());
        writer.delete_term(id_term);
        Ok(())
    }
    
    /// Commit pending changes
    pub fn commit(&self) -> Result<()> {
        let mut writer = self.writer.lock().map_err(|_| anyhow::anyhow!("Lock poisoned"))?;
        writer.commit()?;
        Ok(())
    }
    
    /// Search notes
    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let searcher = self.reader.searcher();
        let query_parser = QueryParser::new(
            self.index.schema(),
            vec![self.fields.title, self.fields.content],
            self.index.tokenizers(),
        );
        let query = query_parser.parse_query(query)?;
        
        let top_docs = searcher.search(&query, &TopDocs::with_limit(limit))?;
        
        let mut results = Vec::new();
        for (score, doc_address) in top_docs {
            let doc = searcher.doc(doc_address)?;
            let id = doc.get_first(self.fields.id)
                .and_then(|v| v.as_text())
                .ok_or_else(|| anyhow::anyhow!("Missing id field"))?;
            let title = doc.get_first(self.fields.title)
                .and_then(|v| v.as_text())
                .unwrap_or("Untitled");
            
            results.push(SearchResult {
                id: id.to_string(),
                title: title.to_string(),
                score,
            });
        }
        
        Ok(results)
    }
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub score: f32,
}


