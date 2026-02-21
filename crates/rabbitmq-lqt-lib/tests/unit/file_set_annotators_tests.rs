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
use rabbitmq_lqt_lib::Severity;
use rabbitmq_lqt_lib::entry_metadata::labels::LogEntryLabels;
use rabbitmq_lqt_lib::file_set_metadata::extract_file_metadata;
use rabbitmq_lqt_lib::parser::ParsedLogEntry;
use std::slice;

fn create_test_entry(
    message: &str,
    subsystem_id: Option<i16>,
    labels: LogEntryLabels,
) -> ParsedLogEntry {
    ParsedLogEntry {
        sequence_id: 1,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Info,
        process_id: "<0.208.0>".to_string(),
        message: message.to_string(),
        message_lowercased: message.to_lowercase(),
        subsystem_id,
        labels,
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    }
}

#[test]
fn test_extract_rabbitmq_version_from_startup_banner() {
    let entry = create_test_entry(
        "Starting RabbitMQ 4.2.0 on Erlang 27.3.4.3",
        None,
        LogEntryLabels::empty(),
    );

    let metadata = extract_file_metadata(&[entry], "/tmp/test.log".to_string(), "rabbit@node1", 1);

    assert_eq!(metadata.rabbitmq_versions, vec!["4.2.0"]);
    assert_eq!(metadata.erlang_versions, vec!["27.3.4.3"]);
}

#[test]
fn test_extract_erlang_version_from_startup_banner() {
    let entry = create_test_entry(
        "Starting RabbitMQ 3.13.0 on Erlang 26.2.1",
        None,
        LogEntryLabels::empty(),
    );

    let metadata = extract_file_metadata(&[entry], "/tmp/test.log".to_string(), "rabbit@node1", 1);

    assert_eq!(metadata.rabbitmq_versions, vec!["3.13.0"]);
    assert_eq!(metadata.erlang_versions, vec!["26.2.1"]);
}

#[test]
fn test_extract_tls_library() {
    let entry = create_test_entry("TLS/DTLS: OpenSSL 3.0.2", None, LogEntryLabels::empty());

    let metadata = extract_file_metadata(&[entry], "/tmp/test.log".to_string(), "rabbit@node1", 1);

    assert_eq!(metadata.tls_library, Some("OpenSSL".to_string()));
}

#[test]
fn test_extract_plugins_from_startup_complete() {
    let message = r#"Server startup complete; 4 plugins started
 * rabbitmq_management
 * rabbitmq_prometheus
 * rabbitmq_shovel
 * rabbitmq_federation"#;

    let entry = create_test_entry(message, None, LogEntryLabels::empty());

    let metadata = extract_file_metadata(&[entry], "/tmp/test.log".to_string(), "rabbit@node1", 1);

    assert_eq!(metadata.enabled_plugins.len(), 4);
    assert!(
        metadata
            .enabled_plugins
            .contains(&"rabbitmq_management".to_string())
    );
    assert!(
        metadata
            .enabled_plugins
            .contains(&"rabbitmq_prometheus".to_string())
    );
    assert!(
        metadata
            .enabled_plugins
            .contains(&"rabbitmq_shovel".to_string())
    );
    assert!(
        metadata
            .enabled_plugins
            .contains(&"rabbitmq_federation".to_string())
    );
}

#[test]
fn test_extract_nodes_from_provided_node_name() {
    let entry = create_test_entry("Test message", None, LogEntryLabels::empty());

    let metadata =
        extract_file_metadata(&[entry], "/tmp/test.log".to_string(), "rabbit@server1", 1);

    assert_eq!(metadata.nodes.len(), 1);
    assert_eq!(metadata.nodes[0], "rabbit@server1");
}

#[test]
fn test_aggregate_subsystems_from_entries() {
    let entry1 = create_test_entry("Test message 1", Some(1), LogEntryLabels::empty());
    let entry2 = create_test_entry("Test message 2", Some(2), LogEntryLabels::empty());
    let entry3 = create_test_entry("Test message 3", Some(1), LogEntryLabels::empty());

    let metadata = extract_file_metadata(
        &[entry1, entry2, entry3],
        "/tmp/test.log".to_string(),
        "rabbit@node1",
        3,
    );

    assert!(metadata.subsystems.len() >= 2);
}

