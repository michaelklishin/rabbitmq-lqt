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
use rabbitmq_lqt_lib::entry_metadata::label_annotators::{AccessControlAnnotator, LabelAnnotator};
use rabbitmq_lqt_lib::entry_metadata::labels::LogEntryLabels;

#[test]
fn test_asked_to_set_permissions() {
    let entry = create_test_entry(
        "Asked to set permissions for user 'admin' in vhost '/'",
        Severity::Info,
    );
    let annotator = AccessControlAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_asked_to_set_permissions_case_insensitive() {
    let entry = create_test_entry("ASKED TO SET PERMISSIONS FOR USER 'test'", Severity::Info);
    let annotator = AccessControlAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_successfully_set_permissions() {
    let entry = create_test_entry(
        "Successfully set permissions for user 'guest'",
        Severity::Info,
    );
    let annotator = AccessControlAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_successfully_set_permissions_case_insensitive() {
    let entry = create_test_entry(
        "SUCCESSFULLY SET PERMISSIONS FOR USER 'admin'",
        Severity::Info,
    );
    let annotator = AccessControlAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_no_match_unrelated() {
    let entry = create_test_entry("Connection established", Severity::Info);
    let annotator = AccessControlAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_annotate_applies_access_control_label() {
    let annotator = AccessControlAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);
    assert!(labels.contains(LogEntryLabels::ACCESS_CONTROL));
}
