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

//! Label annotators for RabbitMQ log entries.
//!
//! This module contains annotators that examine log entries and apply semantic labels
//! (represented as bitflags in `LogEntryLabels`). Label annotators are read-only -
//! they check if an entry matches certain patterns and return which labels should be applied,
//! but they don't modify the entry itself.
//!
//! Each annotator is a zero-sized type implementing the `Annotator` trait.

use crate::entry_metadata::annotator::Annotator;
use crate::entry_metadata::labels::LogEntryLabels;
use crate::entry_metadata::shared::{
    matches_consumer_delivery_timeout, matches_cq_storage, matches_federation, matches_plugins,
    matches_policies, matches_shovels, matches_virtual_hosts,
};
use crate::parser::ParsedLogEntry;
use regex::Regex;
use std::sync::LazyLock;

pub trait LabelAnnotator: Annotator {
    fn annotate(&self, labels: &mut LogEntryLabels);
}

#[derive(Debug)]
pub struct ErlProcessCrashAnnotator;

impl Annotator for ErlProcessCrashAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.is_multiline()
            && (entry.message_lowercased.contains("crasher:")
                || entry.message_lowercased.contains("reason for termination"))
    }
}

impl LabelAnnotator for ErlProcessCrashAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::ERL_PROCESS_CRASH;
    }
}

#[derive(Debug)]
pub struct UndefinedFnAnnotator;

impl Annotator for UndefinedFnAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains(":undef,") || msg_lower.contains("undefined function")
    }
}

impl LabelAnnotator for UndefinedFnAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::UNDEFINED_FN;
    }
}

#[derive(Debug)]
pub struct ProcessStopsAnnotator;

impl Annotator for ProcessStopsAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.contains("terminating with reason")
            || entry.message_lowercased.contains("terminated with reason")
            || entry.message_lowercased.contains("stopped with reason")
            || entry.message_lowercased.contains("exiting with reason")
            || entry.message_lowercased.contains("shutdown with reason")
    }
}

impl LabelAnnotator for ProcessStopsAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::PROCESS_STOPS;
    }
}

#[derive(Debug)]
pub struct RaftBasedAnnotator;

impl Annotator for RaftBasedAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.contains("pre_vote")
            || entry.message_lowercased.contains("election called")
            || entry.message_lowercased.contains("election triggered")
            || entry.message_lowercased.contains("trigger election")
            || entry.message_lowercased.contains("ra_log:")
            || entry.message_lowercased.contains("vote granted for term")
            || entry
                .message_lowercased
                .contains("recovered -> follower in term")
            || entry
                .message_lowercased
                .contains("recover -> recovered in term")
            || entry
                .message_lowercased
                .contains("follower -> pre_vote in term")
            || entry
                .message_lowercased
                .contains("pre_vote -> candidate in term")
            || entry
                .message_lowercased
                .contains("candidate -> leader in term")
            || entry
                .message_lowercased
                .contains("recovery of state machine version")
            || entry
                .message_lowercased
                .contains("recovering state machine version")
            || entry
                .message_lowercased
                .contains("scanning for cluster changes")
            || entry.message_lowercased.contains("ra:")
            || entry.message_lowercased.contains("ra_log")
            || entry.message_lowercased.contains("starting ra system")
            || entry.message_lowercased.contains("ra system")
            || entry.message_lowercased.contains("ra: starting")
            || entry.message_lowercased.contains("ra_coordination")
            || entry.message_lowercased.contains("wal:")
            || entry.message_lowercased.contains("wal_")
            || entry.message_lowercased.contains("ra_system_recover")
            || entry.message_lowercased.contains("ra server")
            || entry.message_lowercased.contains("stopping member")
            || entry
                .message_lowercased
                .contains("metadata store member is caught up")
    }
}

impl LabelAnnotator for RaftBasedAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::RAFT;
    }
}

#[derive(Debug)]
pub struct ElectionsAnnotator;

impl Annotator for ElectionsAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("pre_vote")
            || msg_lower.contains("election called")
            || msg_lower.contains("election triggered")
            || msg_lower.contains("trigger election")
            || msg_lower.contains("vote granted for term")
            || msg_lower.contains("catch up on replication to the raft cluster leader")
    }
}

