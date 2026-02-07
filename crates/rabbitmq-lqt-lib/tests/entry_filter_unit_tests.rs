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

mod test_helpers;

use chrono::{TimeZone, Utc};
use rabbitmq_lqt_lib::Severity;
use rabbitmq_lqt_lib::entry_metadata::annotate_entry;
use rabbitmq_lqt_lib::entry_metadata::labels::LogEntryLabels;
use rabbitmq_lqt_lib::entry_metadata::subsystems::Subsystem;
use rabbitmq_lqt_lib::filter::EntryFilter;
use test_helpers::{create_test_entry, create_test_entry_with_subsystem};

#[test]
fn test_empty_filter_matches_everything() {
    let filter = EntryFilter::default();
    assert!(filter.is_empty());

    let entry = create_test_entry(
        "Setting up a table for connection tracking on this node: tracked_connection",
        Severity::Info,
    );
    assert!(filter.matches(&entry));
}

#[test]
fn test_severity_filter_matches_at_or_above() {
    let filter = EntryFilter::default().severity("warning");
    assert!(!filter.is_empty());

    let warning_entry = create_test_entry(
        "accepting client connection: <0.1667.0> (127.0.0.1:52898 -> 127.0.0.1:5672)",
        Severity::Warning,
    );
    assert!(filter.matches(&warning_entry));

    let error_entry = create_test_entry(
        "Error on AMQP connection <0.1667.0> (127.0.0.1:52898 -> 127.0.0.1:5672): enotconn (socket is not connected)",
        Severity::Error,
    );
    assert!(filter.matches(&error_entry));

    let info_entry = create_test_entry(
        "accepting client connection: <0.1667.0> (127.0.0.1:52898 -> 127.0.0.1:5672)",
        Severity::Info,
    );
    assert!(!filter.matches(&info_entry));
}

#[test]
fn test_subsystem_filter() {
    let filter = EntryFilter::default().subsystem("connections");

    let conn_entry = create_test_entry_with_subsystem(
        "accepting client connection: <0.1667.0> (127.0.0.1:52898 -> 127.0.0.1:5672)",
        Severity::Info,
        Subsystem::Connections,
    );
    assert!(filter.matches(&conn_entry));

    let boot_entry = create_test_entry_with_subsystem(
        "Running boot step codec_correctness_check defined by app rabbit",
        Severity::Info,
        Subsystem::Boot,
    );
    assert!(!filter.matches(&boot_entry));

    let no_subsystem = create_test_entry(
        "accepting client connection: <0.1667.0> (127.0.0.1:52898 -> 127.0.0.1:5672)",
        Severity::Info,
    );
    assert!(!filter.matches(&no_subsystem));
}

#[test]
fn test_single_label_filter() {
    let filter = EntryFilter::default().add_label("connections");

    let mut conn_entry = create_test_entry(
        "Setting up a table for connection tracking on this node: tracked_connection",
        Severity::Info,
    );
    annotate_entry(&mut conn_entry);
    assert!(conn_entry.labels.contains(LogEntryLabels::CONNECTIONS));
    assert!(filter.matches(&conn_entry));

    let mut boot_entry = create_test_entry(
        "Running boot step codec_correctness_check defined by app rabbit",
        Severity::Info,
    );
    annotate_entry(&mut boot_entry);
    assert!(!filter.matches(&boot_entry));
}

#[test]
fn test_multiple_labels_or_logic() {
    let filter = EntryFilter::default()
        .add_label("connections")
        .add_label("shovels");

    let mut conn_entry = create_test_entry(
        "Setting up a table for connection tracking on this node: tracked_connection",
        Severity::Info,
    );
    annotate_entry(&mut conn_entry);
    assert!(conn_entry.labels.contains(LogEntryLabels::CONNECTIONS));
    assert!(filter.matches(&conn_entry));

    let mut shovel_entry = create_test_entry(
        "Shovel 'my-shovel' connected to destination",
        Severity::Info,
    );
    annotate_entry(&mut shovel_entry);
    assert!(shovel_entry.labels.contains(LogEntryLabels::SHOVELS));
    assert!(filter.matches(&shovel_entry));
}

#[test]
fn test_multiple_labels_and_logic() {
    let filter = EntryFilter::default()
        .add_label("connections")
        .add_label("access_control")
        .matching_all_labels(true);

    let mut entry_both =
        create_test_entry("User authenticated successfully by backend", Severity::Info);
    annotate_entry(&mut entry_both);
    assert!(entry_both.labels.contains(LogEntryLabels::CONNECTIONS));
    assert!(entry_both.labels.contains(LogEntryLabels::ACCESS_CONTROL));
    assert!(filter.matches(&entry_both));

    let mut conn_only = create_test_entry(
        "Setting up a table for connection tracking on this node: tracked_connection",
        Severity::Info,
    );
    annotate_entry(&mut conn_only);
    assert!(conn_only.labels.contains(LogEntryLabels::CONNECTIONS));
    assert!(!conn_only.labels.contains(LogEntryLabels::ACCESS_CONTROL));
    assert!(!filter.matches(&conn_only));
}

