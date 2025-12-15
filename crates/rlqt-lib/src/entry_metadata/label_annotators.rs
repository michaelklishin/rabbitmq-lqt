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
    matches_consumer_delivery_timeout, matches_cq_storage, matches_federation, matches_management,
    matches_oauth2, matches_plugins, matches_policies, matches_raft, matches_shovels,
    matches_streams, matches_virtual_hosts,
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
        let msg_lower = &entry.message_lowercased;
        matches_raft(msg_lower)
            || msg_lower.contains("pre_vote")
            || msg_lower.contains("election called")
            || msg_lower.contains("election triggered")
            || msg_lower.contains("trigger election")
            || msg_lower.contains("vote granted for term")
            || msg_lower.contains("recovered -> follower in term")
            || msg_lower.contains("recover -> recovered in term")
            || msg_lower.contains("follower -> pre_vote in term")
            || msg_lower.contains("pre_vote -> candidate in term")
            || msg_lower.contains("candidate -> leader in term")
            || msg_lower.contains("recovery of state machine version")
            || msg_lower.contains("recovering state machine version")
            || msg_lower.contains("scanning for cluster changes")
            || msg_lower.contains("ra_coordination")
            || msg_lower.contains("wal_")
            || msg_lower.contains("ra server")
            || msg_lower.contains("stopping member")
            || msg_lower.contains("metadata store member is caught up")
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
pub struct ExceptionsAnnotator;

impl Annotator for ExceptionsAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("channel error on connection")
            || msg_lower.contains("error on amqp connection")
    }
}

impl LabelAnnotator for ExceptionsAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::EXCEPTIONS;
    }
}

#[derive(Debug)]
pub struct ChannelErrorsAnnotator;

impl Annotator for ChannelErrorsAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("precondition_failed")
            || msg_lower.contains("not_found")
            || msg_lower.contains("resource_locked")
    }
}

impl LabelAnnotator for ChannelErrorsAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CHANNELS | LogEntryLabels::EXCEPTIONS;
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
        matches_streams(&entry.message_lowercased)
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
            || msg_lower.starts_with("mirrored supervisor")
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
        msg_lower.contains("all queue leaders are balanced") || msg_lower.contains("quorum queue")
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

#[derive(Debug)]
pub struct DbMetadataAnnotator;

impl Annotator for DbMetadataAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.starts_with("db: initialization")
            || msg_lower.starts_with("db: this node is a virgin node")
            || msg_lower.starts_with("db: checking cluster membership")
            || msg_lower.starts_with("db: removing member")
            || msg_lower.starts_with("db: removing members")
            || msg_lower.starts_with("db: mapping metadata")
            || msg_lower.starts_with("db: starting tree projection")
            || msg_lower.starts_with("adding this node")
                && msg_lower.contains("to the remote node's cluster")
            || msg_lower.starts_with("cluster expanded from")
            || msg_lower.starts_with("detaching this node")
                && msg_lower.contains("from its cluster")
            || msg_lower.contains("cannot query members in store")
            || msg_lower.starts_with("deleting server rabbitmq_metadata")
    }
}

impl LabelAnnotator for DbMetadataAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::KHEPRI;
    }
}

#[derive(Debug)]
pub struct MetadataStoreLabelsAnnotator;

impl Annotator for MetadataStoreLabelsAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .starts_with("rabbitmq metadata store:")
    }
}

impl LabelAnnotator for MetadataStoreLabelsAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::KHEPRI;
    }
}

#[derive(Debug)]
pub struct MetadataStoreVotingAnnotator;

impl Annotator for MetadataStoreVotingAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.starts_with("rabbitmq metadata store:")
            && (msg_lower.contains("granting vote")
                || msg_lower.contains("declining vote")
                || msg_lower.contains("granting pre-vote")
                || msg_lower.contains("declining pre-vote")
                || msg_lower.contains("leader"))
    }
}

impl LabelAnnotator for MetadataStoreVotingAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::RAFT | LogEntryLabels::ELECTIONS;
    }
}

#[derive(Debug)]
pub struct MetadataStoreRaftAnnotator;

impl Annotator for MetadataStoreRaftAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.starts_with("rabbitmq metadata store:")
            && (msg_lower.contains("applying ra cluster change")
                || msg_lower.contains("committing ra cluster change")
                || msg_lower.contains("enabling ra cluster changes")
                || msg_lower.contains("snapshot")
                || msg_lower.contains("follower")
                || msg_lower.contains("terminating")
                || msg_lower.contains("command not appended"))
    }
}

impl LabelAnnotator for MetadataStoreRaftAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::RAFT;
    }
}

#[derive(Debug)]
pub struct MetadataStoreClusterChangeAnnotator;

impl Annotator for MetadataStoreClusterChangeAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.starts_with("rabbitmq metadata store:") && msg_lower.contains("cluster change")
    }
}

impl LabelAnnotator for MetadataStoreClusterChangeAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CLUSTERING;
    }
}

#[derive(Debug)]
pub struct ClusterStatusFilesAnnotator;

impl Annotator for ClusterStatusFilesAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .starts_with("preparing cluster status files")
    }
}

impl LabelAnnotator for ClusterStatusFilesAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CLUSTERING;
    }
}

#[derive(Debug)]
pub struct ClusterNotReadyAnnotator;

impl Annotator for ClusterNotReadyAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .contains("cluster is not ready for a membership change")
    }
}

impl LabelAnnotator for ClusterNotReadyAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::RAFT | LogEntryLabels::CLUSTERING;
    }
}

#[derive(Debug)]
pub struct ReleaseCursorAnnotator;

impl Annotator for ReleaseCursorAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .contains("move release cursor after")
    }
}

impl LabelAnnotator for ReleaseCursorAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::RAFT;
    }
}

#[derive(Debug)]
pub struct RuntimeParametersLabelAnnotator;

impl Annotator for RuntimeParametersLabelAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .contains("asked to set or update runtime parameter")
    }
}

impl LabelAnnotator for RuntimeParametersLabelAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::RUNTIME_PARAMETERS;
    }
}

#[derive(Debug)]
pub struct ConsistentHashingAnnotator;

impl Annotator for ConsistentHashingAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .starts_with("consistent hashing exchange:")
    }
}

impl LabelAnnotator for ConsistentHashingAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::EXCHANGES;
    }
}

#[derive(Debug)]
pub struct MqttRetainedStoreAnnotator;

impl Annotator for MqttRetainedStoreAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("mqtt retained message store")
    }
}

impl LabelAnnotator for MqttRetainedStoreAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::MQTT;
    }
}

