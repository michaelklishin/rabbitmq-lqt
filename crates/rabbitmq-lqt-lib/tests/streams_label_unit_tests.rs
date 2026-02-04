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
use rabbitmq_lqt_lib::entry_metadata::label_annotators::annotate_labels;
use rabbitmq_lqt_lib::entry_metadata::labels::LogEntryLabels;
use test_helpers::create_test_entry;

#[test]
fn test_started_stream_tcp_listener() {
    let entry = create_test_entry("started Stream TCP listener on [::]:5552", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::STREAMS));
}

#[test]
fn test_stopped_stream_tcp_listener() {
    let entry = create_test_entry("stopped Stream TCP listener on [::]:5552", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::STREAMS));
}

#[test]
fn test_asked_to_remove_stream_replicas() {
    let entry = create_test_entry(
        "Asked to remove all stream replicas from node flopsy@sunnyside",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::STREAMS));
}

#[test]
fn test_rabbit_stream() {
    let entry = create_test_entry("rabbit_stream: some operation completed", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::STREAMS));
}

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
fn test_stream_coordinator() {
    let entry = create_test_entry(
        "Starting stream coordinator on nodes [rabbit@node1, rabbit@node2]",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::STREAMS));
    assert!(labels.contains(LogEntryLabels::RAFT));
}

#[test]
fn test_stream_replica() {
    let entry = create_test_entry(
        "Stream replica started on node rabbit@node1",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::STREAMS));
}

#[test]
fn test_stream_member() {
    let entry = create_test_entry(
        "Stream member joined cluster on node rabbit@node2",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::STREAMS));
}

#[test]
fn test_no_match_unrelated() {
    let entry = create_test_entry("ra: starting system quorum_queues", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(!labels.contains(LogEntryLabels::STREAMS));
}

#[test]
fn test_stream_osiris_log_data_directory() {
    let entry = create_test_entry(
        "Stream: rabbitmqadmin_definitions_import_test1_stream_1_1764319106807498875 will use /Users/antares/Tools/rabbitmq/generic/var/lib/rabbitmq/mnesia/rabbit@sunnyside/stream/rabbitmqadmin_definitions_import_test1_stream_1_1764319106807498875 for osiris log data directory",
        Severity::Debug,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::STREAMS));
}

#[test]
fn test_stream_osiris_log_data_directory_variant() {
    let entry = create_test_entry(
        "Stream: rabbitmqadmin_definitions_import_test1_stream_3_asjdh will use /Users/antares/Tools/rabbitmq/generic/var/lib/rabbitmq/mnesia/rabbit@sunnyside/stream/rabbitmqadmin_definitions_import_test1_stream_1_sadf78a6fd87a6f for osiris log data directory",
        Severity::Debug,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::STREAMS));
}
