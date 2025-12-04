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
use rlqt_lib::entry_metadata::subsystem_annotators::annotate_subsystems;
use rlqt_lib::entry_metadata::subsystems::Subsystem;
use test_helpers::create_test_entry;

#[test]
fn test_metadata_store_rabbitmq_metadata_store() {
    let mut entry = create_test_entry(
        "RabbitMQ metadata store: leader call - leader not known",
        Severity::Warning,
    );
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::MetadataStore.to_id()));
}

#[test]
fn test_metadata_store_mnesia_khepri_data_copy() {
    let mut entry = create_test_entry(
        "Mnesia->Khepri data copy: Table `rabbit_user` does not exist, skipping its migration",
        Severity::Debug,
    );
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::MetadataStore.to_id()));
}

#[test]
fn test_metadata_store_syncing_mnesia_khepri() {
    let mut entry = create_test_entry("Syncing Mnesia->Khepri clusters membership", Severity::Info);
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::MetadataStore.to_id()));
}

#[test]
fn test_metadata_store_khepri_based() {
    let mut entry = create_test_entry("Khepri-based RabbitMQ metadata store ready", Severity::Info);
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::MetadataStore.to_id()));
}

#[test]
fn test_metadata_store_starting_khepri() {
    let mut entry = create_test_entry(
        "Starting Khepri-based RabbitMQ metadata store",
        Severity::Info,
    );
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::MetadataStore.to_id()));
}

#[test]
fn test_metadata_store_db_init() {
    let mut entry = create_test_entry("DB: initialize Khepri", Severity::Debug);
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::MetadataStore.to_id()));
}

#[test]
fn test_metadata_store_db_virgin() {
    let mut entry = create_test_entry("DB: this node is virgin: false", Severity::Debug);
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::MetadataStore.to_id()));
}

#[test]
fn test_metadata_store_starting_mnesia() {
    let mut entry = create_test_entry("Starting Mnesia", Severity::Info);
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::MetadataStore.to_id()));
}

#[test]
fn test_metadata_store_waiting_for_mnesia_tables() {
    let mut entry = create_test_entry(
        "Waiting for Mnesia tables for 30000 ms, 9 retries left",
        Severity::Debug,
    );
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::MetadataStore.to_id()));
}

#[test]
fn test_metadata_store_successfully_synced_tables() {
    let mut entry = create_test_entry("Successfully synced tables from a peer", Severity::Info);
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::MetadataStore.to_id()));
}

#[test]
fn test_metadata_store_mnesia_khepri_cluster_sync() {
    let mut entry = create_test_entry(
        "Mnesia->Khepri cluster sync: Mnesia cluster: [rabbit@sunnyside]",
        Severity::Debug,
    );
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::MetadataStore.to_id()));
}
