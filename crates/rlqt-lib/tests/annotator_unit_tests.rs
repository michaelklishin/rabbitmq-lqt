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

use chrono::Utc;
use rlqt_lib::constants::{ALARMS_DOC_URL_ID, METADATA_STORE_DOC_URL_ID};
use rlqt_lib::entry_metadata::Annotator;
use rlqt_lib::entry_metadata::annotate_labels;
use rlqt_lib::entry_metadata::doc_url_annotators::{
    DocUrlAnnotator, FreeDiskSpaceAlarmDocAnnotator, MetadataStoreDocAnnotator, annotate_doc_urls,
};
use rlqt_lib::entry_metadata::labels::LogEntryLabels;
use rlqt_lib::entry_metadata::resolution_url_annotators::annotate_resolution_or_discussion_urls;
use rlqt_lib::entry_metadata::subsystem_annotators::{
    AccessControlAnnotator, BootAnnotator, FeatureFlagsAnnotator, MetadataStoreAnnotator,
    PeerDiscoveryAnnotator, PluginsAnnotator, RaftBasedAnnotator, SubsystemAnnotator,
    annotate_subsystems,
};
use rlqt_lib::entry_metadata::subsystems::Subsystem;
use rlqt_lib::{ParsedLogEntry, Severity};
use test_helpers::create_test_entry;

#[test]
fn test_subsystem_display() {
    assert_eq!(Subsystem::MetadataStore.to_string(), "metadata_store");
    assert_eq!(Subsystem::FeatureFlags.to_string(), "feature_flags");
    assert_eq!(Subsystem::Boot.to_string(), "boot");
    assert_eq!(Subsystem::Raft.to_string(), "raft");
    assert_eq!(Subsystem::PeerDiscovery.to_string(), "peer_discovery");
    assert_eq!(Subsystem::Plugins.to_string(), "plugins");
    assert_eq!(Subsystem::AccessControl.to_string(), "access_control");
}

#[test]
fn test_subsystem_from_str() {
    assert_eq!(
        "metadata_store".parse::<Subsystem>().unwrap(),
        Subsystem::MetadataStore
    );
    assert_eq!(
        "feature_flags".parse::<Subsystem>().unwrap(),
        Subsystem::FeatureFlags
    );
    assert_eq!("boot".parse::<Subsystem>().unwrap(), Subsystem::Boot);
    assert_eq!("raft".parse::<Subsystem>().unwrap(), Subsystem::Raft);
    assert_eq!(
        "peer_discovery".parse::<Subsystem>().unwrap(),
        Subsystem::PeerDiscovery
    );
    assert_eq!("plugins".parse::<Subsystem>().unwrap(), Subsystem::Plugins);
    assert_eq!(
        "access_control".parse::<Subsystem>().unwrap(),
        Subsystem::AccessControl
    );
    assert!("unknown".parse::<Subsystem>().is_err());
}

