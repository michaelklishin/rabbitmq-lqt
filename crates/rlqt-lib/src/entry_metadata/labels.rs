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
use bitflags::bitflags;
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{self, Formatter};

/// Label name constants - single source of truth for label identifiers
pub const LABEL_UNLABELLED: &str = "unlabelled";
pub const LABEL_ERL_PROCESS_CRASH: &str = "erl_process_crash";
pub const LABEL_UNDEFINED_FN: &str = "undefined_fn";
pub const LABEL_PROCESS_STOPS: &str = "process_stops";
pub const LABEL_RAFT: &str = "raft";
pub const LABEL_ELECTIONS: &str = "elections";
pub const LABEL_QUEUES: &str = "queues";
pub const LABEL_AUTO_DELETE: &str = "auto_delete";
pub const LABEL_EXCLUSIVE: &str = "exclusive";
pub const LABEL_CHANNEL_EXCEPTIONS: &str = "channel_exceptions";
pub const LABEL_DELETE: &str = "delete";
pub const LABEL_QUEUE_FEDERATION: &str = "queue_federation";
pub const LABEL_VIRTUAL_HOSTS: &str = "virtual_hosts";
pub const LABEL_CONNECTIONS: &str = "connections";
pub const LABEL_ACCESS_CONTROL: &str = "access_control";
pub const LABEL_SHOVELS: &str = "shovels";
pub const LABEL_CQ_STORES: &str = "cq_stores";
pub const LABEL_DISCONNECTS: &str = "disconnects";
pub const LABEL_FEDERATION: &str = "federation";
pub const LABEL_DELETION_PROTECTION: &str = "deletion_protection";
pub const LABEL_MULTILINE: &str = "multiline";
pub const LABEL_STREAMS: &str = "streams";
pub const LABEL_LIMITS: &str = "limits";
pub const LABEL_WORKER_POOL: &str = "worker_pool";
pub const LABEL_PEER_DISCOVERY_CLASSIC: &str = "peer_discovery:classic";
pub const LABEL_PLUGINS: &str = "plugins";
pub const LABEL_EXCHANGES: &str = "exchanges";
pub const LABEL_STARTUP_BANNER: &str = "startup_banner";

/// Array of all label names in the order they're defined
pub const LABEL_NAMES: &[&str] = &[
    LABEL_UNLABELLED,
    LABEL_ERL_PROCESS_CRASH,
    LABEL_UNDEFINED_FN,
    LABEL_PROCESS_STOPS,
    LABEL_RAFT,
    LABEL_ELECTIONS,
    LABEL_QUEUES,
    LABEL_AUTO_DELETE,
    LABEL_EXCLUSIVE,
    LABEL_CHANNEL_EXCEPTIONS,
    LABEL_DELETE,
    LABEL_QUEUE_FEDERATION,
    LABEL_VIRTUAL_HOSTS,
    LABEL_CONNECTIONS,
    LABEL_ACCESS_CONTROL,
    LABEL_SHOVELS,
    LABEL_CQ_STORES,
    LABEL_DISCONNECTS,
    LABEL_FEDERATION,
    LABEL_DELETION_PROTECTION,
    LABEL_MULTILINE,
    LABEL_STREAMS,
    LABEL_LIMITS,
    LABEL_WORKER_POOL,
    LABEL_PEER_DISCOVERY_CLASSIC,
    LABEL_PLUGINS,
    LABEL_EXCHANGES,
    LABEL_STARTUP_BANNER,
];