impl LabelAnnotator for ElectionsAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::ELECTIONS | LogEntryLabels::RAFT;
    }
}

static QUEUE_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"queue\s+'[^']+'\s+in\s+vhost").expect("QUEUE_PATTERN is a valid regex")
});

static AUTO_DELETE_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"all of its consumers \(\d+\) were on a channel that was closed")
        .expect("AUTO_DELETE_PATTERN is a valid regex")
});

static EXCLUSIVE_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"because its declaring connection <\d+\.\d+\.\d+> was closed")
        .expect("EXCLUSIVE_PATTERN is a valid regex")
});

#[derive(Debug)]
pub struct QueuesAnnotator;

impl Annotator for QueuesAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        QUEUE_PATTERN.is_match(msg_lower)
            || msg_lower.contains("finished rebuilding index")
            || msg_lower.contains("rebuilding message location index from")
            || msg_lower.contains("priority queues enabled")
            || msg_lower.contains("declaring queue")
            || msg_lower.contains("durable queue")
            || msg_lower.contains("queue index")
            || msg_lower.contains("queue indices")
    }
}

impl LabelAnnotator for QueuesAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::QUEUES;
    }
}

#[derive(Debug)]
pub struct AutoDeleteAnnotator;

impl Annotator for AutoDeleteAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        AUTO_DELETE_PATTERN.is_match(msg_lower) || msg_lower.contains("auto-delete queue")
    }
}

impl LabelAnnotator for AutoDeleteAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::AUTO_DELETE;
    }
}

#[derive(Debug)]
pub struct ExclusiveAnnotator;

impl Annotator for ExclusiveAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        EXCLUSIVE_PATTERN.is_match(msg_lower) || msg_lower.contains("exclusive queue")
    }
}

impl LabelAnnotator for ExclusiveAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::EXCLUSIVE;
    }
}

#[derive(Debug)]
pub struct ChannelExceptionsAnnotator;

impl Annotator for ChannelExceptionsAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .contains("channel error on connection")
    }
}

impl LabelAnnotator for ChannelExceptionsAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CHANNEL_EXCEPTIONS;
    }
}

#[derive(Debug)]
pub struct DeleteAnnotator;

impl Annotator for DeleteAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("terminating with reason 'delete'")
            || msg_lower.contains("because it's being deleted")
            || msg_lower.contains("deleting vhost")
    }
}

impl LabelAnnotator for DeleteAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::DELETE;
    }
}

#[derive(Debug)]
pub struct QueueFederationAnnotator;

impl Annotator for QueueFederationAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        matches_federation(&entry.message_lowercased)
    }
}

impl LabelAnnotator for QueueFederationAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::QUEUE_FEDERATION;
    }
}

#[derive(Debug)]
pub struct FederationAnnotator;

impl Annotator for FederationAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        matches_federation(&entry.message_lowercased)
    }
}

impl LabelAnnotator for FederationAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::FEDERATION;
    }
}

#[derive(Debug)]
pub struct VirtualHostsAnnotator;

impl Annotator for VirtualHostsAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        matches_virtual_hosts(&entry.message_lowercased)
    }
}

impl LabelAnnotator for VirtualHostsAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::VIRTUAL_HOSTS;
    }
}

#[derive(Debug)]
pub struct ConnectionsAnnotator;

impl Annotator for ConnectionsAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("authenticated successfully by backend")
            || msg_lower.contains("client address during authn phase")
            || msg_lower.contains("authenticated and granted access to vhost")
            || msg_lower.contains("accepting amqp connection")
            || msg_lower.contains("amqp_network_connection")
            || msg_lower.contains("amqp_direct_connection")
            || msg_lower.contains("handshake_timeout")
            || msg_lower.contains("failed to authenticate")
            || msg_lower.contains("connection_closed")
            || msg_lower.starts_with("started tcp listener")
            || msg_lower.starts_with("started tls")
            || msg_lower.contains("epmd monitor")
            || msg_lower.contains("inter-node communication")
            || msg_lower.contains("credit flow")
    }
}