#[test]
fn test_aggregate_labels_from_entries() {
    let entry1 = create_test_entry(
        "Test 1",
        None,
        LogEntryLabels::QUEUES | LogEntryLabels::RAFT,
    );
    let entry2 = create_test_entry(
        "Test 2",
        None,
        LogEntryLabels::ELECTIONS | LogEntryLabels::RAFT,
    );
    let entry3 = create_test_entry("Test 3", None, LogEntryLabels::SHOVELS);

    let metadata = extract_file_metadata(
        &[entry1, entry2, entry3],
        "/tmp/test.log".to_string(),
        "rabbit@node1",
        3,
    );

    assert!(metadata.labels.contains(&"queues".to_string()));
    assert!(metadata.labels.contains(&"raft".to_string()));
    assert!(metadata.labels.contains(&"elections".to_string()));
    assert!(metadata.labels.contains(&"shovels".to_string()));
}

#[test]
fn test_deduplicates_labels() {
    let entry1 = create_test_entry("Test 1", None, LogEntryLabels::QUEUES);
    let entry2 = create_test_entry("Test 2", None, LogEntryLabels::QUEUES);
    let entry3 = create_test_entry("Test 3", None, LogEntryLabels::QUEUES);

    let metadata = extract_file_metadata(
        &[entry1, entry2, entry3],
        "/tmp/test.log".to_string(),
        "rabbit@node1",
        3,
    );

    assert_eq!(
        metadata.labels.iter().filter(|l| *l == "queues").count(),
        1,
        "Labels should be deduplicated"
    );
}

#[test]
fn test_handles_missing_startup_banner() {
    let entry = create_test_entry("Regular log message", None, LogEntryLabels::empty());

    let metadata = extract_file_metadata(&[entry], "/tmp/test.log".to_string(), "rabbit@node1", 1);

    assert!(metadata.rabbitmq_versions.is_empty());
    assert!(metadata.erlang_versions.is_empty());
}

#[test]
fn test_handles_missing_tls_info() {
    let entry = create_test_entry("Regular log message", None, LogEntryLabels::empty());

    let metadata = extract_file_metadata(&[entry], "/tmp/test.log".to_string(), "rabbit@node1", 1);

    assert_eq!(metadata.tls_library, None);
}

#[test]
fn test_handles_empty_entries() {
    let metadata = extract_file_metadata(&[], "/tmp/test.log".to_string(), "rabbit@node1", 0);

    assert_eq!(metadata.nodes.len(), 1);
    assert_eq!(metadata.nodes[0], "rabbit@node1");

    assert_eq!(metadata.subsystems.len(), 0);
    assert_eq!(metadata.labels.len(), 0);
    assert_eq!(metadata.enabled_plugins.len(), 0);
}

#[test]
fn test_multiple_startup_banners_collects_all_versions() {
    let entry1 = create_test_entry(
        "Starting RabbitMQ 3.12.0 on Erlang 25.0",
        None,
        LogEntryLabels::empty(),
    );
    let entry2 = create_test_entry(
        "Starting RabbitMQ 4.2.0 on Erlang 27.3.4.3",
        None,
        LogEntryLabels::empty(),
    );

    let metadata = extract_file_metadata(
        &[entry1, entry2],
        "/tmp/test.log".to_string(),
        "rabbit@node1",
        2,
    );

    assert_eq!(metadata.rabbitmq_versions, vec!["3.12.0", "4.2.0"]);
    assert_eq!(metadata.erlang_versions, vec!["25.0", "27.3.4.3"]);
}

#[test]
fn test_extract_plugins_with_duplicates() {
    let message = r#"Server startup complete; 4 plugins started
 * rabbitmq_management
 * rabbitmq_prometheus
 * rabbitmq_management
 * rabbitmq_shovel"#;

    let entry = create_test_entry(message, None, LogEntryLabels::empty());

    let metadata = extract_file_metadata(&[entry], "/tmp/test.log".to_string(), "rabbit@node1", 1);

    assert_eq!(metadata.enabled_plugins.len(), 3);
    assert!(
        metadata
            .enabled_plugins
            .contains(&"rabbitmq_management".to_string())
    );
    assert!(
        metadata
            .enabled_plugins
            .contains(&"rabbitmq_prometheus".to_string())
    );
    assert!(
        metadata
            .enabled_plugins
            .contains(&"rabbitmq_shovel".to_string())
    );
}

#[test]
fn test_extract_zero_plugins() {
    let message = "Server startup complete; 0 plugins started";

    let entry = create_test_entry(message, None, LogEntryLabels::empty());

    let metadata = extract_file_metadata(&[entry], "/tmp/test.log".to_string(), "rabbit@node1", 1);

    assert_eq!(metadata.enabled_plugins.len(), 0);
}

#[test]
fn test_tls_library_extracts_name_only() {
    let entry = create_test_entry(
        "TLS/DTLS: OpenSSL 3.0.2 15 Mar 2022",
        None,
        LogEntryLabels::empty(),
    );

    let metadata = extract_file_metadata(&[entry], "/tmp/test.log".to_string(), "rabbit@node1", 1);

    assert_eq!(metadata.tls_library, Some("OpenSSL".to_string()));
}

