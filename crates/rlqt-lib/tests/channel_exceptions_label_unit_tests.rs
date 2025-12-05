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
use rlqt_lib::entry_metadata::label_annotators::{
    ChannelErrorsAnnotator, ExceptionsAnnotator, LabelAnnotator, annotate_labels,
};
use rlqt_lib::entry_metadata::labels::LogEntryLabels;
use test_helpers::create_test_entry;

#[test]
fn test_channel_error() {
    let entry = create_test_entry("Channel error on connection <0.123.0>", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::EXCEPTIONS));
}

#[test]
fn test_channel_error_case_insensitive() {
    let entry = create_test_entry("channel error on connection <0.456.0>", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::EXCEPTIONS));
}

#[test]
fn test_channel_error_in_longer_message() {
    let entry = create_test_entry(
        "Error occurred: Channel error on connection due to timeout",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::EXCEPTIONS));
}

#[test]
fn test_no_match_different_error() {
    let entry = create_test_entry("Connection error occurred", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(!labels.contains(LogEntryLabels::EXCEPTIONS));
}

#[test]
fn test_exceptions_annotator_matches_channel_error() {
    let entry = create_test_entry("Channel error on connection <0.456.0>", Severity::Info);
    let annotator = ExceptionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_exceptions_annotator_matches_channel_error_case_insensitive() {
    let entry = create_test_entry("CHANNEL ERROR ON CONNECTION <0.456.0>", Severity::Info);
    let annotator = ExceptionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_exceptions_annotator_matches_amqp_connection_error() {
    let entry = create_test_entry("Error on AMQP connection <0.456.0>", Severity::Info);
    let annotator = ExceptionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_exceptions_annotator_no_match() {
    let entry = create_test_entry("Unrelated message", Severity::Info);
    let annotator = ExceptionsAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_exceptions_annotator_annotates() {
    let annotator = ExceptionsAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);
    assert!(labels.contains(LogEntryLabels::EXCEPTIONS));
}

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
    assert!(labels.contains(LogEntryLabels::EXCEPTIONS));
}

#[test]
fn test_boot_failed() {
    let entry = create_test_entry(
        "BOOT FAILED\n===========\nError during startup: eaddrinuse",
        Severity::Error,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::EXCEPTIONS));
}

#[test]
fn test_ranch_listener_failed() {
    let entry = create_test_entry(
        "Failed to start Ranch listener rabbit_web_mqtt_listener_tls in ranch_ssl:listen([{port,15676}]) for reason eaddrinuse (address already in use)",
        Severity::Error,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::EXCEPTIONS));
    assert!(labels.contains(LogEntryLabels::NETWORKING));
}