#[derive(Debug)]
pub struct MqttConnectionsAnnotator;

impl Annotator for MqttConnectionsAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.starts_with("mqtt accepting")
            || msg_lower.contains("mqtt connection accepted")
            || msg_lower.contains("mqtt connection rejected")
    }
}

impl LabelAnnotator for MqttConnectionsAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::MQTT | LogEntryLabels::CONNECTIONS;
    }
}

#[derive(Debug)]
pub struct MqttDisconnectsAnnotator;

impl Annotator for MqttDisconnectsAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.contains("mqtt closing connection")
    }
}

impl LabelAnnotator for MqttDisconnectsAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::MQTT | LogEntryLabels::DISCONNECTS;
    }
}

#[derive(Debug)]
pub struct MqttAccessControlAnnotator;

impl Annotator for MqttAccessControlAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.contains("mqtt login failed")
    }
}

impl LabelAnnotator for MqttAccessControlAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::MQTT | LogEntryLabels::ACCESS_CONTROL;
    }
}

#[derive(Debug)]
pub struct MqttProtocolErrorAnnotator;

impl Annotator for MqttProtocolErrorAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.contains("mqtt protocol error")
    }
}

impl LabelAnnotator for MqttProtocolErrorAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::MQTT | LogEntryLabels::EXCEPTIONS;
    }
}

#[derive(Debug)]
pub struct MqttTlsAnnotator;

impl Annotator for MqttTlsAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.contains("mqtt detected tls error")
    }
}

impl LabelAnnotator for MqttTlsAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::MQTT | LogEntryLabels::TLS;
    }
}

#[derive(Debug)]
pub struct MqttNetworkingAnnotator;

impl Annotator for MqttNetworkingAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .contains("mqtt detected network error")
    }
}

impl LabelAnnotator for MqttNetworkingAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::MQTT | LogEntryLabels::NETWORKING;
    }
}

#[derive(Debug)]
pub struct StompAccessControlAnnotator;

impl Annotator for StompAccessControlAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.contains("stomp login failed")
    }
}

impl LabelAnnotator for StompAccessControlAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::STOMP | LogEntryLabels::ACCESS_CONTROL;
    }
}

#[derive(Debug)]
pub struct StompProtocolErrorAnnotator;

impl Annotator for StompProtocolErrorAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.contains("stomp error frame sent")
    }
}

impl LabelAnnotator for StompProtocolErrorAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::STOMP | LogEntryLabels::EXCEPTIONS;
    }
}

#[derive(Debug)]
pub struct StompConnectionsAnnotator;

impl Annotator for StompConnectionsAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .starts_with("accepting stomp connection")
    }
}

impl LabelAnnotator for StompConnectionsAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::STOMP | LogEntryLabels::CONNECTIONS;
    }
}

#[derive(Debug)]
pub struct StompDisconnectsAnnotator;

impl Annotator for StompDisconnectsAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .starts_with("closing stomp connection")
    }
}

impl LabelAnnotator for StompDisconnectsAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::STOMP | LogEntryLabels::DISCONNECTS;
    }
}

#[derive(Debug)]
pub struct StompTlsAnnotator;

impl Annotator for StompTlsAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .contains("stomp detected tls error")
    }
}

impl LabelAnnotator for StompTlsAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::STOMP | LogEntryLabels::TLS;
    }
}

#[derive(Debug)]
pub struct StompNetworkingAnnotator;

impl Annotator for StompNetworkingAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .contains("stomp detected network error")
    }
}

impl LabelAnnotator for StompNetworkingAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::STOMP | LogEntryLabels::NETWORKING;
    }
}

#[derive(Debug)]
pub struct StompHeartbeatAnnotator;

impl Annotator for StompHeartbeatAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .contains("stomp detected missed client heartbeat")
    }
}

impl LabelAnnotator for StompHeartbeatAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::STOMP | LogEntryLabels::TIMEOUTS;
    }
}

#[derive(Debug)]
pub struct ConsumerTimeoutAnnotator;

impl Annotator for ConsumerTimeoutAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        (msg_lower.contains("consumer") && msg_lower.contains("timed out"))
            || msg_lower.contains("consumer_timeout")
    }
}

impl LabelAnnotator for ConsumerTimeoutAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CONSUMERS | LogEntryLabels::TIMEOUTS;
    }
}

#[derive(Debug)]
pub struct StreamsLabelsAnnotator;

impl Annotator for StreamsLabelsAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("stream coordinator") || msg_lower.contains("restarting stream")
    }
}

impl LabelAnnotator for StreamsLabelsAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::STREAMS | LogEntryLabels::RAFT;
    }
}

#[derive(Debug)]
pub struct QuorumQueueLabelsAnnotator;

impl Annotator for QuorumQueueLabelsAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("membership reconciliation") || msg_lower.contains("shrinking")
    }
}

impl LabelAnnotator for QuorumQueueLabelsAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::QUORUM_QUEUES | LogEntryLabels::RAFT;
    }
}

#[derive(Debug)]
pub struct LeadershipTransferAnnotator;

impl Annotator for LeadershipTransferAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.contains("leadership transfer")
    }
}

impl LabelAnnotator for LeadershipTransferAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::QUORUM_QUEUES | LogEntryLabels::RAFT | LogEntryLabels::ELECTIONS;
    }
}

#[derive(Debug)]
pub struct HttpAnnotator;

impl Annotator for HttpAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.starts_with("http api:")
    }
}

impl LabelAnnotator for HttpAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::HTTP;
    }
}

#[derive(Debug)]
pub struct HttpAccessDeniedAnnotator;

impl Annotator for HttpAccessDeniedAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.contains("http access denied")
    }
}

impl LabelAnnotator for HttpAccessDeniedAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::HTTP | LogEntryLabels::ACCESS_CONTROL;
    }
}

#[derive(Debug)]
pub struct ManagementPluginAnnotator;

impl Annotator for ManagementPluginAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.starts_with("management plugin:")
    }
}

impl LabelAnnotator for ManagementPluginAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::PLUGINS;
    }
}

#[derive(Debug)]
pub struct StatisticsDatabaseAnnotator;

impl Annotator for StatisticsDatabaseAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .contains("statistics database started")
    }
}

impl LabelAnnotator for StatisticsDatabaseAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::PLUGINS | LogEntryLabels::METRICS;
    }
}

#[derive(Debug)]
pub struct QueueRebalanceAnnotator;

impl Annotator for QueueRebalanceAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .contains("user initiated queue rebalance")
    }
}

