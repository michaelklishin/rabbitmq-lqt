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
use rlqt_lib::entry_metadata::label_annotators::annotate_labels;
use rlqt_lib::entry_metadata::labels::LogEntryLabels;
use test_helpers::create_test_entry;

#[test]
fn test_started_stream_tcp_listener() {
    let entry = create_test_entry("started Stream TCP listener on [::]:5552", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::STREAMS));
}

#[test]
fn test_stopped_stream_tcp_listener() {
    let entry = create_test_entry("stopped Stream TCP listener on [::]:5552", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::STREAMS));
}

#[test]
fn test_asked_to_remove_stream_replicas() {
    let entry = create_test_entry(
        "Asked to remove all stream replicas from node flopsy@sunnyside",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::STREAMS));
}

#[test]
fn test_rabbit_stream() {
    let entry = create_test_entry("rabbit_stream: some operation completed", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::STREAMS));
}

#[test]
fn test_no_match_unrelated() {
    let entry = create_test_entry("Some other log message", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(!labels.contains(LogEntryLabels::STREAMS));
}