bitflags! {
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
    pub struct LogEntryLabels: u64 {
        const UNLABELLED          = 1 << 0;
        const ERL_PROCESS_CRASH   = 1 << 1;
        const UNDEFINED_FN        = 1 << 2;
        const PROCESS_STOPS       = 1 << 3;
        const RAFT                = 1 << 4;
        const ELECTIONS           = 1 << 5;
        const QUEUES              = 1 << 6;
        const AUTO_DELETE         = 1 << 7;
        const EXCLUSIVE           = 1 << 8;
        const CHANNEL_EXCEPTIONS  = 1 << 9;
        const DELETE              = 1 << 10;
        const QUEUE_FEDERATION    = 1 << 11;
        const VIRTUAL_HOSTS       = 1 << 12;
        const CONNECTIONS         = 1 << 13;
        const ACCESS_CONTROL      = 1 << 14;
        const SHOVELS             = 1 << 15;
        const CQ_STORES           = 1 << 16;
        const DISCONNECTS         = 1 << 17;
        const FEDERATION          = 1 << 18;
        const DELETION_PROTECTION = 1 << 19;
        const MULTILINE           = 1 << 20;
        const STREAMS             = 1 << 21;
        const LIMITS              = 1 << 22;
        const WORKER_POOL         = 1 << 23;
        const PEER_DISCOVERY_CLASSIC = 1 << 24;
        const PLUGINS             = 1 << 25;
        const EXCHANGES           = 1 << 26;
        const STARTUP_BANNER      = 1 << 27;
    }
}

impl LogEntryLabels {
    #[inline]
    pub fn merge(&mut self, other: Self) {
        *self |= other;
    }
}

impl Serialize for LogEntryLabels {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;

        if self.contains(Self::UNLABELLED) {
            map.serialize_entry(LABEL_UNLABELLED, &true)?;
        }
        if self.contains(Self::ERL_PROCESS_CRASH) {
            map.serialize_entry(LABEL_ERL_PROCESS_CRASH, &true)?;
        }
        if self.contains(Self::UNDEFINED_FN) {
            map.serialize_entry(LABEL_UNDEFINED_FN, &true)?;
        }
        if self.contains(Self::PROCESS_STOPS) {
            map.serialize_entry(LABEL_PROCESS_STOPS, &true)?;
        }
        if self.contains(Self::RAFT) {
            map.serialize_entry(LABEL_RAFT, &true)?;
        }
        if self.contains(Self::ELECTIONS) {
            map.serialize_entry(LABEL_ELECTIONS, &true)?;
        }
        if self.contains(Self::QUEUES) {
            map.serialize_entry(LABEL_QUEUES, &true)?;
        }
        if self.contains(Self::AUTO_DELETE) {
            map.serialize_entry(LABEL_AUTO_DELETE, &true)?;
        }
        if self.contains(Self::EXCLUSIVE) {
            map.serialize_entry(LABEL_EXCLUSIVE, &true)?;
        }
        if self.contains(Self::CHANNEL_EXCEPTIONS) {
            map.serialize_entry(LABEL_CHANNEL_EXCEPTIONS, &true)?;
        }
        if self.contains(Self::DELETE) {
            map.serialize_entry(LABEL_DELETE, &true)?;
        }
        if self.contains(Self::QUEUE_FEDERATION) {
            map.serialize_entry(LABEL_QUEUE_FEDERATION, &true)?;
        }
        if self.contains(Self::VIRTUAL_HOSTS) {
            map.serialize_entry(LABEL_VIRTUAL_HOSTS, &true)?;
        }
        if self.contains(Self::CONNECTIONS) {
            map.serialize_entry(LABEL_CONNECTIONS, &true)?;
        }
        if self.contains(Self::ACCESS_CONTROL) {
            map.serialize_entry(LABEL_ACCESS_CONTROL, &true)?;
        }
        if self.contains(Self::SHOVELS) {
            map.serialize_entry(LABEL_SHOVELS, &true)?;
        }
        if self.contains(Self::CQ_STORES) {
            map.serialize_entry(LABEL_CQ_STORES, &true)?;
        }
        if self.contains(Self::DISCONNECTS) {
            map.serialize_entry(LABEL_DISCONNECTS, &true)?;
        }
        if self.contains(Self::FEDERATION) {
            map.serialize_entry(LABEL_FEDERATION, &true)?;
        }
        if self.contains(Self::DELETION_PROTECTION) {
            map.serialize_entry(LABEL_DELETION_PROTECTION, &true)?;
        }
        if self.contains(Self::MULTILINE) {
            map.serialize_entry(LABEL_MULTILINE, &true)?;
        }
        if self.contains(Self::STREAMS) {
            map.serialize_entry(LABEL_STREAMS, &true)?;
        }
        if self.contains(Self::LIMITS) {
            map.serialize_entry(LABEL_LIMITS, &true)?;
        }
        if self.contains(Self::WORKER_POOL) {
            map.serialize_entry(LABEL_WORKER_POOL, &true)?;
        }
        if self.contains(Self::PEER_DISCOVERY_CLASSIC) {
            map.serialize_entry(LABEL_PEER_DISCOVERY_CLASSIC, &true)?;
        }
        if self.contains(Self::PLUGINS) {
            map.serialize_entry(LABEL_PLUGINS, &true)?;
        }
        if self.contains(Self::EXCHANGES) {
            map.serialize_entry(LABEL_EXCHANGES, &true)?;
        }
        if self.contains(Self::STARTUP_BANNER) {
            map.serialize_entry(LABEL_STARTUP_BANNER, &true)?;
        }

