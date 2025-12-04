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

use rlqt_lib::entry_metadata::subsystems::Subsystem;

#[test]
fn test_subsystem_to_id_all_variants() {
    assert_eq!(Subsystem::MetadataStore.to_id(), 1);
    assert_eq!(Subsystem::FeatureFlags.to_id(), 2);
    assert_eq!(Subsystem::Boot.to_id(), 3);
    assert_eq!(Subsystem::Raft.to_id(), 4);
    assert_eq!(Subsystem::PeerDiscovery.to_id(), 5);
    assert_eq!(Subsystem::Plugins.to_id(), 6);
    assert_eq!(Subsystem::AccessControl.to_id(), 7);
    assert_eq!(Subsystem::Connections.to_id(), 8);
    assert_eq!(Subsystem::ShovelPlugin.to_id(), 9);
    assert_eq!(Subsystem::ClassicQueues.to_id(), 10);
    assert_eq!(Subsystem::VirtualHosts.to_id(), 11);
    assert_eq!(Subsystem::RuntimeParameters.to_id(), 12);
    assert_eq!(Subsystem::FederationPlugin.to_id(), 13);
    assert_eq!(Subsystem::MqttPlugin.to_id(), 14);
}

#[test]
fn test_subsystem_from_id_all_variants() {
    assert_eq!(Subsystem::from_id(1), Some(Subsystem::MetadataStore));
    assert_eq!(Subsystem::from_id(2), Some(Subsystem::FeatureFlags));
    assert_eq!(Subsystem::from_id(3), Some(Subsystem::Boot));
    assert_eq!(Subsystem::from_id(4), Some(Subsystem::Raft));
    assert_eq!(Subsystem::from_id(5), Some(Subsystem::PeerDiscovery));
    assert_eq!(Subsystem::from_id(6), Some(Subsystem::Plugins));
    assert_eq!(Subsystem::from_id(7), Some(Subsystem::AccessControl));
    assert_eq!(Subsystem::from_id(8), Some(Subsystem::Connections));
    assert_eq!(Subsystem::from_id(9), Some(Subsystem::ShovelPlugin));
    assert_eq!(Subsystem::from_id(10), Some(Subsystem::ClassicQueues));
    assert_eq!(Subsystem::from_id(11), Some(Subsystem::VirtualHosts));
    assert_eq!(Subsystem::from_id(12), Some(Subsystem::RuntimeParameters));
    assert_eq!(Subsystem::from_id(13), Some(Subsystem::FederationPlugin));
    assert_eq!(Subsystem::from_id(14), Some(Subsystem::MqttPlugin));
    assert_eq!(Subsystem::from_id(15), Some(Subsystem::Policies));
    assert_eq!(Subsystem::from_id(16), Some(Subsystem::MaintenanceMode));
    assert_eq!(Subsystem::from_id(17), Some(Subsystem::ErlangOtp));
    assert_eq!(Subsystem::from_id(18), Some(Subsystem::Exchanges));
    assert_eq!(Subsystem::from_id(19), Some(Subsystem::Channels));
}

#[test]
fn test_subsystem_from_id_invalid() {
    assert_eq!(Subsystem::from_id(0), None);
    assert_eq!(Subsystem::from_id(20), None);
    assert_eq!(Subsystem::from_id(-1), None);
    assert_eq!(Subsystem::from_id(100), None);
    assert_eq!(Subsystem::from_id(i16::MAX), None);
    assert_eq!(Subsystem::from_id(i16::MIN), None);
}

#[test]
fn test_subsystem_round_trip_conversion() {
    let subsystems = [
        Subsystem::MetadataStore,
        Subsystem::FeatureFlags,
        Subsystem::Boot,
        Subsystem::Raft,
        Subsystem::PeerDiscovery,
        Subsystem::Plugins,
        Subsystem::AccessControl,
        Subsystem::Connections,
        Subsystem::ShovelPlugin,
        Subsystem::ClassicQueues,
        Subsystem::VirtualHosts,
        Subsystem::RuntimeParameters,
        Subsystem::FederationPlugin,
        Subsystem::MqttPlugin,
        Subsystem::Policies,
        Subsystem::MaintenanceMode,
        Subsystem::ErlangOtp,
        Subsystem::Exchanges,
        Subsystem::Channels,
    ];

    for subsystem in subsystems {
        let id = subsystem.to_id();
        let recovered = Subsystem::from_id(id);
        assert_eq!(
            recovered,
            Some(subsystem),
            "Round-trip failed for {:?} (id={})",
            subsystem,
            id
        );
    }
}

#[test]
fn test_subsystem_id_uniqueness() {
    let subsystems = [
        Subsystem::MetadataStore,
        Subsystem::FeatureFlags,
        Subsystem::Boot,
        Subsystem::Raft,
        Subsystem::PeerDiscovery,
        Subsystem::Plugins,
        Subsystem::AccessControl,
        Subsystem::Connections,
        Subsystem::ShovelPlugin,
        Subsystem::ClassicQueues,
        Subsystem::VirtualHosts,
        Subsystem::RuntimeParameters,
        Subsystem::FederationPlugin,
        Subsystem::MqttPlugin,
        Subsystem::Policies,
        Subsystem::MaintenanceMode,
        Subsystem::ErlangOtp,
        Subsystem::Exchanges,
        Subsystem::Channels,
    ];

    let mut ids = Vec::new();
    for subsystem in subsystems {
        let id = subsystem.to_id();
        assert!(
            !ids.contains(&id),
            "Duplicate ID {} found for {:?}",
            id,
            subsystem
        );
        ids.push(id);
    }

    assert_eq!(ids.len(), 19, "Expected 19 unique subsystem IDs");
}

#[test]
fn test_subsystem_id_range() {
    let subsystems = [
        Subsystem::MetadataStore,
        Subsystem::FeatureFlags,
        Subsystem::Boot,
        Subsystem::Raft,
        Subsystem::PeerDiscovery,
        Subsystem::Plugins,
        Subsystem::AccessControl,
        Subsystem::Connections,
        Subsystem::ShovelPlugin,
        Subsystem::ClassicQueues,
        Subsystem::VirtualHosts,
        Subsystem::RuntimeParameters,
        Subsystem::FederationPlugin,
        Subsystem::MqttPlugin,
        Subsystem::Policies,
        Subsystem::MaintenanceMode,
        Subsystem::ErlangOtp,
        Subsystem::Exchanges,
        Subsystem::Channels,
    ];

    for subsystem in subsystems {
        let id = subsystem.to_id();
        assert!(
            (1..=19).contains(&id),
            "ID {} for {:?} is outside expected range [1, 19]",
            id,
            subsystem
        );
    }
}
