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

//! Subsystem annotators for RabbitMQ log entries.
//!
//! This module contains annotators that identify which RabbitMQ subsystem a log entry
//! relates to (e.g., connections, queues, virtual hosts). Unlike label annotators,
//! subsystem annotators mutate the entry by setting its `subsystem_id` field.
//!
//! Each log entry can belong to at most one subsystem. The first matching annotator wins.

use crate::entry_metadata::annotator::Annotator;
use crate::entry_metadata::shared::{
    matches_cq_storage, matches_federation, matches_plugins, matches_policies, matches_shovels,
    matches_virtual_hosts,
};
use crate::entry_metadata::subsystems::Subsystem;
use crate::parser::ParsedLogEntry;
use regex::Regex;
use std::sync::LazyLock;

pub trait SubsystemAnnotator: Annotator {
    fn annotate(&self, entry: &mut ParsedLogEntry);
}

#[derive(Debug)]
pub struct MetadataStoreAnnotator;

impl Annotator for MetadataStoreAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.starts_with("rabbitmq metadata store:")
            || msg_lower.starts_with("mnesia->khepri data copy:")
            || msg_lower.starts_with("syncing mnesia->khepri")
            || msg_lower.starts_with("khepri-based")
            || msg_lower.starts_with("starting khepri")
            || msg_lower.starts_with("db: ")
            || msg_lower.starts_with("starting mnesia")
            || msg_lower.contains("cannot query members in store")
    }
}

impl SubsystemAnnotator for MetadataStoreAnnotator {
    fn annotate(&self, entry: &mut ParsedLogEntry) {
        entry.subsystem_id = Some(Subsystem::MetadataStore.to_id());
    }
}

#[derive(Debug)]
pub struct FeatureFlagsAnnotator;

impl Annotator for FeatureFlagsAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.starts_with("feature flags:")
    }
}

impl SubsystemAnnotator for FeatureFlagsAnnotator {
    fn annotate(&self, entry: &mut ParsedLogEntry) {
        entry.subsystem_id = Some(Subsystem::FeatureFlags.to_id());
    }
}

#[derive(Debug)]
pub struct BootAnnotator;

impl Annotator for BootAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.starts_with("applying mfa:")
            || msg_lower.starts_with("finished mfa:")
            || msg_lower.starts_with("running boot step")
            || msg_lower.starts_with("boot steps:")
            || msg_lower.starts_with("home dir")
            || msg_lower.contains("internal cluster id")
            || msg_lower.contains("boot state")
            || msg_lower.starts_with("logging: configured log handlers")
            || msg_lower.starts_with("fhc read buffering")
            || msg_lower.starts_with("fhc write buffering")
    }
}

impl SubsystemAnnotator for BootAnnotator {
    fn annotate(&self, entry: &mut ParsedLogEntry) {
        entry.subsystem_id = Some(Subsystem::Boot.to_id());
    }
}

#[derive(Debug)]
pub struct RaftBasedAnnotator;

impl Annotator for RaftBasedAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.starts_with("wal:")
            || msg_lower.starts_with("ra system")
            || msg_lower.starts_with("ra:")
            || msg_lower.starts_with("starting ra system:")
            || msg_lower.starts_with("ra_log_")
            || msg_lower.starts_with("trigger election in")
            || msg_lower.starts_with("ra_system_recover")
            || msg_lower.starts_with("segment_writer")
    }
}

impl SubsystemAnnotator for RaftBasedAnnotator {
    fn annotate(&self, entry: &mut ParsedLogEntry) {
        entry.subsystem_id = Some(Subsystem::Raft.to_id());
    }
}

#[derive(Debug)]
pub struct PeerDiscoveryAnnotator;

impl Annotator for PeerDiscoveryAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.starts_with("peer discovery:")
    }
}

impl SubsystemAnnotator for PeerDiscoveryAnnotator {
    fn annotate(&self, entry: &mut ParsedLogEntry) {
        entry.subsystem_id = Some(Subsystem::PeerDiscovery.to_id());
    }
}

#[derive(Debug)]
pub struct PluginsAnnotator;

impl Annotator for PluginsAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        matches_plugins(&entry.message_lowercased)
    }
}

impl SubsystemAnnotator for PluginsAnnotator {
    fn annotate(&self, entry: &mut ParsedLogEntry) {
        entry.subsystem_id = Some(Subsystem::Plugins.to_id());
    }
}

#[derive(Debug)]
pub struct AccessControlAnnotator;

impl Annotator for AccessControlAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("authenticated successfully by")
            || msg_lower.contains("successfully set permissions for user")
            || msg_lower.contains("asked to set permissions for user")
            || msg_lower.contains("sasl_not_supported")
    }
}

impl SubsystemAnnotator for AccessControlAnnotator {
    fn annotate(&self, entry: &mut ParsedLogEntry) {
        entry.subsystem_id = Some(Subsystem::AccessControl.to_id());
    }
}

static CONNECTION_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"connection\s+(?:\d+\.\d+\.\d+\.\d+|\[[0-9a-f:.]+\]):\d+\s+->\s+(?:\d+\.\d+\.\d+\.\d+|\[[0-9a-f:.]+\]):\d+",
    )
    .unwrap()
});

