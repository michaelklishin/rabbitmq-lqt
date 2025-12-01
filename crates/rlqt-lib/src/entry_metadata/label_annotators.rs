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
    matches_cq_storage, matches_federation, matches_plugins, matches_shovels, matches_virtual_hosts,
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
    }
}

impl LabelAnnotator for ConnectionsAnnotator {
    fn annotate(&self, labels: &mut LogEntryLabels) {
        *labels |= LogEntryLabels::CONNECTIONS | LogEntryLabels::ACCESS_CONTROL;
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
        entry.message_lowercased.contains("rabbit_stream")
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
    ];

    let mut labels = LogEntryLabels::default();

    for annotator in ANNOTATORS {
        if annotator.does_match(entry) {
            annotator.annotate(&mut labels);
        }
    }

    labels
}
