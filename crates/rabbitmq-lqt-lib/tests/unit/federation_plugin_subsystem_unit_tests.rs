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
use rabbitmq_lqt_lib::entry_metadata::subsystem_annotators::{
    FederationAnnotator, SubsystemAnnotator,
};
use rabbitmq_lqt_lib::entry_metadata::subsystems::Subsystem;

#[test]
fn test_federation_queue() {
    let entry = create_test_entry("Started federation queue 'my-queue'", Severity::Info);
    let annotator = FederationAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_disconnecting_from_queue() {
    let entry = create_test_entry("Disconnecting from queue 'test-queue'", Severity::Info);
    let annotator = FederationAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_federation_upstream_component() {
    let entry = create_test_entry(
        "Asked to start a dynamic shovel for component 'federation-upstream'",
        Severity::Info,
    );
    let annotator = FederationAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_federation_upstream_component_case_insensitive() {
    let entry = create_test_entry(
        "CONFIGURATION FOR COMPONENT 'FEDERATION-UPSTREAM'",
        Severity::Info,
    );
    let annotator = FederationAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_no_match_unrelated() {
    let entry = create_test_entry("Queue created successfully", Severity::Info);
    let annotator = FederationAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_annotate_sets_subsystem() {
    let mut entry = create_test_entry("federation queue started", Severity::Info);
    let annotator = FederationAnnotator;
    annotator.annotate(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Federation.to_id()));
}
