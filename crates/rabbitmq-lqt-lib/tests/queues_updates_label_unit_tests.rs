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
fn test_finished_rebuilding_index() {
    let entry = create_test_entry("Finished rebuilding index for queue", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::QUEUES));
}

#[test]
fn test_finished_rebuilding_index_uppercase() {
    let entry = create_test_entry("FINISHED REBUILDING INDEX", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::QUEUES));
}

#[test]
fn test_rebuilding_message_location_index() {
    let entry = create_test_entry(
        "Rebuilding message location index from disk",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::QUEUES));
}

#[test]
fn test_rebuilding_message_location_index_uppercase() {
    let entry = create_test_entry(
        "REBUILDING MESSAGE LOCATION INDEX FROM storage",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::QUEUES));
}

#[test]
fn test_rebuilding_message_location_index_case_insensitive() {
    let entry = create_test_entry(
        "Rebuilding Message Location Index From /var/lib/rabbitmq",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::QUEUES));
}

#[test]
fn test_priority_queues_enabled() {
    let entry = create_test_entry(
        "Priority queues enabled, real BQ is rabbit_variable_queue",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::QUEUES));
}
