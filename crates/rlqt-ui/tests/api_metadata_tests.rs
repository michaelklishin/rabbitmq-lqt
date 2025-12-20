// Copyright (C) 2025-2026 Michael S. Klishin and Contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use chrono::Utc;
use rlqt_lib::DatabaseConnection;
use rlqt_lib::Severity;
use rlqt_lib::create_database;
use rlqt_lib::entry_metadata::labels::LogEntryLabels;
use rlqt_lib::parser::ParsedLogEntry;
use rlqt_lib::rel_db::FileMetadata;
use rlqt_lib::rel_db::NodeLogEntry;
use rlqt_lib::rel_db::file_metadata;
use rlqt_ui::server::create_router_for_testing;
use serde_json::Value;
use std::sync::Arc;
use tempfile::TempDir;
use tower::ServiceExt;

fn setup_test_db() -> (TempDir, DatabaseConnection) {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let db = create_database(&db_path).unwrap();
    (temp_dir, db)
}

fn insert_file_metadata(
    db: &DatabaseConnection,
    file_path: &str,
    nodes: Vec<&str>,
    subsystems: Vec<&str>,
    labels: Vec<&str>,
) {
    let model = file_metadata::Model {
        file_path: file_path.to_string(),
        rabbitmq_versions: vec!["4.2.0".to_string()],
        erlang_versions: vec!["27.3.4.3".to_string()],
        tls_library: Some("OpenSSL".to_string()),
        oldest_entry_at: Some(Utc::now()),
        most_recent_entry_at: Some(Utc::now()),
        total_lines: 100,
        total_entries: 50,
        nodes: nodes.iter().map(|s| s.to_string()).collect(),
        subsystems: subsystems.iter().map(|s| s.to_string()).collect(),
        labels: labels.iter().map(|s| s.to_string()).collect(),
        enabled_plugins: vec![
            "rabbitmq_management".to_string(),
            "rabbitmq_prometheus".to_string(),
        ],
    };

    FileMetadata::insert_metadata(db, model).unwrap();
}

#[tokio::test]
async fn test_get_metadata_returns_nodes_from_file_metadata() {
    let (_temp_dir, db) = setup_test_db();

    insert_file_metadata(
        &db,
        "/path/to/rabbit@node1.log",
        vec!["rabbit@node1"],
        vec!["boot", "connections"],
        vec!["queues", "raft"],
    );

    insert_file_metadata(
        &db,
        "/path/to/rabbit@node2.log",
        vec!["rabbit@node2"],
        vec!["plugins"],
        vec!["federation"],
    );

    let app = create_router_for_testing(Arc::new(db));

    let request = Request::builder()
        .uri("/api/metadata")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    let nodes = json["nodes"].as_array().unwrap();
    assert_eq!(nodes.len(), 2);

    let node_strings: Vec<String> = nodes
        .iter()
        .map(|n| n.as_str().unwrap().to_string())
        .collect();
    assert!(node_strings.contains(&"rabbit@node1".to_string()));
    assert!(node_strings.contains(&"rabbit@node2".to_string()));
}

#[tokio::test]
async fn test_get_metadata_returns_subsystems_from_file_metadata() {
    let (_temp_dir, db) = setup_test_db();

    insert_file_metadata(
        &db,
        "/path/to/rabbit@node1.log",
        vec!["rabbit@node1"],
        vec!["boot", "connections", "raft"],
        vec![],
    );

    insert_file_metadata(
        &db,
        "/path/to/rabbit@node2.log",
        vec!["rabbit@node2"],
        vec!["plugins", "connections"],
        vec![],
    );

    let app = create_router_for_testing(Arc::new(db));

    let request = Request::builder()
        .uri("/api/metadata")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    let subsystems = json["subsystems"].as_array().unwrap();
    assert!(subsystems.len() >= 4);

    let subsystem_strings: Vec<String> = subsystems
        .iter()
        .map(|s| s.as_str().unwrap().to_string())
        .collect();
    assert!(subsystem_strings.contains(&"boot".to_string()));
    assert!(subsystem_strings.contains(&"connections".to_string()));
    assert!(subsystem_strings.contains(&"raft".to_string()));
    assert!(subsystem_strings.contains(&"plugins".to_string()));
}

