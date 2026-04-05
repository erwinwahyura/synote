//! Full-text search using Tantivy
//! 
//! Features:
//! - Fast full-text search across all note content
//! - BM25 scoring for relevance
//! - Incremental indexing

pub mod tantivy;

pub use tantivy::{SearchIndex, SearchResult};