#[test]
fn test_time_range_since() {
    let cutoff = Utc.with_ymd_and_hms(2025, 6, 15, 12, 0, 0).unwrap();
    let filter = EntryFilter::default().since(cutoff);

    let mut after = create_test_entry(
        "accepting client connection: <0.1667.0> (127.0.0.1:52898 -> 127.0.0.1:5672)",
        Severity::Info,
    );
    after.timestamp = Utc.with_ymd_and_hms(2025, 6, 15, 13, 0, 0).unwrap();
    assert!(filter.matches(&after));

    let mut before = create_test_entry(
        "accepting client connection: <0.1667.0> (127.0.0.1:52898 -> 127.0.0.1:5672)",
        Severity::Info,
    );
    before.timestamp = Utc.with_ymd_and_hms(2025, 6, 15, 11, 0, 0).unwrap();
    assert!(!filter.matches(&before));
}

#[test]
fn test_time_range_to() {
    let cutoff = Utc.with_ymd_and_hms(2025, 6, 15, 12, 0, 0).unwrap();
    let filter = EntryFilter::default().to(cutoff);

    let mut before = create_test_entry(
        "accepting client connection: <0.1667.0> (127.0.0.1:52898 -> 127.0.0.1:5672)",
        Severity::Info,
    );
    before.timestamp = Utc.with_ymd_and_hms(2025, 6, 15, 11, 0, 0).unwrap();
    assert!(filter.matches(&before));

    let mut after = create_test_entry(
        "accepting client connection: <0.1667.0> (127.0.0.1:52898 -> 127.0.0.1:5672)",
        Severity::Info,
    );
    after.timestamp = Utc.with_ymd_and_hms(2025, 6, 15, 13, 0, 0).unwrap();
    assert!(!filter.matches(&after));
}

#[test]
fn test_time_range_since_and_to() {
    let since = Utc.with_ymd_and_hms(2025, 6, 15, 10, 0, 0).unwrap();
    let to = Utc.with_ymd_and_hms(2025, 6, 15, 14, 0, 0).unwrap();
    let filter = EntryFilter::default().since(since).to(to);

    let mut within = create_test_entry(
        "accepting client connection: <0.1667.0> (127.0.0.1:52898 -> 127.0.0.1:5672)",
        Severity::Info,
    );
    within.timestamp = Utc.with_ymd_and_hms(2025, 6, 15, 12, 0, 0).unwrap();
    assert!(filter.matches(&within));

    let mut before = create_test_entry(
        "accepting client connection: <0.1667.0> (127.0.0.1:52898 -> 127.0.0.1:5672)",
        Severity::Info,
    );
    before.timestamp = Utc.with_ymd_and_hms(2025, 6, 15, 9, 0, 0).unwrap();
    assert!(!filter.matches(&before));

    let mut after = create_test_entry(
        "accepting client connection: <0.1667.0> (127.0.0.1:52898 -> 127.0.0.1:5672)",
        Severity::Info,
    );
    after.timestamp = Utc.with_ymd_and_hms(2025, 6, 15, 15, 0, 0).unwrap();
    assert!(!filter.matches(&after));
}

#[test]
fn test_erlang_pid_filter() {
    let filter = EntryFilter::default().erlang_pid("<0.208.0>");

    let entry = create_test_entry(
        "accepting client connection: <0.1667.0> (127.0.0.1:52898 -> 127.0.0.1:5672)",
        Severity::Info,
    );
    // test_helpers creates entries with PID "<0.208.0>"
    assert!(filter.matches(&entry));

    let mut other_pid = create_test_entry(
        "accepting client connection: <0.1667.0> (127.0.0.1:52898 -> 127.0.0.1:5672)",
        Severity::Info,
    );
    other_pid.process_id = "<0.999.0>".to_string();
    assert!(!filter.matches(&other_pid));
}

#[test]
fn test_has_doc_url_filter() {
    let filter = EntryFilter::default().has_doc_url(true);

    let mut with_doc = create_test_entry(
        "accepting client connection: <0.1667.0> (127.0.0.1:52898 -> 127.0.0.1:5672)",
        Severity::Info,
    );
    with_doc.doc_url_id = Some(1);
    assert!(filter.matches(&with_doc));

    let without_doc = create_test_entry(
        "accepting client connection: <0.1667.0> (127.0.0.1:52898 -> 127.0.0.1:5672)",
        Severity::Info,
    );
    assert!(!filter.matches(&without_doc));
}

