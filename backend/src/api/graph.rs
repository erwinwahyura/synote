use crate::state::AppState;
use crate::links::LinksIndex;
use axum::{
    extract::{Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize)]
pub struct GraphResponse {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

#[derive(Serialize)]
pub struct GraphNode {
    pub id: String,
    pub title: String,
    pub size: f32, // Based on number of connections
}

#[derive(Serialize)]
pub struct GraphEdge {
    pub source: String,
    pub target: String,
    #[serde(rename = "type")]
    pub edge_type: String, // "link" for wikilinks, "tag" for shared tags
}

#[derive(Deserialize)]
pub struct GraphQuery {
    #[serde(default)]
    pub include_tags: bool, // Whether to include tag-based connections
}

/// GET /api/graph - Get graph data for visualization
/// 
/// Returns all notes as nodes and wikilinks as edges.
/// Optionally includes tag-based connections (notes sharing the same tag).
pub async fn get_graph(
    State(app_state): State<AppState>,
    Query(params): Query<GraphQuery>,
) -> Result<Json<GraphResponse>, axum::http::StatusCode> {
    // Get all notes
    let all_notes = app_state.storage.list()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Index all notes to extract tags
    for note in &all_notes {
        app_state.tags_index.index_note(note);
    }
    
    // Build nodes
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    
    // First pass: create all nodes and index outgoing links
    for note in &all_notes {
        // Get outgoing links for this note
        let outgoing = crate::links::parse_wikilinks(&note.content, &all_notes);
        
        // Add node
        nodes.push(GraphNode {
            id: note.id.to_string(),
            title: note.title.clone(),
            size: 10.0, // Base size, will adjust
        });
        
        // Add edges for each resolved outgoing link (WIKILINK edges)
        for link in outgoing {
            if let Some(target_id) = link.target_id {
                edges.push(GraphEdge {
                    source: note.id.to_string(),
                    target: target_id.to_string(),
                    edge_type: "link".to_string(),
                });
            }
        }
    }
    
    // Second pass: add TAG-based edges if requested
    if params.include_tags {
        // Get all tags and their associated notes
        let all_tags = app_state.tags_index.get_all_tags();
        
        for (tag_name, _count) in all_tags {
            let note_ids = app_state.tags_index.get_notes_with_tag(&tag_name);
            
            // Create edges between all notes sharing this tag
            // (connect first note to all others to avoid complete graph explosion)
            if note_ids.len() > 1 {
                let first_id = note_ids[0];
                for other_id in &note_ids[1..] {
                    edges.push(GraphEdge {
                        source: first_id.to_string(),
                        target: other_id.to_string(),
                        edge_type: format!("tag:{}", tag_name), // Tag-based edge
                    });
                }
            }
        }
    }
    
    // Third pass: calculate node sizes based on connection count
    let mut connection_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for edge in &edges {
        *connection_counts.entry(edge.source.clone()).or_insert(0) += 1;
        *connection_counts.entry(edge.target.clone()).or_insert(0) += 1;
    }
    
    for node in &mut nodes {
        let count = connection_counts.get(&node.id).cloned().unwrap_or(0);
        node.size = 10.0 + (count as f32 * 2.0).min(30.0); // Scale but cap
    }
    
    Ok(Json(GraphResponse { nodes, edges }))
}
