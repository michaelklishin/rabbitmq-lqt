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
use rlqt_lib::entry_metadata::subsystem_annotators::{ClassicQueuesAnnotator, SubsystemAnnotator};
use rlqt_lib::entry_metadata::subsystems::Subsystem;
use test_helpers::create_test_entry;

#[test]
fn test_message_refcount() {
    let entry = create_test_entry("Message refcount is 42", Severity::Info);
    let annotator = ClassicQueuesAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_message_refcount_case_insensitive() {
    let entry = create_test_entry("MESSAGE REFCOUNT updated", Severity::Info);
    let annotator = ClassicQueuesAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_finished_rebuilding_index() {
    let entry = create_test_entry("Finished rebuilding index for queue", Severity::Info);
    let annotator = ClassicQueuesAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_finished_rebuilding_index_case_insensitive() {
    let entry = create_test_entry("FINISHED REBUILDING INDEX successfully", Severity::Info);
    let annotator = ClassicQueuesAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_rebuilding_indices_from_scratch() {
    let entry = create_test_entry("Rebuilding indices from scratch", Severity::Info);
    let annotator = ClassicQueuesAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_rebuilding_indices_from_scratch_case_insensitive() {
    let entry = create_test_entry(
        "REBUILDING INDICES FROM SCRATCH for recovery",
        Severity::Info,
    );
    let annotator = ClassicQueuesAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_rebuilding_message_location_index() {
    let entry = create_test_entry("Rebuilding message location index", Severity::Info);
    let annotator = ClassicQueuesAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_rebuilding_message_location_index_case_insensitive() {
    let entry = create_test_entry("REBUILDING MESSAGE LOCATION INDEX started", Severity::Info);
    let annotator = ClassicQueuesAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_no_match_message_store_for_directory() {
    let entry = create_test_entry(
        "Message store for directory /var/lib/rabbitmq",
        Severity::Info,
    );
    let annotator = ClassicQueuesAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_no_match_unrelated_message() {
    let entry = create_test_entry("Connection established", Severity::Info);
    let annotator = ClassicQueuesAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_no_match_partial_match() {
    let entry = create_test_entry("Message processing complete", Severity::Info);
    let annotator = ClassicQueuesAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_annotate_sets_subsystem() {
    let mut entry = create_test_entry("Message refcount is 5", Severity::Info);
    let annotator = ClassicQueuesAnnotator;
    annotator.annotate(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::ClassicQueues.to_id()));
}