impl LabelAnnotator for QueueRebalanceAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::PLUGINS | LogEntryLabels::QUORUM_QUEUES;
    }
}

#[derive(Debug)]
pub struct DefinitionFileSizeAnnotator;

impl Annotator for DefinitionFileSizeAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .contains("uploaded definition file size")
    }
}

impl LabelAnnotator for DefinitionFileSizeAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::PLUGINS | LogEntryLabels::DEFINITIONS | LogEntryLabels::LIMITS;
    }
}

#[derive(Debug)]
pub struct ClusteringLabelsAnnotator;

impl Annotator for ClusteringLabelsAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("partial partition disconnect")
            || msg_lower.contains("has joined the cluster")
    }
}

impl LabelAnnotator for ClusteringLabelsAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CLUSTERING;
    }
}

#[derive(Debug)]
pub struct InetTcpErrorAnnotator;

impl Annotator for InetTcpErrorAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .contains("protocol 'inet_tcp': register/listen error")
    }
}

impl LabelAnnotator for InetTcpErrorAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CLUSTERING | LogEntryLabels::NETWORKING;
    }
}

#[derive(Debug)]
pub struct MnesiaEventAnnotator;

impl Annotator for MnesiaEventAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .contains("mnesia_event got {inconsistent_database")
    }
}

impl LabelAnnotator for MnesiaEventAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CLUSTERING;
    }
}

#[derive(Debug)]
pub struct ClassicQueueMirroringAnnotator;

impl Annotator for ClassicQueueMirroringAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("queue master")
            || msg_lower.contains("queue_master")
            || msg_lower.contains("master queue")
            || msg_lower.contains("min_masters")
            || msg_lower.contains("queue mirror")
            || msg_lower.contains("queue_mirror")
            || msg_lower.contains("mirror queue")
            || msg_lower.contains("ha-promote")
            || msg_lower.contains("ha_promote")
            || msg_lower.contains("ha-sync")
            || msg_lower.contains("promoting mirror")
            || msg_lower.contains("synchronised mirror")
            || msg_lower.contains("rabbit_mirror_queue_")
            || msg_lower.contains("classic_queue_mirroring")
            || msg_lower.contains("classic_mirrored_queue")
    }
}

impl LabelAnnotator for ClassicQueueMirroringAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CLASSIC_QUEUES
            | LogEntryLabels::QUEUES
            | LogEntryLabels::CLUSTERING
            | LogEntryLabels::DEPRECATED_FEATURES;
    }
}

#[derive(Debug)]
pub struct ClosedConnectionsAnnotator;

impl Annotator for ClosedConnectionsAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .contains("closed local connections")
    }
}

impl LabelAnnotator for ClosedConnectionsAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::DISCONNECTS | LogEntryLabels::CONNECTIONS;
    }
}

#[derive(Debug)]
pub struct QueueDroppedMessagesAnnotator;

impl Annotator for QueueDroppedMessagesAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("queue dropped") && msg_lower.contains("messages")
    }
}

impl LabelAnnotator for QueueDroppedMessagesAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::QUEUES | LogEntryLabels::CLASSIC_QUEUES;
    }
}

#[derive(Debug)]
pub struct BindingRecoverAnnotator;

impl Annotator for BindingRecoverAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.contains("rabbit_binding:recover")
    }
}

impl LabelAnnotator for BindingRecoverAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::VIRTUAL_HOSTS;
    }
}

#[derive(Debug)]
pub struct HandshakeTimeoutAnnotator;

impl Annotator for HandshakeTimeoutAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.contains("handshake_timeout")
    }
}

impl LabelAnnotator for HandshakeTimeoutAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CONNECTIONS | LogEntryLabels::TIMEOUTS;
    }
}

#[derive(Debug)]
pub struct ChannelsLabelAnnotator;

impl Annotator for ChannelsLabelAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("channel error on connection")
    }
}

impl LabelAnnotator for ChannelsLabelAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CHANNELS;
    }
}

#[derive(Debug)]
pub struct SeedVhostUserAnnotator;

impl Annotator for SeedVhostUserAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.starts_with("will seed default virtual host and user")
            || msg_lower.starts_with("will not seed default virtual host and user")
    }
}

impl LabelAnnotator for SeedVhostUserAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::VIRTUAL_HOSTS | LogEntryLabels::ACCESS_CONTROL;
    }
}

#[derive(Debug)]
pub struct NotSeedDefinitionsAnnotator;

impl Annotator for NotSeedDefinitionsAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .starts_with("will not seed default virtual host and user")
    }
}

impl LabelAnnotator for NotSeedDefinitionsAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::DEFINITIONS;
    }
}

#[derive(Debug)]
pub struct DbClusteringAnnotator;

impl Annotator for DbClusteringAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        (msg_lower.starts_with("db: ") && msg_lower.contains("cluster"))
            || msg_lower.starts_with("resetting member")
            || msg_lower.contains("from the remote node's cluster")
    }
}

impl LabelAnnotator for DbClusteringAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CLUSTERING | LogEntryLabels::KHEPRI;
    }
}

#[derive(Debug)]
pub struct DefinitionsLoadAnnotator;

impl Annotator for DefinitionsLoadAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        (msg_lower.starts_with("will use module") && msg_lower.contains("to import definitions"))
            || msg_lower.starts_with("will try to load definitions from")
            || msg_lower.contains("there are fewer than target cluster size")
            || msg_lower.starts_with("applying definitions from file")
    }
}

impl LabelAnnotator for DefinitionsLoadAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::DEFINITIONS;
    }
}

#[derive(Debug)]
pub struct GlobalParameterClusteringAnnotator;

impl Annotator for GlobalParameterClusteringAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("setting global parameter 'cluster_name'")
            || msg_lower.contains("setting global parameter 'internal_cluster_id'")
    }
}

impl LabelAnnotator for GlobalParameterClusteringAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CLUSTERING;
    }
}

#[derive(Debug)]
pub struct RuntimeParametersImportAnnotator;

impl Annotator for RuntimeParametersImportAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("importing sequentially")
            && msg_lower.contains("global runtime parameters")
    }
}

impl LabelAnnotator for RuntimeParametersImportAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::RUNTIME_PARAMETERS | LogEntryLabels::DEFINITIONS;
    }
}

#[derive(Debug)]
pub struct RemoveQueuesFromNodeAnnotator;

impl Annotator for RemoveQueuesFromNodeAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .starts_with("will remove all queues from node")
    }
}

impl LabelAnnotator for RemoveQueuesFromNodeAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::QUEUES | LogEntryLabels::CLUSTERING;
    }
}

