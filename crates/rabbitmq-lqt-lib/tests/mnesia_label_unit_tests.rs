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
fn test_mnesia_waiting_for_tables() {
    let entry = create_test_entry(
        "Waiting for Mnesia tables for 30000 ms, 9 retries left",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::MNESIA));
}

#[test]
fn test_no_match_without_mnesia_keyword() {
    let entry = create_test_entry("Successfully synced tables from a peer", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(!labels.contains(LogEntryLabels::MNESIA));
}

#[test]
fn test_no_match_mnesia_only_in_path() {
    let entry = create_test_entry(
        "Making sure data directory '/Users/antares/Tools/rabbitmq/generic/var/lib/rabbitmq/mnesia/rabbit@sunnyside/msg_stores/vhosts/9FIC234PN23PFSWT1G3TOFRJN' for vhost 'rabbitmqadmin.test-vhosts-delete-multiple-protects-protected-2' exists",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(!labels.contains(LogEntryLabels::MNESIA));
}

#[test]
fn test_mnesia_to_khepri_migration() {
    let entry = create_test_entry(
        "Mnesia->Khepri data copy: Table `rabbit_user` does not exist, skipping its migration",
        Severity::Debug,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::MNESIA));
    assert!(labels.contains(LogEntryLabels::KHEPRI));
}

#[test]
fn test_application_mnesia_exited() {
    let entry = create_test_entry(
        "Application mnesia exited with reason: stopped",
        Severity::Notice,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::MNESIA));
}

#[test]
fn test_stopping_application_mnesia() {
    let entry = create_test_entry("Stopping application 'mnesia'", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::MNESIA));
}
