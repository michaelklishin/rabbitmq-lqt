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
use rabbitmq_lqt_lib::entry_metadata::annotate_entry;
use rabbitmq_lqt_lib::entry_metadata::labels::LogEntryLabels;
use rabbitmq_lqt_lib::{
    NodeLogEntry, ParsedLogEntry, QueryContext, Severity, Subsystem, create_database,
    open_database, parse_log_file,
};
use std::fs;
use std::io::BufReader;
use tempfile::TempDir;

#[test]
fn test_create_database() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let db = create_database(&db_path).unwrap();

    let count = NodeLogEntry::count_all(&db).unwrap();
    assert_eq!(count, 0);
}

#[test]
fn test_insert_and_query() {
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
            message: "Test message 1".to_string(),
            message_lowercased: "Test message 1".to_lowercase(),
            subsystem_id: None,
            labels: LogEntryLabels::default(),
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
        ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Error,
            process_id: "<0.2.0>".to_string(),
            message: "Test message 2".to_string(),
            message_lowercased: "Test message 2".to_lowercase(),
            subsystem_id: None,
            labels: LogEntryLabels::default(),
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
    ];

    NodeLogEntry::insert_parsed_entries(&db, &entries, "test-node").unwrap();

    let count = NodeLogEntry::count_all(&db).unwrap();
    assert_eq!(count, 2);

    let ctx = QueryContext::default();
    let queried = NodeLogEntry::query(&db, &ctx).unwrap();
    assert_eq!(queried.len(), 2);
}

#[test]
fn test_query_with_severity_filter() {
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
            message: "Info message".to_string(),
            message_lowercased: "Info message".to_lowercase(),
            subsystem_id: None,
            labels: LogEntryLabels::default(),
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
        ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Error,
            process_id: "<0.2.0>".to_string(),
            message: "Error message".to_string(),
            message_lowercased: "Error message".to_lowercase(),
            subsystem_id: None,
            labels: LogEntryLabels::default(),
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
    ];

    NodeLogEntry::insert_parsed_entries(&db, &entries, "test-node").unwrap();

    let ctx = QueryContext::default().severity("error");
    let errors = NodeLogEntry::query(&db, &ctx).unwrap();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].severity, "error");
}

#[test]
fn test_query_with_limit() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let db = create_database(&db_path).unwrap();

    let entries: Vec<ParsedLogEntry> = (0..10)
        .map(|i| ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Info,
            process_id: format!("<0.{}.0>", i),
            message: format!("Message {}", i),
            message_lowercased: format!("Message {}", i).to_lowercase(),
            subsystem_id: None,
            labels: LogEntryLabels::default(),
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        })
        .collect();

    NodeLogEntry::insert_parsed_entries(&db, &entries, "test-node").unwrap();

    let ctx = QueryContext::default().limit(5);
    let limited = NodeLogEntry::query(&db, &ctx).unwrap();
    assert_eq!(limited.len(), 5);
}

#[test]
fn test_open_existing_database() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");

    {
        let db = create_database(&db_path).unwrap();
        let entries = vec![ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Info,
            process_id: "<0.1.0>".to_string(),
            message: "Test".to_string(),
            message_lowercased: "Test".to_lowercase(),
            subsystem_id: None,
            labels: LogEntryLabels::default(),
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        }];
        NodeLogEntry::insert_parsed_entries(&db, &entries, "test-node").unwrap();
    }

    let db = open_database(&db_path).unwrap();
    let count = NodeLogEntry::count_all(&db).unwrap();
    assert_eq!(count, 1);
}

#[test]
fn test_database_reconnection() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");

    create_database(&db_path).unwrap();

    let db = open_database(&db_path).unwrap();
    let count = NodeLogEntry::count_all(&db).unwrap();
    assert_eq!(count, 0);
}

