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
    matches_cq_storage, matches_federation, matches_management, matches_oauth2, matches_plugins,
    matches_policies, matches_raft, matches_shovels, matches_shutdown, matches_streams,
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
            || msg_lower.starts_with("mnesia->khepri cluster sync:")
            || msg_lower.starts_with("khepri-based")
            || msg_lower.starts_with("starting khepri")
            || msg_lower.starts_with("db: ")
            || msg_lower.starts_with("starting mnesia")
            || msg_lower.starts_with("waiting for mnesia tables")
            || msg_lower.contains("cannot query members in store")
            || msg_lower.starts_with("successfully synced tables")
            || msg_lower.starts_with("adding this node")
                && msg_lower.contains("to the remote node's cluster")
            || msg_lower.starts_with("cluster for store \"rabbitmq_metadata\"")
            || msg_lower.starts_with("detaching this node")
                && msg_lower.contains("from its cluster")
            || msg_lower.starts_with("deleting server rabbitmq_metadata")
            || msg_lower.contains("mnesia_event got {inconsistent_database")
            || msg_lower.starts_with("found the following metadata store members")
            || msg_lower.starts_with("trying to restart local ra server")
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
            || msg_lower.starts_with("fhc read buffering")
            || msg_lower.starts_with("fhc write buffering")
            || msg_lower.starts_with("== boot steps")
            || msg_lower.starts_with("== prelaunch")
            || msg_lower.starts_with("== postlaunch")
            || msg_lower.starts_with("== plugins")
            || msg_lower.starts_with("== clustering ==")
            || msg_lower.starts_with("boot failed")
            || msg_lower.starts_with("failed to start ranch listener")
            || msg_lower.contains("starting rabbit_node_monitor")
            || msg_lower.starts_with("marking rabbitmq as running")
            || msg_lower.contains("systemd")
            || msg_lower.starts_with("decoding encrypted config values")
            || msg_lower.starts_with("opening log file")
            || msg_lower.starts_with("webmachine_log_handler")
            || msg_lower.starts_with("files and directories found in node's data directory")
            || msg_lower.starts_with("prevent_startup_if_node_was_reset")
            || msg_lower.starts_with("'networking' boot step skipped")
            || msg_lower.starts_with("will use") && msg_lower.contains("processes for")
            || msg_lower.starts_with("starting worker pool")
            || msg_lower.starts_with("time to start rabbitmq:")
            || msg_lower.starts_with("register 'rabbit' process")
            || msg_lower.starts_with("will seed default virtual host and user")
            || msg_lower.starts_with("will not seed default virtual host and user")
            || msg_lower.starts_with("will use module")
                && msg_lower.contains("to import definitions")
            || msg_lower.starts_with("will try to load definitions from")
            || msg_lower.contains("there are fewer than target cluster size")
            || msg_lower.starts_with("applying definitions from file")
            || msg_lower.contains("[rabbit_quorum_queue:system_recover/1] rabbit not booted")
            || msg_lower.starts_with("running rabbit_prelaunch:shutdown_func()")
            || msg_lower.starts_with("seeding cluster tags from application environment")
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
        matches_raft(&entry.message_lowercased)
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
        let msg_lower = &entry.message_lowercased;
        msg_lower.starts_with("peer discovery:")
            || msg_lower.starts_with("classic peer discovery backend:")
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
            || msg_lower.contains("successfully set user tags for user")
            || msg_lower.contains("successfully cleared permissions for user")
            || msg_lower.contains("successfully cleared topic permissions")
            || msg_lower.contains("successfully set topic permissions")
            || msg_lower.contains("successfully changed password for user")
            || msg_lower.contains("clearing password for user")
            || msg_lower.contains("failed to add user")
            || msg_lower.contains("failed to delete user")
            || msg_lower.contains("failed to change password for user")
            || msg_lower.contains("failed to set tags for user")
            || msg_lower.contains("failed to set permissions for user")
            || msg_lower.contains("failed to clear permissions for user")
            || msg_lower.starts_with("created user")
            || msg_lower.starts_with("deleted user")
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
pub struct ShovelsAnnotator;

impl Annotator for ShovelsAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        matches_shovels(&entry.message_lowercased)
    }
}

impl SubsystemAnnotator for ShovelsAnnotator {
    fn annotate(&self, entry: &mut ParsedLogEntry) {
        entry.subsystem_id = Some(Subsystem::Shovels.to_id());
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
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("asked to set or update runtime parameter")
            || msg_lower.contains("setting global parameter")
            || msg_lower.contains("importing sequentially")
                && msg_lower.contains("global runtime parameters")
    }
}

