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
use rabbitmq_lqt_lib::entry_metadata::label_annotators::{
    ClusterNotReadyAnnotator, ClusterStatusFilesAnnotator, ClusteringLabelsAnnotator,
    InetTcpErrorAnnotator, LabelAnnotator, MnesiaEventAnnotator, annotate_labels,
};
use rabbitmq_lqt_lib::entry_metadata::labels::LogEntryLabels;

#[test]
fn test_node_up() {
    let entry = create_test_entry("node rabbit@node2 up", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::CLUSTERING));
}

#[test]
fn test_node_down() {
    let entry = create_test_entry("node rabbit@node2 down", Severity::Warning);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::CLUSTERING));
}

#[test]
fn test_node_up_with_fqdn() {
    let entry = create_test_entry("node rabbit@node2.example.com up", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::CLUSTERING));
}

#[test]
fn test_no_match_node_with_extra_text() {
    let entry = create_test_entry("node rabbit@node2 up and running", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(!labels.contains(LogEntryLabels::CLUSTERING));
}

#[test]
fn test_peer_discovery() {
    let entry = create_test_entry(
        "Peer discovery: registration is not supported",
        Severity::Debug,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::CLUSTERING));
}

#[test]
fn test_prevent_startup_if_node_was_reset() {
    let entry = create_test_entry(
        "prevent_startup_if_node_was_reset is disabled",
        Severity::Debug,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::CLUSTERING));
}

#[test]
fn test_inter_node_communication() {
    let entry = create_test_entry(
        "Inter-node communication protocol: inet_tls",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::CLUSTERING));
}

#[test]
fn test_maintenance_mode() {
    let entry = create_test_entry(
        "Maintenance mode is disabled for this node",
        Severity::Debug,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::CLUSTERING));
}

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
    let entry = create_test_entry(
        "Partial partition disconnect from rabbit@other",
        Severity::Error,
    );
    let annotator = ClusteringLabelsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_clustering_labels_annotator_matches_node_joined() {
    let entry = create_test_entry(
        "Node 'rabbit@node2' has joined the cluster",
        Severity::Debug,
    );
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

#[test]
fn test_clustering_banner() {
    let entry = create_test_entry("== Clustering ==", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::CLUSTERING));
}

#[test]
fn test_node_down_connection_closed() {
    let entry = create_test_entry(
        "node rabbit@node1 down: connection_closed",
        Severity::Warning,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::CLUSTERING));
    assert!(labels.contains(LogEntryLabels::DISCONNECTS));
}

#[test]
fn test_starting_rabbit_node_monitor() {
    let entry = create_test_entry(
        "Starting rabbit_node_monitor (partition handling strategy unapplicable with Khepri)",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::CLUSTERING));
}

#[test]
fn test_found_metadata_store_members() {
    let entry = create_test_entry(
        "Found the following metadata store members: [rabbit@node1, rabbit@node2]",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::KHEPRI));
    assert!(labels.contains(LogEntryLabels::CLUSTERING));
}

#[test]
fn test_mirrored_supervisor_initializing() {
    let entry = create_test_entry(
        "Mirrored supervisor: initializing, overall supervisor <0.400.0> joined group rabbit_federation_queue_link_sup_sup",
        Severity::Debug,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::CLUSTERING));
}

#[test]
fn test_mirrored_supervisor_known_group_members() {
    let entry = create_test_entry(
        "Mirrored supervisor: known group rabbit_shovel_dyn_worker_sup_sup members: [<14931.779.0>] on nodes [rabbit@sunnyside]",
        Severity::Debug,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::CLUSTERING));
}

#[test]
fn test_mirrored_supervisor_no_known_peer_members() {
    let entry = create_test_entry(
        "Mirrored supervisor: no known peer members in group rabbit_federation_exchange_link_sup_sup, will delete all child records for it",
        Severity::Debug,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::CLUSTERING));
}
