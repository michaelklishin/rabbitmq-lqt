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
use rlqt_lib::entry_metadata::Annotator;
use rlqt_lib::entry_metadata::label_annotators::{
    LabelAnnotator, LeadershipTransferAnnotator, QuorumQueueLabelsAnnotator,
};
use rlqt_lib::entry_metadata::labels::LogEntryLabels;
use test_helpers::create_test_entry;

#[test]
fn test_quorum_queue_labels_annotator_matches_membership_reconciliation() {
    let entry = create_test_entry("Membership reconciliation in progress", Severity::Info);
    let annotator = QuorumQueueLabelsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_quorum_queue_labels_annotator_matches_recovery_started() {
    let entry = create_test_entry("Recovery procedure started for queue", Severity::Info);
    let annotator = QuorumQueueLabelsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_quorum_queue_labels_annotator_matches_recovery_completed() {
    let entry = create_test_entry("Recovery procedure completed successfully", Severity::Info);
    let annotator = QuorumQueueLabelsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_quorum_queue_labels_annotator_matches_replicas_recovery() {
    let entry = create_test_entry("3 replicas in recovery state", Severity::Info);
    let annotator = QuorumQueueLabelsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_quorum_queue_labels_annotator_matches_shrinking() {
    let entry = create_test_entry("Shrinking quorum queue to fewer members", Severity::Info);
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
    let entry = create_test_entry("Leadership transfer initiated for queue", Severity::Info);
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
