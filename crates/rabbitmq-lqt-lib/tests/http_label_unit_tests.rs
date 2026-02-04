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
use rabbitmq_lqt_lib::entry_metadata::label_annotators::{
    HttpAccessDeniedAnnotator, HttpAnnotator, LabelAnnotator,
};
use rabbitmq_lqt_lib::entry_metadata::labels::LogEntryLabels;
use test_helpers::create_test_entry;

#[test]
fn test_http_annotator_matches_http_api_payload_size() {
    let entry = create_test_entry(
        "HTTP API: request exceeded maximum allowed payload size (limit: 8000000 bytes, payload size: 10000000 bytes)",
        Severity::Warning,
    );
    let annotator = HttpAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_http_annotator_matches_http_api_slow_query() {
    let entry = create_test_entry(
        "HTTP API: ~s slow query mode requested - extended sort on ~0p",
        Severity::Debug,
    );
    let annotator = HttpAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_http_annotator_no_match() {
    let entry = create_test_entry("ra: starting system quorum_queues", Severity::Info);
    let annotator = HttpAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_http_annotator_sets_label() {
    let annotator = HttpAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);
    assert!(labels.contains(LogEntryLabels::HTTP));
}

#[test]
fn test_http_access_denied_annotator_matches() {
    let entry = create_test_entry(
        "HTTP access denied: user 'guest' - Not management user",
        Severity::Warning,
    );
    let annotator = HttpAccessDeniedAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_http_access_denied_annotator_sets_both_labels() {
    let annotator = HttpAccessDeniedAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);
    assert!(labels.contains(LogEntryLabels::HTTP));
    assert!(labels.contains(LogEntryLabels::ACCESS_CONTROL));
}

#[test]
fn test_http_annotator_matches_web_dispatch_registry_terminating() {
    let entry = create_test_entry(
        "** Generic server rabbit_web_dispatch_registry terminating",
        Severity::Error,
    );
    let annotator = HttpAnnotator;
    assert!(annotator.does_match(&entry));
}
