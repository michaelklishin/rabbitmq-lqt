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
    let entry = create_test_entry(
        "User 'guest' authenticated successfully by backend rabbit_auth_backend_internal",
        Severity::Debug,
    );
    let annotator = ConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_connections_annotator_matches_granted_access() {
    let entry = create_test_entry(
        "connection [::1]:57941 -> [::1]:5672 - perf-test-configuration-0: user 'guest' authenticated and granted access to vhost '/'",
        Severity::Info,
    );
    let annotator = ConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_connections_annotator_matches_connection_closed() {
    let entry = create_test_entry(
        "node hare@sunnyside down: connection_closed",
        Severity::Info,
    );
    let annotator = ConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_connections_annotator_no_match() {
    let entry = create_test_entry("ra: starting system quorum_queues", Severity::Info);
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
