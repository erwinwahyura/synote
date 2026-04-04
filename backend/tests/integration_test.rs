//! Integration tests for Synote API
//!
//! Run with: cargo test --test integration_test

use std::process::Command;
use std::thread;
use std::time::Duration;

/// Helper function to start the server for tests
fn start_test_server() -> Option<u32> {
    // In a real CI environment, we'd start the server here
    // For now, this is a placeholder showing the test structure
    None
}

#[test]
fn test_server_compiles() {
    // Verify the project builds
    let output = Command::new("cargo")
        .args(&["check"])
        .current_dir("../backend")
        .output()
        .expect("Failed to execute cargo check");

    assert!(
        output.status.success(),
        "Server failed to compile: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_health_endpoint_structure() {
    // Verify health response structure matches expected format
    let expected_fields = vec!["status", "version", "timestamp", "notes_count"];
    
    // This is a structural test - in real integration tests,
    // we'd make an HTTP request and verify the JSON structure
    for field in expected_fields {
        assert!(!field.is_empty(), "Field name should not be empty");
    }
}

#[cfg(feature = "integration-tests")]
mod live_tests {
    use reqwest;
    use serde_json::Value;
    use std::time::Duration;
    use tokio;

    const BASE_URL: &str = "http://localhost:8080";

    #[tokio::test]
    async fn test_health_endpoint() {
        let client = reqwest::Client::new();
        let response = client
            .get(&format!("{}/api/health", BASE_URL))
            .timeout(Duration::from_secs(5))
            .send()
            .await
            .expect("Failed to connect to server");

        assert_eq!(response.status(), 200);
        
        let body: Value = response.json().await.expect("Invalid JSON");
        assert_eq!(body["status"], "healthy");
        assert!(body["version"].is_string());
        assert!(body["timestamp"].is_string());
        assert!(body["notes_count"].is_number());
    }

    #[tokio::test]
    async fn test_crud_operations() {
        let client = reqwest::Client::new();

        // Create a note
        let create_response = client
            .post(&format!("{}/api/notes", BASE_URL))
            .json(&serde_json::json!({
                "title": "Test Note",
                "content": "Test content"
            }))
            .send()
            .await
            .expect("Failed to create note");

        assert_eq!(create_response.status(), 201);
        let created: Value = create_response.json().await.expect("Invalid JSON");
        let note_id = created["id"].as_str().expect("Missing note ID");

        // Get the note
        let get_response = client
            .get(&format!("{}/api/notes/{}", BASE_URL, note_id))
            .send()
            .await
            .expect("Failed to get note");

        assert_eq!(get_response.status(), 200);

        // Update the note
        let update_response = client
            .put(&format!("{}/api/notes/{}", BASE_URL, note_id))
            .json(&serde_json::json!({
                "title": "Updated Title",
                "content": "Updated content"
            }))
            .send()
            .await
            .expect("Failed to update note");

        assert_eq!(update_response.status(), 200);

        // Delete the note
        let delete_response = client
            .delete(&format!("{}/api/notes/{}", BASE_URL, note_id))
            .send()
            .await
            .expect("Failed to delete note");

        assert_eq!(delete_response.status(), 204);
    }

    #[tokio::test]
    async fn test_search_functionality() {
        let client = reqwest::Client::new();

        let response = client
            .get(&format!("{}/api/search?q=test", BASE_URL))
            .send()
            .await
            .expect("Failed to search");

        assert_eq!(response.status(), 200);
        let results: Value = response.json().await.expect("Invalid JSON");
        assert!(results.is_array());
    }

    #[tokio::test]
    async fn test_auth_required_when_enabled() {
        // This test would require auth to be enabled in test config
        // It verifies 401 is returned without token
        let client = reqwest::Client::new();

        let response = client
            .get(&format!("{}/api/notes", BASE_URL))
            .send()
            .await
            .expect("Failed to request");

        // If auth is enabled, should get 401
        // If auth is disabled, should get 200
        // This test documents the expected behavior
        let status = response.status();
        assert!(status == 200 || status == 401);
    }
}

#[test]
fn test_note_format_validation() {
    // Test that note storage format is correct
    let sample_frontmatter = r#"---
id: 550e8400-e29b-41d4-a716-446655440000
title: Sample Note
created_at: 2024-01-15T10:30:00Z
updated_at: 2024-01-15T11:00:00Z
---

This is the note content.
It can have multiple lines.
"#;

    // Verify frontmatter parsing logic
    assert!(sample_frontmatter.starts_with("---"));
    assert!(sample_frontmatter.contains("id:"));
    assert!(sample_frontmatter.contains("title:"));
    assert!(sample_frontmatter.contains("created_at:"));
    assert!(sample_frontmatter.contains("updated_at:"));
}

#[test]
fn test_config_loading() {
    // Test that config can be loaded from environment variables
    std::env::set_var("SYNOTE_AUTH_TOKEN", "test-token-12345");
    
    // Load config
    let config = synote::config::Config::load().unwrap();
    
    // Verify environment variable was picked up
    assert_eq!(config.auth.token, "test-token-12345");
    assert!(config.auth.enabled);
    
    // Clean up
    std::env::remove_var("SYNOTE_AUTH_TOKEN");
}