#[test]
fn test_query_with_subsystem_filter() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let db = create_database(&db_path).unwrap();

    let entries = vec![
        ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Debug,
            process_id: "<0.246.0>".to_string(),
            message: "RabbitMQ metadata store: ra_log:init recovered".to_string(),
            message_lowercased: "RabbitMQ metadata store: ra_log:init recovered".to_lowercase(),
            subsystem_id: Some(Subsystem::MetadataStore.to_id()),
            labels: LogEntryLabels::default(),
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
        ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Debug,
            process_id: "<0.264.0>".to_string(),
            message: "Feature flags: controller standing by".to_string(),
            message_lowercased: "Feature flags: controller standing by".to_lowercase(),
            subsystem_id: Some(Subsystem::FeatureFlags.to_id()),
            labels: LogEntryLabels::default(),
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
        ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Info,
            process_id: "<0.208.0>".to_string(),
            message: "Regular log message".to_string(),
            message_lowercased: "Regular log message".to_lowercase(),
            subsystem_id: None,
            labels: LogEntryLabels::default(),
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
    ];

    NodeLogEntry::insert_parsed_entries(&db, &entries, "test-node").unwrap();

    let ctx = QueryContext::default().subsystem("metadata_store");
    let metadata_entries = NodeLogEntry::query(&db, &ctx).unwrap();
    assert_eq!(metadata_entries.len(), 1);
    assert_eq!(
        metadata_entries[0].subsystem_id,
        Some(Subsystem::MetadataStore.to_id())
    );

    let ctx = QueryContext::default().subsystem("feature_flags");
    let feature_entries = NodeLogEntry::query(&db, &ctx).unwrap();
    assert_eq!(feature_entries.len(), 1);
    assert_eq!(
        feature_entries[0].subsystem_id,
        Some(Subsystem::FeatureFlags.to_id())
    );

    let ctx = QueryContext::default();
    let all_entries = NodeLogEntry::query(&db, &ctx).unwrap();
    assert_eq!(all_entries.len(), 3);
}

#[test]
fn test_query_with_label_filter() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let db = create_database(&db_path).unwrap();

    let mut labels_crash = LogEntryLabels::default();
    labels_crash |= LogEntryLabels::ERL_PROCESS_CRASH;

    let mut labels_undef = LogEntryLabels::default();
    labels_undef |= LogEntryLabels::UNDEFINED_FN;

    let mut labels_both = LogEntryLabels::default();
    labels_both |= LogEntryLabels::ERL_PROCESS_CRASH;
    labels_both |= LogEntryLabels::UNDEFINED_FN;

    let entries = vec![
        ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Error,
            process_id: "<0.1.0>".to_string(),
            message: "Line 1\nLine 2 crasher: error\nLine 3".to_string(),
            message_lowercased: "Line 1\nLine 2 crasher: error\nLine 3".to_lowercase(),
            subsystem_id: None,
            labels: labels_crash,
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
        ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Error,
            process_id: "<0.2.0>".to_string(),
            message: "Error: :undef, module missing".to_string(),
            message_lowercased: "Error: :undef, module missing".to_lowercase(),
            subsystem_id: None,
            labels: labels_undef,
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
        ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Error,
            process_id: "<0.3.0>".to_string(),
            message: "Line 1\ncrasher: :undef, foo:bar/2\nLine 3".to_string(),
            message_lowercased: "Line 1\ncrasher: :undef, foo:bar/2\nLine 3".to_lowercase(),
            subsystem_id: None,
            labels: labels_both,
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
        ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Info,
            process_id: "<0.4.0>".to_string(),
            message: "Regular message".to_string(),
            message_lowercased: "Regular message".to_lowercase(),
            subsystem_id: None,
            labels: LogEntryLabels::default(),
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
    ];

    NodeLogEntry::insert_parsed_entries(&db, &entries, "test-node").unwrap();

    let ctx = QueryContext::default().add_label("erl_process_crash");
    let crash_entries = NodeLogEntry::query(&db, &ctx).unwrap();
    assert_eq!(crash_entries.len(), 2);

    let ctx = QueryContext::default().add_label("undefined_fn");
    let undef_entries = NodeLogEntry::query(&db, &ctx).unwrap();
    assert_eq!(undef_entries.len(), 2);

    let ctx = QueryContext::default()
        .add_label("erl_process_crash")
        .add_label("undefined_fn");
    let any_label_entries = NodeLogEntry::query(&db, &ctx).unwrap();
    assert_eq!(any_label_entries.len(), 3);

    let ctx = QueryContext::default();
    let all_entries = NodeLogEntry::query(&db, &ctx).unwrap();
    assert_eq!(all_entries.len(), 4);
}

