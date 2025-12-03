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
fn test_default_queue_type_for_vhost_default() {
    let entry = create_test_entry("Default queue type for vhost '/' is quorum", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
}

#[test]
fn test_default_queue_type_for_vhost_named() {
    let entry = create_test_entry(
        "Default queue type for vhost 'production' is classic",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
}

#[test]
fn test_default_queue_type_case_insensitive() {
    let entry = create_test_entry(
        "DEFAULT QUEUE TYPE FOR VHOST 'staging' is stream",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
}

#[test]
fn test_default_queue_type_mixed_case() {
    let entry = create_test_entry(
        "Default Queue Type for VHost 'test' is quorum",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
}

#[test]
fn test_parsed_virtual_host_tags() {
    let entry = create_test_entry("Parsed virtual host tags <<>> to []", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
}

#[test]
fn test_default_queue_type_of_virtual_host() {
    let entry = create_test_entry(
        "Default queue type of virtual host 'staging' is <<\"classic\">>",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
}

#[test]
fn test_done_with_vhost_reconciliation() {
    let entry = create_test_entry(
        "Done with virtual host processes reconciliation (run 2)",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
}

#[test]
fn test_will_reconcile_vhost() {
    let entry = create_test_entry(
        "Will reconcile virtual host processes on all cluster members...",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
}

#[test]
fn test_will_reschedule_vhost_reconciliation() {
    let entry = create_test_entry(
        "Will reschedule virtual host process reconciliation after 30 seconds",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
}

#[test]
fn test_will_make_sure_processes_of_vhosts() {
    let entry = create_test_entry(
        "Will make sure that processes of 10 virtual hosts are running on all reachable cluster nodes",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
}

#[test]
fn test_virtual_host_is_stopping() {
    let entry = create_test_entry(
        "Virtual host 'rabbitmqadmin.federation.modifications.test1' is stopping",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
}

#[test]
fn test_virtual_host_default_is_stopping() {
    let entry = create_test_entry("Virtual host '/' is stopping", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
}
