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
    LabelAnnotator, StompAccessControlAnnotator, StompConnectionsAnnotator,
    StompDisconnectsAnnotator, StompHeartbeatAnnotator, StompNetworkingAnnotator,
    StompProtocolErrorAnnotator, StompTlsAnnotator,
};
use rlqt_lib::entry_metadata::labels::LogEntryLabels;
use test_helpers::create_test_entry;

#[test]
fn test_stomp_connections_annotator_matches() {
    let entry = create_test_entry(
        "accepting STOMP connection from 192.168.1.1:54321",
        Severity::Info,
    );
    let annotator = StompConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_stomp_connections_annotator_sets_labels() {
    let annotator = StompConnectionsAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);
    assert!(labels.contains(LogEntryLabels::STOMP));
    assert!(labels.contains(LogEntryLabels::CONNECTIONS));
}

#[test]
fn test_stomp_disconnects_annotator_matches() {
    let entry = create_test_entry(
        "closing STOMP connection due to client disconnect",
        Severity::Info,
    );
    let annotator = StompDisconnectsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_stomp_disconnects_annotator_sets_labels() {
    let annotator = StompDisconnectsAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);
    assert!(labels.contains(LogEntryLabels::STOMP));
    assert!(labels.contains(LogEntryLabels::DISCONNECTS));
}

#[test]
fn test_stomp_access_control_annotator_matches() {
    let entry = create_test_entry(
        "STOMP login failed for user 'test': access_refused",
        Severity::Warning,
    );
    let annotator = StompAccessControlAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_stomp_access_control_annotator_sets_labels() {
    let annotator = StompAccessControlAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);
    assert!(labels.contains(LogEntryLabels::STOMP));
    assert!(labels.contains(LogEntryLabels::ACCESS_CONTROL));
}

#[test]
fn test_stomp_protocol_error_annotator_matches() {
    let entry = create_test_entry("STOMP error frame sent to client", Severity::Error);
    let annotator = StompProtocolErrorAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_stomp_protocol_error_annotator_sets_labels() {
    let annotator = StompProtocolErrorAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);
    assert!(labels.contains(LogEntryLabels::STOMP));
    assert!(labels.contains(LogEntryLabels::CHANNEL_EXCEPTIONS));
}

#[test]
fn test_stomp_tls_annotator_matches() {
    let entry = create_test_entry(
        "STOMP detected TLS error: certificate expired",
        Severity::Error,
    );
    let annotator = StompTlsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_stomp_tls_annotator_sets_labels() {
    let annotator = StompTlsAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);
    assert!(labels.contains(LogEntryLabels::STOMP));
    assert!(labels.contains(LogEntryLabels::TLS));
}

#[test]
fn test_stomp_networking_annotator_matches() {
    let entry = create_test_entry(
        "STOMP detected network error: connection reset",
        Severity::Warning,
    );
    let annotator = StompNetworkingAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_stomp_networking_annotator_sets_labels() {
    let annotator = StompNetworkingAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);
    assert!(labels.contains(LogEntryLabels::STOMP));
    assert!(labels.contains(LogEntryLabels::NETWORKING));
}

#[test]
fn test_stomp_heartbeat_annotator_matches() {
    let entry = create_test_entry("STOMP detected missed client heartbeat", Severity::Warning);
    let annotator = StompHeartbeatAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_stomp_heartbeat_annotator_sets_labels() {
    let annotator = StompHeartbeatAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);
    assert!(labels.contains(LogEntryLabels::STOMP));
    assert!(labels.contains(LogEntryLabels::TIMEOUTS));
}