#[test]
fn test_query_with_combined_filters() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let db = create_database(&db_path).unwrap();

    let mut labels_crash = LogEntryLabels::default();
    labels_crash |= LogEntryLabels::ERL_PROCESS_CRASH;

    let entries = vec![
        ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Error,
            process_id: "<0.1.0>".to_string(),
            message: "Crash in metadata store".to_string(),
            message_lowercased: "Crash in metadata store".to_lowercase(),
            subsystem_id: Some(Subsystem::MetadataStore.to_id()),
            labels: labels_crash,
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
        ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Error,
            process_id: "<0.2.0>".to_string(),
            message: "Crash in raft".to_string(),
            message_lowercased: "Crash in raft".to_lowercase(),
            subsystem_id: Some(Subsystem::Raft.to_id()),
            labels: labels_crash,
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
        ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Info,
            process_id: "<0.3.0>".to_string(),
            message: "Info in metadata store".to_string(),
            message_lowercased: "Info in metadata store".to_lowercase(),
            subsystem_id: Some(Subsystem::MetadataStore.to_id()),
            labels: LogEntryLabels::default(),
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
    ];

    NodeLogEntry::insert_parsed_entries(&db, &entries, "test-node").unwrap();

    let ctx = QueryContext::default()
        .subsystem("metadata_store")
        .add_label("erl_process_crash");
    let filtered_entries = NodeLogEntry::query(&db, &ctx).unwrap();
    assert_eq!(filtered_entries.len(), 1);
    assert_eq!(
        filtered_entries[0].subsystem_id,
        Some(Subsystem::MetadataStore.to_id())
    );

    let ctx = QueryContext::default()
        .severity("error")
        .add_label("erl_process_crash");
    let filtered_entries = NodeLogEntry::query(&db, &ctx).unwrap();
    assert_eq!(filtered_entries.len(), 2);
}

#[test]
fn test_query_with_has_doc_url_filter() {
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
            message: "Entry with doc URL".to_string(),
            message_lowercased: "Entry with doc URL".to_lowercase(),
            subsystem_id: None,
            labels: LogEntryLabels::default(),
            resolution_or_discussion_url_id: None,
            doc_url_id: Some(1),
        },
        ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Info,
            process_id: "<0.2.0>".to_string(),
            message: "Entry without doc URL".to_string(),
            message_lowercased: "Entry without doc URL".to_lowercase(),
            subsystem_id: None,
            labels: LogEntryLabels::default(),
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
        ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Info,
            process_id: "<0.3.0>".to_string(),
            message: "Another entry with doc URL".to_string(),
            message_lowercased: "Another entry with doc URL".to_lowercase(),
            subsystem_id: None,
            labels: LogEntryLabels::default(),
            resolution_or_discussion_url_id: None,
            doc_url_id: Some(2),
        },
    ];

    NodeLogEntry::insert_parsed_entries(&db, &entries, "test-node").unwrap();

    let ctx = QueryContext::default().has_doc_url(true);
    let results = NodeLogEntry::query(&db, &ctx).unwrap();
    assert_eq!(results.len(), 2);
    assert!(results.iter().all(|e| e.doc_url_id.is_some()));

    let ctx = QueryContext::default().has_doc_url(false);
    let results = NodeLogEntry::query(&db, &ctx).unwrap();
    assert_eq!(results.len(), 3);
}

