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
use rabbitmq_lqt_lib::entry_metadata::subsystem_annotators::annotate_subsystems;
use rabbitmq_lqt_lib::entry_metadata::subsystems::Subsystem;

#[test]
fn test_boot_step_running() {
    let mut entry = create_test_entry(
        "Running boot step rabbit_alarm defined by app rabbit",
        Severity::Info,
    );
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Boot.to_id()));
}

#[test]
fn test_boot_state_change() {
    let mut entry = create_test_entry("Change boot state to `ready`", Severity::Debug);
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Boot.to_id()));
}

#[test]
fn test_boot_applying_mfa() {
    let mut entry = create_test_entry(
        "Applying MFA: M = rabbit_alarm, F = start, A = []",
        Severity::Debug,
    );
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Boot.to_id()));
}

#[test]
fn test_boot_finished_mfa() {
    let mut entry = create_test_entry(
        "Finished MFA: M = rabbit_alarm, F = start, A = []",
        Severity::Debug,
    );
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Boot.to_id()));
}

#[test]
fn test_boot_prelaunch() {
    let mut entry = create_test_entry("== Prelaunch DONE ==", Severity::Debug);
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Boot.to_id()));
}

#[test]
fn test_boot_postlaunch() {
    let mut entry = create_test_entry("== Postlaunch DONE ==", Severity::Debug);
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Boot.to_id()));
}

#[test]
fn test_boot_marking_as_running() {
    let mut entry = create_test_entry("Marking RabbitMQ as running", Severity::Info);
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Boot.to_id()));
}

#[test]
fn test_boot_systemd() {
    let mut entry = create_test_entry(
        "Boot state/systemd: notifying of state `ready`",
        Severity::Debug,
    );
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Boot.to_id()));
}

#[test]
fn test_boot_fhc_read_buffering() {
    let mut entry = create_test_entry("FHC read buffering: OFF", Severity::Info);
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Boot.to_id()));
}

#[test]
fn test_boot_fhc_write_buffering() {
    let mut entry = create_test_entry("FHC write buffering: ON", Severity::Info);
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Boot.to_id()));
}

#[test]
fn test_boot_decoding_encrypted() {
    let mut entry = create_test_entry("Decoding encrypted config values (if any)", Severity::Debug);
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Boot.to_id()));
}

#[test]
fn test_boot_opening_log_file() {
    let mut entry = create_test_entry(
        "opening log file: \"/var/log/rabbitmq/access.log\"",
        Severity::Debug,
    );
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Boot.to_id()));
}

#[test]
fn test_boot_webmachine_log_handler() {
    let mut entry = create_test_entry("webmachine_log_handler: closing log file", Severity::Debug);
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Boot.to_id()));
}

#[test]
fn test_boot_files_and_directories() {
    let mut entry = create_test_entry(
        "Files and directories found in node's data directory: cluster_nodes.config, coordination, quorum",
        Severity::Debug,
    );
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Boot.to_id()));
}

#[test]
fn test_boot_prevent_startup() {
    let mut entry = create_test_entry(
        "prevent_startup_if_node_was_reset is disabled",
        Severity::Debug,
    );
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Boot.to_id()));
}

#[test]
fn test_boot_clustering_banner() {
    let mut entry = create_test_entry("== Clustering ==", Severity::Info);
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Boot.to_id()));
}

#[test]
fn test_boot_failed() {
    let mut entry = create_test_entry(
        "BOOT FAILED\n===========\nError during startup",
        Severity::Error,
    );
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Boot.to_id()));
}

#[test]
fn test_boot_ranch_listener_failed() {
    let mut entry = create_test_entry(
        "Failed to start Ranch listener rabbit_web_mqtt_listener_tls in ranch_ssl:listen([{port,15676}]) for reason eaddrinuse (address already in use)",
        Severity::Error,
    );
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Boot.to_id()));
}

#[test]
fn test_boot_starting_rabbit_node_monitor() {
    let mut entry = create_test_entry("Starting rabbit_node_monitor", Severity::Debug);
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Boot.to_id()));
}

#[test]
fn test_boot_seeding_cluster_tags() {
    let mut entry = create_test_entry(
        "Seeding cluster tags from application environment",
        Severity::Info,
    );
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Boot.to_id()));
}
