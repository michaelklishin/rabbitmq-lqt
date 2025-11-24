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
use rlqt_lib::entry_metadata::labels::LogEntryLabels;
use rlqt_lib::{NodeLogEntry, ParsedLogEntry, QueryContext, Severity, create_database};
use tempfile::NamedTempFile;

#[tokio::test]
async fn test_query_with_multiple_labels_or_logic() {
    let temp_db = NamedTempFile::new().unwrap();
    let db = create_database(temp_db.path()).await.unwrap();

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
            message: "Crash only".to_string(),
            message_lowercased: "Crash only".to_lowercase(),
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
            message: "Undef only".to_string(),
            message_lowercased: "Undef only".to_lowercase(),
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
            message: "Both labels".to_string(),
            message_lowercased: "Both labels".to_lowercase(),
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
            message: "No labels".to_string(),
            message_lowercased: "No labels".to_lowercase(),
            subsystem_id: None,
            labels: LogEntryLabels::default(),
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
    ];

    NodeLogEntry::insert_parsed_entries(&db, &entries, "test-node")
        .await
        .unwrap();

    let ctx = QueryContext::default()
        .add_label("erl_process_crash")
        .add_label("undefined_fn")
        .matching_all_labels(false);
    let results = NodeLogEntry::query(&db, &ctx).await.unwrap();
    assert_eq!(results.len(), 3);
}

#[tokio::test]
async fn test_query_with_multiple_labels_and_logic() {
    let temp_db = NamedTempFile::new().unwrap();
    let db = create_database(temp_db.path()).await.unwrap();

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
            message: "Crash only".to_string(),
            message_lowercased: "Crash only".to_lowercase(),
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
            message: "Undef only".to_string(),
            message_lowercased: "Undef only".to_lowercase(),
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
            message: "Both labels".to_string(),
            message_lowercased: "Both labels".to_lowercase(),
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
            message: "No labels".to_string(),
            message_lowercased: "No labels".to_lowercase(),
            subsystem_id: None,
            labels: LogEntryLabels::default(),
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
    ];

    NodeLogEntry::insert_parsed_entries(&db, &entries, "test-node")
        .await
        .unwrap();

    let ctx = QueryContext::default()
        .add_label("erl_process_crash")
        .add_label("undefined_fn")
        .matching_all_labels(true);
    let results = NodeLogEntry::query(&db, &ctx).await.unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].message, "Both labels");
}

#[tokio::test]
async fn test_query_with_three_labels_and_logic() {
    let temp_db = NamedTempFile::new().unwrap();
    let db = create_database(temp_db.path()).await.unwrap();

    let mut labels_two = LogEntryLabels::default();
    labels_two |= LogEntryLabels::RAFT;
    labels_two |= LogEntryLabels::ELECTIONS;

    let mut labels_three = LogEntryLabels::default();
    labels_three |= LogEntryLabels::RAFT;
    labels_three |= LogEntryLabels::ELECTIONS;
    labels_three |= LogEntryLabels::PROCESS_STOPS;

    let entries = vec![
        ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Info,
            process_id: "<0.1.0>".to_string(),
            message: "Raft only".to_string(),
            message_lowercased: "Raft only".to_lowercase(),
            subsystem_id: None,
            labels: {
                let mut l = LogEntryLabels::default();
                l |= LogEntryLabels::RAFT;
                l
            },
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
        ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Info,
            process_id: "<0.2.0>".to_string(),
            message: "Raft and elections".to_string(),
            message_lowercased: "Raft and elections".to_lowercase(),
            subsystem_id: None,
            labels: labels_two,
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
        ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Info,
            process_id: "<0.3.0>".to_string(),
            message: "All three labels".to_string(),
            message_lowercased: "All three labels".to_lowercase(),
            subsystem_id: None,
            labels: labels_three,
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
    ];

    NodeLogEntry::insert_parsed_entries(&db, &entries, "test-node")
        .await
        .unwrap();

    let ctx = QueryContext::default()
        .add_label("raft")
        .add_label("elections")
        .add_label("process_stops")
        .matching_all_labels(true);
    let results = NodeLogEntry::query(&db, &ctx).await.unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].message, "All three labels");
}

