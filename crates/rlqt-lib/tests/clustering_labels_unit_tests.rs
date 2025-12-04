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
    ClusterNotReadyAnnotator, ClusterStatusFilesAnnotator, ClusteringLabelsAnnotator,
    InetTcpErrorAnnotator, LabelAnnotator, MnesiaEventAnnotator,
};
use rlqt_lib::entry_metadata::labels::LogEntryLabels;
use test_helpers::create_test_entry;

#[test]
fn test_cluster_status_files_annotator_matches() {
    let entry = create_test_entry("Preparing cluster status files for node", Severity::Info);
    let annotator = ClusterStatusFilesAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_cluster_status_files_annotator_sets_label() {
    let annotator = ClusterStatusFilesAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);
    assert!(labels.contains(LogEntryLabels::CLUSTERING));
}

#[test]
fn test_cluster_not_ready_annotator_matches() {
    let entry = create_test_entry(
        "Cluster is not ready for a membership change",
        Severity::Warning,
    );
    let annotator = ClusterNotReadyAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_cluster_not_ready_annotator_sets_labels() {
    let annotator = ClusterNotReadyAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);
    assert!(labels.contains(LogEntryLabels::RAFT));
    assert!(labels.contains(LogEntryLabels::CLUSTERING));
}

#[test]
fn test_clustering_labels_annotator_matches_partial_partition() {
    let entry = create_test_entry("Partial partition disconnect detected", Severity::Warning);
    let annotator = ClusteringLabelsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_clustering_labels_annotator_matches_node_restarted() {
    let entry = create_test_entry("Node restarted after crash", Severity::Info);
    let annotator = ClusteringLabelsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_clustering_labels_annotator_matches_node_joined() {
    let entry = create_test_entry("Node joined cluster", Severity::Info);
    let annotator = ClusteringLabelsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_clustering_labels_annotator_sets_label() {
    let annotator = ClusteringLabelsAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);
    assert!(labels.contains(LogEntryLabels::CLUSTERING));
}

#[test]
fn test_inet_tcp_error_annotator_matches() {
    let entry = create_test_entry(
        "Protocol 'inet_tcp': register/listen error: eaddrinuse",
        Severity::Error,
    );
    let annotator = InetTcpErrorAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_inet_tcp_error_annotator_sets_labels() {
    let annotator = InetTcpErrorAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);
    assert!(labels.contains(LogEntryLabels::CLUSTERING));
    assert!(labels.contains(LogEntryLabels::NETWORKING));
}

#[test]
fn test_mnesia_event_annotator_matches() {
    let entry = create_test_entry(
        "mnesia_event got {inconsistent_database, running_partitioned_network, 'rabbit@other'}",
        Severity::Error,
    );
    let annotator = MnesiaEventAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_mnesia_event_annotator_sets_label() {
    let annotator = MnesiaEventAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);
    assert!(labels.contains(LogEntryLabels::CLUSTERING));
}