#[tokio::test]
async fn test_get_metadata_returns_labels_from_file_metadata() {
    let (_temp_dir, db) = setup_test_db();

    insert_file_metadata(
        &db,
        "/path/to/rabbit@node1.log",
        vec!["rabbit@node1"],
        vec![],
        vec!["queues", "raft", "elections"],
    );

    insert_file_metadata(
        &db,
        "/path/to/rabbit@node2.log",
        vec!["rabbit@node2"],
        vec![],
        vec!["federation", "shovels"],
    );

    let app = create_router_for_testing(Arc::new(db));

    let request = Request::builder()
        .uri("/api/metadata")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    let labels = json["labels"].as_array().unwrap();
    assert!(labels.len() >= 5);

    let label_strings: Vec<String> = labels
        .iter()
        .map(|l| l.as_str().unwrap().to_string())
        .collect();
    assert!(label_strings.contains(&"queues".to_string()));
    assert!(label_strings.contains(&"raft".to_string()));
    assert!(label_strings.contains(&"elections".to_string()));
    assert!(label_strings.contains(&"federation".to_string()));
    assert!(label_strings.contains(&"shovels".to_string()));
}

#[tokio::test]
async fn test_get_metadata_deduplicates_and_sorts() {
    let (_temp_dir, db) = setup_test_db();

    insert_file_metadata(
        &db,
        "/path/to/file1.log",
        vec!["rabbit@z_node", "rabbit@a_node"],
        vec!["raft", "boot"],
        vec!["queues", "elections"],
    );

    insert_file_metadata(
        &db,
        "/path/to/file2.log",
        vec!["rabbit@a_node", "rabbit@m_node"],
        vec!["boot", "plugins"],
        vec!["elections", "shovels"],
    );

    let app = create_router_for_testing(Arc::new(db));

    let request = Request::builder()
        .uri("/api/metadata")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    let nodes = json["nodes"].as_array().unwrap();
    let node_strings: Vec<String> = nodes
        .iter()
        .map(|n| n.as_str().unwrap().to_string())
        .collect();

    assert_eq!(
        node_strings,
        vec!["rabbit@a_node", "rabbit@m_node", "rabbit@z_node"]
    );

    let subsystems = json["subsystems"].as_array().unwrap();
    let subsystem_strings: Vec<String> = subsystems
        .iter()
        .map(|s| s.as_str().unwrap().to_string())
        .collect();

    assert_eq!(subsystem_strings[0], "boot");
    assert_eq!(subsystem_strings[1], "plugins");
    assert_eq!(subsystem_strings[2], "raft");
}

#[tokio::test]
async fn test_get_metadata_returns_severities() {
    let (_temp_dir, db) = setup_test_db();

    let app = create_router_for_testing(Arc::new(db));

    let request = Request::builder()
        .uri("/api/metadata")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    let severities = json["severities"].as_array().unwrap();
    assert_eq!(severities.len(), 6);

    let severity_strings: Vec<String> = severities
        .iter()
        .map(|s| s.as_str().unwrap().to_string())
        .collect();
    assert_eq!(
        severity_strings,
        vec!["debug", "info", "notice", "warning", "error", "critical"]
    );
}

#[tokio::test]
async fn test_get_file_metadata_returns_all_files() {
    let (_temp_dir, db) = setup_test_db();

    insert_file_metadata(
        &db,
        "/logs/rabbit@node1.log",
        vec!["rabbit@node1"],
        vec!["boot"],
        vec!["queues"],
    );

    insert_file_metadata(
        &db,
        "/logs/rabbit@node2.log",
        vec!["rabbit@node2"],
        vec!["connections"],
        vec!["raft"],
    );

    let app = create_router_for_testing(Arc::new(db));

    let request = Request::builder()
        .uri("/api/file-metadata")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    let files = json.as_array().unwrap();
    assert_eq!(files.len(), 2);

    let file1 = &files[0];
    assert_eq!(file1["file_path"], "/logs/rabbit@node1.log");
    let rmq_versions = file1["rabbitmq_versions"].as_array().unwrap();
    assert_eq!(rmq_versions.len(), 1);
    assert_eq!(rmq_versions[0], "4.2.0");
    let erl_versions = file1["erlang_versions"].as_array().unwrap();
    assert_eq!(erl_versions.len(), 1);
    assert_eq!(erl_versions[0], "27.3.4.3");
    assert_eq!(file1["tls_library"], "OpenSSL");

    let nodes = file1["nodes"].as_array().unwrap();
    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0], "rabbit@node1");

    let subsystems = file1["subsystems"].as_array().unwrap();
    assert_eq!(subsystems.len(), 1);
    assert_eq!(subsystems[0], "boot");

    let labels = file1["labels"].as_array().unwrap();
    assert_eq!(labels.len(), 1);
    assert_eq!(labels[0], "queues");

    let plugins = file1["enabled_plugins"].as_array().unwrap();
    assert_eq!(plugins.len(), 2);
}

