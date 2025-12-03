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
fn test_rabbit_stomp() {
    let entry = create_test_entry("rabbit_stomp starting on port 61613", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::STOMP));
}

#[test]
fn test_rabbit_web_stomp() {
    let entry = create_test_entry("rabbit_web_stomp starting on port 15674", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::STOMP));
    assert!(labels.contains(LogEntryLabels::WEBSOCKETS));
}

#[test]
fn test_started_stomp() {
    let entry = create_test_entry("started STOMP TCP listener on [::]:61613", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::STOMP));
}

#[test]
fn test_stopped_stomp() {
    let entry = create_test_entry("stopped STOMP TCP listener on [::]:61613", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::STOMP));
    assert!(labels.contains(LogEntryLabels::SHUTDOWN));
}

#[test]
fn test_stomp_listener_startup_plain_without_websockets() {
    let entry = create_test_entry(
        "rabbit_stomp listener started on port 61613",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::STOMP));
    assert!(!labels.contains(LogEntryLabels::WEBSOCKETS));
}