impl SubsystemAnnotator for RuntimeParametersAnnotator {
    fn annotate(&self, entry: &mut ParsedLogEntry) {
        entry.subsystem_id = Some(Subsystem::RuntimeParameters.to_id());
    }
}

#[derive(Debug)]
pub struct FederationAnnotator;

impl Annotator for FederationAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        matches_federation(&entry.message_lowercased)
    }
}

impl SubsystemAnnotator for FederationAnnotator {
    fn annotate(&self, entry: &mut ParsedLogEntry) {
        entry.subsystem_id = Some(Subsystem::Federation.to_id());
    }
}

#[derive(Debug)]
pub struct MqttAnnotator;

impl Annotator for MqttAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .contains("mqtt retained message store")
    }
}

impl SubsystemAnnotator for MqttAnnotator {
    fn annotate(&self, entry: &mut ParsedLogEntry) {
        entry.subsystem_id = Some(Subsystem::Mqtt.to_id());
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

#[derive(Debug)]
pub struct MaintenanceModeAnnotator;

impl Annotator for MaintenanceModeAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("maintenance mode")
            || msg_lower.contains("resetting node maintenance status")
            || msg_lower.contains("unmarking the node as undergoing maintenance")
            || msg_lower.contains("marking the node as undergoing maintenance")
    }
}

impl SubsystemAnnotator for MaintenanceModeAnnotator {
    fn annotate(&self, entry: &mut ParsedLogEntry) {
        entry.subsystem_id = Some(Subsystem::MaintenanceMode.to_id());
    }
}

#[derive(Debug)]
pub struct ErlangOtpAnnotator;

impl Annotator for ErlangOtpAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("supervisor received unexpected message")
    }
}

impl SubsystemAnnotator for ErlangOtpAnnotator {
    fn annotate(&self, entry: &mut ParsedLogEntry) {
        entry.subsystem_id = Some(Subsystem::ErlangOtp.to_id());
    }
}

#[derive(Debug)]
pub struct ExchangesAnnotator;

impl Annotator for ExchangesAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.starts_with("consistent hashing exchange:")
    }
}

impl SubsystemAnnotator for ExchangesAnnotator {
    fn annotate(&self, entry: &mut ParsedLogEntry) {
        entry.subsystem_id = Some(Subsystem::Exchanges.to_id());
    }
}

#[derive(Debug)]
pub struct ChannelsAnnotator;

impl Annotator for ChannelsAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("consumer") && msg_lower.contains("timed out")
            || msg_lower.contains("consumer_timeout")
    }
}

impl SubsystemAnnotator for ChannelsAnnotator {
    fn annotate(&self, entry: &mut ParsedLogEntry) {
        entry.subsystem_id = Some(Subsystem::Channels.to_id());
    }
}

#[derive(Debug)]
pub struct ShutdownSubsystemAnnotator;

impl Annotator for ShutdownSubsystemAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.starts_with("rabbitmq is asked to stop")
            || msg_lower.starts_with("successfully stopped rabbitmq")
            || matches_shutdown(msg_lower)
    }
}

impl SubsystemAnnotator for ShutdownSubsystemAnnotator {
    fn annotate(&self, entry: &mut ParsedLogEntry) {
        entry.subsystem_id = Some(Subsystem::Shutdown.to_id());
    }
}

#[derive(Debug)]
pub struct ClusteringSubsystemAnnotator;

impl Annotator for ClusteringSubsystemAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.starts_with("autoheal:")
            || msg_lower.starts_with("autoheal request")
            || msg_lower.starts_with("autoheal finished")
            || msg_lower.starts_with("mirrored supervisor")
    }
}

impl SubsystemAnnotator for ClusteringSubsystemAnnotator {
    fn annotate(&self, entry: &mut ParsedLogEntry) {
        entry.subsystem_id = Some(Subsystem::Clustering.to_id());
    }
}

#[derive(Debug)]
pub struct LimitsSubsystemAnnotator;

impl Annotator for LimitsSubsystemAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        (msg_lower.contains("alarm")
            && (msg_lower.contains(" set") || msg_lower.contains(" cleared")))
            || msg_lower.contains("resource limit alarm")
            || msg_lower.contains("file descriptor limit alarm")
    }
}

impl SubsystemAnnotator for LimitsSubsystemAnnotator {
    fn annotate(&self, entry: &mut ParsedLogEntry) {
        entry.subsystem_id = Some(Subsystem::Limits.to_id());
    }
}

#[derive(Debug)]
pub struct LoggingSubsystemAnnotator;

impl Annotator for LoggingSubsystemAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.starts_with("logging: configured log handlers")
    }
}