#[derive(Debug)]
pub struct ConnectionsAnnotator;

impl Annotator for ConnectionsAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;

        CONNECTION_PATTERN.is_match(msg_lower)
            || msg_lower.starts_with("client unexpectedly closed tcp connection")
            || msg_lower.starts_with("accepting amqp connection")
            || msg_lower.starts_with("accepting stomp connection")
            || msg_lower.starts_with("mqtt accepting")
            || msg_lower.starts_with("accepting web mqtt connection")
            || msg_lower.starts_with("closing amqp connection")
            || msg_lower.starts_with("closing connection")
            || msg_lower.starts_with("closing all connections")
            || msg_lower.contains("a connection exception connection_forced")
            || msg_lower.contains("client address during authn phase")
            || msg_lower.contains("amqp_network_connection")
            || msg_lower.contains("amqp_direct_connection")
    }
}

impl SubsystemAnnotator for ConnectionsAnnotator {
    fn annotate(&self, entry: &mut ParsedLogEntry) {
        entry.subsystem_id = Some(Subsystem::Connections.to_id());
    }
}

#[derive(Debug)]
pub struct ShovelPluginAnnotator;

impl Annotator for ShovelPluginAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        matches_shovels(&entry.message_lowercased)
    }
}

impl SubsystemAnnotator for ShovelPluginAnnotator {
    fn annotate(&self, entry: &mut ParsedLogEntry) {
        entry.subsystem_id = Some(Subsystem::ShovelPlugin.to_id());
    }
}

#[derive(Debug)]
pub struct ClassicQueuesAnnotator;

impl Annotator for ClassicQueuesAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        matches_cq_storage(&entry.message_lowercased)
    }
}

impl SubsystemAnnotator for ClassicQueuesAnnotator {
    fn annotate(&self, entry: &mut ParsedLogEntry) {
        entry.subsystem_id = Some(Subsystem::ClassicQueues.to_id());
    }
}

#[derive(Debug)]
pub struct VirtualHostsAnnotator;

impl Annotator for VirtualHostsAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        matches_virtual_hosts(&entry.message_lowercased)
    }
}

impl SubsystemAnnotator for VirtualHostsAnnotator {
    fn annotate(&self, entry: &mut ParsedLogEntry) {
        entry.subsystem_id = Some(Subsystem::VirtualHosts.to_id());
    }
}

#[derive(Debug)]
pub struct RuntimeParametersAnnotator;

impl Annotator for RuntimeParametersAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .contains("asked to set or update runtime parameter")
    }
}

impl SubsystemAnnotator for RuntimeParametersAnnotator {
    fn annotate(&self, entry: &mut ParsedLogEntry) {
        entry.subsystem_id = Some(Subsystem::RuntimeParameters.to_id());
    }
}

#[derive(Debug)]
pub struct FederationPluginAnnotator;

impl Annotator for FederationPluginAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        matches_federation(&entry.message_lowercased)
    }
}

impl SubsystemAnnotator for FederationPluginAnnotator {
    fn annotate(&self, entry: &mut ParsedLogEntry) {
        entry.subsystem_id = Some(Subsystem::FederationPlugin.to_id());
    }
}

#[derive(Debug)]
pub struct MqttPluginAnnotator;

impl Annotator for MqttPluginAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .contains("mqtt retained message store")
    }
}

impl SubsystemAnnotator for MqttPluginAnnotator {
    fn annotate(&self, entry: &mut ParsedLogEntry) {
        entry.subsystem_id = Some(Subsystem::MqttPlugin.to_id());
    }
}

#[derive(Debug)]
pub struct PoliciesSubsystemAnnotator;

impl Annotator for PoliciesSubsystemAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        matches_policies(&entry.message_lowercased)
    }
}

impl SubsystemAnnotator for PoliciesSubsystemAnnotator {
    fn annotate(&self, entry: &mut ParsedLogEntry) {
        entry.subsystem_id = Some(Subsystem::Policies.to_id());
    }
}

#[inline]
pub fn annotate_subsystems(entry: &mut ParsedLogEntry) -> &mut ParsedLogEntry {
    const ANNOTATORS: &[&dyn SubsystemAnnotator] = &[
        &MetadataStoreAnnotator,
        &FeatureFlagsAnnotator,
        &BootAnnotator,
        &RaftBasedAnnotator,
        &PeerDiscoveryAnnotator,
        &PluginsAnnotator,
        &AccessControlAnnotator,
        &ConnectionsAnnotator,
        &ShovelPluginAnnotator,
        &ClassicQueuesAnnotator,
        &VirtualHostsAnnotator,
        &RuntimeParametersAnnotator,
        &FederationPluginAnnotator,
        &MqttPluginAnnotator,
        &PoliciesSubsystemAnnotator,
    ];

    for annotator in ANNOTATORS {
        if annotator.does_match(entry) {
            annotator.annotate(entry);
            return entry;
        }
    }

    entry
}
