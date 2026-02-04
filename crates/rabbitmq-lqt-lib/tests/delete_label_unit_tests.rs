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
use rabbitmq_lqt_lib::entry_metadata::label_annotators::annotate_labels;
use rabbitmq_lqt_lib::entry_metadata::labels::LogEntryLabels;
use test_helpers::create_test_entry;

#[test]
fn test_terminating_with_reason_delete() {
    let entry = create_test_entry("Process terminating with reason 'delete'", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::DELETE));
}

#[test]
fn test_being_deleted() {
    let entry = create_test_entry("Queue removed because it's being deleted", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::DELETE));
}

#[test]
fn test_case_insensitive() {
    let entry = create_test_entry(
        "Terminating with reason 'delete' after request",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::DELETE));
}

#[test]
fn test_deleting_vhost_default() {
    let entry = create_test_entry("Deleting vhost '/'", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::DELETE));
    assert!(labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
}

#[test]
fn test_deleting_vhost_named() {
    let entry = create_test_entry("Deleting vhost 'test_env'", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::DELETE));
    assert!(labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
}

#[test]
fn test_deleting_vhost_case_insensitive() {
    let entry = create_test_entry("deleting vhost 'production'", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::DELETE));
    assert!(labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
}

#[test]
fn test_no_match_different_reason() {
    let entry = create_test_entry("terminating with reason 'shutdown'", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(!labels.contains(LogEntryLabels::DELETE));
}
