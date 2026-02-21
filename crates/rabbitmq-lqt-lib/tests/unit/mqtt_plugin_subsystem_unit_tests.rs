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

use crate::test_helpers::create_test_entry;
use rabbitmq_lqt_lib::Severity;
use rabbitmq_lqt_lib::entry_metadata::Annotator;
use rabbitmq_lqt_lib::entry_metadata::subsystem_annotators::{MqttAnnotator, SubsystemAnnotator};
use rabbitmq_lqt_lib::entry_metadata::subsystems::Subsystem;

#[test]
fn test_mqtt_retained_message_store() {
    let entry = create_test_entry("MQTT retained message store initializing", Severity::Info);
    let annotator = MqttAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_mqtt_retained_message_store_in_middle() {
    let entry = create_test_entry(
        "Starting MQTT retained message store for vhost '/'",
        Severity::Info,
    );
    let annotator = MqttAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_mqtt_retained_message_store_case_insensitive() {
    let entry = create_test_entry("mqtt RETAINED MESSAGE STORE ready", Severity::Info);
    let annotator = MqttAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_no_match_mqtt_without_retained_store() {
    let entry = create_test_entry("MQTT connection accepted", Severity::Info);
    let annotator = MqttAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_no_match_unrelated_message() {
    let entry = create_test_entry("Queue created successfully", Severity::Info);
    let annotator = MqttAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_annotate_sets_subsystem() {
    let mut entry = create_test_entry("MQTT retained message store initialized", Severity::Info);
    let annotator = MqttAnnotator;
    annotator.annotate(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Mqtt.to_id()));
}