impl LabelAnnotator for ConnectionsAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CONNECTIONS | LogEntryLabels::ACCESS_CONTROL;
    }
}

#[derive(Debug)]
pub struct ConnectionTrackingAnnotator;

impl Annotator for ConnectionTrackingAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("setting up a table for") && msg_lower.contains(" connection")
    }
}

impl LabelAnnotator for ConnectionTrackingAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CONNECTIONS;
    }
}

#[derive(Debug)]
pub struct ChannelTrackingAnnotator;

impl Annotator for ChannelTrackingAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("setting up a table for") && msg_lower.contains(" channel")
    }
}

impl LabelAnnotator for ChannelTrackingAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CHANNELS;
    }
}

#[derive(Debug)]
pub struct AccessControlAnnotator;

impl Annotator for AccessControlAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("asked to set permissions for user")
            || msg_lower.contains("successfully set permissions for user")
            || msg_lower.contains("sasl_not_supported")
            || msg_lower.contains("failed authentication by backend")
            || msg_lower.contains("failed to add user")
            || msg_lower.contains("asked to create a new user")
            || msg_lower.contains("asked to delete user")
            || msg_lower.contains("created user")
            || msg_lower.contains("deleted user")
            || msg_lower.contains("asked to set user tags")
            || msg_lower.contains("asked to clear permissions")
    }
}

impl LabelAnnotator for AccessControlAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::ACCESS_CONTROL;
    }
}

#[derive(Debug)]
pub struct ShovelsAnnotator;

impl Annotator for ShovelsAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        matches_shovels(&entry.message_lowercased)
    }
}

impl LabelAnnotator for ShovelsAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::SHOVELS;
    }
}

static CLOSING_CONNECTION_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"closing connection <\d+\.\d+\.\d+>")
        .expect("CLOSING_CONNECTION_PATTERN is a valid regex")
});

#[derive(Debug)]
pub struct CqStoresAnnotator;

impl Annotator for CqStoresAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        matches_cq_storage(&entry.message_lowercased)
    }
}

impl LabelAnnotator for CqStoresAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CQ_STORES;
    }
}

#[derive(Debug)]
pub struct DisconnectsAnnotator;

impl Annotator for DisconnectsAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        CLOSING_CONNECTION_PATTERN.is_match(msg_lower)
            || msg_lower.contains("closing amqp connection")
            || msg_lower.contains("closing all connections in vhost")
            || msg_lower.contains("closing all connections for user")
    }
}

impl LabelAnnotator for DisconnectsAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::DISCONNECTS;
    }
}

#[derive(Debug)]
pub struct DeletionProtectionAnnotator;

impl Annotator for DeletionProtectionAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .contains("is protected from deletion")
    }
}

impl LabelAnnotator for DeletionProtectionAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::DELETION_PROTECTION;
    }
}

#[derive(Debug)]
pub struct MultilineAnnotator;

impl Annotator for MultilineAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.is_multiline()
    }
}

impl LabelAnnotator for MultilineAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::MULTILINE;
    }
}

#[derive(Debug)]
pub struct StreamsAnnotator;

impl Annotator for StreamsAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("rabbit_stream") || msg_lower.contains("stream ")
    }
}

impl LabelAnnotator for StreamsAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::STREAMS;
    }
}

#[derive(Debug)]
pub struct LimitsAnnotator;

impl Annotator for LimitsAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.contains("file handles")
            || entry.message_lowercased.contains("memory high watermark")
            || entry.message_lowercased.contains("disk free limit")
            || entry
                .message_lowercased
                .contains("enabling free disk space monitoring")
    }
}

impl LabelAnnotator for LimitsAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::LIMITS;
    }
}

#[derive(Debug)]
pub struct WorkerPoolAnnotator;

impl Annotator for WorkerPoolAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.contains("worker pool")
            || entry.message_lowercased.contains("worker_pool")
    }
}

impl LabelAnnotator for WorkerPoolAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::WORKER_POOL;
    }
}

#[derive(Debug)]
pub struct PeerDiscoveryClassicAnnotator;

impl Annotator for PeerDiscoveryClassicAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .contains("rabbit_peer_discovery_classic_config")
    }
}