#[tokio::test]
async fn test_query_with_three_labels_or_logic() {
    let temp_db = NamedTempFile::new().unwrap();
    let db = create_database(temp_db.path()).await.unwrap();

    let mut labels_two = LogEntryLabels::default();
    labels_two |= LogEntryLabels::RAFT;
    labels_two |= LogEntryLabels::ELECTIONS;

    let mut labels_three = LogEntryLabels::default();
    labels_three |= LogEntryLabels::RAFT;
    labels_three |= LogEntryLabels::ELECTIONS;
    labels_three |= LogEntryLabels::PROCESS_STOPS;

    let entries = vec![
        ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Info,
            process_id: "<0.1.0>".to_string(),
            message: "Raft only".to_string(),
            message_lowercased: "Raft only".to_lowercase(),
            subsystem_id: None,
            labels: {
                let mut l = LogEntryLabels::default();
                l |= LogEntryLabels::RAFT;
                l
            },
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
        ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Info,
            process_id: "<0.2.0>".to_string(),
            message: "Raft and elections".to_string(),
            message_lowercased: "Raft and elections".to_lowercase(),
            subsystem_id: None,
            labels: labels_two,
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
        ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Info,
            process_id: "<0.3.0>".to_string(),
            message: "All three labels".to_string(),
            message_lowercased: "All three labels".to_lowercase(),
            subsystem_id: None,
            labels: labels_three,
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
        ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Info,
            process_id: "<0.4.0>".to_string(),
            message: "No matching labels".to_string(),
            message_lowercased: "No matching labels".to_lowercase(),
            subsystem_id: None,
            labels: LogEntryLabels::default(),
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
    ];

    NodeLogEntry::insert_parsed_entries(&db, &entries, "test-node")
        .await
        .unwrap();

    let ctx = QueryContext::default()
        .add_label("raft")
        .add_label("elections")
        .add_label("process_stops")
        .matching_all_labels(false);
    let results = NodeLogEntry::query(&db, &ctx).await.unwrap();
    assert_eq!(results.len(), 3);
}

#[tokio::test]
async fn test_single_label_unaffected_by_matching_all_labels() {
    let temp_db = NamedTempFile::new().unwrap();
    let db = create_database(temp_db.path()).await.unwrap();

    let mut labels_crash = LogEntryLabels::default();
    labels_crash |= LogEntryLabels::ERL_PROCESS_CRASH;

    let entries = vec![
        ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Error,
            process_id: "<0.1.0>".to_string(),
            message: "Has crash label".to_string(),
            message_lowercased: "Has crash label".to_lowercase(),
            subsystem_id: None,
            labels: labels_crash,
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
        ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Info,
            process_id: "<0.2.0>".to_string(),
            message: "No crash label".to_string(),
            message_lowercased: "No crash label".to_lowercase(),
            subsystem_id: None,
            labels: LogEntryLabels::default(),
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
    ];

    NodeLogEntry::insert_parsed_entries(&db, &entries, "test-node")
        .await
        .unwrap();

    let ctx_or = QueryContext::default()
        .add_label("erl_process_crash")
        .matching_all_labels(false);
    let results_or = NodeLogEntry::query(&db, &ctx_or).await.unwrap();

    let ctx_and = QueryContext::default()
        .add_label("erl_process_crash")
        .matching_all_labels(true);
    let results_and = NodeLogEntry::query(&db, &ctx_and).await.unwrap();

    assert_eq!(results_or.len(), 1);
    assert_eq!(results_and.len(), 1);
    assert_eq!(results_or[0].message, results_and[0].message);
}