#[test]
fn test_query_with_has_resolution_or_discussion_url_filter() {
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
            message: "Entry with issue URL".to_string(),
            message_lowercased: "Entry with issue URL".to_lowercase(),
            subsystem_id: None,
            labels: LogEntryLabels::default(),
            resolution_or_discussion_url_id: Some(123),
            doc_url_id: None,
        },
        ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Info,
            process_id: "<0.2.0>".to_string(),
            message: "Entry without issue URL".to_string(),
            message_lowercased: "Entry without issue URL".to_lowercase(),
            subsystem_id: None,
            labels: LogEntryLabels::default(),
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
    ];

    NodeLogEntry::insert_parsed_entries(&db, &entries, "test-node").unwrap();

    let ctx = QueryContext::default().has_resolution_or_discussion_url(true);
    let results = NodeLogEntry::query(&db, &ctx).unwrap();
    assert_eq!(results.len(), 1);
    assert!(results[0].resolution_or_discussion_url_id.is_some());

    let ctx = QueryContext::default().has_resolution_or_discussion_url(false);
    let results = NodeLogEntry::query(&db, &ctx).unwrap();
    assert_eq!(results.len(), 2);
}

#[test]
fn test_query_with_both_url_filters() {
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
            message: "Entry with both URLs".to_string(),
            message_lowercased: "Entry with both URLs".to_lowercase(),
            subsystem_id: None,
            labels: LogEntryLabels::default(),
            resolution_or_discussion_url_id: Some(100),
            doc_url_id: Some(3),
        },
        ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Info,
            process_id: "<0.2.0>".to_string(),
            message: "Entry with only doc URL".to_string(),
            message_lowercased: "Entry with only doc URL".to_lowercase(),
            subsystem_id: None,
            labels: LogEntryLabels::default(),
            resolution_or_discussion_url_id: None,
            doc_url_id: Some(3),
        },
        ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Info,
            process_id: "<0.3.0>".to_string(),
            message: "Entry with only issue URL".to_string(),
            message_lowercased: "Entry with only issue URL".to_lowercase(),
            subsystem_id: None,
            labels: LogEntryLabels::default(),
            resolution_or_discussion_url_id: Some(100),
            doc_url_id: None,
        },
        ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Info,
            process_id: "<0.4.0>".to_string(),
            message: "Entry with no URLs".to_string(),
            message_lowercased: "Entry with no URLs".to_lowercase(),
            subsystem_id: None,
            labels: LogEntryLabels::default(),
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
    ];

    NodeLogEntry::insert_parsed_entries(&db, &entries, "test-node").unwrap();

    let ctx = QueryContext::default()
        .has_doc_url(true)
        .has_resolution_or_discussion_url(true);
    let results = NodeLogEntry::query(&db, &ctx).unwrap();
    assert_eq!(results.len(), 1);
    assert!(results[0].doc_url_id.is_some());
    assert!(results[0].resolution_or_discussion_url_id.is_some());
}

