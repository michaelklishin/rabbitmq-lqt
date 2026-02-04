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
fn test_queues_label_matches_basic_queue_name() {
    let entry = create_test_entry("queue 'new_queue_2' in vhost '/'", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::QUEUES));
}

#[test]
fn test_queues_label_matches_basic_consume_queue() {
    let entry = create_test_entry(
        "queue 'basic.consume0.6988658399930656' in vhost '/'",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::QUEUES));
}

#[test]
fn test_queues_label_matches_proptest_queue_with_dashes() {
    let entry = create_test_entry(
        "queue 'rust.tests.blocking.proptest.1--663yK-x3' in vhost '/'",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::QUEUES));
}

#[test]
fn test_queues_label_matches_proptest_queue_with_underscores() {
    let entry = create_test_entry(
        "queue 'rust.tests.proptest.f0SUbhXMhZJ-658-f__g' in vhost '/'",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::QUEUES));
}

#[test]
fn test_queues_label_case_insensitive() {
    let entry = create_test_entry("Queue 'MyQueue' In Vhost '/'", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::QUEUES));
}

#[test]
fn test_queues_label_with_multiple_spaces() {
    let entry = create_test_entry("queue  'test_queue'  in  vhost  '/'", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::QUEUES));
}

#[test]
fn test_queues_label_with_tabs() {
    let entry = create_test_entry("queue\t'test_queue'\tin\tvhost '/'", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::QUEUES));
}

#[test]
fn test_queues_label_no_match_empty_queue_name() {
    let entry = create_test_entry("queue '' in vhost '/'", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(!labels.contains(LogEntryLabels::QUEUES));
}

#[test]
fn test_queues_label_queue_with_special_chars() {
    let entry = create_test_entry(
        "queue 'my-queue_123.test@host' in vhost '/'",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::QUEUES));
}

#[test]
fn test_queues_label_no_match_missing_quotes() {
    let entry = create_test_entry("queue test_queue in vhost '/'", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(!labels.contains(LogEntryLabels::QUEUES));
}

#[test]
fn test_queues_label_no_match_missing_in_vhost() {
    let entry = create_test_entry("queue 'test_queue' something else", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(!labels.contains(LogEntryLabels::QUEUES));
}

#[test]
fn test_queues_label_no_match_incomplete_pattern() {
    let entry = create_test_entry("queue 'test_queue'", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(!labels.contains(LogEntryLabels::QUEUES));
}

#[test]
fn test_queues_label_in_longer_message() {
    let entry = create_test_entry(
        "Starting to process queue 'my_queue' in vhost '/' with 100 messages",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::QUEUES));
}

#[test]
fn test_queues_label_multiline_message() {
    let entry = create_test_entry(
        "Processing queue operations:\nqueue 'my_queue' in vhost '/'\nOperation completed",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::QUEUES));
}