impl LabelAnnotator for PeerDiscoveryClassicAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::PEER_DISCOVERY_CLASSIC;
    }
}

#[derive(Debug)]
pub struct PluginsLabelAnnotator;

impl Annotator for PluginsLabelAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        matches_plugins(&entry.message_lowercased)
    }
}

impl LabelAnnotator for PluginsLabelAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::PLUGINS;
    }
}

#[derive(Debug)]
pub struct ExchangesAnnotator;

impl Annotator for ExchangesAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.contains("rabbit_exchange_")
    }
}

impl LabelAnnotator for ExchangesAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::EXCHANGES;
    }
}

#[derive(Debug)]
pub struct StartupBannerAnnotator;

impl Annotator for StartupBannerAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.is_multiline()
            && entry.message_lowercased.contains("config file(s)")
            && entry.message_lowercased.contains("home dir")
    }
}

impl LabelAnnotator for StartupBannerAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::STARTUP_BANNER;
    }
}

#[derive(Debug)]
pub struct ShutdownAnnotator;

impl Annotator for ShutdownAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.starts_with("stopped tcp listener")
            || msg_lower.starts_with("stopped tls")
            || msg_lower.starts_with("stopped ssl")
            || msg_lower.starts_with("stopped mqtt")
            || msg_lower.starts_with("stopped stomp")
            || msg_lower.contains("listener stopped")
    }
}

impl LabelAnnotator for ShutdownAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::SHUTDOWN;
    }
}

#[derive(Debug)]
pub struct DefinitionsAnnotator;

impl Annotator for DefinitionsAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("definition import")
            || msg_lower.contains("asked to import definitions")
            || (msg_lower.starts_with("importing") && msg_lower.contains("concurrently"))
    }
}

impl LabelAnnotator for DefinitionsAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::DEFINITIONS;
    }
}

#[derive(Debug)]
pub struct FeatureFlagsLabelAnnotator;

impl Annotator for FeatureFlagsLabelAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.starts_with("feature flag ") || msg_lower.starts_with("feature flags:")
    }
}

impl LabelAnnotator for FeatureFlagsLabelAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::FEATURE_FLAGS;
    }
}

#[derive(Debug)]
pub struct StompAnnotator;

impl Annotator for StompAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("rabbit_stomp")
            || msg_lower.contains("rabbit_web_stomp")
            || msg_lower.starts_with("started stomp")
            || msg_lower.starts_with("stopped stomp")
    }
}

impl LabelAnnotator for StompAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::STOMP;
    }
}

#[derive(Debug)]
pub struct WebSocketsAnnotator;

impl Annotator for WebSocketsAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("rabbit_web_stomp") || msg_lower.contains("rabbit_web_mqtt")
    }
}

impl LabelAnnotator for WebSocketsAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::WEBSOCKETS;
    }
}

#[derive(Debug)]
pub struct MqttAnnotator;

impl Annotator for MqttAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.starts_with("started mqtt")
            || msg_lower.starts_with("stopped mqtt")
            || msg_lower.contains("rabbit_web_mqtt")
    }
}

impl LabelAnnotator for MqttAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::MQTT;
    }
}

static CLUSTERING_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^node \S+ (up|down)$").expect("CLUSTERING_PATTERN is a valid regex")
});

#[derive(Debug)]
pub struct ClusteringAnnotator;

impl Annotator for ClusteringAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        CLUSTERING_PATTERN.is_match(msg_lower)
            || msg_lower.contains("inter-node communication")
            || msg_lower.contains("inter-node tls")
            || msg_lower.contains("maintenance mode")
            || msg_lower.contains("draining")
            || msg_lower.starts_with("prevent_startup_if_node_was_reset")
            || msg_lower.starts_with("peer discovery:")
    }
}

impl LabelAnnotator for ClusteringAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CLUSTERING;
    }
}

#[derive(Debug)]
pub struct MetricsAnnotator;

impl Annotator for MetricsAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("statistics database")
            || msg_lower.contains("management plugin:")
            || msg_lower.contains("prometheus metrics:")
            || msg_lower.contains("global counters")
            || msg_lower.contains("message rates")
    }
}

