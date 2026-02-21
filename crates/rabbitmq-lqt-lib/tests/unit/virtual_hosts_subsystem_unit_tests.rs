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
use rabbitmq_lqt_lib::entry_metadata::Annotator;
use rabbitmq_lqt_lib::entry_metadata::subsystem_annotators::{
    SubsystemAnnotator, VirtualHostsAnnotator,
};
use rabbitmq_lqt_lib::entry_metadata::subsystems::Subsystem;

#[test]
fn test_started_message_store_of_type() {
    let entry = create_test_entry("Started message store of type transient", Severity::Info);
    let annotator = VirtualHostsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_started_message_store_case_insensitive() {
    let entry = create_test_entry("STARTED MESSAGE STORE OF TYPE persistent", Severity::Info);
    let annotator = VirtualHostsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_message_store_for_directory() {
    let entry = create_test_entry(
        "Message store for directory /var/lib/rabbitmq/mnesia",
        Severity::Info,
    );
    let annotator = VirtualHostsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_message_store_for_directory_case_insensitive() {
    let entry = create_test_entry("MESSAGE STORE FOR DIRECTORY /tmp/rabbit", Severity::Info);
    let annotator = VirtualHostsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_starting_message_stores_for_vhost() {
    let entry = create_test_entry("Starting message stores for vhost '/'", Severity::Info);
    let annotator = VirtualHostsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_starting_message_stores_for_vhost_named() {
    let entry = create_test_entry(
        "Starting message stores for vhost 'production'",
        Severity::Info,
    );
    let annotator = VirtualHostsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_default_queue_type_for_vhost() {
    let entry = create_test_entry("Default queue type for vhost '/' is quorum", Severity::Info);
    let annotator = VirtualHostsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_default_queue_type_case_insensitive() {
    let entry = create_test_entry(
        "DEFAULT QUEUE TYPE FOR VHOST '/' is classic",
        Severity::Info,
    );
    let annotator = VirtualHostsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_adding_vhost() {
    let entry = create_test_entry("Adding vhost '/'", Severity::Info);
    let annotator = VirtualHostsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_adding_vhost_named() {
    let entry = create_test_entry("Adding vhost 'production'", Severity::Info);
    let annotator = VirtualHostsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_adding_vhost_case_insensitive() {
    let entry = create_test_entry("ADDING VHOST 'test'", Severity::Info);
    let annotator = VirtualHostsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_setting_segment_entry_count() {
    let entry = create_test_entry(
        "Setting segment_entry_count for vhost '/' to 2048",
        Severity::Info,
    );
    let annotator = VirtualHostsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_setting_segment_entry_count_different_value() {
    let entry = create_test_entry(
        "Setting segment_entry_count for vhost 'production' to 4096",
        Severity::Info,
    );
    let annotator = VirtualHostsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_setting_segment_entry_count_case_insensitive() {
    let entry = create_test_entry(
        "SETTING SEGMENT_ENTRY_COUNT FOR VHOST '/' to 1024",
        Severity::Info,
    );
    let annotator = VirtualHostsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_deleting_vhost() {
    let entry = create_test_entry("Deleting vhost '/'", Severity::Info);
    let annotator = VirtualHostsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_deleting_vhost_named() {
    let entry = create_test_entry("Deleting vhost 'staging'", Severity::Info);
    let annotator = VirtualHostsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_deleting_vhost_case_insensitive() {
    let entry = create_test_entry("DELETING VHOST 'test'", Severity::Info);
    let annotator = VirtualHostsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_recovering_queues_pattern() {
    let entry = create_test_entry("Recovering 10 queues of type quorum", Severity::Info);
    let annotator = VirtualHostsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_recovering_queues_classic() {
    let entry = create_test_entry("Recovering 5 queues of type classic", Severity::Info);
    let annotator = VirtualHostsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_recovering_queues_stream() {
    let entry = create_test_entry("Recovering 100 queues of type stream", Severity::Info);
    let annotator = VirtualHostsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_recovering_zero_queues() {
    let entry = create_test_entry("Recovering 0 queues of type quorum", Severity::Info);
    let annotator = VirtualHostsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_recovering_data_for_virtual_host() {
    let entry = create_test_entry("Recovering data for virtual host '/'", Severity::Info);
    let annotator = VirtualHostsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_recovering_data_for_virtual_host_named() {
    let entry = create_test_entry(
        "Recovering data for virtual host 'production'",
        Severity::Info,
    );
    let annotator = VirtualHostsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_recovering_data_for_virtual_host_case_insensitive() {
    let entry = create_test_entry("RECOVERING DATA FOR VIRTUAL HOST 'test'", Severity::Info);
    let annotator = VirtualHostsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_no_match_unrelated_message() {
    let entry = create_test_entry("Connection established", Severity::Info);
    let annotator = VirtualHostsAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_no_match_partial_vhost() {
    let entry = create_test_entry("Virtual host configuration", Severity::Info);
    let annotator = VirtualHostsAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_annotate_sets_subsystem() {
    let mut entry = create_test_entry("Adding vhost '/'", Severity::Info);
    let annotator = VirtualHostsAnnotator;
    annotator.annotate(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::VirtualHosts.to_id()));
}

#[test]
fn test_making_sure_data_directory_for_vhost() {
    let entry = create_test_entry(
        "Making sure data directory '/Users/antares/Tools/rabbitmq/generic/var/lib/rabbitmq/mnesia/rabbit@sunnyside/msg_stores/vhosts/A4X2V27YYX9QUVIGVX5MF58JO' for vhost 'rabbitmqadmin.test-vhosts-delete-multiple-protects-protected-3' exists",
        Severity::Debug,
    );
    let annotator = VirtualHostsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_making_sure_data_directory_for_vhost_linux_path() {
    let entry = create_test_entry(
        "Making sure data directory '/home/antares/Tools/rabbitmq/generic/var/lib/rabbitmq/mnesia/rabbit@sunnyside/msg_stores/vhosts/BG2SCZNHO1T30EHH913BAFTNA' for vhost 'rabbitmqadmin.test-vhosts-delete-multiple-protects-protected-1' exists",
        Severity::Debug,
    );
    let annotator = VirtualHostsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_making_sure_data_directory_for_vhost_short_path() {
    let entry = create_test_entry(
        "Making sure data directory '/data/rabbitmq/var/lib/rabbitmq/mnesia/rabbit@sunnyside/msg_stores/vhosts/DUO4TWE7I18HMUL4J8JOK8IZ2' for vhost 'rabbitmqadmin.exchange_vhost_3' exists",
        Severity::Debug,
    );
    let annotator = VirtualHostsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_deleting_message_store_directory_for_vhost() {
    let entry = create_test_entry(
        "Deleting message store directory for vhost 'rabbitmqadmin.exchange_vhost_4' at '/Users/antares/Tools/rabbitmq/generic/var/lib/rabbitmq/mnesia/rabbit@sunnyside/msg_stores/vhosts/BHSXL8V66T6NH5C86SDNB8NPG'",
        Severity::Debug,
    );
    let annotator = VirtualHostsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_deleting_message_store_directory_for_vhost_with_slash() {
    let entry = create_test_entry(
        "Deleting message store directory for vhost 'rust/rabbitmqadmin' at '/Users/antares/Tools/rabbitmq/generic/var/lib/rabbitmq/mnesia/rabbit@sunnyside/msg_stores/vhosts/D3JIXVJ6KFALWG4SVPIFGZGAH'",
        Severity::Debug,
    );
    let annotator = VirtualHostsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_deleting_message_store_directory_for_vhost_linux_path() {
    let entry = create_test_entry(
        "Deleting message store directory for vhost 'rabbitmqadmin.test_operator_policies_bulk_policy_keys_manipulation.1' at '/home/antares/Tools/rabbitmq/generic/var/lib/rabbitmq/mnesia/rabbit@sunnyside/msg_stores/vhosts/19KA1Z5XPTSAXB5J9HD2XDPCQ'",
        Severity::Debug,
    );
    let annotator = VirtualHostsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_deleting_message_store_directory_for_vhost_standard_path() {
    let entry = create_test_entry(
        "Deleting message store directory for vhost 'rabbitmqadmin.federation.exchange.test7' at '/var/lib/rabbitmq/mnesia/rabbit@sunnyside/msg_stores/vhosts/1LOONF6SMWSBO2BRHGDQX72M8'",
        Severity::Debug,
    );
    let annotator = VirtualHostsAnnotator;
    assert!(annotator.does_match(&entry));
}
