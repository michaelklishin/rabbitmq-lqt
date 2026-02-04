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

use rabbitmq_lqt_lib::Severity;
use rabbitmq_lqt_lib::entry_metadata::Annotator;
use rabbitmq_lqt_lib::entry_metadata::label_annotators::{
    LabelAnnotator, LeadershipTransferAnnotator, QuorumQueueLabelsAnnotator, annotate_labels,
};
use rabbitmq_lqt_lib::entry_metadata::labels::LogEntryLabels;
use test_helpers::create_test_entry;

#[test]
fn test_all_queue_leaders_balanced() {
    let entry = create_test_entry("All queue leaders are balanced", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::QUORUM_QUEUES));
    assert!(labels.contains(LogEntryLabels::QUEUES));
    assert!(labels.contains(LogEntryLabels::RAFT));
}

#[test]
fn test_quorum_queue_recovery_message() {
    let entry = create_test_entry(
        "Quorum queue recovery: configured member of queue 'events' in vhost '/' was not found on this node. Starting member as a new one.",
        Severity::Warning,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::QUORUM_QUEUES));
    assert!(labels.contains(LogEntryLabels::QUEUES));
    assert!(labels.contains(LogEntryLabels::RAFT));
}

#[test]
fn test_will_start_replicas_for_quorum_queue() {
    let entry = create_test_entry(
        "Will start up to 3 replicas for quorum queue 'events' in vhost '/'",
        Severity::Debug,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::QUORUM_QUEUES));
    assert!(labels.contains(LogEntryLabels::QUEUES));
    assert!(labels.contains(LogEntryLabels::RAFT));
}

#[test]
fn test_shrinking_all_quorum_queues() {
    let entry = create_test_entry(
        "Shrinking all quorum queues to a single node: rabbit@node1",
        Severity::Warning,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::QUORUM_QUEUES));
    assert!(labels.contains(LogEntryLabels::QUEUES));
    assert!(labels.contains(LogEntryLabels::RAFT));
}

#[test]
fn test_quorum_queue_labels_annotator_matches_membership_reconciliation() {
    let entry = create_test_entry(
        "Quorum Queue membership reconciliation scheduled: {node_up,rabbit@node2}",
        Severity::Debug,
    );
    let annotator = QuorumQueueLabelsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_quorum_queue_labels_annotator_matches_shrinking() {
    let entry = create_test_entry(
        "Shrinking queue 'events' in vhost '/' to a single node: rabbit@node1",
        Severity::Warning,
    );
    let annotator = QuorumQueueLabelsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_quorum_queue_labels_annotator_matches_shrinking_finished() {
    let entry = create_test_entry("Shrinking finished", Severity::Warning);
    let annotator = QuorumQueueLabelsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_quorum_queue_labels_annotator_sets_labels() {
    let annotator = QuorumQueueLabelsAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);
    assert!(labels.contains(LogEntryLabels::QUORUM_QUEUES));
    assert!(labels.contains(LogEntryLabels::RAFT));
}

#[test]
fn test_leadership_transfer_annotator_matches() {
    let entry = create_test_entry(
        "Leadership transfer for quorum queues hosted on this node has been initiated",
        Severity::Info,
    );
    let annotator = LeadershipTransferAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_leadership_transfer_annotator_sets_all_labels() {
    let annotator = LeadershipTransferAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);
    assert!(labels.contains(LogEntryLabels::QUORUM_QUEUES));
    assert!(labels.contains(LogEntryLabels::RAFT));
    assert!(labels.contains(LogEntryLabels::ELECTIONS));
}

#[test]
fn test_delivery_limit_not_set_defaulting() {
    let entry = create_test_entry(
        "delivery_limit not set, defaulting to 20 for queue 'quorum_queue_1' in vhost '/'",
        Severity::Debug,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::QUORUM_QUEUES));
}

#[test]
fn test_no_match_unrelated() {
    let entry = create_test_entry("client unexpectedly closed TCP connection", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(!labels.contains(LogEntryLabels::QUORUM_QUEUES));
}
