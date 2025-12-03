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

use rlqt_lib::Severity;
use rlqt_lib::entry_metadata::label_annotators::annotate_labels;
use rlqt_lib::entry_metadata::labels::LogEntryLabels;
use test_helpers::create_test_entry;

#[test]
fn test_federation_queue() {
    let entry = create_test_entry("Started federation queue 'my-queue'", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::FEDERATION));
}

#[test]
fn test_disconnecting_from_queue() {
    let entry = create_test_entry("Disconnecting from queue 'test-queue'", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::FEDERATION));
}

#[test]
fn test_federation_upstream_component() {
    let entry = create_test_entry(
        "Configuration changed for component 'federation-upstream'",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::FEDERATION));
}

#[test]
fn test_federation_upstream_component_case_insensitive() {
    let entry = create_test_entry(
        "CONFIGURATION FOR COMPONENT 'FEDERATION-UPSTREAM'",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::FEDERATION));
}

#[test]
fn test_no_match_unrelated() {
    let entry = create_test_entry("Queue created successfully", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(!labels.contains(LogEntryLabels::FEDERATION));
}

#[test]
fn test_both_federation_and_queue_federation_labels() {
    let entry = create_test_entry("federation queue started", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::FEDERATION));
    assert!(labels.contains(LogEntryLabels::QUEUE_FEDERATION));
}

#[test]
fn test_starting_pg_scope_queue_federation() {
    let entry = create_test_entry(
        "Starting pg scope rabbitmq_queue_federation_pg_scope",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::FEDERATION));
}

#[test]
fn test_starting_pg_scope_exchange_federation() {
    let entry = create_test_entry(
        "Starting pg scope rabbitmq_exchange_federation_pg_scope",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::FEDERATION));
}

#[test]
fn test_stopping_pg_scope_queue_federation() {
    let entry = create_test_entry(
        "Stopping pg scope rabbitmq_queue_federation_pg_scope",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::FEDERATION));
}

#[test]
fn test_stopping_pg_scope_exchange_federation() {
    let entry = create_test_entry(
        "Stopping pg scope rabbitmq_exchange_federation_pg_scope",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::FEDERATION));
}
