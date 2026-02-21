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
use rabbitmq_lqt_lib::entry_metadata::label_annotators::annotate_labels;
use rabbitmq_lqt_lib::entry_metadata::labels::LogEntryLabels;

#[test]
fn test_stopped_tcp_listener() {
    let entry = create_test_entry("stopped TCP listener on [::]:5672", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::SHUTDOWN));
}

#[test]
fn test_stopped_tls_listener() {
    let entry = create_test_entry("stopped TLS (SSL) listener on [::]:5671", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::SHUTDOWN));
}

#[test]
fn test_stopped_mqtt_listener() {
    let entry = create_test_entry("stopped MQTT TCP listener on [::]:1883", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::SHUTDOWN));
    assert!(labels.contains(LogEntryLabels::MQTT));
}

#[test]
fn test_stopped_stomp_listener() {
    let entry = create_test_entry("stopped STOMP TCP listener on [::]:61613", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::SHUTDOWN));
    assert!(labels.contains(LogEntryLabels::STOMP));
}

#[test]
fn test_stopped_ssl_listener() {
    let entry = create_test_entry("stopped SSL listener on [::]:5671", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::SHUTDOWN));
}

#[test]
fn test_stopping_ranch_listeners_for_protocol() {
    let entry = create_test_entry(
        "Stopping Ranch listeners for protocol amqp",
        Severity::Debug,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::SHUTDOWN));
    assert!(labels.contains(LogEntryLabels::NETWORKING));
}

#[test]
fn test_stopping_ra_systems_shutdown() {
    let entry = create_test_entry("Stopping Ra systems", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::SHUTDOWN));
}