impl LabelAnnotator for MetricsAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::METRICS;
    }
}

#[derive(Debug)]
pub struct TlsAnnotator;

impl Annotator for TlsAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("inter-node tls")
            || msg_lower.contains("tls options")
            || msg_lower.contains("tls connection")
            || msg_lower.contains("ssl options")
            || msg_lower.contains("ssl connection")
            || msg_lower.contains("client certificates")
    }
}

impl LabelAnnotator for TlsAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::TLS;
    }
}

#[derive(Debug)]
pub struct QuorumQueuesAnnotator;

impl Annotator for QuorumQueuesAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("all queue leaders are balanced")
            || msg_lower.contains("all leaders balanced")
            || msg_lower.contains("rebalancing leader")
            || msg_lower.contains("leader balanced")
            || msg_lower.contains("leader rebalanced")
            || msg_lower.contains("quorum queue")
            || msg_lower.starts_with("rabbit_quorum_queue: ")
    }
}

impl LabelAnnotator for QuorumQueuesAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::QUORUM_QUEUES | LogEntryLabels::QUEUES | LogEntryLabels::RAFT;
    }
}

#[derive(Debug)]
pub struct NetworkingAnnotator;

impl Annotator for NetworkingAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("http listener registry")
            || msg_lower.contains("listener stopped")
            || msg_lower.contains("listener started")
    }
}

impl LabelAnnotator for NetworkingAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::NETWORKING;
    }
}

#[derive(Debug)]
pub struct ClassicQueuesAnnotator;

impl Annotator for ClassicQueuesAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("classic queue")
            || msg_lower.starts_with("mirrored ")
            || msg_lower.contains("classic queue mirroring")
    }
}

impl LabelAnnotator for ClassicQueuesAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CLASSIC_QUEUES | LogEntryLabels::QUEUES;
    }
}

#[derive(Debug)]
pub struct PoliciesAnnotator;

impl Annotator for PoliciesAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        matches_policies(&entry.message_lowercased)
    }
}

impl LabelAnnotator for PoliciesAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::POLICIES;
    }
}

#[derive(Debug)]
pub struct TimeoutsAnnotator;

impl Annotator for TimeoutsAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg = &entry.message_lowercased;
        msg.contains("timeout") || msg.contains("timed out")
    }
}

impl LabelAnnotator for TimeoutsAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::TIMEOUTS;
    }
}

#[derive(Debug)]
pub struct ConsumerDeliveryTimeoutAnnotator;

impl Annotator for ConsumerDeliveryTimeoutAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        matches_consumer_delivery_timeout(&entry.message_lowercased)
    }
}

impl LabelAnnotator for ConsumerDeliveryTimeoutAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CONSUMERS | LogEntryLabels::TIMEOUTS;
    }
}

#[derive(Debug)]
pub struct DeprecatedFeaturesAnnotator;

impl Annotator for DeprecatedFeaturesAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.starts_with("deprecated features:")
    }
}

impl LabelAnnotator for DeprecatedFeaturesAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::DEPRECATED_FEATURES;
    }
}

#[derive(Debug)]
pub struct MaintenanceModeLabelAnnotator;

impl Annotator for MaintenanceModeLabelAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("maintenance mode")
            || msg_lower.contains("resetting node maintenance status")
            || msg_lower.contains("unmarking the node as undergoing maintenance")
            || msg_lower.contains("marking the node as undergoing maintenance")
    }
}

impl LabelAnnotator for MaintenanceModeLabelAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::MAINTENANCE_MODE;
    }
}

#[derive(Debug)]
pub struct KhepriAnnotator;

impl Annotator for KhepriAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("segment_writer in 'coordination'") || msg_lower.contains("khepri")
    }
}

impl LabelAnnotator for KhepriAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::KHEPRI;
    }
}

#[derive(Debug)]
pub struct SegmentWriterQuorumQueuesAnnotator;

impl Annotator for SegmentWriterQuorumQueuesAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .contains("segment_writer in 'quorum_queues'")
    }
}

