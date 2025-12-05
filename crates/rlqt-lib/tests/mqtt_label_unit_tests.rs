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

mod test_helpers;

use rlqt_lib::Severity;
use rlqt_lib::entry_metadata::Annotator;
use rlqt_lib::entry_metadata::label_annotators::{
    LabelAnnotator, MqttAccessControlAnnotator, MqttConnectionsAnnotator, MqttDisconnectsAnnotator,
    MqttNetworkingAnnotator, MqttProtocolErrorAnnotator, MqttRetainedStoreAnnotator,
    MqttTlsAnnotator, annotate_labels,
};
use rlqt_lib::entry_metadata::labels::LogEntryLabels;
use test_helpers::create_test_entry;

#[test]
fn test_started_mqtt() {
    let entry = create_test_entry("started MQTT TCP listener on [::]:1883", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::MQTT));
    assert!(!labels.contains(LogEntryLabels::WEBSOCKETS));
}

#[test]
fn test_stopped_mqtt() {
    let entry = create_test_entry("stopped MQTT TCP listener on [::]:1883", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::MQTT));
    assert!(labels.contains(LogEntryLabels::SHUTDOWN));
}

#[test]
fn test_rabbit_web_mqtt() {
    let entry = create_test_entry("rabbit_web_mqtt starting on port 15675", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::MQTT));
    assert!(labels.contains(LogEntryLabels::WEBSOCKETS));
}

#[test]
fn test_mqtt_retained_store_annotator_matches() {
    let entry = create_test_entry("MQTT retained message store initialized", Severity::Info);
    let annotator = MqttRetainedStoreAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_mqtt_retained_store_annotator_sets_label() {
    let annotator = MqttRetainedStoreAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);
    assert!(labels.contains(LogEntryLabels::MQTT));
}

#[test]
fn test_mqtt_connections_annotator_matches_accepting() {
    let entry = create_test_entry(
        "MQTT accepting TCP connection from 192.168.1.1:54321",
        Severity::Info,
    );
    let annotator = MqttConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_mqtt_connections_annotator_matches_accepted() {
    let entry = create_test_entry("MQTT connection accepted", Severity::Info);
    let annotator = MqttConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_mqtt_connections_annotator_matches_rejected() {
    let entry = create_test_entry("MQTT connection rejected", Severity::Warning);
    let annotator = MqttConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_mqtt_connections_annotator_sets_labels() {
    let annotator = MqttConnectionsAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);
    assert!(labels.contains(LogEntryLabels::MQTT));
    assert!(labels.contains(LogEntryLabels::CONNECTIONS));
}

#[test]
fn test_mqtt_disconnects_annotator_matches() {
    let entry = create_test_entry(
        "MQTT closing connection due to client disconnect",
        Severity::Info,
    );
    let annotator = MqttDisconnectsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_mqtt_disconnects_annotator_sets_labels() {
    let annotator = MqttDisconnectsAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);
    assert!(labels.contains(LogEntryLabels::MQTT));
    assert!(labels.contains(LogEntryLabels::DISCONNECTS));
}

#[test]
fn test_mqtt_access_control_annotator_matches() {
    let entry = create_test_entry(
        "MQTT login failed for user 'test': access_refused",
        Severity::Warning,
    );
    let annotator = MqttAccessControlAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_mqtt_access_control_annotator_sets_labels() {
    let annotator = MqttAccessControlAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);
    assert!(labels.contains(LogEntryLabels::MQTT));
    assert!(labels.contains(LogEntryLabels::ACCESS_CONTROL));
}

#[test]
fn test_mqtt_protocol_error_annotator_matches() {
    let entry = create_test_entry("MQTT protocol error: invalid packet", Severity::Error);
    let annotator = MqttProtocolErrorAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_mqtt_protocol_error_annotator_sets_labels() {
    let annotator = MqttProtocolErrorAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);
    assert!(labels.contains(LogEntryLabels::MQTT));
    assert!(labels.contains(LogEntryLabels::EXCEPTIONS));
}

#[test]
fn test_mqtt_tls_annotator_matches() {
    let entry = create_test_entry(
        "MQTT detected TLS error: certificate expired",
        Severity::Error,
    );
    let annotator = MqttTlsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_mqtt_tls_annotator_sets_labels() {
    let annotator = MqttTlsAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);
    assert!(labels.contains(LogEntryLabels::MQTT));
    assert!(labels.contains(LogEntryLabels::TLS));
}

#[test]
fn test_mqtt_networking_annotator_matches() {
    let entry = create_test_entry(
        "MQTT detected network error: connection reset",
        Severity::Warning,
    );
    let annotator = MqttNetworkingAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_mqtt_networking_annotator_sets_labels() {
    let annotator = MqttNetworkingAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);
    assert!(labels.contains(LogEntryLabels::MQTT));
    assert!(labels.contains(LogEntryLabels::NETWORKING));
}
