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
use rabbitmq_lqt_lib::entry_metadata::label_annotators::annotate_labels;
use rabbitmq_lqt_lib::entry_metadata::labels::LogEntryLabels;

#[test]
fn test_recovery_of_state_machine_version() {
    let entry = create_test_entry(
        "RabbitMQ metadata store: recovery of state machine version 1:1 from index 0 to 21 took 3ms",
        Severity::Debug,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::RAFT));
    assert!(!labels.contains(LogEntryLabels::ELECTIONS));
}

#[test]
fn test_recovering_state_machine_version() {
    let entry = create_test_entry(
        "RabbitMQ metadata store: recovering state machine version 0:1 from index 0 to 21",
        Severity::Debug,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::RAFT));
    assert!(!labels.contains(LogEntryLabels::ELECTIONS));
}

#[test]
fn test_scanning_for_cluster_changes() {
    let entry = create_test_entry(
        "RabbitMQ metadata store: scanning for cluster changes 1:0",
        Severity::Debug,
    );
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
fn test_queue_vote_granted_for_term() {
    let entry = create_test_entry(
        "queue 'qq.1' in vhost '/': vote granted for term 1 votes 1",
        Severity::Debug,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::RAFT));
    assert!(labels.contains(LogEntryLabels::ELECTIONS));
}

#[test]
fn test_queue_scanning_for_cluster_changes() {
    let entry = create_test_entry(
        "queue 'qq.1' in vhost '/': scanning for cluster changes 1:0",
        Severity::Debug,
    );
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

#[test]
fn test_starting_ra_systems() {
    let entry = create_test_entry("Starting Ra systems", Severity::Debug);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::RAFT));
}

#[test]
fn test_ra_systems_lowercase() {
    let entry = create_test_entry("ra systems initialized", Severity::Debug);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::RAFT));
}

#[test]
fn test_stopping_ra_systems() {
    let entry = create_test_entry("Stopping Ra systems", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::RAFT));
    assert!(labels.contains(LogEntryLabels::SHUTDOWN));
}

#[test]
fn test_trying_to_restart_local_ra_server() {
    let entry = create_test_entry(
        "Trying to restart local Ra server for store \"rabbitmq_metadata\" in Ra system \"coordination\"",
        Severity::Debug,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::KHEPRI));
    assert!(labels.contains(LogEntryLabels::RAFT));
}

#[test]
fn test_ra_node_left_cluster() {
    let entry = create_test_entry(
        "Ra node {rabbitmq_metadata,rabbit@node1} has successfully left the cluster.",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::RAFT));
}

#[test]
fn test_ra_snapshot_skipping() {
    let entry = create_test_entry(
        "ra_snapshot: RabbitMQ metadata store: skipping snapshot.dat as did not validate. Err: enoent",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::RAFT));
}

#[test]
fn test_ra_monitors_target_not_recognised() {
    let entry = create_test_entry(
        "ra_monitors: target {rabbitmq_metadata,rabbit@unknown} not recognised",
        Severity::Debug,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::RAFT));
}
