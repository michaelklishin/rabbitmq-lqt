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
fn test_osiris_writer() {
    let entry = create_test_entry(
        "osiris_writer:init/1: name: stream_name last offset: -1 committed chunk id: -1 epoch: 1",
        Severity::Debug,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::STREAMS));
}

#[test]
fn test_osiris_log() {
    let entry = create_test_entry(
        "stream_name [osiris_log:open_new_segment/1] 00000000000000000000.segment",
        Severity::Debug,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::STREAMS));
}

#[test]
fn test_ranch_started() {
    let entry = create_test_entry("Started Ranch", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::NETWORKING));
}

#[test]
fn test_ranch_stopping() {
    let entry = create_test_entry("Stopping Ranch listeners", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::NETWORKING));
}

#[test]
fn test_http_listener() {
    let entry = create_test_entry(
        "Starting HTTP listener with transport ranch_tcp",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::NETWORKING));
}

#[test]
fn test_client_provided_name() {
    let entry = create_test_entry(
        "connection 127.0.0.1:64169 -> 127.0.0.1:5672 has a client-provided name: Shovel test_shovel",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::CONNECTIONS));
}

#[test]
fn test_deleting_pid_file() {
    let entry = create_test_entry(
        "Deleting PID file: /var/lib/rabbitmq/mnesia/rabbit@node.pid",
        Severity::Debug,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::SHUTDOWN));
}

#[test]
fn test_deleting_unknown_files() {
    let entry = create_test_entry("Deleting unknown files/folders: []", Severity::Debug);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::RAFT));
}

#[test]
fn test_ready_to_start_listeners() {
    let entry = create_test_entry("Ready to start client connection listeners", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::CONNECTIONS));
    assert!(labels.contains(LogEntryLabels::NETWORKING));
}

#[test]
fn test_tracing_vhost() {
    let entry = create_test_entry(
        "Tracing is already disabled for vhost 'test-vhost'",
        Severity::Debug,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
}

#[test]
fn test_refreshing_channels() {
    let entry = create_test_entry(
        "Refreshing state of channels, 0 sessions and 0 non AMQP 0.9.1 connections after virtual host tracing changes...",
        Severity::Debug,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::CHANNELS));
}

#[test]
fn test_epmd_port() {
    let entry = create_test_entry(
        "Getting epmd port node 'rabbit', 10 retries left",
        Severity::Debug,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::CLUSTERING));
    assert!(labels.contains(LogEntryLabels::NETWORKING));
}

#[test]
fn test_rabbit_on_node_up() {
    let entry = create_test_entry("rabbit on node hare@sunnyside up", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::CLUSTERING));
}

#[test]
fn test_rabbit_on_node_down() {
    let entry = create_test_entry("rabbit on node rabbit@other down", Severity::Warning);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::CLUSTERING));
}

#[test]
fn test_vhost_reconciliation() {
    let entry = create_test_entry(
        "Will stop virtual host process reconciliation after 10 runs",
        Severity::Debug,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
}

#[test]
fn test_set_stop_reason() {
    let entry = create_test_entry("Set stop reason to: normal", Severity::Debug);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::SHUTDOWN));
}

#[test]
fn test_metadata_store_leader_elections() {
    let entry = create_test_entry(
        "RabbitMQ metadata store: leader call - leader not known",
        Severity::Warning,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::RAFT));
    assert!(labels.contains(LogEntryLabels::ELECTIONS));
}

#[test]
fn test_will_declare_exchange() {
    let entry = create_test_entry(
        "Will declare an exchange {resource,<<\"/\">>,exchange,<<>>}",
        Severity::Debug,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::EXCHANGES));
}

#[test]
fn test_autoheal() {
    let entry = create_test_entry(
        "Autoheal: we were selected to restart; winner is rabbit@other",
        Severity::Warning,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::CLUSTERING));
}

#[test]
fn test_cluster_tags() {
    let entry = create_test_entry(
        "Seeding cluster tags from application environment key...",
        Severity::Debug,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::CLUSTERING));
}

#[test]
fn test_checking_cluster_consistency() {
    let entry = create_test_entry("Checking cluster consistency", Severity::Debug);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::CLUSTERING));
}

#[test]
fn test_feature_flags_extended() {
    let entry = create_test_entry(
        "Feature flags: registry refresh needed: yes",
        Severity::Debug,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::FEATURE_FLAGS));
}

#[test]
fn test_peer_discovery_clustering() {
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