#[derive(Debug)]
pub struct ClassicPeerDiscoveryAnnotator;

impl Annotator for ClassicPeerDiscoveryAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .starts_with("classic peer discovery backend:")
    }
}

impl LabelAnnotator for ClassicPeerDiscoveryAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CLUSTERING;
    }
}

#[derive(Debug)]
pub struct QuorumQueueBootAnnotator;

impl Annotator for QuorumQueueBootAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .contains("[rabbit_quorum_queue:system_recover/1] rabbit not booted")
    }
}

impl LabelAnnotator for QuorumQueueBootAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::QUORUM_QUEUES;
    }
}

#[derive(Debug)]
pub struct LimitsAlarmAnnotator;

impl Annotator for LimitsAlarmAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        (msg_lower.contains("alarm")
            && (msg_lower.contains(" set") || msg_lower.contains(" cleared")))
            || msg_lower.contains("resource limit alarm")
            || msg_lower.contains("file descriptor limit alarm")
    }
}

impl LabelAnnotator for LimitsAlarmAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::LIMITS;
    }
}

#[derive(Debug)]
pub struct ConnectionTrackingVhostAnnotator;

impl Annotator for ConnectionTrackingVhostAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .contains("setting up a table for per-vhost connection")
    }
}

impl LabelAnnotator for ConnectionTrackingVhostAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CONNECTIONS | LogEntryLabels::VIRTUAL_HOSTS;
    }
}

#[derive(Debug)]
pub struct ConnectionTrackingUserAnnotator;

impl Annotator for ConnectionTrackingUserAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .contains("setting up a table for per-user connection")
    }
}

impl LabelAnnotator for ConnectionTrackingUserAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CONNECTIONS | LogEntryLabels::ACCESS_CONTROL;
    }
}

#[derive(Debug)]
pub struct QueueRebalanceLabelsAnnotator;

impl Annotator for QueueRebalanceLabelsAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("queue rebalance")
            || msg_lower.contains("starting queue rebalance")
            || msg_lower.contains("finished queue rebalance")
            || msg_lower.contains("migrating queue")
            || msg_lower.contains("error migrating queue")
            || (msg_lower.contains("queue") && msg_lower.contains("migrated to"))
    }
}

impl LabelAnnotator for QueueRebalanceLabelsAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::QUORUM_QUEUES | LogEntryLabels::RAFT;
    }
}

#[derive(Debug)]
pub struct VhostDefaultLimitsAnnotator;

impl Annotator for VhostDefaultLimitsAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .contains("applied default limits to vhost")
    }
}

impl LabelAnnotator for VhostDefaultLimitsAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::VIRTUAL_HOSTS | LogEntryLabels::LIMITS;
    }
}

#[derive(Debug)]
pub struct VhostDefaultPolicyAnnotator;

impl Annotator for VhostDefaultPolicyAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .contains("applied default operator policy to vhost")
    }
}

impl LabelAnnotator for VhostDefaultPolicyAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::VIRTUAL_HOSTS | LogEntryLabels::POLICIES;
    }
}

#[derive(Debug)]
pub struct VhostDefaultUserAnnotator;

impl Annotator for VhostDefaultUserAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .contains("added default user to vhost")
    }
}

impl LabelAnnotator for VhostDefaultUserAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::VIRTUAL_HOSTS | LogEntryLabels::ACCESS_CONTROL;
    }
}

#[derive(Debug)]
pub struct ChannelFailureAnnotator;

impl Annotator for ChannelFailureAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("timed out getting channel")
            || msg_lower.contains("failed to refresh channel config")
            || msg_lower.contains("failed to refresh channel interceptors")
            || msg_lower.contains("channel is stopping with")
    }
}

impl LabelAnnotator for ChannelFailureAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CHANNELS;
    }
}

#[derive(Debug)]
pub struct ChannelTimeoutAnnotator;

impl Annotator for ChannelTimeoutAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .contains("timed out getting channel")
    }
}

impl LabelAnnotator for ChannelTimeoutAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::TIMEOUTS;
    }
}

#[derive(Debug)]
pub struct Amqp10SessionAnnotator;

impl Annotator for Amqp10SessionAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("amqp 1.0 created session process")
            || msg_lower.contains("amqp 1.0 closed session process")
    }
}

impl LabelAnnotator for Amqp10SessionAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::SESSIONS | LogEntryLabels::CHANNELS | LogEntryLabels::AMQP10;
    }
}

#[derive(Debug)]
pub struct Amqp10SessionDisconnectAnnotator;

impl Annotator for Amqp10SessionDisconnectAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .contains("amqp 1.0 closed session process")
    }
}

impl LabelAnnotator for Amqp10SessionDisconnectAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::DISCONNECTS;
    }
}

#[derive(Debug)]
pub struct Amqp10ConnectionErrorAnnotator;

impl Annotator for Amqp10ConnectionErrorAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .contains("error on amqp 1.0 connection")
    }
}

impl LabelAnnotator for Amqp10ConnectionErrorAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |=
            LogEntryLabels::CONNECTIONS | LogEntryLabels::EXCEPTIONS | LogEntryLabels::AMQP10;
    }
}

#[derive(Debug)]
pub struct ShutdownExtendedAnnotator;

impl Annotator for ShutdownExtendedAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.starts_with("rabbitmq is asked to stop")
            || msg_lower.starts_with("successfully stopped rabbitmq")
            || msg_lower.starts_with("running rabbit_prelaunch:shutdown_func()")
    }
}

impl LabelAnnotator for ShutdownExtendedAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::SHUTDOWN;
    }
}

#[derive(Debug)]
pub struct RefreshedChannelStatesAnnotator;

impl Annotator for RefreshedChannelStatesAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .starts_with("refreshed channel states")
    }
}

impl LabelAnnotator for RefreshedChannelStatesAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CHANNELS;
    }
}

#[derive(Debug)]
pub struct OAuth2LabelAnnotator;

impl Annotator for OAuth2LabelAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        matches_oauth2(&entry.message_lowercased)
    }
}

impl LabelAnnotator for OAuth2LabelAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::OAUTH2;
    }
}

#[derive(Debug)]
pub struct SqlExpressionAnnotator;

impl Annotator for SqlExpressionAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("sql expression")
            || msg_lower.contains("sql filter")
            || msg_lower.contains("selector expression")
    }
}

impl LabelAnnotator for SqlExpressionAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::SQL | LogEntryLabels::AMQP10;
    }
}

#[derive(Debug)]
pub struct SacCoordinatorAnnotator;

impl Annotator for SacCoordinatorAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("sac coordinator") || msg_lower.contains("single active consumer")
    }
}

