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
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

/// RabbitMQ subsystems that can be identified from log messages
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    IntoPrimitive,
    TryFromPrimitive,
    Display,
    EnumString,
)]
#[repr(i16)]
#[strum(serialize_all = "snake_case")]
pub enum Subsystem {
    MetadataStore = 1,
    FeatureFlags = 2,
    Boot = 3,
    Raft = 4,
    PeerDiscovery = 5,
    Plugins = 6,
    AccessControl = 7,
    Connections = 8,
    Shovels = 9,
    ClassicQueues = 10,
    VirtualHosts = 11,
    RuntimeParameters = 12,
    Federation = 13,
    Mqtt = 14,
    Policies = 15,
    MaintenanceMode = 16,
    ErlangOtp = 17,
    Exchanges = 18,
    Channels = 19,
    Shutdown = 20,
    Clustering = 21,
    Limits = 22,
    Logging = 23,
    Streams = 24,
    Queues = 25,
    OAuth2 = 26,
    Management = 27,
    Metrics = 28,
    Amqp10 = 29,
}

impl Subsystem {
    #[inline]
    pub fn to_id(self) -> i16 {
        self.into()
    }

    #[inline]
    pub fn from_id(id: i16) -> Option<Self> {
        Self::try_from(id).ok()
    }
}
