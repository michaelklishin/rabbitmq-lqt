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
use rlqt_lib::entry_metadata::label_annotators::{ConnectionsAnnotator, LabelAnnotator};
use rlqt_lib::entry_metadata::labels::LogEntryLabels;
use test_helpers::create_test_entry;

#[test]
fn test_connections_annotator_matches_authenticated_by_backend() {
    let entry = create_test_entry("User authenticated successfully by backend", Severity::Info);
    let annotator = ConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_connections_annotator_matches_client_address() {
    let entry = create_test_entry(
        "Client address during authN phase: 192.168.1.100",
        Severity::Info,
    );
    let annotator = ConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_connections_annotator_matches_granted_access() {
    let entry = create_test_entry(
        "User authenticated and granted access to vhost '/'",
        Severity::Info,
    );
    let annotator = ConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_connections_annotator_matches_failed_to_authenticate() {
    let entry = create_test_entry(
        "Connection (<0.1234.0>) from 10.0.0.1:54321 failed to authenticate",
        Severity::Warning,
    );
    let annotator = ConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_connections_annotator_matches_connection_closed() {
    let entry = create_test_entry(
        "connection_closed: connection <0.1234.0> closed",
        Severity::Info,
    );
    let annotator = ConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_connections_annotator_no_match() {
    let entry = create_test_entry("Unrelated message", Severity::Info);
    let annotator = ConnectionsAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_connections_annotator_annotates_both_labels() {
    let annotator = ConnectionsAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);
    assert!(labels.contains(LogEntryLabels::CONNECTIONS));
    assert!(labels.contains(LogEntryLabels::ACCESS_CONTROL));
}