        map.end()
    }
}

impl<'de> Deserialize<'de> for LogEntryLabels {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LabelsVisitor;

        impl<'de> Visitor<'de> for LabelsVisitor {
            type Value = LogEntryLabels;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("a map of label flags")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut labels = LogEntryLabels::empty();

                while let Some(key) = map.next_key::<String>()? {
                    let value: bool = map.next_value()?;
                    if value {
                        match key.as_str() {
                            LABEL_UNLABELLED => labels |= LogEntryLabels::UNLABELLED,
                            LABEL_ERL_PROCESS_CRASH => labels |= LogEntryLabels::ERL_PROCESS_CRASH,
                            LABEL_UNDEFINED_FN => labels |= LogEntryLabels::UNDEFINED_FN,
                            LABEL_PROCESS_STOPS => labels |= LogEntryLabels::PROCESS_STOPS,
                            LABEL_RAFT => labels |= LogEntryLabels::RAFT,
                            LABEL_ELECTIONS => labels |= LogEntryLabels::ELECTIONS,
                            LABEL_QUEUES => labels |= LogEntryLabels::QUEUES,
                            LABEL_AUTO_DELETE => labels |= LogEntryLabels::AUTO_DELETE,
                            LABEL_EXCLUSIVE => labels |= LogEntryLabels::EXCLUSIVE,
                            LABEL_CHANNEL_EXCEPTIONS => {
                                labels |= LogEntryLabels::CHANNEL_EXCEPTIONS
                            }
                            LABEL_DELETE => labels |= LogEntryLabels::DELETE,
                            LABEL_QUEUE_FEDERATION => labels |= LogEntryLabels::QUEUE_FEDERATION,
                            LABEL_VIRTUAL_HOSTS => labels |= LogEntryLabels::VIRTUAL_HOSTS,
                            LABEL_CONNECTIONS => labels |= LogEntryLabels::CONNECTIONS,
                            LABEL_ACCESS_CONTROL => labels |= LogEntryLabels::ACCESS_CONTROL,
                            LABEL_SHOVELS => labels |= LogEntryLabels::SHOVELS,
                            LABEL_CQ_STORES => labels |= LogEntryLabels::CQ_STORES,
                            LABEL_DISCONNECTS => labels |= LogEntryLabels::DISCONNECTS,
                            LABEL_FEDERATION => labels |= LogEntryLabels::FEDERATION,
                            LABEL_DELETION_PROTECTION => {
                                labels |= LogEntryLabels::DELETION_PROTECTION
                            }
                            LABEL_MULTILINE => labels |= LogEntryLabels::MULTILINE,
                            LABEL_STREAMS => labels |= LogEntryLabels::STREAMS,
                            LABEL_LIMITS => labels |= LogEntryLabels::LIMITS,
                            LABEL_WORKER_POOL => labels |= LogEntryLabels::WORKER_POOL,
                            LABEL_PEER_DISCOVERY_CLASSIC => {
                                labels |= LogEntryLabels::PEER_DISCOVERY_CLASSIC
                            }
                            LABEL_PLUGINS => labels |= LogEntryLabels::PLUGINS,
                            LABEL_EXCHANGES => labels |= LogEntryLabels::EXCHANGES,
                            LABEL_STARTUP_BANNER => labels |= LogEntryLabels::STARTUP_BANNER,
                            _ => {}
                        }
                    }
                }

                Ok(labels)
            }
        }

        deserializer.deserialize_map(LabelsVisitor)
    }
}