#[test]
fn test_query_with_unlabelled_filter() {
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
            message: "Generic message with no labels".to_string(),
            message_lowercased: "generic message with no labels".to_string(),
            subsystem_id: None,
            labels: LogEntryLabels::UNLABELLED,
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
        ParsedLogEntry {
            sequence_id: 1,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Info,
            process_id: "<0.2.0>".to_string(),
            message: "User authenticated successfully by backend".to_string(),
            message_lowercased: "user authenticated successfully by backend".to_string(),
            subsystem_id: Some(Subsystem::AccessControl.to_id()),
            labels: LogEntryLabels::ACCESS_CONTROL,
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
        ParsedLogEntry {
            sequence_id: 2,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Info,
            process_id: "<0.3.0>".to_string(),
            message: "Another generic message".to_string(),
            message_lowercased: "another generic message".to_string(),
            subsystem_id: None,
            labels: LogEntryLabels::UNLABELLED,
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
    ];

    NodeLogEntry::insert_parsed_entries(&db, &entries, "test-node").unwrap();

    let ctx = QueryContext::default().add_label("unlabelled");
    let results = NodeLogEntry::query(&db, &ctx).unwrap();
    assert_eq!(results.len(), 2);
    assert!(results[0].get_labels().contains(LogEntryLabels::UNLABELLED));
    assert!(results[1].get_labels().contains(LogEntryLabels::UNLABELLED));
}

#[test]
fn test_query_excluding_unlabelled() {
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
            message: "Generic message".to_string(),
            message_lowercased: "generic message".to_string(),
            subsystem_id: None,
            labels: LogEntryLabels::UNLABELLED,
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
        ParsedLogEntry {
            sequence_id: 1,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Warning,
            process_id: "<0.2.0>".to_string(),
            message: "Pre-vote message".to_string(),
            message_lowercased: "pre-vote message".to_string(),
            subsystem_id: Some(Subsystem::Raft.to_id()),
            labels: LogEntryLabels::ELECTIONS | LogEntryLabels::RAFT,
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
    ];

    NodeLogEntry::insert_parsed_entries(&db, &entries, "test-node").unwrap();

    let ctx = QueryContext::default().add_label("elections");
    let results = NodeLogEntry::query(&db, &ctx).unwrap();
    assert_eq!(results.len(), 1);
    assert!(results[0].get_labels().contains(LogEntryLabels::ELECTIONS));
    assert!(!results[0].get_labels().contains(LogEntryLabels::UNLABELLED));
}

#[test]
fn test_end_to_end_unlabelled_annotation() {
    let temp_dir = TempDir::new().unwrap();
    let temp_log_file = temp_dir.path().join("test.log");
    let db_path = temp_dir.path().join("test.db");

    let log_contents = r#"2025-10-27 18:23:00.123456+00:00 [info] <0.1.0> Some generic log message without any specific subsystem or labels
2025-10-27 18:23:01.234567+00:00 [info] <0.2.0> User 'admin' authenticated successfully by backend internal
2025-10-27 18:23:02.345678+00:00 [info] <0.3.0> Another message that won't match any annotators
2025-10-27 18:23:03.456789+00:00 [info] <0.4.0> Deleting auto-delete queue 'my.queue' in vhost '/' because all consumers were removed
"#;

    fs::write(&temp_log_file, log_contents).unwrap();

    let file = fs::File::open(&temp_log_file).unwrap();
    let reader = BufReader::new(file);
    let parse_results = parse_log_file(reader).unwrap();
    assert_eq!(parse_results.entries.len(), 4, "Expected 4 log entries");

    let mut entries = parse_results.entries;
    for entry in &mut entries {
        annotate_entry(entry);
    }

    assert!(
        entries[0].labels.contains(LogEntryLabels::UNLABELLED),
        "First entry should have UNLABELLED label"
    );
    assert!(
        entries[1].labels.contains(LogEntryLabels::CONNECTIONS),
        "Second entry should have CONNECTIONS label"
    );
    assert!(
        entries[1].labels.contains(LogEntryLabels::ACCESS_CONTROL),
        "Second entry should have ACCESS_CONTROL label"
    );
    assert!(
        !entries[1].labels.contains(LogEntryLabels::UNLABELLED),
        "Second entry should NOT have UNLABELLED label"
    );
    assert!(
        entries[2].labels.contains(LogEntryLabels::UNLABELLED),
        "Third entry should have UNLABELLED label"
    );
    assert!(
        entries[3].labels.contains(LogEntryLabels::AUTO_DELETE),
        "Fourth entry should have AUTO_DELETE label"
    );
    assert!(
        !entries[3].labels.contains(LogEntryLabels::UNLABELLED),
        "Fourth entry should NOT have UNLABELLED label"
    );

    let db = create_database(&db_path).unwrap();
    NodeLogEntry::insert_parsed_entries(&db, &entries, "test-node").unwrap();

    let ctx = QueryContext::default().add_label("unlabelled");
    let results = NodeLogEntry::query(&db, &ctx).unwrap();
    assert_eq!(results.len(), 2, "Should find 2 unlabelled entries");
    assert!(results[0].get_labels().contains(LogEntryLabels::UNLABELLED));
    assert!(results[1].get_labels().contains(LogEntryLabels::UNLABELLED));

    let ctx = QueryContext::default().add_label("access_control");
    let results = NodeLogEntry::query(&db, &ctx).unwrap();
    assert_eq!(results.len(), 1, "Should find 1 access_control entry");
    assert!(!results[0].get_labels().contains(LogEntryLabels::UNLABELLED));
}

#[test]
fn test_bulk_insert_with_chunking() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let db = create_database(&db_path).unwrap();

    let base_time = Utc::now();
    let entry_count = 4500;
    let entries: Vec<ParsedLogEntry> = (0..entry_count)
        .map(|i| ParsedLogEntry {
            sequence_id: i,
            explicit_id: Some(i as i64 + 1),
            timestamp: base_time + chrono::Duration::seconds(i as i64),
            severity: if i % 3 == 0 {
                Severity::Info
            } else if i % 3 == 1 {
                Severity::Warning
            } else {
                Severity::Error
            },
            process_id: format!("<0.{}.0>", i % 100),
            message: format!("Test message {}", i),
            message_lowercased: format!("test message {}", i),
            subsystem_id: None,
            labels: LogEntryLabels::default(),
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        })
        .collect();

    NodeLogEntry::insert_parsed_entries(&db, &entries, "test-node").unwrap();

    let count = NodeLogEntry::count_all(&db).unwrap();
    assert_eq!(count, entry_count as u64);

    let ctx = QueryContext::default().severity("info");
    let results = NodeLogEntry::query(&db, &ctx).unwrap();
    assert_eq!(results.len(), 1500);

    let ctx = QueryContext::default().severity("warning");
    let results = NodeLogEntry::query(&db, &ctx).unwrap();
    assert_eq!(results.len(), 1500);

    let ctx = QueryContext::default().severity("error");
    let results = NodeLogEntry::query(&db, &ctx).unwrap();
    assert_eq!(results.len(), 1500);
}

