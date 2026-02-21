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

use crate::test_helpers::create_test_entry;
use rabbitmq_lqt_lib::Severity;
use rabbitmq_lqt_lib::entry_metadata::label_annotators::annotate_labels;
use rabbitmq_lqt_lib::entry_metadata::labels::LogEntryLabels;

#[test]
fn test_recovering_zero_queues() {
    let entry = create_test_entry("Recovering 0 queues of type quorum", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
}

#[test]
fn test_recovering_ten_queues() {
    let entry = create_test_entry("Recovering 10 queues of type classic", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
}

#[test]
fn test_recovering_many_queues() {
    let entry = create_test_entry("Recovering 43 queues of type stream", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
}

#[test]
fn test_recovering_large_number() {
    let entry = create_test_entry("Recovering 99000 queues of type quorum", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
}

#[test]
fn test_deleting_vhost_default() {
    let entry = create_test_entry("Deleting vhost '/'", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
}

#[test]
fn test_deleting_vhost_named() {
    let entry = create_test_entry("Deleting vhost 'production'", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
}

#[test]
fn test_starting_message_stores_default() {
    let entry = create_test_entry("Starting message stores for vhost '/'", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
}

#[test]
fn test_starting_message_stores_named() {
    let entry = create_test_entry(
        "Starting message stores for vhost 'staging'",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
}

#[test]
fn test_started_message_store() {
    let entry = create_test_entry("Started message store of type transient", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
}

#[test]
fn test_message_store_for_directory() {
    let entry = create_test_entry(
        "Message store for directory /var/lib/rabbitmq",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
}

#[test]
fn test_adding_vhost_default() {
    let entry = create_test_entry("Adding vhost '/'", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
}

#[test]
fn test_adding_vhost_named() {
    let entry = create_test_entry("Adding vhost 'development'", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
}

#[test]
fn test_case_insensitive() {
    let entry = create_test_entry("DELETING VHOST 'testenv'", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
}

#[test]
fn test_recovering_data_for_virtual_host() {
    let entry = create_test_entry("Recovering data for virtual host '/'", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
}

#[test]
fn test_recovering_data_for_virtual_host_named() {
    let entry = create_test_entry(
        "Recovering data for virtual host 'production'",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
}

#[test]
fn test_recovering_data_case_insensitive() {
    let entry = create_test_entry("RECOVERING DATA FOR VIRTUAL HOST 'test'", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
}

#[test]
fn test_no_match_unrelated() {
    let entry = create_test_entry("Some other log message", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(!labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
}

#[test]
fn test_making_sure_data_directory_for_vhost() {
    let entry = create_test_entry(
        "Making sure data directory '/Users/antares/Tools/rabbitmq/generic/var/lib/rabbitmq/mnesia/rabbit@sunnyside/msg_stores/vhosts/9FIC234PN23PFSWT1G3TOFRJN' for vhost 'rabbitmqadmin.test-vhosts-delete-multiple-protects-protected-2' exists",
        Severity::Debug,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
}

#[test]
fn test_deleting_message_store_directory_for_vhost() {
    let entry = create_test_entry(
        "Deleting message store directory for vhost 'rabbitmqadmin.exchange_vhost_4' at '/Users/antares/Tools/rabbitmq/generic/var/lib/rabbitmq/mnesia/rabbit@sunnyside/msg_stores/vhosts/BHSXL8V66T6NH5C86SDNB8NPG'",
        Severity::Debug,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
}

#[test]
fn test_updating_virtual_host_record() {
    let entry = create_test_entry(
        "Updating a virtual host record {vhost,<<\"/\">>,[],",
        Severity::Debug,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
}

#[test]
fn test_updated_virtual_host_record() {
    let entry = create_test_entry(
        "Updated a virtual host record {vhost,<<\"/\">>,[],",
        Severity::Debug,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
}

#[test]
fn test_stopping_vhost_supervisor() {
    let entry = create_test_entry(
        "Stopping vhost supervisor <0.439.0> for vhost 'cmq-definition-import'",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
    assert!(labels.contains(LogEntryLabels::SHUTDOWN));
}

#[test]
fn test_virtual_host_stopping() {
    let entry = create_test_entry("virtual host 'production' is stopping", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
}
