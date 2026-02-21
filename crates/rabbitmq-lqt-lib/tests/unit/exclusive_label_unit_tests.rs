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
fn test_exclusive_pattern_1() {
    let entry = create_test_entry(
        "because its declaring connection <0.5423961.0> was closed",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::EXCLUSIVE));
}

#[test]
fn test_exclusive_pattern_2() {
    let entry = create_test_entry(
        "because its declaring connection <0.5422509.0> was closed",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::EXCLUSIVE));
}

#[test]
fn test_exclusive_queue_substring() {
    let entry = create_test_entry("Deleting exclusive queue 'test_queue'", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::EXCLUSIVE));
}

#[test]
fn test_exclusive_case_insensitive() {
    let entry = create_test_entry(
        "Because its declaring connection <0.123.0> was closed",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::EXCLUSIVE));
}

#[test]
fn test_exclusive_in_longer_message() {
    let entry = create_test_entry(
        "Queue deleted because its declaring connection <0.999.1> was closed unexpectedly",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::EXCLUSIVE));
}

#[test]
fn test_no_match_missing_connection() {
    let entry = create_test_entry("because its declaring was closed", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(!labels.contains(LogEntryLabels::EXCLUSIVE));
}

#[test]
fn test_no_match_different_format() {
    let entry = create_test_entry("connection was closed", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(!labels.contains(LogEntryLabels::EXCLUSIVE));
}