#[test]
fn test_metadata_store_annotator_matches() {
    let entry = create_test_entry(
        "RabbitMQ metadata store: ra_log:init recovered",
        Severity::Debug,
    );

    let annotator = MetadataStoreAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_metadata_store_annotator_no_match() {
    let entry = ParsedLogEntry {
        sequence_id: 0,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Info,
        process_id: "<0.208.0>".to_string(),
        message: "Some other message".to_string(),
        message_lowercased: "Some other message".to_lowercase(),
        subsystem_id: None,
        labels: LogEntryLabels::default(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    let annotator = MetadataStoreAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_metadata_store_annotator_annotates() {
    let mut entry = ParsedLogEntry {
        sequence_id: 0,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Debug,
        process_id: "<0.246.0>".to_string(),
        message: "RabbitMQ metadata store: post_init -> recover".to_string(),
        message_lowercased: "RabbitMQ metadata store: post_init -> recover".to_lowercase(),
        subsystem_id: None,
        labels: LogEntryLabels::default(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    let annotator = MetadataStoreAnnotator;
    annotator.annotate(&mut entry);

    assert_eq!(entry.subsystem_id, Some(Subsystem::MetadataStore.to_id()));
}

#[test]
fn test_feature_flags_annotator_matches() {
    let entry = ParsedLogEntry {
        sequence_id: 0,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Debug,
        process_id: "<0.264.0>".to_string(),
        message: "Feature flags: controller standing by".to_string(),
        message_lowercased: "Feature flags: controller standing by".to_lowercase(),
        subsystem_id: None,
        labels: LogEntryLabels::default(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    let annotator = FeatureFlagsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_feature_flags_annotator_no_match() {
    let entry = ParsedLogEntry {
        sequence_id: 0,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Info,
        process_id: "<0.208.0>".to_string(),
        message: "Some other message".to_string(),
        message_lowercased: "Some other message".to_lowercase(),
        subsystem_id: None,
        labels: LogEntryLabels::default(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    let annotator = FeatureFlagsAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_feature_flags_annotator_annotates() {
    let mut entry = ParsedLogEntry {
        sequence_id: 0,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Debug,
        process_id: "<0.264.0>".to_string(),
        message: "Feature flags: REFRESHING after applications load...".to_string(),
        message_lowercased: "Feature flags: REFRESHING after applications load...".to_lowercase(),
        subsystem_id: None,
        labels: LogEntryLabels::default(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    let annotator = FeatureFlagsAnnotator;
    annotator.annotate(&mut entry);

    assert_eq!(entry.subsystem_id, Some(Subsystem::FeatureFlags.to_id()));
}

#[test]
fn test_annotate_entry_with_metadata_store() {
    let mut entry = ParsedLogEntry {
        sequence_id: 0,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Notice,
        process_id: "<0.246.0>".to_string(),
        message: "RabbitMQ metadata store: candidate -> leader in term: 1".to_string(),
        message_lowercased: "RabbitMQ metadata store: candidate -> leader in term: 1"
            .to_lowercase(),
        subsystem_id: None,
        labels: LogEntryLabels::default(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    annotate_subsystems(&mut entry);

    assert_eq!(entry.subsystem_id, Some(Subsystem::MetadataStore.to_id()));
}

#[test]
fn test_annotate_entry_with_feature_flags() {
    let mut entry = ParsedLogEntry {
        sequence_id: 0,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Debug,
        process_id: "<0.264.0>".to_string(),
        message: "Feature flags: registering controller globally".to_string(),
        message_lowercased: "Feature flags: registering controller globally".to_lowercase(),
        subsystem_id: None,
        labels: LogEntryLabels::default(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    annotate_subsystems(&mut entry);

    assert_eq!(entry.subsystem_id, Some(Subsystem::FeatureFlags.to_id()));
}

#[test]
fn test_annotate_entry_no_match() {
    let mut entry = ParsedLogEntry {
        sequence_id: 0,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Info,
        process_id: "<0.208.0>".to_string(),
        message: "Regular log message".to_string(),
        message_lowercased: "Regular log message".to_lowercase(),
        subsystem_id: None,
        labels: LogEntryLabels::default(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    annotate_subsystems(&mut entry);

    assert_eq!(entry.subsystem_id, None);
}

#[test]
fn test_metadata_store_annotator_mnesia_khepri_data_copy() {
    let entry = ParsedLogEntry {
        sequence_id: 0,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Info,
        process_id: "<0.300.0>".to_string(),
        message: "Mnesia->Khepri data copy: starting sync".to_string(),
        message_lowercased: "Mnesia->Khepri data copy: starting sync".to_lowercase(),
        subsystem_id: None,
        labels: LogEntryLabels::default(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    let annotator = MetadataStoreAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_metadata_store_annotator_syncing_mnesia_khepri() {
    let entry = ParsedLogEntry {
        sequence_id: 0,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Info,
        process_id: "<0.300.0>".to_string(),
        message: "Syncing Mnesia->Khepri database state".to_string(),
        message_lowercased: "Syncing Mnesia->Khepri database state".to_lowercase(),
        subsystem_id: None,
        labels: LogEntryLabels::default(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    let annotator = MetadataStoreAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_boot_annotator_applying_mfa() {
    let entry = ParsedLogEntry {
        sequence_id: 0,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Info,
        process_id: "<0.100.0>".to_string(),
        message: "Applying MFA: {rabbit, start, []}".to_string(),
        message_lowercased: "Applying MFA: {rabbit, start, []}".to_lowercase(),
        subsystem_id: None,
        labels: LogEntryLabels::default(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    let annotator = BootAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_boot_annotator_finished_mfa() {
    let entry = ParsedLogEntry {
        sequence_id: 0,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Info,
        process_id: "<0.100.0>".to_string(),
        message: "Finished MFA: {rabbit, start, []}".to_string(),
        message_lowercased: "Finished MFA: {rabbit, start, []}".to_lowercase(),
        subsystem_id: None,
        labels: LogEntryLabels::default(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    let annotator = BootAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_boot_annotator_running_boot_step() {
    let entry = ParsedLogEntry {
        sequence_id: 0,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Debug,
        process_id: "<0.100.0>".to_string(),
        message: "Running boot step database defined by app rabbit".to_string(),
        message_lowercased: "Running boot step database defined by app rabbit".to_lowercase(),
        subsystem_id: None,
        labels: LogEntryLabels::default(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    let annotator = BootAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_boot_annotator_no_match() {
    let entry = ParsedLogEntry {
        sequence_id: 0,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Info,
        process_id: "<0.208.0>".to_string(),
        message: "Some other message".to_string(),
        message_lowercased: "Some other message".to_lowercase(),
        subsystem_id: None,
        labels: LogEntryLabels::default(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    let annotator = BootAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_boot_annotator_annotates() {
    let mut entry = ParsedLogEntry {
        sequence_id: 0,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Info,
        process_id: "<0.100.0>".to_string(),
        message: "Applying MFA: {rabbit, start, []}".to_string(),
        message_lowercased: "Applying MFA: {rabbit, start, []}".to_lowercase(),
        subsystem_id: None,
        labels: LogEntryLabels::default(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    let annotator = BootAnnotator;
    annotator.annotate(&mut entry);

    assert_eq!(entry.subsystem_id, Some(Subsystem::Boot.to_id()));
}

#[test]
fn test_annotate_entry_with_boot() {
    let mut entry = ParsedLogEntry {
        sequence_id: 0,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Info,
        process_id: "<0.100.0>".to_string(),
        message: "Running boot step init_config".to_string(),
        message_lowercased: "Running boot step init_config".to_lowercase(),
        subsystem_id: None,
        labels: LogEntryLabels::default(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    annotate_subsystems(&mut entry);

    assert_eq!(entry.subsystem_id, Some(Subsystem::Boot.to_id()));
}

#[test]
fn test_boot_annotator_boot_steps() {
    let entry = ParsedLogEntry {
        sequence_id: 0,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Info,
        process_id: "<0.100.0>".to_string(),
        message: "Boot steps: initialization complete".to_string(),
        message_lowercased: "Boot steps: initialization complete".to_lowercase(),
        subsystem_id: None,
        labels: LogEntryLabels::default(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    let annotator = BootAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_raft_annotator_wal_lowercase() {
    let entry = ParsedLogEntry {
        sequence_id: 0,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Debug,
        process_id: "<0.250.0>".to_string(),
        message: "wal: writing entry to disk".to_string(),
        message_lowercased: "wal: writing entry to disk".to_lowercase(),
        subsystem_id: None,
        labels: LogEntryLabels::default(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    let annotator = RaftBasedAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_raft_annotator_wal_uppercase() {
    let entry = ParsedLogEntry {
        sequence_id: 0,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Debug,
        process_id: "<0.250.0>".to_string(),
        message: "WAL: sync completed".to_string(),
        message_lowercased: "WAL: sync completed".to_lowercase(),
        subsystem_id: None,
        labels: LogEntryLabels::default(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    let annotator = RaftBasedAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_raft_annotator_ra_system() {
    let entry = ParsedLogEntry {
        sequence_id: 0,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Info,
        process_id: "<0.250.0>".to_string(),
        message: "Ra system started successfully".to_string(),
        message_lowercased: "Ra system started successfully".to_lowercase(),
        subsystem_id: None,
        labels: LogEntryLabels::default(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    let annotator = RaftBasedAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_raft_annotator_ra_colon() {
    let entry = ParsedLogEntry {
        sequence_id: 0,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Debug,
        process_id: "<0.250.0>".to_string(),
        message: "ra: processing command".to_string(),
        message_lowercased: "ra: processing command".to_lowercase(),
        subsystem_id: None,
        labels: LogEntryLabels::default(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    let annotator = RaftBasedAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_raft_annotator_starting_ra_system() {
    let entry = ParsedLogEntry {
        sequence_id: 0,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Info,
        process_id: "<0.250.0>".to_string(),
        message: "starting Ra system: coordination".to_string(),
        message_lowercased: "starting Ra system: coordination".to_lowercase(),
        subsystem_id: None,
        labels: LogEntryLabels::default(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    let annotator = RaftBasedAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_raft_annotator_ra_log_underscore() {
    let entry = ParsedLogEntry {
        sequence_id: 0,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Debug,
        process_id: "<0.250.0>".to_string(),
        message: "ra_log_wal: writing entries".to_string(),
        message_lowercased: "ra_log_wal: writing entries".to_lowercase(),
        subsystem_id: None,
        labels: LogEntryLabels::default(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    let annotator = RaftBasedAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_raft_annotator_no_match() {
    let entry = ParsedLogEntry {
        sequence_id: 0,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Info,
        process_id: "<0.208.0>".to_string(),
        message: "Some other message".to_string(),
        message_lowercased: "Some other message".to_lowercase(),
        subsystem_id: None,
        labels: LogEntryLabels::default(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    let annotator = RaftBasedAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_raft_annotator_annotates() {
    let mut entry = ParsedLogEntry {
        sequence_id: 0,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Debug,
        process_id: "<0.250.0>".to_string(),
        message: "ra: leader elected".to_string(),
        message_lowercased: "ra: leader elected".to_lowercase(),
        subsystem_id: None,
        labels: LogEntryLabels::default(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    let annotator = RaftBasedAnnotator;
    annotator.annotate(&mut entry);

    assert_eq!(entry.subsystem_id, Some(Subsystem::Raft.to_id()));
}

#[test]
fn test_annotate_entry_with_raft() {
    let mut entry = ParsedLogEntry {
        sequence_id: 0,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Info,
        process_id: "<0.250.0>".to_string(),
        message: "Ra system coordination initialized".to_string(),
        message_lowercased: "Ra system coordination initialized".to_lowercase(),
        subsystem_id: None,
        labels: LogEntryLabels::default(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    annotate_subsystems(&mut entry);

    assert_eq!(entry.subsystem_id, Some(Subsystem::Raft.to_id()));
}

#[test]
fn test_peer_discovery_annotator_matches() {
    let entry = ParsedLogEntry {
        sequence_id: 0,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Info,
        process_id: "<0.300.0>".to_string(),
        message: "Peer discovery: backend registered".to_string(),
        message_lowercased: "Peer discovery: backend registered".to_lowercase(),
        subsystem_id: None,
        labels: LogEntryLabels::default(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    let annotator = PeerDiscoveryAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_peer_discovery_annotator_no_match() {
    let entry = ParsedLogEntry {
        sequence_id: 0,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Info,
        process_id: "<0.208.0>".to_string(),
        message: "Some other message".to_string(),
        message_lowercased: "Some other message".to_lowercase(),
        subsystem_id: None,
        labels: LogEntryLabels::default(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    let annotator = PeerDiscoveryAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_peer_discovery_annotator_annotates() {
    let mut entry = ParsedLogEntry {
        sequence_id: 0,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Info,
        process_id: "<0.300.0>".to_string(),
        message: "Peer discovery: cluster formation started".to_string(),
        message_lowercased: "Peer discovery: cluster formation started".to_lowercase(),
        subsystem_id: None,
        labels: LogEntryLabels::default(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    let annotator = PeerDiscoveryAnnotator;
    annotator.annotate(&mut entry);

    assert_eq!(entry.subsystem_id, Some(Subsystem::PeerDiscovery.to_id()));
}

#[test]
fn test_annotate_entry_with_peer_discovery() {
    let mut entry = ParsedLogEntry {
        sequence_id: 0,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Info,
        process_id: "<0.300.0>".to_string(),
        message: "Peer discovery: backend registered".to_string(),
        message_lowercased: "Peer discovery: backend registered".to_lowercase(),
        subsystem_id: None,
        labels: LogEntryLabels::default(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    annotate_subsystems(&mut entry);

    assert_eq!(entry.subsystem_id, Some(Subsystem::PeerDiscovery.to_id()));
}

#[test]
fn test_plugins_annotator_loading_plugins() {
    let entry = ParsedLogEntry {
        sequence_id: 0,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Info,
        process_id: "<0.350.0>".to_string(),
        message: "Loading the following plugins: rabbitmq_management".to_string(),
        message_lowercased: "Loading the following plugins: rabbitmq_management".to_lowercase(),
        subsystem_id: None,
        labels: LogEntryLabels::default(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    let annotator = PluginsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_plugins_annotator_setting_plugins_up() {
    let entry = ParsedLogEntry {
        sequence_id: 0,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Info,
        process_id: "<0.350.0>".to_string(),
        message: "Setting plugins up".to_string(),
        message_lowercased: "Setting plugins up".to_lowercase(),
        subsystem_id: None,
        labels: LogEntryLabels::default(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    let annotator = PluginsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_plugins_annotator_no_match() {
    let entry = ParsedLogEntry {
        sequence_id: 0,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Info,
        process_id: "<0.208.0>".to_string(),
        message: "Some other message".to_string(),
        message_lowercased: "Some other message".to_lowercase(),
        subsystem_id: None,
        labels: LogEntryLabels::default(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    let annotator = PluginsAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_plugins_annotator_annotates() {
    let mut entry = ParsedLogEntry {
        sequence_id: 0,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Info,
        process_id: "<0.350.0>".to_string(),
        message: "Loading the following plugins: rabbitmq_management, rabbitmq_prometheus"
            .to_string(),
        message_lowercased:
            "Loading the following plugins: rabbitmq_management, rabbitmq_prometheus".to_lowercase(),
        subsystem_id: None,
        labels: LogEntryLabels::default(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    let annotator = PluginsAnnotator;
    annotator.annotate(&mut entry);

    assert_eq!(entry.subsystem_id, Some(Subsystem::Plugins.to_id()));
}

#[test]
fn test_annotate_entry_with_plugins() {
    let mut entry = ParsedLogEntry {
        sequence_id: 0,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Info,
        process_id: "<0.350.0>".to_string(),
        message: "Setting plugins up".to_string(),
        message_lowercased: "Setting plugins up".to_lowercase(),
        subsystem_id: None,
        labels: LogEntryLabels::default(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    annotate_subsystems(&mut entry);

    assert_eq!(entry.subsystem_id, Some(Subsystem::Plugins.to_id()));
}

#[test]
fn test_access_control_annotator_matches() {
    let entry = ParsedLogEntry {
        sequence_id: 0,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Info,
        process_id: "<0.400.0>".to_string(),
        message: "User 'admin' authenticated successfully by backend rabbit_auth_backend_internal"
            .to_string(),
        message_lowercased:
            "User 'admin' authenticated successfully by backend rabbit_auth_backend_internal"
                .to_lowercase(),
        subsystem_id: None,
        labels: LogEntryLabels::default(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    let annotator = AccessControlAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_access_control_annotator_no_match() {
    let entry = ParsedLogEntry {
        sequence_id: 0,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Info,
        process_id: "<0.208.0>".to_string(),
        message: "Some other message".to_string(),
        message_lowercased: "Some other message".to_lowercase(),
        subsystem_id: None,
        labels: LogEntryLabels::default(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    let annotator = AccessControlAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_access_control_annotator_annotates() {
    let mut entry = ParsedLogEntry {
        sequence_id: 0,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Info,
        process_id: "<0.400.0>".to_string(),
        message: "Connection authenticated successfully by mechanism PLAIN".to_string(),
        message_lowercased: "Connection authenticated successfully by mechanism PLAIN"
            .to_lowercase(),
        subsystem_id: None,
        labels: LogEntryLabels::default(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    let annotator = AccessControlAnnotator;
    annotator.annotate(&mut entry);

    assert_eq!(entry.subsystem_id, Some(Subsystem::AccessControl.to_id()));
}

#[test]
fn test_annotate_entry_with_access_control() {
    let mut entry = ParsedLogEntry {
        sequence_id: 0,
        explicit_id: None,
        timestamp: Utc::now(),
        severity: Severity::Info,
        process_id: "<0.400.0>".to_string(),
        message: "User 'guest' authenticated successfully by internal auth".to_string(),
        message_lowercased: "User 'guest' authenticated successfully by internal auth"
            .to_lowercase(),
        subsystem_id: None,
        labels: LogEntryLabels::default(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    };

    annotate_subsystems(&mut entry);

    assert_eq!(entry.subsystem_id, Some(Subsystem::AccessControl.to_id()));
}

#[test]
fn test_metadata_store_doc_annotator_matches() {
    let mut entry = create_test_entry(
        "RabbitMQ metadata store: ra_log:init recovered",
        Severity::Debug,
    );
    entry.subsystem_id = Some(Subsystem::MetadataStore.to_id());

    let annotator = MetadataStoreDocAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_metadata_store_doc_annotator_mnesia_khepri() {
    let mut entry = create_test_entry("Mnesia->Khepri data copy: starting sync", Severity::Info);
    entry.subsystem_id = Some(Subsystem::MetadataStore.to_id());

    let annotator = MetadataStoreDocAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_metadata_store_doc_annotator_no_match() {
    let entry = create_test_entry("Some other message", Severity::Info);

    let annotator = MetadataStoreDocAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_metadata_store_doc_annotator_annotates() {
    let mut entry = create_test_entry(
        "RabbitMQ metadata store: post_init -> recover",
        Severity::Debug,
    );
    entry.subsystem_id = Some(Subsystem::MetadataStore.to_id());

    let annotator = MetadataStoreDocAnnotator;
    annotator.annotate(&mut entry);

    assert_eq!(entry.doc_url_id, Some(METADATA_STORE_DOC_URL_ID));
}

#[test]
fn test_free_disk_space_alarm_doc_annotator_matches() {
    let entry = create_test_entry("Disk free limit set to 50MB", Severity::Warning);

    let annotator = FreeDiskSpaceAlarmDocAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_free_disk_space_alarm_doc_annotator_no_match() {
    let entry = create_test_entry("Disk usage is normal", Severity::Info);

    let annotator = FreeDiskSpaceAlarmDocAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_free_disk_space_alarm_doc_annotator_annotates() {
    let mut entry = create_test_entry("Disk free limit set to 1000MB", Severity::Notice);

    let annotator = FreeDiskSpaceAlarmDocAnnotator;
    annotator.annotate(&mut entry);

    assert_eq!(entry.doc_url_id, Some(ALARMS_DOC_URL_ID));
}

#[test]
fn test_annotate_doc_urls_metadata_store() {
    let mut entry = create_test_entry(
        "RabbitMQ metadata store: candidate -> leader in term: 1",
        Severity::Notice,
    );
    entry.subsystem_id = Some(Subsystem::MetadataStore.to_id());

    annotate_doc_urls(&mut entry);

    assert_eq!(entry.doc_url_id, Some(METADATA_STORE_DOC_URL_ID));
}

#[test]
fn test_annotate_doc_urls_disk_alarm() {
    let mut entry = create_test_entry("Disk free limit set to 2000MB", Severity::Info);

    annotate_doc_urls(&mut entry);

    assert_eq!(entry.doc_url_id, Some(ALARMS_DOC_URL_ID));
}

#[test]
fn test_annotate_doc_urls_no_match() {
    let mut entry = create_test_entry("Regular log message", Severity::Info);

    annotate_doc_urls(&mut entry);

    assert_eq!(entry.doc_url_id, None);
}

#[test]
fn test_metadata_store_doc_annotator_requires_subsystem() {
    let mut entry = create_test_entry(
        "RabbitMQ metadata store: ra_log:init recovered",
        Severity::Debug,
    );

    let annotator = MetadataStoreDocAnnotator;
    assert!(!annotator.does_match(&entry));

    entry.subsystem_id = Some(Subsystem::MetadataStore.to_id());
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_doc_url_annotator_priority() {
    let mut entry = create_test_entry("Disk free limit set to 50MB", Severity::Warning);

    annotate_doc_urls(&mut entry);

    assert_eq!(entry.doc_url_id, Some(ALARMS_DOC_URL_ID));
}

#[test]
fn test_full_annotation_pipeline_with_doc_url() {
    let mut entry = create_test_entry(
        "RabbitMQ metadata store: candidate -> leader in term: 1",
        Severity::Notice,
    );

    assert_eq!(entry.subsystem_id, None);
    assert_eq!(entry.doc_url_id, None);
    assert_eq!(entry.resolution_or_discussion_url_id, None);

    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::MetadataStore.to_id()));

    entry.labels = annotate_labels(&entry);

    annotate_doc_urls(&mut entry);
    assert_eq!(entry.doc_url_id, Some(METADATA_STORE_DOC_URL_ID));
    assert_eq!(entry.resolution_or_discussion_url_id, None);
}

#[test]
fn test_full_annotation_pipeline_no_urls() {
    let mut entry = create_test_entry("Some random log message", Severity::Info);

    annotate_subsystems(&mut entry);
    entry.labels = annotate_labels(&entry);
    annotate_doc_urls(&mut entry);
    annotate_resolution_or_discussion_urls(&mut entry);

    assert_eq!(entry.subsystem_id, None);
    assert_eq!(entry.doc_url_id, None);
    assert_eq!(entry.resolution_or_discussion_url_id, None);
}

#[test]
fn test_doc_url_not_set_without_subsystem() {
    let mut entry = create_test_entry("RabbitMQ metadata store: some message", Severity::Info);

    annotate_doc_urls(&mut entry);

    assert_eq!(entry.doc_url_id, None);
}

#[test]
fn test_doc_url_not_overwritten_if_already_set() {
    let mut entry = create_test_entry("RabbitMQ metadata store: test", Severity::Info);
    entry.subsystem_id = Some(Subsystem::MetadataStore.to_id());
    entry.doc_url_id = Some(99);

    annotate_doc_urls(&mut entry);

    assert_eq!(entry.doc_url_id, Some(99));
}

#[test]
fn test_resolution_or_discussion_url_not_overwritten_if_already_set() {
    let mut entry = create_test_entry("Some message", Severity::Info);
    entry.resolution_or_discussion_url_id = Some(123);

    annotate_resolution_or_discussion_urls(&mut entry);

    assert_eq!(entry.resolution_or_discussion_url_id, Some(123));
}

#[test]
fn test_multiple_annotators_first_match_wins() {
    let mut entry = create_test_entry("Disk free limit set to 50GB", Severity::Info);

    annotate_doc_urls(&mut entry);

    assert_eq!(entry.doc_url_id, Some(ALARMS_DOC_URL_ID));
}

#[test]
fn test_annotation_order_subsystem_then_urls() {
    let mut entry = create_test_entry("RabbitMQ metadata store: initializing", Severity::Info);

    annotate_subsystems(&mut entry);
    annotate_doc_urls(&mut entry);

    assert_eq!(entry.subsystem_id, Some(Subsystem::MetadataStore.to_id()));
    assert_eq!(entry.doc_url_id, Some(METADATA_STORE_DOC_URL_ID));
}

#[test]
fn test_annotate_entry_sets_unlabelled_label_when_no_labels() {
    use rlqt_lib::entry_metadata::annotate_entry;

    let mut entry = create_test_entry("Some generic log message", Severity::Info);

    annotate_entry(&mut entry);

    assert!(entry.labels.contains(LogEntryLabels::UNLABELLED));
}

#[test]
fn test_annotate_entry_does_not_set_unlabelled_label_when_labels_exist() {
    use rlqt_lib::entry_metadata::annotate_entry;

    let mut entry = create_test_entry("User authenticated successfully by backend", Severity::Info);

    annotate_entry(&mut entry);

    assert!(!entry.labels.contains(LogEntryLabels::UNLABELLED));
    assert!(entry.labels.contains(LogEntryLabels::ACCESS_CONTROL));
}