#[tokio::test]
async fn test_and_logic_with_no_matching_entries() {
    let temp_db = NamedTempFile::new().unwrap();
    let db = create_database(temp_db.path()).await.unwrap();

    let mut labels_crash = LogEntryLabels::default();
    labels_crash |= LogEntryLabels::ERL_PROCESS_CRASH;

    let mut labels_undef = LogEntryLabels::default();
    labels_undef |= LogEntryLabels::UNDEFINED_FN;

    let entries = vec![
        ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Error,
            process_id: "<0.1.0>".to_string(),
            message: "Crash only".to_string(),
            message_lowercased: "Crash only".to_lowercase(),
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
            message: "Undef only".to_string(),
            message_lowercased: "Undef only".to_lowercase(),
            subsystem_id: None,
            labels: labels_undef,
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
    ];

    NodeLogEntry::insert_parsed_entries(&db, &entries, "test-node")
        .await
        .unwrap();

    let ctx = QueryContext::default()
        .add_label("erl_process_crash")
        .add_label("undefined_fn")
        .matching_all_labels(true);
    let results = NodeLogEntry::query(&db, &ctx).await.unwrap();
    assert_eq!(results.len(), 0);
}

#[tokio::test]
async fn test_and_logic_combined_with_other_filters() {
    let temp_db = NamedTempFile::new().unwrap();
    let db = create_database(temp_db.path()).await.unwrap();

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
            message: "Error with both labels".to_string(),
            message_lowercased: "Error with both labels".to_lowercase(),
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
            process_id: "<0.2.0>".to_string(),
            message: "Info with both labels".to_string(),
            message_lowercased: "Info with both labels".to_lowercase(),
            subsystem_id: None,
            labels: labels_both,
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
        ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Error,
            process_id: "<0.3.0>".to_string(),
            message: "Error without labels".to_string(),
            message_lowercased: "Error without labels".to_lowercase(),
            subsystem_id: None,
            labels: LogEntryLabels::default(),
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
    ];

    NodeLogEntry::insert_parsed_entries(&db, &entries, "test-node")
        .await
        .unwrap();

    let ctx = QueryContext::default()
        .severity("error")
        .add_label("erl_process_crash")
        .add_label("undefined_fn")
        .matching_all_labels(true);
    let results = NodeLogEntry::query(&db, &ctx).await.unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].message, "Error with both labels");
}

#[tokio::test]
async fn test_new_labels_with_and_logic() {
    let temp_db = NamedTempFile::new().unwrap();
    let db = create_database(temp_db.path()).await.unwrap();

    let mut labels_queues = LogEntryLabels::default();
    labels_queues |= LogEntryLabels::QUEUES;

    let mut labels_delete = LogEntryLabels::default();
    labels_delete |= LogEntryLabels::DELETE;

    let mut labels_both = LogEntryLabels::default();
    labels_both |= LogEntryLabels::QUEUES;
    labels_both |= LogEntryLabels::DELETE;

    let entries = vec![
        ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Info,
            process_id: "<0.1.0>".to_string(),
            message: "Queue deletion event".to_string(),
            message_lowercased: "Queue deletion event".to_lowercase(),
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
            process_id: "<0.2.0>".to_string(),
            message: "Queue event only".to_string(),
            message_lowercased: "Queue event only".to_lowercase(),
            subsystem_id: None,
            labels: labels_queues,
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
        ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp: Utc::now(),
            severity: Severity::Info,
            process_id: "<0.3.0>".to_string(),
            message: "Delete event only".to_string(),
            message_lowercased: "Delete event only".to_lowercase(),
            subsystem_id: None,
            labels: labels_delete,
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
    ];

    NodeLogEntry::insert_parsed_entries(&db, &entries, "test-node")
        .await
        .unwrap();

    let ctx = QueryContext::default()
        .add_label("queues")
        .add_label("delete")
        .matching_all_labels(true);
    let results = NodeLogEntry::query(&db, &ctx).await.unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].message, "Queue deletion event");
}
