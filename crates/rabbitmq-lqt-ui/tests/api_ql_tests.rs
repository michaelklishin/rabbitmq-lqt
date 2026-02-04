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

//! Unit tests for the QL API endpoint.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use chrono::Utc;
use rabbitmq_lqt_lib::Severity;
use rabbitmq_lqt_lib::create_database;
use rabbitmq_lqt_lib::entry_metadata::labels::LogEntryLabels;
use rabbitmq_lqt_lib::parser::ParsedLogEntry;
use rabbitmq_lqt_lib::rel_db::NodeLogEntry;
use rabbitmq_lqt_ui::server::create_router_for_testing;
use serde_json::Value;
use std::sync::Arc;
use tempfile::TempDir;
use tower::ServiceExt;

fn setup_test_db() -> (TempDir, rabbitmq_lqt_lib::DatabaseConnection) {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let db = create_database(&db_path).unwrap();
    (temp_dir, db)
}

fn insert_test_entries(db: &rabbitmq_lqt_lib::DatabaseConnection) {
    let entry1 = ParsedLogEntry {
        sequence_id: 1,
        explicit_id: Some(1),
        timestamp: Utc::now(),
        severity: Severity::Error,
        process_id: "<0.1.0>".to_string(),
        message: "Connection refused".to_string(),
        message_lowercased: "connection refused".to_string(),
        subsystem_id: Some(1), // connections
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
        message: "Queue timeout".to_string(),
        message_lowercased: "queue timeout".to_string(),
        subsystem_id: Some(2), // queues
        labels: LogEntryLabels::empty(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    let entry3 = ParsedLogEntry {
        sequence_id: 3,
        explicit_id: Some(3),
        timestamp: Utc::now(),
        severity: Severity::Info,
        process_id: "<0.3.0>".to_string(),
        message: "Boot complete".to_string(),
        message_lowercased: "boot complete".to_string(),
        subsystem_id: None,
        labels: LogEntryLabels::empty(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    NodeLogEntry::insert_parsed_entries(db, &[entry1, entry2], "rabbit@node1").unwrap();
    NodeLogEntry::insert_parsed_entries(db, &[entry3], "rabbit@node2").unwrap();
}

#[tokio::test]
async fn test_ql_query_severity_filter() {
    let (_temp_dir, db) = setup_test_db();
    insert_test_entries(&db);

    let app = create_router_for_testing(Arc::new(db));

    let request = Request::builder()
        .uri("/api/logs/ql?query=%7Bseverity%3D%22error%22%7D")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["total"], 1);
    let entries = json["entries"].as_array().unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0]["severity"], "error");
}

#[tokio::test]
async fn test_ql_query_with_limit() {
    let (_temp_dir, db) = setup_test_db();
    insert_test_entries(&db);

    let app = create_router_for_testing(Arc::new(db));

    let request = Request::builder()
        .uri("/api/logs/ql?query=*&limit=2")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["total"], 2);
}

#[tokio::test]
async fn test_ql_query_invalid_syntax_returns_error() {
    let (_temp_dir, db) = setup_test_db();

    let app = create_router_for_testing(Arc::new(db));

    let request = Request::builder()
        .uri("/api/logs/ql?query=%7Binvalid")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert!(json["error"].as_str().is_some());
}

#[tokio::test]
async fn test_ql_query_empty_query_returns_error() {
    let (_temp_dir, db) = setup_test_db();

    let app = create_router_for_testing(Arc::new(db));

    let request = Request::builder()
        .uri("/api/logs/ql?query=")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert!(json["error"].as_str().unwrap().contains("Empty query"));
}

#[tokio::test]
async fn test_ql_query_preset_errors() {
    let (_temp_dir, db) = setup_test_db();
    insert_test_entries(&db);

    let app = create_router_for_testing(Arc::new(db));

    let request = Request::builder()
        .uri("/api/logs/ql?query=%3Aerrors")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["total"], 1);
    let entries = json["entries"].as_array().unwrap();
    assert_eq!(entries[0]["severity"], "error");
}

#[tokio::test]
async fn test_ql_query_wildcard() {
    let (_temp_dir, db) = setup_test_db();
    insert_test_entries(&db);

    let app = create_router_for_testing(Arc::new(db));

    let request = Request::builder()
        .uri("/api/logs/ql?query=*")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["total"], 3);
}

#[tokio::test]
async fn test_ql_query_message_contains() {
    let (_temp_dir, db) = setup_test_db();
    insert_test_entries(&db);

    let app = create_router_for_testing(Arc::new(db));

    let request = Request::builder()
        .uri("/api/logs/ql?query=message%20contains%20%22timeout%22")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["total"], 1);
    let entries = json["entries"].as_array().unwrap();
    assert!(entries[0]["message"].as_str().unwrap().contains("timeout"));
}

#[tokio::test]
async fn test_ql_query_missing_query_param_returns_error() {
    let (_temp_dir, db) = setup_test_db();

    let app = create_router_for_testing(Arc::new(db));

    let request = Request::builder()
        .uri("/api/logs/ql")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_ql_query_sql_injection_single_quote() {
    let (_temp_dir, db) = setup_test_db();
    insert_test_entries(&db);

    let app = create_router_for_testing(Arc::new(db));

    // Attempt SQL injection with single quotes - should be safely escaped
    let request = Request::builder()
        .uri("/api/logs/ql?query=message%20contains%20%22%27%3B%20DROP%20TABLE%20node_log_entries%3B--%22")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    // Should return OK (not crash or execute the injection)
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    // Should return 0 results (no matches), not an error from SQL injection
    assert_eq!(json["total"], 0);
}

#[tokio::test]
async fn test_ql_query_sql_injection_like_wildcards() {
    let (_temp_dir, db) = setup_test_db();
    insert_test_entries(&db);

    let app = create_router_for_testing(Arc::new(db));

    // Attempt to abuse LIKE wildcards - % should be escaped
    let request = Request::builder()
        .uri("/api/logs/ql?query=message%20contains%20%22%25%22")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    // Should not match all entries (% wildcard should be escaped)
    assert_eq!(json["total"], 0);
}

#[tokio::test]
async fn test_ql_query_backslash_escaping() {
    let (_temp_dir, db) = setup_test_db();
    insert_test_entries(&db);

    let app = create_router_for_testing(Arc::new(db));

    // Backslashes should be properly escaped
    let request = Request::builder()
        .uri("/api/logs/ql?query=message%20contains%20%22%5C%5C%22")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
