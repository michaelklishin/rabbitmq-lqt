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
fn test_recovery_of_state_machine_version() {
    let entry = create_test_entry(
        "Starting recovery of state machine version 2",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::RAFT));
    assert!(!labels.contains(LogEntryLabels::ELECTIONS));
}

#[test]
fn test_recovering_state_machine_version() {
    let entry = create_test_entry("Now recovering state machine version 3", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::RAFT));
    assert!(!labels.contains(LogEntryLabels::ELECTIONS));
}

#[test]
fn test_scanning_for_cluster_changes() {
    let entry = create_test_entry("Raft scanning for cluster changes", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::RAFT));
    assert!(!labels.contains(LogEntryLabels::ELECTIONS));
}

#[test]
fn test_vote_granted_for_term() {
    let entry = create_test_entry(
        "RabbitMQ metadata store: vote granted for term 1 votes 1",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::RAFT));
    assert!(labels.contains(LogEntryLabels::ELECTIONS));
}

#[test]
fn test_vote_granted_for_term_case_insensitive() {
    let entry = create_test_entry("VOTE GRANTED FOR TERM 5", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::RAFT));
    assert!(labels.contains(LogEntryLabels::ELECTIONS));
}

#[test]
fn test_case_insensitive() {
    let entry = create_test_entry("SCANNING FOR CLUSTER CHANGES detected", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::RAFT));
}

#[test]
fn test_no_match_unrelated() {
    let entry = create_test_entry("Some other operation", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(!labels.contains(LogEntryLabels::RAFT));
}

#[test]
fn test_ra_system_recover() {
    let entry = create_test_entry(
        "ra_system_recover: no server recovery configured",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::RAFT));
}

#[test]
fn test_ra_server_exited() {
    let entry = create_test_entry(
        "Ra server {rabbitmq_metadata,rabbit@sunnyside} already exited",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::RAFT));
}

#[test]
fn test_wait_for_ra_server() {
    let entry = create_test_entry(
        "Wait for Ra server {rabbitmq_metadata,rabbit@sunnyside} to exit in store \"rabbitmq_metadata\"",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::RAFT));
}

#[test]
fn test_stopping_member() {
    let entry = create_test_entry(
        "Stopping member {rabbitmq_metadata,rabbit@sunnyside} in store \"rabbitmq_metadata\"",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::RAFT));
}

#[test]
fn test_metadata_store_caught_up() {
    let entry = create_test_entry(
        "local Khepri-based RabbitMQ metadata store member is caught up to the Raft cluster leader",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::RAFT));
}
