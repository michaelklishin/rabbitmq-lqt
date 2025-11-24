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
fn test_shovel_connected() {
    let entry = create_test_entry(
        "Shovel 'my-shovel' connected to destination",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::SHOVELS));
}

#[test]
fn test_shovel_in_vhost() {
    let entry = create_test_entry("Shovel 'test' in virtual host '/' started", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::SHOVELS));
}

#[test]
fn test_shovel_topology() {
    let entry = create_test_entry(
        "Shovel 'backup' has finished setting up its topology",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::SHOVELS));
}

#[test]
fn test_rabbit_shovel_supervisor() {
    let entry = create_test_entry(
        "Starting rabbit_shovel_dyn_worker_sup_sup process",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::SHOVELS));
}

#[test]
fn test_asked_to_start_dynamic_shovel() {
    let entry = create_test_entry(
        "Asked to start a dynamic Shovel named 'my-shovel'",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::SHOVELS));
}

#[test]
fn test_rabbit_shovel_worker() {
    let entry = create_test_entry(
        "Starting rabbit_shovel_worker for shovel 'test'",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::SHOVELS));
}

#[test]
fn test_for_component_shovel() {
    let entry = create_test_entry(
        "Configuration for component 'shovel' updated",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::SHOVELS));
}

#[test]
fn test_for_component_shovel_case_insensitive() {
    let entry = create_test_entry("PARAMETERS FOR COMPONENT 'SHOVEL' applied", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::SHOVELS));
}

#[test]
fn test_shovel_received_with_single_quotes() {
    let entry = create_test_entry(
        "Shovel 'test_basic_dest_shovel' received a 'queue.declare-ok' from the server",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::SHOVELS));
}

#[test]
fn test_shovel_received_with_printed_binary_name() {
    let entry = create_test_entry(
        "Shovel <<\"test_basic_dest_shovel\">> received a 'basic.cancel' from the server",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::SHOVELS));
}

#[test]
fn test_shovel_received_case_insensitive() {
    let entry = create_test_entry(
        "SHOVEL 'MY_SHOVEL' RECEIVED A 'BASIC.ACK' FROM THE SERVER",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::SHOVELS));
}

#[test]
fn test_no_match_unrelated() {
    let entry = create_test_entry("Queue created successfully", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(!labels.contains(LogEntryLabels::SHOVELS));
}
