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
use rlqt_lib::entry_metadata::subsystem_annotators::{ShovelPluginAnnotator, SubsystemAnnotator};
use rlqt_lib::entry_metadata::subsystems::Subsystem;
use test_helpers::create_test_entry;

#[test]
fn test_rabbit_shovel_dyn_worker_sup_sup() {
    let entry = create_test_entry("Starting rabbit_shovel_dyn_worker_sup_sup", Severity::Info);
    let annotator = ShovelPluginAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_rabbit_shovel_dyn_worker_sup_sup_in_middle() {
    let entry = create_test_entry(
        "Process rabbit_shovel_dyn_worker_sup_sup terminated",
        Severity::Info,
    );
    let annotator = ShovelPluginAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_shovel_connected_pattern() {
    let entry = create_test_entry(
        "Shovel 'my-shovel' connected to destination",
        Severity::Info,
    );
    let annotator = ShovelPluginAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_shovel_connected_pattern_different_name() {
    let entry = create_test_entry(
        "Shovel 'data-replication' connected successfully",
        Severity::Info,
    );
    let annotator = ShovelPluginAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_shovel_connected_case_insensitive() {
    let entry = create_test_entry("SHOVEL 'test-shovel' CONNECTED", Severity::Info);
    let annotator = ShovelPluginAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_shovel_in_vhost_pattern() {
    let entry = create_test_entry(
        "Shovel 'my-shovel' in virtual host '/' started",
        Severity::Info,
    );
    let annotator = ShovelPluginAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_shovel_in_vhost_named_vhost() {
    let entry = create_test_entry(
        "Shovel 'prod-shovel' in virtual host 'production' running",
        Severity::Info,
    );
    let annotator = ShovelPluginAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_shovel_in_vhost_case_insensitive() {
    let entry = create_test_entry("SHOVEL 'test' IN VIRTUAL HOST '/test'", Severity::Info);
    let annotator = ShovelPluginAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_shovel_topology_pattern() {
    let entry = create_test_entry(
        "Shovel 'my-shovel' has finished setting up its topology successfully",
        Severity::Info,
    );
    let annotator = ShovelPluginAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_shovel_topology_pattern_different_name() {
    let entry = create_test_entry(
        "Shovel 'backup-shovel' has finished setting up its topology on destination",
        Severity::Info,
    );
    let annotator = ShovelPluginAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_shovel_topology_case_insensitive() {
    let entry = create_test_entry(
        "SHOVEL 'test' HAS FINISHED SETTING UP ITS TOPOLOGY",
        Severity::Info,
    );
    let annotator = ShovelPluginAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_no_match_simple_shovel_word() {
    let entry = create_test_entry("Please shovel the snow", Severity::Info);
    let annotator = ShovelPluginAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_no_match_shovel_without_patterns() {
    let entry = create_test_entry("Starting shovel connection", Severity::Info);
    let annotator = ShovelPluginAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_no_match_unrelated_message() {
    let entry = create_test_entry("Connection established", Severity::Info);
    let annotator = ShovelPluginAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_no_match_shovel_incomplete_pattern() {
    let entry = create_test_entry("Shovel 'my-shovel' status", Severity::Info);
    let annotator = ShovelPluginAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_asked_to_start_dynamic_shovel() {
    let entry = create_test_entry(
        "Asked to start a dynamic Shovel named 'my-shovel'",
        Severity::Info,
    );
    let annotator = ShovelPluginAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_asked_to_start_dynamic_shovel_case_insensitive() {
    let entry = create_test_entry(
        "ASKED TO START A DYNAMIC SHOVEL NAMED 'test-shovel'",
        Severity::Info,
    );
    let annotator = ShovelPluginAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_rabbit_shovel_worker() {
    let entry = create_test_entry(
        "Starting rabbit_shovel_worker for shovel 'my-shovel'",
        Severity::Info,
    );
    let annotator = ShovelPluginAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_rabbit_shovel_worker_in_message() {
    let entry = create_test_entry(
        "Process rabbit_shovel_worker terminated normally",
        Severity::Info,
    );
    let annotator = ShovelPluginAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_for_component_shovel() {
    let entry = create_test_entry(
        "Configuration for component 'shovel' updated",
        Severity::Info,
    );
    let annotator = ShovelPluginAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_for_component_shovel_case_insensitive() {
    let entry = create_test_entry("PARAMETERS FOR COMPONENT 'SHOVEL' applied", Severity::Info);
    let annotator = ShovelPluginAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_shovel_received_with_single_quotes() {
    let entry = create_test_entry(
        "Shovel 'test_basic_dest_shovel' received a 'queue.declare-ok' from the server",
        Severity::Info,
    );
    let annotator = ShovelPluginAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_shovel_received_with_angle_brackets() {
    let entry = create_test_entry(
        "Shovel <<\"test_basic_dest_shovel\">> received a 'basic.cancel' from the server",
        Severity::Info,
    );
    let annotator = ShovelPluginAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_shovel_received_case_insensitive() {
    let entry = create_test_entry(
        "SHOVEL 'MY_SHOVEL' RECEIVED A 'BASIC.ACK' FROM THE SERVER",
        Severity::Info,
    );
    let annotator = ShovelPluginAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_shovel_received_different_command() {
    let entry = create_test_entry(
        "Shovel 'another_shovel' received a 'basic.deliver' from the server",
        Severity::Info,
    );
    let annotator = ShovelPluginAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_annotate_sets_subsystem() {
    let mut entry = create_test_entry("Shovel 'test' connected", Severity::Info);
    let annotator = ShovelPluginAnnotator;
    annotator.annotate(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::ShovelPlugin.to_id()));
}
