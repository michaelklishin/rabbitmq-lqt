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
use rabbitmq_lqt_lib::entry_metadata::label_annotators::{
    LabelAnnotator, QueueFederationAnnotator,
};
use rabbitmq_lqt_lib::entry_metadata::labels::LogEntryLabels;

#[test]
fn test_queue_federation_annotator_matches_federation_queue() {
    let entry = create_test_entry("Federation queue started", Severity::Info);
    let annotator = QueueFederationAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_queue_federation_annotator_matches_disconnecting() {
    let entry = create_test_entry("Disconnecting from queue upstream", Severity::Info);
    let annotator = QueueFederationAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_queue_federation_annotator_matches_case_insensitive() {
    let entry = create_test_entry("FEDERATION QUEUE started", Severity::Info);
    let annotator = QueueFederationAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_queue_federation_annotator_no_match() {
    let entry = create_test_entry("Unrelated message", Severity::Info);
    let annotator = QueueFederationAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_queue_federation_annotator_annotates() {
    let annotator = QueueFederationAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);
    assert!(labels.contains(LogEntryLabels::QUEUE_FEDERATION));
}