impl SubsystemAnnotator for LoggingSubsystemAnnotator {
    fn annotate(&self, entry: &mut ParsedLogEntry) {
        entry.subsystem_id = Some(Subsystem::Logging.to_id());
    }
}

#[derive(Debug)]
pub struct StreamsSubsystemAnnotator;

impl Annotator for StreamsSubsystemAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        matches_streams(&entry.message_lowercased)
    }
}

impl SubsystemAnnotator for StreamsSubsystemAnnotator {
    fn annotate(&self, entry: &mut ParsedLogEntry) {
        entry.subsystem_id = Some(Subsystem::Streams.to_id());
    }
}

#[derive(Debug)]
pub struct QueuesSubsystemAnnotator;

impl Annotator for QueuesSubsystemAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.starts_with("will remove all queues from node")
            || msg_lower.contains("queue rebalance")
            || msg_lower.contains("starting queue rebalance")
            || msg_lower.contains("finished queue rebalance")
            || msg_lower.contains("migrating queue")
            || msg_lower.contains("error migrating queue")
            || (msg_lower.contains("queue") && msg_lower.contains("migrated to"))
    }
}

impl SubsystemAnnotator for QueuesSubsystemAnnotator {
    fn annotate(&self, entry: &mut ParsedLogEntry) {
        entry.subsystem_id = Some(Subsystem::Queues.to_id());
    }
}

#[derive(Debug)]
pub struct OAuth2SubsystemAnnotator;

impl Annotator for OAuth2SubsystemAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        matches_oauth2(&entry.message_lowercased)
    }
}

impl SubsystemAnnotator for OAuth2SubsystemAnnotator {
    fn annotate(&self, entry: &mut ParsedLogEntry) {
        entry.subsystem_id = Some(Subsystem::OAuth2.to_id());
    }
}

#[derive(Debug)]
pub struct ManagementSubsystemAnnotator;

impl Annotator for ManagementSubsystemAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        matches_management(&entry.message_lowercased)
    }
}

impl SubsystemAnnotator for ManagementSubsystemAnnotator {
    fn annotate(&self, entry: &mut ParsedLogEntry) {
        entry.subsystem_id = Some(Subsystem::Management.to_id());
    }
}

#[derive(Debug)]
pub struct MetricsSubsystemAnnotator;

impl Annotator for MetricsSubsystemAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("aggregated metrics")
            || msg_lower.contains("prometheus metrics:")
            || msg_lower.contains("global counters")
            || msg_lower.contains("message rates")
    }
}

impl SubsystemAnnotator for MetricsSubsystemAnnotator {
    fn annotate(&self, entry: &mut ParsedLogEntry) {
        entry.subsystem_id = Some(Subsystem::Metrics.to_id());
    }
}

#[derive(Debug)]
pub struct Amqp10SubsystemAnnotator;

impl Annotator for Amqp10SubsystemAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("amqp 1.0")
            || msg_lower.contains("sql expression")
            || msg_lower.contains("amqp10")
    }
}

impl SubsystemAnnotator for Amqp10SubsystemAnnotator {
    fn annotate(&self, entry: &mut ParsedLogEntry) {
        entry.subsystem_id = Some(Subsystem::Amqp10.to_id());
    }
}

#[inline]
pub fn annotate_subsystems(entry: &mut ParsedLogEntry) -> &mut ParsedLogEntry {
    const ANNOTATORS: &[&dyn SubsystemAnnotator] = &[
        &MaintenanceModeAnnotator,
        &ShutdownSubsystemAnnotator,
        &LimitsSubsystemAnnotator,
        &ClusteringSubsystemAnnotator,
        &LoggingSubsystemAnnotator,
        &StreamsSubsystemAnnotator,
        &QueuesSubsystemAnnotator,
        &MetadataStoreAnnotator,
        &FeatureFlagsAnnotator,
        &BootAnnotator,
        &RaftBasedAnnotator,
        &PeerDiscoveryAnnotator,
        &OAuth2SubsystemAnnotator,
        &ManagementSubsystemAnnotator,
        &MetricsSubsystemAnnotator,
        &PluginsAnnotator,
        &Amqp10SubsystemAnnotator,
        &AccessControlAnnotator,
        &ConnectionsAnnotator,
        &ShovelsAnnotator,
        &ClassicQueuesAnnotator,
        &VirtualHostsAnnotator,
        &RuntimeParametersAnnotator,
        &FederationAnnotator,
        &MqttAnnotator,
        &PoliciesSubsystemAnnotator,
        &ErlangOtpAnnotator,
        &ExchangesAnnotator,
        &ChannelsAnnotator,
    ];

    for annotator in ANNOTATORS {
        if annotator.does_match(entry) {
            annotator.annotate(entry);
            return entry;
        }
    }

    entry
}
