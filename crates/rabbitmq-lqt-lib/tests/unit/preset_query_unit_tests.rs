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

use chrono::Utc;
use rabbitmq_lqt_lib::entry_metadata::labels::LogEntryLabels;
use rabbitmq_lqt_lib::{
    NodeLogEntry, ParsedLogEntry, QueryContext, QueryPreset, Severity, create_database,
};
use std::str::FromStr;
use tempfile::TempDir;

#[test]
fn test_preset_from_str() {
    let preset = QueryPreset::from_str("errors_or_crashes").unwrap();
    assert_eq!(preset, QueryPreset::ErrorsOrCrashes);
}

#[test]
fn test_preset_from_str_unknown() {
    let result = QueryPreset::from_str("unknown_preset");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Unknown preset: unknown_preset");
}

#[test]
fn test_preset_display() {
    let preset = QueryPreset::ErrorsOrCrashes;
    assert_eq!(preset.to_string(), "errors_or_crashes");
}

#[test]
fn test_preset_severity() {
    let preset = QueryPreset::ErrorsOrCrashes;
    assert_eq!(preset.severity(), Some("error"));
}

#[test]
fn test_preset_labels() {
    let preset = QueryPreset::ErrorsOrCrashes;
    let labels = preset.labels();
    assert!(labels.contains(LogEntryLabels::ERL_PROCESS_CRASH));
    assert!(labels.contains(LogEntryLabels::EXCEPTIONS));
}

#[test]
fn test_preset_into_query_context() {
    let preset = QueryPreset::ErrorsOrCrashes;
    let _ctx: QueryContext = preset.into();
    // The conversion itself is the test - if it compiles and runs, it works
    // The actual query behavior is tested in the database tests below
}

#[test]
fn test_query_with_preset_matches_error_severity() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let db = create_database(&db_path).unwrap();

    let entries = vec![
        ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Error,
            process_id: "<0.1.0>".to_string(),
            message: "Error message without crash labels".to_string(),
            message_lowercased: "error message without crash labels".to_string(),
            subsystem_id: None,
            labels: LogEntryLabels::empty(),
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
        ParsedLogEntry {
            sequence_id: 1,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Info,
            process_id: "<0.2.0>".to_string(),
            message: "Info message".to_string(),
            message_lowercased: "info message".to_string(),
            subsystem_id: None,
            labels: LogEntryLabels::empty(),
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
    ];

    NodeLogEntry::insert_parsed_entries(&db, &entries, "test-node").unwrap();

    let ctx = QueryContext::from(QueryPreset::ErrorsOrCrashes);
    let results = NodeLogEntry::query(&db, &ctx).unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, "error");
}

#[test]
fn test_query_with_preset_matches_erl_process_crash_label() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let db = create_database(&db_path).unwrap();

    let entries = vec![
        ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Warning,
            process_id: "<0.1.0>".to_string(),
            message: "Process crash (not error severity)".to_string(),
            message_lowercased: "process crash (not error severity)".to_string(),
            subsystem_id: None,
            labels: LogEntryLabels::ERL_PROCESS_CRASH,
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
        ParsedLogEntry {
            sequence_id: 1,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Info,
            process_id: "<0.2.0>".to_string(),
            message: "Info message".to_string(),
            message_lowercased: "info message".to_string(),
            subsystem_id: None,
            labels: LogEntryLabels::empty(),
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
    ];

    NodeLogEntry::insert_parsed_entries(&db, &entries, "test-node").unwrap();

    let ctx = QueryContext::from(QueryPreset::ErrorsOrCrashes);
    let results = NodeLogEntry::query(&db, &ctx).unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, "warning");
    assert!(
        results[0]
            .get_labels()
            .contains(LogEntryLabels::ERL_PROCESS_CRASH)
    );
}

