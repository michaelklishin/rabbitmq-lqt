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
fn test_autoheal_in_progress() {
    let mut entry = create_test_entry(
        "Autoheal: in progress, requesting report from rabbit@node2",
        Severity::Info,
    );
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Clustering.to_id()));
}

#[test]
fn test_autoheal_final_node_stopped() {
    let mut entry = create_test_entry(
        "Autoheal: final node has stopped, starting...",
        Severity::Info,
    );
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Clustering.to_id()));
}

#[test]
fn test_autoheal_aborting_winner_down() {
    let mut entry = create_test_entry(
        "Autoheal: aborting - winner rabbit@node1 went down",
        Severity::Info,
    );
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Clustering.to_id()));
}

#[test]
fn test_autoheal_waiting_for_winner_decision() {
    let mut entry = create_test_entry(
        "Autoheal: rabbit@node3 went down, waiting for winner decision ",
        Severity::Info,
    );
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Clustering.to_id()));
}

#[test]
fn test_autoheal_aborting_node_down() {
    let mut entry = create_test_entry(
        "Autoheal: aborting - rabbit@node2 went down",
        Severity::Info,
    );
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Clustering.to_id()));
}

#[test]
fn test_autoheal_i_am_the_winner() {
    let mut entry = create_test_entry(
        "Autoheal: I am the winner, waiting for [rabbit@node2,rabbit@node3] to stop",
        Severity::Info,
    );
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Clustering.to_id()));
}

#[test]
fn test_autoheal_request_sent() {
    let mut entry = create_test_entry("Autoheal request sent to rabbit@node1", Severity::Info);
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Clustering.to_id()));
}

#[test]
fn test_autoheal_request_received() {
    let mut entry = create_test_entry(
        "Autoheal request received from rabbit@node2",
        Severity::Info,
    );
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Clustering.to_id()));
}

#[test]
fn test_autoheal_request_denied() {
    let mut entry = create_test_entry(
        "Autoheal request denied: Local nodes down: [rabbit@node3]",
        Severity::Info,
    );
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Clustering.to_id()));
}

#[test]
fn test_autoheal_finished_according_to_winner() {
    let mut entry = create_test_entry(
        "Autoheal finished according to winner rabbit@node1",
        Severity::Info,
    );
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Clustering.to_id()));
}

#[test]
fn test_autoheal_we_were_selected_to_restart() {
    let mut entry = create_test_entry(
        "Autoheal: we were selected to restart; winner is rabbit@node1",
        Severity::Warning,
    );
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Clustering.to_id()));
}

#[test]
fn test_mirrored_supervisor_initializing() {
    let mut entry = create_test_entry(
        "Mirrored supervisor: initializing, overall supervisor <0.400.0> joined group rabbit_federation_queue_link_sup_sup",
        Severity::Debug,
    );
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Clustering.to_id()));
}

#[test]
fn test_mirrored_supervisor_known_group_members() {
    let mut entry = create_test_entry(
        "Mirrored supervisor: known group rabbit_federation_queue_link_sup_sup members: [<14931.337.0>] on nodes [rabbit@sunnyside]",
        Severity::Debug,
    );
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Clustering.to_id()));
}

#[test]
fn test_mirrored_supervisor_no_known_peer_members() {
    let mut entry = create_test_entry(
        "Mirrored supervisor: no known peer members in group rabbit_federation_queue_link_sup_sup, will delete all child records for it",
        Severity::Debug,
    );
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Clustering.to_id()));
}

#[test]
fn test_mirrored_supervisor_asked_to_consider_starting() {
    let mut entry = create_test_entry(
        "Mirrored supervisor: asked to consider starting a child, group: rabbit_shovel_dyn_worker_sup_sup",
        Severity::Debug,
    );
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Clustering.to_id()));
}
