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
fn test_all_queue_leaders_balanced() {
    let entry = create_test_entry("All queue leaders are balanced", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::QUORUM_QUEUES));
    assert!(labels.contains(LogEntryLabels::QUEUES));
    assert!(labels.contains(LogEntryLabels::RAFT));
}

#[test]
fn test_all_leaders_balanced() {
    let entry = create_test_entry("All leaders balanced across nodes", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::QUORUM_QUEUES));
    assert!(labels.contains(LogEntryLabels::RAFT));
}

#[test]
fn test_rebalancing_leader() {
    let entry = create_test_entry("Rebalancing leader for queue 'my-queue'", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::QUORUM_QUEUES));
    assert!(labels.contains(LogEntryLabels::RAFT));
}

#[test]
fn test_leader_balanced() {
    let entry = create_test_entry("Leader balanced for queue 'my-queue'", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::QUORUM_QUEUES));
}

#[test]
fn test_leader_rebalanced() {
    let entry = create_test_entry("Leader rebalanced successfully", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::QUORUM_QUEUES));
}

#[test]
fn test_quorum_queue_explicit() {
    let entry = create_test_entry(
        "Quorum queue 'events' created on node rabbit@node1",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::QUORUM_QUEUES));
    assert!(labels.contains(LogEntryLabels::QUEUES));
    assert!(labels.contains(LogEntryLabels::RAFT));
}

#[test]
fn test_rabbit_quorum_queue_prefix() {
    let entry = create_test_entry(
        "rabbit_quorum_queue: policy may not have been successfully applied. Error: timeout",
        Severity::Warning,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::QUORUM_QUEUES));
    assert!(labels.contains(LogEntryLabels::QUEUES));
    assert!(labels.contains(LogEntryLabels::RAFT));
}