#[test]
fn test_has_resolution_or_discussion_url_filter() {
    let filter = EntryFilter::default().has_resolution_or_discussion_url(true);

    let mut with_url = create_test_entry(
        "accepting client connection: <0.1667.0> (127.0.0.1:52898 -> 127.0.0.1:5672)",
        Severity::Info,
    );
    with_url.resolution_or_discussion_url_id = Some(1);
    assert!(filter.matches(&with_url));

    let without_url = create_test_entry(
        "accepting client connection: <0.1667.0> (127.0.0.1:52898 -> 127.0.0.1:5672)",
        Severity::Info,
    );
    assert!(!filter.matches(&without_url));
}

#[test]
fn test_combined_severity_and_subsystem_filter() {
    let filter = EntryFilter::default()
        .severity("warning")
        .subsystem("connections");

    let matching = create_test_entry_with_subsystem(
        "Error on AMQP connection <0.1667.0> (127.0.0.1:52898 -> 127.0.0.1:5672): enotconn (socket is not connected)",
        Severity::Warning,
        Subsystem::Connections,
    );
    assert!(filter.matches(&matching));

    let wrong_severity = create_test_entry_with_subsystem(
        "accepting client connection: <0.1667.0> (127.0.0.1:52898 -> 127.0.0.1:5672)",
        Severity::Info,
        Subsystem::Connections,
    );
    assert!(!filter.matches(&wrong_severity));

    let wrong_subsystem = create_test_entry_with_subsystem(
        "Running boot step codec_correctness_check defined by app rabbit",
        Severity::Warning,
        Subsystem::Boot,
    );
    assert!(!filter.matches(&wrong_subsystem));
}

#[test]
fn test_combined_severity_and_label_filter() {
    let filter = EntryFilter::default()
        .severity("warning")
        .add_label("connections");

    let mut matching = create_test_entry(
        "Setting up a table for connection tracking on this node: tracked_connection",
        Severity::Warning,
    );
    annotate_entry(&mut matching);
    assert!(matching.labels.contains(LogEntryLabels::CONNECTIONS));
    assert!(filter.matches(&matching));

    let mut wrong_severity = create_test_entry(
        "Setting up a table for connection tracking on this node: tracked_connection",
        Severity::Info,
    );
    annotate_entry(&mut wrong_severity);
    assert!(!filter.matches(&wrong_severity));
}

#[test]
fn test_filter_batch() {
    let filter = EntryFilter::default().severity("warning");

    let entries = vec![
        create_test_entry(
            "accepting client connection: <0.1667.0> (127.0.0.1:52898 -> 127.0.0.1:5672)",
            Severity::Info,
        ),
        create_test_entry(
            "Error on AMQP connection <0.1667.0> (127.0.0.1:52898 -> 127.0.0.1:5672): enotconn (socket is not connected)",
            Severity::Warning,
        ),
        create_test_entry(
            "Running boot step codec_correctness_check defined by app rabbit",
            Severity::Error,
        ),
    ];

    let filtered = filter.filter(&entries);
    assert_eq!(filtered.len(), 2);
    assert_eq!(filtered[0].severity, Severity::Warning);
    assert_eq!(filtered[1].severity, Severity::Error);
}

#[test]
fn test_is_empty_with_various_filters() {
    assert!(EntryFilter::default().is_empty());
    assert!(!EntryFilter::default().severity("warning").is_empty());
    assert!(!EntryFilter::default().subsystem("connections").is_empty());
    assert!(!EntryFilter::default().add_label("connections").is_empty());
    assert!(!EntryFilter::default().erlang_pid("<0.208.0>").is_empty());
    assert!(!EntryFilter::default().since(Utc::now()).is_empty());
    assert!(!EntryFilter::default().to(Utc::now()).is_empty());
    assert!(
        !EntryFilter::default()
            .has_resolution_or_discussion_url(true)
            .is_empty()
    );
    assert!(!EntryFilter::default().has_doc_url(true).is_empty());
}

#[test]
fn test_unlabelled_filter() {
    let filter = EntryFilter::default().add_label("unlabelled");

    let mut unlabelled_entry = create_test_entry(
        "Applying MFA: M = rabbit_db, F = init, A = []",
        Severity::Info,
    );
    annotate_entry(&mut unlabelled_entry);
    assert!(unlabelled_entry.labels.contains(LogEntryLabels::UNLABELLED));
    assert!(filter.matches(&unlabelled_entry));

    let mut labelled_entry = create_test_entry(
        "Setting up a table for connection tracking on this node: tracked_connection",
        Severity::Info,
    );
    annotate_entry(&mut labelled_entry);
    assert!(!labelled_entry.labels.contains(LogEntryLabels::UNLABELLED));
    assert!(!filter.matches(&labelled_entry));
}

#[test]
fn test_filter_returns_empty_when_nothing_matches() {
    let filter = EntryFilter::default().severity("error");

    let entries = vec![
        create_test_entry(
            "Setting up a table for connection tracking on this node: tracked_connection",
            Severity::Info,
        ),
        create_test_entry(
            "Running boot step codec_correctness_check defined by app rabbit",
            Severity::Debug,
        ),
    ];

    let filtered = filter.filter(&entries);
    assert!(filtered.is_empty());
}