impl LabelAnnotator for SegmentWriterQuorumQueuesAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::QUORUM_QUEUES | LogEntryLabels::RAFT;
    }
}

#[derive(Debug)]
pub struct SegmentWriterAnnotator;

impl Annotator for SegmentWriterAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.starts_with("segment_writer")
    }
}

impl LabelAnnotator for SegmentWriterAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::RAFT;
    }
}

#[derive(Debug)]
pub struct OsirisAnnotator;

impl Annotator for OsirisAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("osiris_writer") || msg_lower.contains("osiris_log")
    }
}

impl LabelAnnotator for OsirisAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::STREAMS;
    }
}

#[derive(Debug)]
pub struct RanchAnnotator;

impl Annotator for RanchAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.starts_with("started ranch")
            || msg_lower.starts_with("stopping ranch")
            || msg_lower.contains("http[s] listener")
            || msg_lower.contains("http listener")
    }
}

impl LabelAnnotator for RanchAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::NETWORKING;
    }
}

#[derive(Debug)]
pub struct ClientProvidedNameAnnotator;

impl Annotator for ClientProvidedNameAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .contains("has a client-provided name")
    }
}

impl LabelAnnotator for ClientProvidedNameAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CONNECTIONS;
    }
}

#[derive(Debug)]
pub struct DeletingPidFileAnnotator;

impl Annotator for DeletingPidFileAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.starts_with("deleting pid file")
    }
}

impl LabelAnnotator for DeletingPidFileAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::SHUTDOWN;
    }
}

#[derive(Debug)]
pub struct DeletingUnknownFilesAnnotator;

impl Annotator for DeletingUnknownFilesAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .starts_with("deleting unknown files/folders")
    }
}

impl LabelAnnotator for DeletingUnknownFilesAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::RAFT;
    }
}

#[derive(Debug)]
pub struct ReadyToStartListenersAnnotator;

impl Annotator for ReadyToStartListenersAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .starts_with("ready to start client connection listeners")
    }
}

impl LabelAnnotator for ReadyToStartListenersAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CONNECTIONS | LogEntryLabels::NETWORKING;
    }
}

#[derive(Debug)]
pub struct TracingVhostAnnotator;

impl Annotator for TracingVhostAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("tracing") && msg_lower.contains("vhost")
    }
}

impl LabelAnnotator for TracingVhostAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::VIRTUAL_HOSTS;
    }
}

#[derive(Debug)]
pub struct RefreshingChannelsAnnotator;

impl Annotator for RefreshingChannelsAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .starts_with("refreshing state of channels")
    }
}

impl LabelAnnotator for RefreshingChannelsAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CHANNELS;
    }
}

#[derive(Debug)]
pub struct EpmdAnnotator;

impl Annotator for EpmdAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.starts_with("getting epmd port")
    }
}

impl LabelAnnotator for EpmdAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CLUSTERING | LogEntryLabels::NETWORKING;
    }
}

static RABBIT_ON_NODE_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^rabbit on node \S+ (up|down)$").expect("RABBIT_ON_NODE_PATTERN is a valid regex")
});

#[derive(Debug)]
pub struct RabbitOnNodeAnnotator;

impl Annotator for RabbitOnNodeAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        RABBIT_ON_NODE_PATTERN.is_match(&entry.message_lowercased)
    }
}

impl LabelAnnotator for RabbitOnNodeAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CLUSTERING;
    }
}

#[derive(Debug)]
pub struct VhostReconciliationAnnotator;

impl Annotator for VhostReconciliationAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .contains("virtual host process reconciliation")
    }
}

impl LabelAnnotator for VhostReconciliationAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::VIRTUAL_HOSTS;
    }
}

#[derive(Debug)]
pub struct SetStopReasonAnnotator;

impl Annotator for SetStopReasonAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.starts_with("set stop reason to")
    }
}

impl LabelAnnotator for SetStopReasonAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::SHUTDOWN;
    }
}

#[derive(Debug)]
pub struct MetadataStoreLeaderAnnotator;

impl Annotator for MetadataStoreLeaderAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.starts_with("rabbitmq metadata store:") && msg_lower.contains("leader")
    }
}