impl LabelAnnotator for SacCoordinatorAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::STREAMS | LogEntryLabels::RAFT;
    }
}

#[derive(Debug)]
pub struct AggregatedMetricsAnnotator;

impl Annotator for AggregatedMetricsAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.contains("aggregated metrics")
    }
}

impl LabelAnnotator for AggregatedMetricsAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::METRICS;
    }
}

#[derive(Debug)]
pub struct DefinitionsModuleAnnotator;

impl Annotator for DefinitionsModuleAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        (msg_lower.starts_with("will use module") && msg_lower.contains("to import definitions"))
            || msg_lower.contains("definitions source")
    }
}

impl LabelAnnotator for DefinitionsModuleAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::DEFINITIONS;
    }
}

#[derive(Debug)]
pub struct ManagementLabelAnnotator;

impl Annotator for ManagementLabelAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        matches_management(&entry.message_lowercased)
    }
}

impl LabelAnnotator for ManagementLabelAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::PLUGINS;
    }
}

#[derive(Debug)]
pub struct HttpApiListenerAnnotator;

impl Annotator for HttpApiListenerAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.starts_with("http api:")
            || msg_lower.contains("restarting http api listener")
            || msg_lower.contains("http api listener")
    }
}

impl LabelAnnotator for HttpApiListenerAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::HTTP | LogEntryLabels::NETWORKING;
    }
}

#[derive(Debug)]
pub struct Amqp10SessionExceptionAnnotator;

impl Annotator for Amqp10SessionExceptionAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("amqp 1.0 session exception")
            || msg_lower.contains("error on amqp 1.0 session")
    }
}

impl LabelAnnotator for Amqp10SessionExceptionAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::AMQP10 | LogEntryLabels::SESSIONS | LogEntryLabels::EXCEPTIONS;
    }
}

#[derive(Debug)]
pub struct NodeMembershipAnnotator;

impl Annotator for NodeMembershipAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("node membership changed")
            || msg_lower.contains("node membership change")
            || msg_lower.contains("cluster membership")
    }
}

impl LabelAnnotator for NodeMembershipAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CLUSTERING | LogEntryLabels::RAFT;
    }
}

#[derive(Debug)]
pub struct StreamReplicaAnnotator;

impl Annotator for StreamReplicaAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("stream replica") || msg_lower.contains("stream member")
    }
}

impl LabelAnnotator for StreamReplicaAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::STREAMS;
    }
}

#[derive(Debug)]
pub struct QuorumQueueRecoveryAnnotator;

impl Annotator for QuorumQueueRecoveryAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("quorum queue recovery")
            || msg_lower.contains("recovering quorum queues")
            || msg_lower.contains("recovered quorum queues")
    }
}

impl LabelAnnotator for QuorumQueueRecoveryAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::QUORUM_QUEUES | LogEntryLabels::RAFT;
    }
}

#[derive(Debug)]
pub struct ClassicQueueRecoveryAnnotator;

impl Annotator for ClassicQueueRecoveryAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("recovering classic queues")
            || msg_lower.contains("recovered classic queues")
    }
}

impl LabelAnnotator for ClassicQueueRecoveryAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CLASSIC_QUEUES | LogEntryLabels::QUEUES;
    }
}

#[derive(Debug)]
pub struct PasswordHashAnnotator;

impl Annotator for PasswordHashAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("password hashing")
            || msg_lower.contains("password_hashing")
            || msg_lower.contains("default_pass_hash")
    }
}

impl LabelAnnotator for PasswordHashAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::ACCESS_CONTROL;
    }
}

#[derive(Debug)]
pub struct CredentialValidatorAnnotator;

impl Annotator for CredentialValidatorAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("credential validator")
            || msg_lower.contains("credential_validator")
            || msg_lower.contains("password strength")
    }
}

impl LabelAnnotator for CredentialValidatorAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::ACCESS_CONTROL;
    }
}

#[derive(Debug)]
pub struct InternalAuthAnnotator;

impl Annotator for InternalAuthAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("rabbit_auth_backend_internal")
            || msg_lower.contains("internal auth backend")
    }
}

impl LabelAnnotator for InternalAuthAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::ACCESS_CONTROL;
    }
}

#[derive(Debug)]
pub struct LdapAuthAnnotator;

impl Annotator for LdapAuthAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("ldap") || msg_lower.contains("rabbit_auth_backend_ldap")
    }
}

impl LabelAnnotator for LdapAuthAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::ACCESS_CONTROL | LogEntryLabels::PLUGINS;
    }
}

#[derive(Debug)]
pub struct CacheAuthAnnotator;

impl Annotator for CacheAuthAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("rabbit_auth_backend_cache")
            || msg_lower.contains("auth cache")
            || msg_lower.contains("auth_cache")
    }
}

impl LabelAnnotator for CacheAuthAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::ACCESS_CONTROL | LogEntryLabels::PLUGINS;
    }
}

#[derive(Debug)]
pub struct TlsCertificateAnnotator;

impl Annotator for TlsCertificateAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("certificate")
            && (msg_lower.contains("tls")
                || msg_lower.contains("ssl")
                || msg_lower.contains("x509"))
    }
}

impl LabelAnnotator for TlsCertificateAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::TLS;
    }
}

#[derive(Debug)]
pub struct TlsHandshakeAnnotator;

impl Annotator for TlsHandshakeAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("tls handshake")
            || msg_lower.contains("ssl handshake")
            || msg_lower.contains("handshake error")
    }
}

impl LabelAnnotator for TlsHandshakeAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::TLS | LogEntryLabels::CONNECTIONS;
    }
}

#[derive(Debug)]
pub struct DnsLookupAnnotator;

impl Annotator for DnsLookupAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("dns lookup")
            || msg_lower.contains("dns resolution")
            || msg_lower.contains("inet_res")
    }
}

impl LabelAnnotator for DnsLookupAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::NETWORKING;
    }
}

#[derive(Debug)]
pub struct SocketErrorAnnotator;

impl Annotator for SocketErrorAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("socket error")
            || msg_lower.contains("socket closed")
            || msg_lower.contains("econnrefused")
            || msg_lower.contains("econnreset")
            || msg_lower.contains("etimedout")
    }
}

impl LabelAnnotator for SocketErrorAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::NETWORKING | LogEntryLabels::EXCEPTIONS;
    }
}

#[derive(Debug)]
pub struct PrometheusMetricsAnnotator;

impl Annotator for PrometheusMetricsAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.contains("prometheus metrics:")
    }
}

