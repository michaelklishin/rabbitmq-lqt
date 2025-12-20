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
fn test_khepri_segment_writer_coordination() {
    let entry = create_test_entry(
        "segment_writer in 'coordination': completed flush of 1 writers from wal file 0000000000000001.wal in 19ms",
        Severity::Debug,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::KHEPRI));
    assert!(labels.contains(LogEntryLabels::RAFT));
}

#[test]
fn test_khepri_based_metadata_store() {
    let entry = create_test_entry("Khepri-based RabbitMQ metadata store ready", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::KHEPRI));
}

#[test]
fn test_khepri_starting() {
    let entry = create_test_entry(
        "Starting Khepri-based RabbitMQ metadata store",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::KHEPRI));
}

#[test]
fn test_khepri_mnesia_migration() {
    let entry = create_test_entry(
        "Mnesia->Khepri data copy: Table `rabbit_user` does not exist, skipping its migration",
        Severity::Debug,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::KHEPRI));
    assert!(labels.contains(LogEntryLabels::MNESIA));
}
