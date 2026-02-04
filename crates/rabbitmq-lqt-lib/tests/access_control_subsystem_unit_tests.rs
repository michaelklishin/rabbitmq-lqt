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

use rabbitmq_lqt_lib::Severity;
use rabbitmq_lqt_lib::entry_metadata::Annotator;
use rabbitmq_lqt_lib::entry_metadata::subsystem_annotators::{
    AccessControlAnnotator, SubsystemAnnotator,
};
use rabbitmq_lqt_lib::entry_metadata::subsystems::Subsystem;
use test_helpers::create_test_entry;

#[test]
fn test_successfully_set_permissions() {
    let entry = create_test_entry(
        "Successfully set permissions for user 'admin'",
        Severity::Info,
    );
    let annotator = AccessControlAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_successfully_set_permissions_case_insensitive() {
    let entry = create_test_entry(
        "successfully set permissions for user 'guest'",
        Severity::Info,
    );
    let annotator = AccessControlAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_authenticated_successfully_by() {
    let entry = create_test_entry("User authenticated successfully by backend", Severity::Info);
    let annotator = AccessControlAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_annotate_sets_subsystem() {
    let mut entry = create_test_entry(
        "Successfully set permissions for user 'test'",
        Severity::Info,
    );
    let annotator = AccessControlAnnotator;
    annotator.annotate(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::AccessControl.to_id()));
}