#[tokio::test]
async fn test_get_file_metadata_empty_when_no_files() {
    let (_temp_dir, db) = setup_test_db();

    let app = create_router_for_testing(Arc::new(db));

    let request = Request::builder()
        .uri("/api/file-metadata")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    let files = json.as_array().unwrap();
    assert_eq!(files.len(), 0);
}

#[tokio::test]
async fn test_get_file_metadata_returns_actual_plugin_names() {
    let (_temp_dir, db) = setup_test_db();

    insert_file_metadata(
        &db,
        "/logs/rabbit@node1.log",
        vec!["rabbit@node1"],
        vec!["boot"],
        vec![],
    );

    let app = create_router_for_testing(Arc::new(db));

    let request = Request::builder()
        .uri("/api/file-metadata")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    let files = json.as_array().unwrap();
    let file1 = &files[0];
    let plugins = file1["enabled_plugins"].as_array().unwrap();

    assert_eq!(plugins.len(), 2);
    let plugin_names: Vec<String> = plugins
        .iter()
        .map(|p| p.as_str().unwrap().to_string())
        .collect();
    assert!(plugin_names.contains(&"rabbitmq_management".to_string()));
    assert!(plugin_names.contains(&"rabbitmq_prometheus".to_string()));
}

#[tokio::test]
async fn test_get_stats_returns_total_and_per_node_counts() {
    let (_temp_dir, db) = setup_test_db();

    let entry1 = ParsedLogEntry {
        sequence_id: 1,
        explicit_id: Some(1),
        timestamp: Utc::now(),
        severity: Severity::Info,
        process_id: "<0.1.0>".to_string(),
        message: "Test message 1".to_string(),
        message_lowercased: "test message 1".to_string(),
        subsystem_id: None,
        labels: LogEntryLabels::empty(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    let entry2 = ParsedLogEntry {
        sequence_id: 2,
        explicit_id: Some(2),
        timestamp: Utc::now(),
        severity: Severity::Warning,
        process_id: "<0.2.0>".to_string(),
        message: "Test message 2".to_string(),
        message_lowercased: "test message 2".to_string(),
        subsystem_id: None,
        labels: LogEntryLabels::empty(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    let entry3 = ParsedLogEntry {
        sequence_id: 3,
        explicit_id: Some(3),
        timestamp: Utc::now(),
        severity: Severity::Error,
        process_id: "<0.3.0>".to_string(),
        message: "Test message 3".to_string(),
        message_lowercased: "test message 3".to_string(),
        subsystem_id: None,
        labels: LogEntryLabels::empty(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    NodeLogEntry::insert_parsed_entries(&db, &[entry1, entry2], "rabbit@node1").unwrap();
    NodeLogEntry::insert_parsed_entries(&db, &[entry3], "rabbit@node2").unwrap();

    let app = create_router_for_testing(Arc::new(db));

    let request = Request::builder()
        .uri("/api/stats")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["total_entries"], 3);

    let nodes = json["nodes"].as_array().unwrap();
    assert_eq!(nodes.len(), 2);

    assert_eq!(nodes[0]["node"], "rabbit@node1");
    assert_eq!(nodes[0]["count"], 2);

    assert_eq!(nodes[1]["node"], "rabbit@node2");
    assert_eq!(nodes[1]["count"], 1);
}

#[tokio::test]
async fn test_get_stats_empty_database() {
    let (_temp_dir, db) = setup_test_db();

    let app = create_router_for_testing(Arc::new(db));

    let request = Request::builder()
        .uri("/api/stats")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["total_entries"], 0);
    let nodes = json["nodes"].as_array().unwrap();
    assert_eq!(nodes.len(), 0);
}

#[tokio::test]
async fn test_get_metadata_returns_sorted_collections() {
    let (_temp_dir, db) = setup_test_db();

    insert_file_metadata(
        &db,
        "/path/to/file.log",
        vec!["rabbit@z_node", "rabbit@a_node", "rabbit@m_node"],
        vec!["raft", "boot", "plugins", "connections"],
        vec!["shovels", "queues", "elections", "federation"],
    );

    let app = create_router_for_testing(Arc::new(db));

    let request = Request::builder()
        .uri("/api/metadata")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    let nodes: Vec<String> = json["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|n| n.as_str().unwrap().to_string())
        .collect();
    assert_eq!(
        nodes,
        vec!["rabbit@a_node", "rabbit@m_node", "rabbit@z_node"]
    );

    let subsystems: Vec<String> = json["subsystems"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| s.as_str().unwrap().to_string())
        .collect();
    assert_eq!(subsystems, vec!["boot", "connections", "plugins", "raft"]);

    let labels: Vec<String> = json["labels"]
        .as_array()
        .unwrap()
        .iter()
        .map(|l| l.as_str().unwrap().to_string())
        .collect();
    assert_eq!(labels.len(), 54, "Should return all 54 available labels");
    assert!(labels.contains(&"elections".to_string()));
    assert!(labels.contains(&"federation".to_string()));
    assert!(labels.contains(&"queues".to_string()));
    assert!(labels.contains(&"shovels".to_string()));
}

#[tokio::test]
async fn test_get_file_metadata_returns_sorted_plugins() {
    let (_temp_dir, db) = setup_test_db();

    insert_file_metadata(
        &db,
        "/logs/rabbit@node1.log",
        vec!["rabbit@node1"],
        vec!["boot"],
        vec![],
    );

    let app = create_router_for_testing(Arc::new(db));

    let request = Request::builder()
        .uri("/api/file-metadata")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    let files = json.as_array().unwrap();
    let file1 = &files[0];
    let plugins: Vec<String> = file1["enabled_plugins"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| p.as_str().unwrap().to_string())
        .collect();

    assert_eq!(plugins.len(), 2);
    assert_eq!(plugins, vec!["rabbitmq_management", "rabbitmq_prometheus"]);
}

#[tokio::test]
async fn test_get_metadata_handles_special_characters() {
    let (_temp_dir, db) = setup_test_db();

    insert_file_metadata(
        &db,
        "/logs/test.log",
        vec!["rabbit@node-1", "rabbit@node_2", "rabbit@node.3"],
        vec!["sub-system", "sub_system", "sub.system"],
        vec!["label-1", "label_2", "label.3"],
    );

    let app = create_router_for_testing(Arc::new(db));

    let request = Request::builder()
        .uri("/api/metadata")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    let nodes: Vec<String> = json["nodes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|n| n.as_str().unwrap().to_string())
        .collect();
    assert_eq!(nodes.len(), 3);

    let subsystems: Vec<String> = json["subsystems"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| s.as_str().unwrap().to_string())
        .collect();
    assert_eq!(subsystems.len(), 3);

    let labels: Vec<String> = json["labels"]
        .as_array()
        .unwrap()
        .iter()
        .map(|l| l.as_str().unwrap().to_string())
        .collect();
    assert_eq!(
        labels.len(),
        54,
        "Should return all 54 available labels regardless of what's in the database"
    );
}

#[tokio::test]
async fn test_get_file_metadata_returns_timestamps() {
    let (_temp_dir, db) = setup_test_db();

    insert_file_metadata(
        &db,
        "/logs/rabbit@node1.log",
        vec!["rabbit@node1"],
        vec!["boot"],
        vec!["queues"],
    );

    let app = create_router_for_testing(Arc::new(db));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/file-metadata")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    let files = json.as_array().unwrap();
    assert_eq!(files.len(), 1);

    let file = &files[0];
    assert!(file["oldest_entry_at"].is_string());
    assert!(file["most_recent_entry_at"].is_string());
}

#[tokio::test]
async fn test_get_file_metadata_returns_counts() {
    let (_temp_dir, db) = setup_test_db();

    insert_file_metadata(
        &db,
        "/logs/rabbit@node1.log",
        vec!["rabbit@node1"],
        vec!["boot"],
        vec!["queues"],
    );

    let app = create_router_for_testing(Arc::new(db));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/file-metadata")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    let files = json.as_array().unwrap();
    assert_eq!(files.len(), 1);

    let file = &files[0];
    assert_eq!(file["total_lines"], 100);
    assert_eq!(file["total_entries"], 50);
}
