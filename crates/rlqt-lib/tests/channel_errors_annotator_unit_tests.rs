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
use rlqt_lib::entry_metadata::label_annotators::{ChannelErrorsAnnotator, LabelAnnotator};
use rlqt_lib::entry_metadata::labels::LogEntryLabels;
use test_helpers::create_test_entry;

#[test]
fn test_channel_errors_annotator_matches_precondition_failed() {
    let entry = create_test_entry(
        "Channel error on connection <0.456.0>: PRECONDITION_FAILED - queue 'test' has no consumers",
        Severity::Error,
    );
    let annotator = ChannelErrorsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_channel_errors_annotator_matches_not_found() {
    let entry = create_test_entry(
        "Channel error on connection <0.456.0>: NOT_FOUND - queue 'missing-queue' not found",
        Severity::Error,
    );
    let annotator = ChannelErrorsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_channel_errors_annotator_matches_resource_locked() {
    let entry = create_test_entry(
        "Channel error on connection <0.456.0>: RESOURCE_LOCKED - cannot obtain exclusive access to locked queue",
        Severity::Error,
    );
    let annotator = ChannelErrorsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_channel_errors_annotator_matches_case_insensitive() {
    let entry = create_test_entry("Channel error: precondition_failed", Severity::Error);
    let annotator = ChannelErrorsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_channel_errors_annotator_no_match() {
    let entry = create_test_entry("Unrelated message about channels", Severity::Info);
    let annotator = ChannelErrorsAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_channel_errors_annotator_annotates_both_labels() {
    let annotator = ChannelErrorsAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);
    assert!(labels.contains(LogEntryLabels::CHANNELS));
    assert!(labels.contains(LogEntryLabels::CHANNEL_EXCEPTIONS));
}
