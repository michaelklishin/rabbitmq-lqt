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
    DbMetadataAnnotator, LabelAnnotator, MetadataStoreClusterChangeAnnotator,
    MetadataStoreLabelsAnnotator, MetadataStoreRaftAnnotator, MetadataStoreVotingAnnotator,
};
use rabbitmq_lqt_lib::entry_metadata::labels::LogEntryLabels;
use test_helpers::create_test_entry;

#[test]
fn test_db_metadata_annotator_matches_initialization() {
    let entry = create_test_entry("DB: initialization started", Severity::Info);
    let annotator = DbMetadataAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_db_metadata_annotator_matches_virgin_node() {
    let entry = create_test_entry(
        "DB: this node is a virgin node, setting up initial schema",
        Severity::Info,
    );
    let annotator = DbMetadataAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_db_metadata_annotator_matches_checking_membership() {
    let entry = create_test_entry("DB: checking cluster membership", Severity::Info);
    let annotator = DbMetadataAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_db_metadata_annotator_sets_khepri_label() {
    let annotator = DbMetadataAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);
    assert!(labels.contains(LogEntryLabels::KHEPRI));
}

#[test]
fn test_metadata_store_labels_annotator_matches() {
    let entry = create_test_entry(
        "RabbitMQ metadata store: operation completed",
        Severity::Info,
    );
    let annotator = MetadataStoreLabelsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_metadata_store_labels_annotator_sets_khepri_label() {
    let annotator = MetadataStoreLabelsAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);
    assert!(labels.contains(LogEntryLabels::KHEPRI));
}

#[test]
fn test_metadata_store_voting_annotator_matches_granting_vote() {
    let entry = create_test_entry(
        "RabbitMQ metadata store: granting vote to node@host",
        Severity::Info,
    );
    let annotator = MetadataStoreVotingAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_metadata_store_voting_annotator_matches_declining_vote() {
    let entry = create_test_entry(
        "RabbitMQ metadata store: declining vote request",
        Severity::Info,
    );
    let annotator = MetadataStoreVotingAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_metadata_store_voting_annotator_matches_leader() {
    let entry = create_test_entry("RabbitMQ metadata store: becoming leader", Severity::Info);
    let annotator = MetadataStoreVotingAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_metadata_store_voting_annotator_sets_labels() {
    let annotator = MetadataStoreVotingAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);
    assert!(labels.contains(LogEntryLabels::RAFT));
    assert!(labels.contains(LogEntryLabels::ELECTIONS));
}

#[test]
fn test_metadata_store_raft_annotator_matches_snapshot() {
    let entry = create_test_entry("RabbitMQ metadata store: writing snapshot", Severity::Info);
    let annotator = MetadataStoreRaftAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_metadata_store_raft_annotator_matches_follower() {
    let entry = create_test_entry(
        "RabbitMQ metadata store: transitioning to follower state",
        Severity::Info,
    );
    let annotator = MetadataStoreRaftAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_metadata_store_raft_annotator_sets_label() {
    let annotator = MetadataStoreRaftAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);
    assert!(labels.contains(LogEntryLabels::RAFT));
}

#[test]
fn test_metadata_store_cluster_change_annotator_matches() {
    let entry = create_test_entry(
        "RabbitMQ metadata store: applying cluster change",
        Severity::Info,
    );
    let annotator = MetadataStoreClusterChangeAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_metadata_store_cluster_change_annotator_sets_label() {
    let annotator = MetadataStoreClusterChangeAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);
    assert!(labels.contains(LogEntryLabels::CLUSTERING));
}