impl LabelAnnotator for PrometheusMetricsAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::METRICS | LogEntryLabels::PLUGINS;
    }
}

#[derive(Debug)]
pub struct ShovelTopologyAnnotator;

impl Annotator for ShovelTopologyAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("shovel topology")
            || msg_lower.contains("shovel has finished setting up its topology")
    }
}

impl LabelAnnotator for ShovelTopologyAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::SHOVELS;
    }
}

#[derive(Debug)]
pub struct ShovelConnectionAnnotator;

impl Annotator for ShovelConnectionAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("shovel connected")
            || msg_lower.contains("shovel could not connect")
            || msg_lower.contains("shovel connection")
    }
}

impl LabelAnnotator for ShovelConnectionAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::SHOVELS | LogEntryLabels::CONNECTIONS;
    }
}

#[derive(Debug)]
pub struct FederationLinkAnnotator;

impl Annotator for FederationLinkAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("federation link")
            || msg_lower.contains("federation upstream")
            || msg_lower.contains("federation-upstream")
    }
}

impl LabelAnnotator for FederationLinkAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::FEDERATION;
    }
}

#[derive(Debug)]
pub struct MemoryAlarmAnnotator;

impl Annotator for MemoryAlarmAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("memory alarm")
            || msg_lower.contains("vm_memory_high_watermark")
            || msg_lower.contains("memory high watermark set")
    }
}

impl LabelAnnotator for MemoryAlarmAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::LIMITS;
    }
}

#[derive(Debug)]
pub struct DiskAlarmAnnotator;

impl Annotator for DiskAlarmAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("disk alarm")
            || msg_lower.contains("disk_free_limit")
            || msg_lower.contains("disk free space")
    }
}

impl LabelAnnotator for DiskAlarmAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::LIMITS;
    }
}

#[derive(Debug)]
pub struct FileDescriptorAlarmAnnotator;

impl Annotator for FileDescriptorAlarmAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("file descriptor") || msg_lower.contains("file handles reserved")
    }
}

impl LabelAnnotator for FileDescriptorAlarmAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::LIMITS;
    }
}

#[derive(Debug)]
pub struct HeartbeatTimeoutAnnotator;

impl Annotator for HeartbeatTimeoutAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("heartbeat timeout")
            || msg_lower.contains("missed heartbeat")
            || msg_lower.contains("missed client heartbeat")
    }
}

impl LabelAnnotator for HeartbeatTimeoutAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::TIMEOUTS | LogEntryLabels::CONNECTIONS;
    }
}

#[derive(Debug)]
pub struct BootStepAnnotator;

impl Annotator for BootStepAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.starts_with("running boot step")
            || msg_lower.starts_with("boot steps:")
            || msg_lower.contains("boot step")
    }
}

impl LabelAnnotator for BootStepAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::PLUGINS;
    }
}

#[derive(Debug)]
pub struct VhostSupervisorAnnotator;

impl Annotator for VhostSupervisorAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("rabbit_vhost_sup")
            || msg_lower.contains("vhost supervisor")
            || msg_lower.contains("vhost_sup")
    }
}

impl LabelAnnotator for VhostSupervisorAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::VIRTUAL_HOSTS;
    }
}

#[derive(Debug)]
pub struct ConsumerCancellationAnnotator;

impl Annotator for ConsumerCancellationAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("consumer cancellation") || msg_lower.contains("consumer cancelled")
    }
}

impl LabelAnnotator for ConsumerCancellationAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CONSUMERS | LogEntryLabels::CHANNELS;
    }
}

#[derive(Debug)]
pub struct BasicGetAnnotator;

impl Annotator for BasicGetAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.contains("basic.get")
    }
}

impl LabelAnnotator for BasicGetAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CHANNELS;
    }
}

#[derive(Debug)]
pub struct BasicConsumeAnnotator;

impl Annotator for BasicConsumeAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.contains("basic.consume")
    }
}

impl LabelAnnotator for BasicConsumeAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CHANNELS | LogEntryLabels::CONSUMERS;
    }
}

#[derive(Debug)]
pub struct TransientQueueAnnotator;

impl Annotator for TransientQueueAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("transient queue") || msg_lower.contains("non-durable queue")
    }
}

impl LabelAnnotator for TransientQueueAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::QUEUES;
    }
}

#[derive(Debug)]
pub struct LazyQueueAnnotator;

impl Annotator for LazyQueueAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.contains("lazy queue")
    }
}

impl LabelAnnotator for LazyQueueAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CLASSIC_QUEUES | LogEntryLabels::QUEUES;
    }
}

#[derive(Debug)]
pub struct DeadLetterAnnotator;

impl Annotator for DeadLetterAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("dead letter")
            || msg_lower.contains("dead-letter")
            || msg_lower.contains("dlx")
    }
}

impl LabelAnnotator for DeadLetterAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::QUEUES;
    }
}

#[derive(Debug)]
pub struct MessageTtlAnnotator;

impl Annotator for MessageTtlAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("message ttl") || msg_lower.contains("x-message-ttl")
    }
}

impl LabelAnnotator for MessageTtlAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::QUEUES | LogEntryLabels::TIMEOUTS;
    }
}

#[derive(Debug)]
pub struct QueueLengthLimitAnnotator;

impl Annotator for QueueLengthLimitAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("queue length limit")
            || msg_lower.contains("x-max-length")
            || (msg_lower.contains("overflow") && msg_lower.contains("queue"))
    }
}

impl LabelAnnotator for QueueLengthLimitAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::QUEUES | LogEntryLabels::LIMITS;
    }
}

#[derive(Debug)]
pub struct NetworkPartitionAnnotator;

impl Annotator for NetworkPartitionAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("network partition")
            || msg_lower.contains("partition detected")
            || msg_lower.contains("partition handling")
    }
}

impl LabelAnnotator for NetworkPartitionAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CLUSTERING | LogEntryLabels::NETWORKING;
    }
}

#[derive(Debug)]
pub struct ConsistentHashExchangeAnnotator;

impl Annotator for ConsistentHashExchangeAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.contains("x-consistent-hash")
    }
}

impl LabelAnnotator for ConsistentHashExchangeAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::EXCHANGES | LogEntryLabels::PLUGINS;
    }
}

#[derive(Debug)]
pub struct HeadersExchangeAnnotator;

impl Annotator for HeadersExchangeAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.contains("headers exchange")
    }
}

impl LabelAnnotator for HeadersExchangeAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::EXCHANGES;
    }
}

#[derive(Debug)]
pub struct TopicExchangeAnnotator;

impl Annotator for TopicExchangeAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.contains("topic exchange")
    }
}