#[test]
fn test_partial_startup_banner_only_rabbitmq() {
    let entry = create_test_entry("Starting RabbitMQ 4.2.0", None, LogEntryLabels::empty());

    let metadata = extract_file_metadata(&[entry], "/tmp/test.log".to_string(), "rabbit@node1", 1);

    assert!(metadata.rabbitmq_versions.is_empty());
    assert!(metadata.erlang_versions.is_empty());
}

#[test]
fn test_plugin_count_mismatch_still_extracts_plugins() {
    let message = r#"Server startup complete; 10 plugins started
 * rabbitmq_management
 * rabbitmq_prometheus"#;

    let entry = create_test_entry(message, None, LogEntryLabels::empty());

    let metadata = extract_file_metadata(&[entry], "/tmp/test.log".to_string(), "rabbit@node1", 1);

    assert_eq!(metadata.enabled_plugins.len(), 2);
    assert!(
        metadata
            .enabled_plugins
            .contains(&"rabbitmq_management".to_string())
    );
    assert!(
        metadata
            .enabled_plugins
            .contains(&"rabbitmq_prometheus".to_string())
    );
}

#[test]
fn test_extracts_oldest_and_newest_entry_timestamps() {
    let entry1 = create_test_entry("Message 1", None, LogEntryLabels::empty());
    let entry2 = create_test_entry("Message 2", None, LogEntryLabels::empty());
    let entry3 = create_test_entry("Message 3", None, LogEntryLabels::empty());

    let metadata = extract_file_metadata(
        &[entry1.clone(), entry2, entry3.clone()],
        "/tmp/test.log".to_string(),
        "rabbit@node1",
        3,
    );

    assert_eq!(metadata.oldest_entry_at, Some(entry1.timestamp));
    assert_eq!(metadata.most_recent_entry_at, Some(entry3.timestamp));
}

#[test]
fn test_empty_entries_has_no_timestamps() {
    let metadata = extract_file_metadata(&[], "/tmp/test.log".to_string(), "rabbit@node1", 0);

    assert_eq!(metadata.oldest_entry_at, None);
    assert_eq!(metadata.most_recent_entry_at, None);
}

#[test]
fn test_single_entry_has_same_timestamps() {
    let entry = create_test_entry("Message", None, LogEntryLabels::empty());

    let metadata = extract_file_metadata(
        slice::from_ref(&entry),
        "/tmp/test.log".to_string(),
        "rabbit@node1",
        1,
    );

    assert_eq!(metadata.oldest_entry_at, Some(entry.timestamp));
    assert_eq!(metadata.most_recent_entry_at, Some(entry.timestamp));
}

#[test]
fn test_extracts_total_lines_and_entries() {
    let entry1 = create_test_entry("Message 1", None, LogEntryLabels::empty());
    let entry2 = create_test_entry("Message 2", None, LogEntryLabels::empty());
    let entry3 = create_test_entry("Message 3", None, LogEntryLabels::empty());

    let metadata = extract_file_metadata(
        &[entry1, entry2, entry3],
        "/tmp/test.log".to_string(),
        "rabbit@node1",
        10,
    );

    assert_eq!(metadata.total_entries, 3);
    assert_eq!(metadata.total_lines, 10);
}

#[test]
fn test_extract_version_with_leading_whitespace() {
    let entry = create_test_entry(
        " Starting RabbitMQ 4.2.0 on Erlang 27.3.4.2 [jit]",
        None,
        LogEntryLabels::empty(),
    );

    let metadata = extract_file_metadata(&[entry], "/tmp/test.log".to_string(), "rabbit@node1", 1);

    assert_eq!(metadata.rabbitmq_versions, vec!["4.2.0"]);
    assert_eq!(metadata.erlang_versions, vec!["27.3.4.2 [jit]"]);
}

#[test]
fn test_empty_file_has_zero_counts() {
    let metadata = extract_file_metadata(&[], "/tmp/test.log".to_string(), "rabbit@node1", 0);

    assert_eq!(metadata.total_entries, 0);
    assert_eq!(metadata.total_lines, 0);
}

#[test]
fn test_multiline_entries_count_correctly() {
    let entry = create_test_entry("Line 1\nLine 2\nLine 3", None, LogEntryLabels::empty());

    let metadata = extract_file_metadata(&[entry], "/tmp/test.log".to_string(), "rabbit@node1", 5);

    assert_eq!(metadata.total_entries, 1);
    assert_eq!(metadata.total_lines, 5);
}
