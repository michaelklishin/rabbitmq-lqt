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

use crate::test_helpers::create_test_entry;
use rabbitmq_lqt_lib::Severity;
use rabbitmq_lqt_lib::entry_metadata::Annotator;
use rabbitmq_lqt_lib::entry_metadata::label_annotators::{LabelAnnotator, VirtualHostsAnnotator};
use rabbitmq_lqt_lib::entry_metadata::labels::LogEntryLabels;

#[test]
fn test_virtual_hosts_annotator_matches_pattern() {
    let entry = create_test_entry("Recovering 10 queues of type quorum", Severity::Info);
    let annotator = VirtualHostsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_virtual_hosts_annotator_matches_deleting_vhost() {
    let entry = create_test_entry("Deleting vhost 'production'", Severity::Info);
    let annotator = VirtualHostsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_virtual_hosts_annotator_matches_starting_message_stores() {
    let entry = create_test_entry("Starting message stores for vhost '/'", Severity::Info);
    let annotator = VirtualHostsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_virtual_hosts_annotator_matches_started_message_store() {
    let entry = create_test_entry("Started message store of type transient", Severity::Info);
    let annotator = VirtualHostsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_virtual_hosts_annotator_matches_message_store_directory() {
    let entry = create_test_entry(
        "Message store for directory /var/lib/rabbitmq",
        Severity::Info,
    );
    let annotator = VirtualHostsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_virtual_hosts_annotator_matches_adding_vhost() {
    let entry = create_test_entry("Adding vhost 'development'", Severity::Info);
    let annotator = VirtualHostsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_virtual_hosts_annotator_matches_default_queue_type() {
    let entry = create_test_entry("Default queue type for vhost '/' is quorum", Severity::Info);
    let annotator = VirtualHostsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_virtual_hosts_annotator_matches_setting_segment_entry_count() {
    let entry = create_test_entry(
        "Setting segment_entry_count for vhost '/' to 2048",
        Severity::Info,
    );
    let annotator = VirtualHostsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_virtual_hosts_annotator_matches_deletion_protection() {
    let entry = create_test_entry(
        "Enabling deletion protection for virtual host '/'",
        Severity::Info,
    );
    let annotator = VirtualHostsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_virtual_hosts_annotator_no_match() {
    let entry = create_test_entry("Unrelated message", Severity::Info);
    let annotator = VirtualHostsAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_virtual_hosts_annotator_annotates() {
    let annotator = VirtualHostsAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);
    assert!(labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
}
