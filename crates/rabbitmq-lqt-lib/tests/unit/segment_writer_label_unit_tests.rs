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
fn test_segment_writer_quorum_queues() {
    let entry = create_test_entry(
        "segment_writer in 'quorum_queues': completed flush of 1 writers from wal file 0000000000000003.wal in 169ms",
        Severity::Debug,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::QUORUM_QUEUES));
    assert!(labels.contains(LogEntryLabels::RAFT));
}

#[test]
fn test_segment_writer_quorum_queues_handle_cast() {
    let entry = create_test_entry(
        "segment_writer in 'quorum_queues': handle_cast for 2F_PER9DL7BBC1421W took 6ms",
        Severity::Debug,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::QUORUM_QUEUES));
    assert!(labels.contains(LogEntryLabels::RAFT));
}

#[test]
fn test_segment_writer_generic() {
    let entry = create_test_entry(
        "segment_writer: upgrading segment file names to new format in directory /var/lib/rabbitmq/mnesia/rabbit@node/quorum/rabbit@node",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::RAFT));
}

#[test]
fn test_segment_writer_coordination_has_khepri_and_raft() {
    let entry = create_test_entry(
        "segment_writer in 'coordination': completed flush of 1 writers from wal file 0000000000000002.wal in 14ms",
        Severity::Debug,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::KHEPRI));
    assert!(labels.contains(LogEntryLabels::RAFT));
}
