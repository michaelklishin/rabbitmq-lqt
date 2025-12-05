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

use rlqt_lib::entry_metadata::labels::{LABEL_NAMES, LogEntryLabels};

#[test]
fn test_serialize_deserialize_roundtrip_empty() {
    let labels = LogEntryLabels::empty();
    let serialized = serde_json::to_string(&labels).unwrap();
    let deserialized: LogEntryLabels = serde_json::from_str(&serialized).unwrap();
    assert_eq!(labels, deserialized);
}

#[test]
fn test_serialize_deserialize_roundtrip_single_label() {
    let labels = LogEntryLabels::ELECTIONS;
    let serialized = serde_json::to_string(&labels).unwrap();
    let deserialized: LogEntryLabels = serde_json::from_str(&serialized).unwrap();
    assert_eq!(labels, deserialized);
}

#[test]
fn test_serialize_deserialize_roundtrip_multiple_labels() {
    let labels = LogEntryLabels::RAFT | LogEntryLabels::ELECTIONS | LogEntryLabels::QUEUES;
    let serialized = serde_json::to_string(&labels).unwrap();
    let deserialized: LogEntryLabels = serde_json::from_str(&serialized).unwrap();
    assert_eq!(labels, deserialized);
}

#[test]
fn test_serialize_deserialize_roundtrip_all_labels() {
    let labels = LogEntryLabels::all();
    let serialized = serde_json::to_string(&labels).unwrap();
    let deserialized: LogEntryLabels = serde_json::from_str(&serialized).unwrap();
    assert_eq!(labels, deserialized);
}

#[test]
fn test_serialize_deserialize_roundtrip_each_label_individually() {
    let all_flags = [
        LogEntryLabels::UNLABELLED,
        LogEntryLabels::ERL_PROCESS_CRASH,
        LogEntryLabels::UNDEFINED_FN,
        LogEntryLabels::PROCESS_STOPS,
        LogEntryLabels::RAFT,
        LogEntryLabels::ELECTIONS,
        LogEntryLabels::QUEUES,
        LogEntryLabels::AUTO_DELETE,
        LogEntryLabels::EXCLUSIVE,
        LogEntryLabels::EXCEPTIONS,
        LogEntryLabels::DELETE,
        LogEntryLabels::QUEUE_FEDERATION,
        LogEntryLabels::VIRTUAL_HOSTS,
        LogEntryLabels::CONNECTIONS,
        LogEntryLabels::ACCESS_CONTROL,
        LogEntryLabels::SHOVELS,
        LogEntryLabels::CQ_STORES,
        LogEntryLabels::DISCONNECTS,
        LogEntryLabels::FEDERATION,
        LogEntryLabels::DELETION_PROTECTION,
        LogEntryLabels::MULTILINE,
        LogEntryLabels::STREAMS,
        LogEntryLabels::LIMITS,
        LogEntryLabels::WORKER_POOL,
        LogEntryLabels::PEER_DISCOVERY_CLASSIC,
        LogEntryLabels::PLUGINS,
        LogEntryLabels::EXCHANGES,
        LogEntryLabels::STARTUP_BANNER,
        LogEntryLabels::CHANNELS,
        LogEntryLabels::SHUTDOWN,
        LogEntryLabels::DEFINITIONS,
        LogEntryLabels::FEATURE_FLAGS,
        LogEntryLabels::STOMP,
        LogEntryLabels::WEBSOCKETS,
        LogEntryLabels::MQTT,
        LogEntryLabels::CLUSTERING,
        LogEntryLabels::METRICS,
        LogEntryLabels::TLS,
        LogEntryLabels::QUORUM_QUEUES,
        LogEntryLabels::NETWORKING,
        LogEntryLabels::CLASSIC_QUEUES,
        LogEntryLabels::POLICIES,
        LogEntryLabels::TIMEOUTS,
        LogEntryLabels::CONSUMERS,
        LogEntryLabels::DEPRECATED_FEATURES,
        LogEntryLabels::MAINTENANCE_MODE,
        LogEntryLabels::KHEPRI,
        LogEntryLabels::RUNTIME_PARAMETERS,
        LogEntryLabels::HTTP,
        LogEntryLabels::SESSIONS,
        LogEntryLabels::AMQP10,
    ];

    assert_eq!(
        all_flags.len(),
        LABEL_NAMES.len(),
        "Test must cover all {} labels",
        LABEL_NAMES.len()
    );

    for label in all_flags {
        let serialized = serde_json::to_string(&label).unwrap();
        let deserialized: LogEntryLabels = serde_json::from_str(&serialized).unwrap();
        assert_eq!(
            label, deserialized,
            "Round-trip failed for label with bits {:?}",
            label
        );
    }
}

#[test]
fn test_deserialize_ignores_unknown_labels() {
    let json = r#"{"elections": true, "unknown_label": true, "raft": true}"#;
    let labels: LogEntryLabels = serde_json::from_str(json).unwrap();
    assert!(labels.contains(LogEntryLabels::ELECTIONS));
    assert!(labels.contains(LogEntryLabels::RAFT));
}

#[test]
fn test_deserialize_ignores_false_values() {
    let json = r#"{"elections": true, "raft": false}"#;
    let labels: LogEntryLabels = serde_json::from_str(json).unwrap();
    assert!(labels.contains(LogEntryLabels::ELECTIONS));
    assert!(!labels.contains(LogEntryLabels::RAFT));
}

#[test]
fn test_serialize_only_includes_set_labels() {
    let labels = LogEntryLabels::ELECTIONS | LogEntryLabels::RAFT;
    let serialized = serde_json::to_string(&labels).unwrap();
    let value: serde_json::Value = serde_json::from_str(&serialized).unwrap();
    let obj = value.as_object().unwrap();

    assert_eq!(obj.len(), 2);
    assert_eq!(obj.get("elections"), Some(&serde_json::Value::Bool(true)));
    assert_eq!(obj.get("raft"), Some(&serde_json::Value::Bool(true)));
}

#[test]
fn test_serialize_empty_produces_empty_object() {
    let labels = LogEntryLabels::empty();
    let serialized = serde_json::to_string(&labels).unwrap();
    assert_eq!(serialized, "{}");
}

#[test]
fn test_deserialize_empty_object() {
    let json = "{}";
    let labels: LogEntryLabels = serde_json::from_str(json).unwrap();
    assert!(labels.is_empty());
}