#[test]
fn test_query_with_offset() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let db = create_database(&db_path).unwrap();

    let entries: Vec<ParsedLogEntry> = (0..10)
        .map(|i| ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Info,
            process_id: format!("<0.{}.0>", i),
            message: format!("Message {}", i),
            message_lowercased: format!("message {}", i),
            subsystem_id: None,
            labels: LogEntryLabels::default(),
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        })
        .collect();

    NodeLogEntry::insert_parsed_entries(&db, &entries, "test-node").unwrap();

    let ctx = QueryContext::default().offset(7);
    let results = NodeLogEntry::query(&db, &ctx).unwrap();
    assert_eq!(results.len(), 3);
}

#[test]
fn test_query_with_tail() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let db = create_database(&db_path).unwrap();

    let entries: Vec<ParsedLogEntry> = (0..10)
        .map(|i| ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Info,
            process_id: format!("<0.{}.0>", i),
            message: format!("Message {}", i),
            message_lowercased: format!("message {}", i),
            subsystem_id: None,
            labels: LogEntryLabels::default(),
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        })
        .collect();

    NodeLogEntry::insert_parsed_entries(&db, &entries, "test-node").unwrap();

    let ctx = QueryContext::default().tail(3);
    let results = NodeLogEntry::query(&db, &ctx).unwrap();
    assert_eq!(results.len(), 3);
}
