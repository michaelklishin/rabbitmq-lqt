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
use rabbitmq_lqt_lib::entry_metadata::subsystem_annotators::annotate_subsystems;
use rabbitmq_lqt_lib::entry_metadata::subsystems::Subsystem;
use test_helpers::create_test_entry;

#[test]
fn test_starting_ra_systems() {
    let mut entry = create_test_entry("Starting Ra systems", Severity::Debug);
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Raft.to_id()));
}

#[test]
fn test_ra_system_started() {
    let mut entry = create_test_entry("Ra system quorum_queues started", Severity::Debug);
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Raft.to_id()));
}

#[test]
fn test_starting_ra_system_coordination() {
    let mut entry = create_test_entry("starting Ra system: coordination", Severity::Debug);
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Raft.to_id()));
}

#[test]
fn test_wal_message() {
    let mut entry = create_test_entry("wal: writing entry to disk", Severity::Debug);
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Raft.to_id()));
}

#[test]
fn test_ra_colon() {
    let mut entry = create_test_entry(
        "ra: starting system quorum_queues for domain ~tp",
        Severity::Info,
    );
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Raft.to_id()));
}

#[test]
fn test_ra_log_wal() {
    let mut entry = create_test_entry("ra_log_wal: writing entries", Severity::Debug);
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Raft.to_id()));
}

#[test]
fn test_trigger_election_in() {
    let mut entry = create_test_entry("Trigger election in store", Severity::Debug);
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Raft.to_id()));
}

#[test]
fn test_ra_system_recover() {
    let mut entry = create_test_entry(
        "ra_system_recover: no server recovery configured",
        Severity::Info,
    );
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Raft.to_id()));
}

#[test]
fn test_segment_writer() {
    let mut entry = create_test_entry("segment_writer: flushing segment", Severity::Debug);
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Raft.to_id()));
}

#[test]
fn test_no_match_unrelated() {
    let mut entry = create_test_entry("Connection established", Severity::Info);
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, None);
}