impl LabelAnnotator for TopicExchangeAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::EXCHANGES;
    }
}

#[derive(Debug)]
pub struct DirectExchangeAnnotator;

impl Annotator for DirectExchangeAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.contains("direct exchange")
    }
}

impl LabelAnnotator for DirectExchangeAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::EXCHANGES;
    }
}

#[derive(Debug)]
pub struct FanoutExchangeAnnotator;

impl Annotator for FanoutExchangeAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.contains("fanout exchange")
    }
}

impl LabelAnnotator for FanoutExchangeAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::EXCHANGES;
    }
}

#[derive(Debug)]
pub struct ExchangeToExchangeAnnotator;

impl Annotator for ExchangeToExchangeAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.contains("e2e binding")
    }
}

impl LabelAnnotator for ExchangeToExchangeAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::EXCHANGES;
    }
}

#[derive(Debug)]
pub struct BindingAnnotator;

impl Annotator for BindingAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("binding created")
            || msg_lower.contains("binding deleted")
            || msg_lower.contains("binding recovered")
    }
}

impl LabelAnnotator for BindingAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::EXCHANGES | LogEntryLabels::QUEUES;
    }
}

#[derive(Debug)]
pub struct PgScopeAnnotator;

impl Annotator for PgScopeAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.contains("pg scope")
    }
}

impl LabelAnnotator for PgScopeAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CLUSTERING | LogEntryLabels::RAFT;
    }
}

#[derive(Debug)]
pub struct RaLogAnnotator;

impl Annotator for RaLogAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("ra_log:") || msg_lower.contains("ra_log_")
    }
}

impl LabelAnnotator for RaLogAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::RAFT;
    }
}

#[derive(Debug)]
pub struct RaSnapshotAnnotator;

impl Annotator for RaSnapshotAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("snapshot") && (msg_lower.contains("ra") || msg_lower.contains("raft"))
    }
}

impl LabelAnnotator for RaSnapshotAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::RAFT;
    }
}

#[derive(Debug)]
pub struct RaMemberAnnotator;

impl Annotator for RaMemberAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("ra member") || msg_lower.contains("ra_server")
    }
}

impl LabelAnnotator for RaMemberAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::RAFT;
    }
}

#[derive(Debug)]
pub struct OperatorPolicyAnnotator;

impl Annotator for OperatorPolicyAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.contains("operator policy")
    }
}

impl LabelAnnotator for OperatorPolicyAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::POLICIES;
    }
}

#[derive(Debug)]
pub struct UserPolicyAnnotator;

impl Annotator for UserPolicyAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        (msg_lower.contains("user policy") || msg_lower.contains("policy applied"))
            && !msg_lower.contains("operator")
    }
}

impl LabelAnnotator for UserPolicyAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::POLICIES;
    }
}

#[derive(Debug)]
pub struct EffectivePolicyAnnotator;

impl Annotator for EffectivePolicyAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.contains("effective policy")
    }
}

impl LabelAnnotator for EffectivePolicyAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::POLICIES | LogEntryLabels::QUEUES;
    }
}

#[derive(Debug)]
pub struct ClusteringBannerAnnotator;

impl Annotator for ClusteringBannerAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.starts_with("== clustering ==")
    }
}

impl LabelAnnotator for ClusteringBannerAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CLUSTERING;
    }
}

#[derive(Debug)]
pub struct BootFailedAnnotator;

impl Annotator for BootFailedAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.starts_with("boot failed")
    }
}

impl LabelAnnotator for BootFailedAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::EXCEPTIONS;
    }
}

#[derive(Debug)]
pub struct RanchListenerFailedAnnotator;

impl Annotator for RanchListenerFailedAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .starts_with("failed to start ranch listener")
    }
}

impl LabelAnnotator for RanchListenerFailedAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::EXCEPTIONS | LogEntryLabels::NETWORKING;
    }
}

#[derive(Debug)]
pub struct NodeMonitorStartAnnotator;

impl Annotator for NodeMonitorStartAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .contains("starting rabbit_node_monitor")
    }
}

impl LabelAnnotator for NodeMonitorStartAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CLUSTERING;
    }
}

#[derive(Debug)]
pub struct NodeDownWithReasonAnnotator;

impl Annotator for NodeDownWithReasonAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg = &entry.message_lowercased;
        msg.starts_with("node ") && msg.contains(" down: ")
    }
}

impl LabelAnnotator for NodeDownWithReasonAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CLUSTERING | LogEntryLabels::DISCONNECTS;
    }
}

#[derive(Debug)]
pub struct VhostRecordUpdateAnnotator;

impl Annotator for VhostRecordUpdateAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.starts_with("updating a virtual host record")
            || msg_lower.starts_with("updated a virtual host record")
    }
}

impl LabelAnnotator for VhostRecordUpdateAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::VIRTUAL_HOSTS;
    }
}

#[derive(Debug)]
pub struct VhostSupervisorStopAnnotator;

impl Annotator for VhostSupervisorStopAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .starts_with("stopping vhost supervisor")
    }
}

impl LabelAnnotator for VhostSupervisorStopAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::VIRTUAL_HOSTS | LogEntryLabels::SHUTDOWN;
    }
}

#[derive(Debug)]
pub struct DeliveryLimitDefaultAnnotator;

impl Annotator for DeliveryLimitDefaultAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .contains("delivery_limit not set, defaulting")
    }
}

impl LabelAnnotator for DeliveryLimitDefaultAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::QUORUM_QUEUES;
    }
}

#[derive(Debug)]
pub struct RaServerExitedAnnotator;

impl Annotator for RaServerExitedAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        let msg_lower = &entry.message_lowercased;
        msg_lower.contains("ra server") && msg_lower.contains("already exited")
    }
}

impl LabelAnnotator for RaServerExitedAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::RAFT;
    }
}

#[derive(Debug)]
pub struct MetadataStoreMembersAnnotator;

impl Annotator for MetadataStoreMembersAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .starts_with("found the following metadata store members")
    }
}

impl LabelAnnotator for MetadataStoreMembersAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::KHEPRI | LogEntryLabels::CLUSTERING;
    }
}

#[derive(Debug)]
pub struct RestartingRaServerAnnotator;

impl Annotator for RestartingRaServerAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry
            .message_lowercased
            .starts_with("trying to restart local ra server")
    }
}

impl LabelAnnotator for RestartingRaServerAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::KHEPRI | LogEntryLabels::RAFT;
    }
}

#[derive(Debug)]
pub struct StoppingRanchAnnotator;

