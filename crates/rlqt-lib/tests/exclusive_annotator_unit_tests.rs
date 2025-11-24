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
use rlqt_lib::entry_metadata::label_annotators::{ExclusiveAnnotator, LabelAnnotator};
use rlqt_lib::entry_metadata::labels::LogEntryLabels;
use test_helpers::create_test_entry;

#[test]
fn test_exclusive_annotator_matches_pattern() {
    let entry = create_test_entry(
        "because its declaring connection <0.123.0> was closed",
        Severity::Info,
    );
    let annotator = ExclusiveAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_exclusive_annotator_matches_substring() {
    let entry = create_test_entry("exclusive queue removed", Severity::Info);
    let annotator = ExclusiveAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_exclusive_annotator_no_match() {
    let entry = create_test_entry("Unrelated message", Severity::Info);
    let annotator = ExclusiveAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_exclusive_annotator_annotates() {
    let annotator = ExclusiveAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);
    assert!(labels.contains(LogEntryLabels::EXCLUSIVE));
}
