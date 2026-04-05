use crate::state::AppState;
use crate::links::LinksIndex;
use axum::{
    extract::State,
    Json,
};
use serde::Serialize;
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
}

/// GET /api/graph - Get graph data for visualization
/// 
/// Returns all notes as nodes and all wikilinks as edges
pub async fn get_graph(
    State(app_state): State<AppState>,
) -> Result<Json<GraphResponse>, axum::http::StatusCode> {
    // Get all notes
    let all_notes = app_state.storage.list()
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    
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
        
        // Add edges for each resolved outgoing link
        for link in outgoing {
            if let Some(target_id) = link.target_id {
                edges.push(GraphEdge {
                    source: note.id.to_string(),
                    target: target_id.to_string(),
                });
            }
        }
    }
    
    // Second pass: calculate node sizes based on connection count
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