#[test]
fn test_query_with_preset_matches_exceptions_label() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let db = create_database(&db_path).unwrap();

    let entries = vec![
        ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Info,
            process_id: "<0.1.0>".to_string(),
            message: "Exception occurred".to_string(),
            message_lowercased: "exception occurred".to_string(),
            subsystem_id: None,
            labels: LogEntryLabels::EXCEPTIONS,
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
        ParsedLogEntry {
            sequence_id: 1,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Info,
            process_id: "<0.2.0>".to_string(),
            message: "Normal info message".to_string(),
            message_lowercased: "normal info message".to_string(),
            subsystem_id: None,
            labels: LogEntryLabels::empty(),
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
    ];

    NodeLogEntry::insert_parsed_entries(&db, &entries, "test-node").unwrap();

    let ctx = QueryContext::from(QueryPreset::ErrorsOrCrashes);
    let results = NodeLogEntry::query(&db, &ctx).unwrap();

    assert_eq!(results.len(), 1);
    assert!(results[0].get_labels().contains(LogEntryLabels::EXCEPTIONS));
}

#[test]
fn test_query_with_preset_or_logic() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let db = create_database(&db_path).unwrap();

    let entries = vec![
        ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Error,
            process_id: "<0.1.0>".to_string(),
            message: "Error without labels".to_string(),
            message_lowercased: "error without labels".to_string(),
            subsystem_id: None,
            labels: LogEntryLabels::empty(),
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
        ParsedLogEntry {
            sequence_id: 1,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Warning,
            process_id: "<0.2.0>".to_string(),
            message: "Warning with crash label".to_string(),
            message_lowercased: "warning with crash label".to_string(),
            subsystem_id: None,
            labels: LogEntryLabels::ERL_PROCESS_CRASH,
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
        ParsedLogEntry {
            sequence_id: 2,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Info,
            process_id: "<0.3.0>".to_string(),
            message: "Info with exceptions label".to_string(),
            message_lowercased: "info with exceptions label".to_string(),
            subsystem_id: None,
            labels: LogEntryLabels::EXCEPTIONS,
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
        ParsedLogEntry {
            sequence_id: 3,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Error,
            process_id: "<0.4.0>".to_string(),
            message: "Error with both crash and exceptions".to_string(),
            message_lowercased: "error with both crash and exceptions".to_string(),
            subsystem_id: None,
            labels: LogEntryLabels::ERL_PROCESS_CRASH | LogEntryLabels::EXCEPTIONS,
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
        ParsedLogEntry {
            sequence_id: 4,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Info,
            process_id: "<0.5.0>".to_string(),
            message: "Normal info message".to_string(),
            message_lowercased: "normal info message".to_string(),
            subsystem_id: None,
            labels: LogEntryLabels::empty(),
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
    ];

    NodeLogEntry::insert_parsed_entries(&db, &entries, "test-node").unwrap();

    let ctx = QueryContext::from(QueryPreset::ErrorsOrCrashes);
    let results = NodeLogEntry::query(&db, &ctx).unwrap();

    assert_eq!(results.len(), 4);
}

#[test]
fn test_query_with_preset_and_node_filter() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let db = create_database(&db_path).unwrap();

    let entries = vec![ParsedLogEntry {
        sequence_id: 0,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Error,
        process_id: "<0.1.0>".to_string(),
        message: "Error on node1".to_string(),
        message_lowercased: "error on node1".to_string(),
        subsystem_id: None,
        labels: LogEntryLabels::empty(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    }];
    NodeLogEntry::insert_parsed_entries(&db, &entries, "rabbit@node1").unwrap();

    let entries = vec![ParsedLogEntry {
        sequence_id: 1,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Error,
        process_id: "<0.2.0>".to_string(),
        message: "Error on node2".to_string(),
        message_lowercased: "error on node2".to_string(),
        subsystem_id: None,
        labels: LogEntryLabels::empty(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    }];
    NodeLogEntry::insert_parsed_entries(&db, &entries, "rabbit@node2").unwrap();

    let ctx = QueryContext::from(QueryPreset::ErrorsOrCrashes).node("rabbit@node1");
    let results = NodeLogEntry::query(&db, &ctx).unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].node, "rabbit@node1");
}

#[test]
fn test_query_with_preset_and_time_filter() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let db = create_database(&db_path).unwrap();

    let base_time = Utc::now();
    let earlier = base_time - chrono::Duration::hours(2);
    let later = base_time + chrono::Duration::hours(2);

    let entries = vec![
        ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: earlier,
            severity: Severity::Error,
            process_id: "<0.1.0>".to_string(),
            message: "Earlier error".to_string(),
            message_lowercased: "earlier error".to_string(),
            subsystem_id: None,
            labels: LogEntryLabels::empty(),
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
        ParsedLogEntry {
            sequence_id: 1,
            explicit_id: None,
            timestamp: later,
            severity: Severity::Error,
            process_id: "<0.2.0>".to_string(),
            message: "Later error".to_string(),
            message_lowercased: "later error".to_string(),
            subsystem_id: None,
            labels: LogEntryLabels::empty(),
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
    ];

    NodeLogEntry::insert_parsed_entries(&db, &entries, "test-node").unwrap();

    let ctx = QueryContext::from(QueryPreset::ErrorsOrCrashes).since(base_time);
    let results = NodeLogEntry::query(&db, &ctx).unwrap();

    assert_eq!(results.len(), 1);
    assert!(results[0].timestamp >= base_time);
}

#[test]
fn test_query_with_preset_and_to_time_filter() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let db = create_database(&db_path).unwrap();

    let base_time = Utc::now();
    let earlier = base_time - chrono::Duration::hours(2);
    let later = base_time + chrono::Duration::hours(2);

    let entries = vec![
        ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: earlier,
            severity: Severity::Error,
            process_id: "<0.1.0>".to_string(),
            message: "Earlier error".to_string(),
            message_lowercased: "earlier error".to_string(),
            subsystem_id: None,
            labels: LogEntryLabels::empty(),
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
        ParsedLogEntry {
            sequence_id: 1,
            explicit_id: None,
            timestamp: later,
            severity: Severity::Error,
            process_id: "<0.2.0>".to_string(),
            message: "Later error".to_string(),
            message_lowercased: "later error".to_string(),
            subsystem_id: None,
            labels: LogEntryLabels::empty(),
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
    ];

    NodeLogEntry::insert_parsed_entries(&db, &entries, "test-node").unwrap();

    let ctx = QueryContext::from(QueryPreset::ErrorsOrCrashes).to(base_time);
    let results = NodeLogEntry::query(&db, &ctx).unwrap();

    assert_eq!(results.len(), 1);
    assert!(results[0].timestamp <= base_time);
}

#[test]
fn test_query_with_preset_no_matches() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let db = create_database(&db_path).unwrap();

    let entries = vec![
        ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Info,
            process_id: "<0.1.0>".to_string(),
            message: "Normal info message".to_string(),
            message_lowercased: "normal info message".to_string(),
            subsystem_id: None,
            labels: LogEntryLabels::empty(),
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
        ParsedLogEntry {
            sequence_id: 1,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Warning,
            process_id: "<0.2.0>".to_string(),
            message: "Warning without crash labels".to_string(),
            message_lowercased: "warning without crash labels".to_string(),
            subsystem_id: None,
            labels: LogEntryLabels::empty(),
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
    ];

    NodeLogEntry::insert_parsed_entries(&db, &entries, "test-node").unwrap();

    let ctx = QueryContext::from(QueryPreset::ErrorsOrCrashes);
    let results = NodeLogEntry::query(&db, &ctx).unwrap();

    assert_eq!(results.len(), 0);
}

#[test]
fn test_query_with_preset_limit() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let db = create_database(&db_path).unwrap();

    let entries: Vec<ParsedLogEntry> = (0..10)
        .map(|i| ParsedLogEntry {
            sequence_id: i,
            explicit_id: None,
            timestamp: Utc::now() + chrono::Duration::seconds(i as i64),
            severity: Severity::Error,
            process_id: format!("<0.{}.0>", i),
            message: format!("Error message {}", i),
            message_lowercased: format!("error message {}", i),
            subsystem_id: None,
            labels: LogEntryLabels::empty(),
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        })
        .collect();

    NodeLogEntry::insert_parsed_entries(&db, &entries, "test-node").unwrap();

    let ctx = QueryContext::from(QueryPreset::ErrorsOrCrashes).limit(5);
    let results = NodeLogEntry::query(&db, &ctx).unwrap();

    assert_eq!(results.len(), 5);
}