impl LabelAnnotator for MetadataStoreLeaderAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::RAFT | LogEntryLabels::ELECTIONS;
    }
}

#[derive(Debug)]
pub struct WillDeclareExchangeAnnotator;

impl Annotator for WillDeclareExchangeAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .starts_with("will declare an exchange")
    }
}

impl LabelAnnotator for WillDeclareExchangeAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::EXCHANGES;
    }
}

#[derive(Debug)]
pub struct AutohealAnnotator;

impl Annotator for AutohealAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.contains("autoheal")
    }
}

impl LabelAnnotator for AutohealAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CLUSTERING;
    }
}

#[derive(Debug)]
pub struct ClusterTagsAnnotator;

impl Annotator for ClusterTagsAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.contains("cluster tags")
    }
}

impl LabelAnnotator for ClusterTagsAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CLUSTERING;
    }
}

#[derive(Debug)]
pub struct CheckingClusterConsistencyAnnotator;

impl Annotator for CheckingClusterConsistencyAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .starts_with("checking cluster consistency")
    }
}

impl LabelAnnotator for CheckingClusterConsistencyAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CLUSTERING;
    }
}

#[inline]
pub fn annotate_labels(entry: &ParsedLogEntry) -> LogEntryLabels {
    const ANNOTATORS: &[&dyn LabelAnnotator] = &[
        &ErlProcessCrashAnnotator,
        &UndefinedFnAnnotator,
        &ProcessStopsAnnotator,
        &ElectionsAnnotator,
        &RaftBasedAnnotator,
        &QueuesAnnotator,
        &AutoDeleteAnnotator,
        &ExclusiveAnnotator,
        &ChannelExceptionsAnnotator,
        &DeleteAnnotator,
        &QueueFederationAnnotator,
        &FederationAnnotator,
        &VirtualHostsAnnotator,
        &ConnectionsAnnotator,
        &ConnectionTrackingAnnotator,
        &ChannelTrackingAnnotator,
        &AccessControlAnnotator,
        &ShovelsAnnotator,
        &CqStoresAnnotator,
        &DisconnectsAnnotator,
        &DeletionProtectionAnnotator,
        &MultilineAnnotator,
        &StreamsAnnotator,
        &LimitsAnnotator,
        &WorkerPoolAnnotator,
        &PeerDiscoveryClassicAnnotator,
        &PluginsLabelAnnotator,
        &ExchangesAnnotator,
        &StartupBannerAnnotator,
        &ShutdownAnnotator,
        &DefinitionsAnnotator,
        &FeatureFlagsLabelAnnotator,
        &StompAnnotator,
        &WebSocketsAnnotator,
        &MqttAnnotator,
        &ClusteringAnnotator,
        &MetricsAnnotator,
        &TlsAnnotator,
        &QuorumQueuesAnnotator,
        &NetworkingAnnotator,
        &ClassicQueuesAnnotator,
        &PoliciesAnnotator,
        &TimeoutsAnnotator,
        &ConsumerDeliveryTimeoutAnnotator,
        &DeprecatedFeaturesAnnotator,
        &MaintenanceModeLabelAnnotator,
        &KhepriAnnotator,
        &SegmentWriterQuorumQueuesAnnotator,
        &SegmentWriterAnnotator,
        &OsirisAnnotator,
        &RanchAnnotator,
        &ClientProvidedNameAnnotator,
        &DeletingPidFileAnnotator,
        &DeletingUnknownFilesAnnotator,
        &ReadyToStartListenersAnnotator,
        &TracingVhostAnnotator,
        &RefreshingChannelsAnnotator,
        &EpmdAnnotator,
        &RabbitOnNodeAnnotator,
        &VhostReconciliationAnnotator,
        &SetStopReasonAnnotator,
        &MetadataStoreLeaderAnnotator,
        &WillDeclareExchangeAnnotator,
        &AutohealAnnotator,
        &ClusterTagsAnnotator,
        &CheckingClusterConsistencyAnnotator,
    ];

    let mut labels = LogEntryLabels::default();

    for annotator in ANNOTATORS {
        if annotator.does_match(entry) {
            annotator.annotate(&mut labels);
        }
    }

    labels
}