impl Annotator for StoppingRanchAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.starts_with("stopping ranch")
    }
}

impl LabelAnnotator for StoppingRanchAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::SHUTDOWN | LogEntryLabels::NETWORKING;
    }
}

#[derive(Debug)]
pub struct StoppingRaSystemsAnnotator;

impl Annotator for StoppingRaSystemsAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.starts_with("stopping ra systems")
    }
}

impl LabelAnnotator for StoppingRaSystemsAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::RAFT | LogEntryLabels::SHUTDOWN;
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
        &ExceptionsAnnotator,
        &ChannelErrorsAnnotator,
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
        &DbMetadataAnnotator,
        &MetadataStoreLabelsAnnotator,
        &MetadataStoreVotingAnnotator,
        &MetadataStoreRaftAnnotator,
        &MetadataStoreClusterChangeAnnotator,
        &ClusterStatusFilesAnnotator,
        &ClusterNotReadyAnnotator,
        &ReleaseCursorAnnotator,
        &RuntimeParametersLabelAnnotator,
        &ConsistentHashingAnnotator,
        &MqttRetainedStoreAnnotator,
        &MqttConnectionsAnnotator,
        &MqttDisconnectsAnnotator,
        &MqttAccessControlAnnotator,
        &MqttProtocolErrorAnnotator,
        &MqttTlsAnnotator,
        &MqttNetworkingAnnotator,
        &StompAccessControlAnnotator,
        &StompProtocolErrorAnnotator,
        &StompConnectionsAnnotator,
        &StompDisconnectsAnnotator,
        &StompTlsAnnotator,
        &StompNetworkingAnnotator,
        &StompHeartbeatAnnotator,
        &ConsumerTimeoutAnnotator,
        &StreamsLabelsAnnotator,
        &QuorumQueueLabelsAnnotator,
        &LeadershipTransferAnnotator,
        &HttpAnnotator,
        &HttpAccessDeniedAnnotator,
        &ManagementPluginAnnotator,
        &StatisticsDatabaseAnnotator,
        &QueueRebalanceAnnotator,
        &DefinitionFileSizeAnnotator,
        &ClusteringLabelsAnnotator,
        &InetTcpErrorAnnotator,
        &MnesiaEventAnnotator,
        &ClassicQueueMirroringAnnotator,
        &ClosedConnectionsAnnotator,
        &QueueDroppedMessagesAnnotator,
        &BindingRecoverAnnotator,
        &HandshakeTimeoutAnnotator,
        &ChannelsLabelAnnotator,
        &SeedVhostUserAnnotator,
        &NotSeedDefinitionsAnnotator,
        &DbClusteringAnnotator,
        &DefinitionsLoadAnnotator,
        &GlobalParameterClusteringAnnotator,
        &RuntimeParametersImportAnnotator,
        &RemoveQueuesFromNodeAnnotator,
        &ClassicPeerDiscoveryAnnotator,
        &QuorumQueueBootAnnotator,
        &LimitsAlarmAnnotator,
        &ConnectionTrackingVhostAnnotator,
        &ConnectionTrackingUserAnnotator,
        &QueueRebalanceLabelsAnnotator,
        &VhostDefaultLimitsAnnotator,
        &VhostDefaultPolicyAnnotator,
        &VhostDefaultUserAnnotator,
        &ChannelFailureAnnotator,
        &ChannelTimeoutAnnotator,
        &Amqp10SessionAnnotator,
        &Amqp10SessionDisconnectAnnotator,
        &Amqp10ConnectionErrorAnnotator,
        &ShutdownExtendedAnnotator,
        &RefreshedChannelStatesAnnotator,
        &OAuth2LabelAnnotator,
        &SqlExpressionAnnotator,
        &SacCoordinatorAnnotator,
        &AggregatedMetricsAnnotator,
        &DefinitionsModuleAnnotator,
        &ManagementLabelAnnotator,
        &HttpApiListenerAnnotator,
        &Amqp10SessionExceptionAnnotator,
        &NodeMembershipAnnotator,
        &StreamReplicaAnnotator,
        &QuorumQueueRecoveryAnnotator,
        &ClassicQueueRecoveryAnnotator,
        &PasswordHashAnnotator,
        &CredentialValidatorAnnotator,
        &InternalAuthAnnotator,
        &LdapAuthAnnotator,
        &CacheAuthAnnotator,
        &TlsCertificateAnnotator,
        &TlsHandshakeAnnotator,
        &DnsLookupAnnotator,
        &SocketErrorAnnotator,
        &PrometheusMetricsAnnotator,
        &ShovelTopologyAnnotator,
        &ShovelConnectionAnnotator,
        &FederationLinkAnnotator,
        &MemoryAlarmAnnotator,
        &DiskAlarmAnnotator,
        &FileDescriptorAlarmAnnotator,
        &HeartbeatTimeoutAnnotator,
        &BootStepAnnotator,
        &VhostSupervisorAnnotator,
        &ConsumerCancellationAnnotator,
        &BasicGetAnnotator,
        &BasicConsumeAnnotator,
        &TransientQueueAnnotator,
        &LazyQueueAnnotator,
        &DeadLetterAnnotator,
        &MessageTtlAnnotator,
        &QueueLengthLimitAnnotator,
        &NetworkPartitionAnnotator,
        &ConsistentHashExchangeAnnotator,
        &HeadersExchangeAnnotator,
        &TopicExchangeAnnotator,
        &DirectExchangeAnnotator,
        &FanoutExchangeAnnotator,
        &ExchangeToExchangeAnnotator,
        &BindingAnnotator,
        &PgScopeAnnotator,
        &RaLogAnnotator,
        &RaSnapshotAnnotator,
        &RaMemberAnnotator,
        &OperatorPolicyAnnotator,
        &UserPolicyAnnotator,
        &EffectivePolicyAnnotator,
        &ClusteringBannerAnnotator,
        &BootFailedAnnotator,
        &RanchListenerFailedAnnotator,
        &NodeMonitorStartAnnotator,
        &NodeDownWithReasonAnnotator,
        &VhostRecordUpdateAnnotator,
        &VhostSupervisorStopAnnotator,
        &DeliveryLimitDefaultAnnotator,
        &RaServerExitedAnnotator,
        &MetadataStoreMembersAnnotator,
        &RestartingRaServerAnnotator,
        &StoppingRanchAnnotator,
        &StoppingRaSystemsAnnotator,
    ];

    let mut labels = LogEntryLabels::default();

    for annotator in ANNOTATORS {
        if annotator.does_match(entry) {
            annotator.annotate(&mut labels);
        }
    }

    labels
}
