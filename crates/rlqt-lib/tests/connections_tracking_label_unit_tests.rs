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
fn test_setting_up_connection_tracking_table() {
    let entry = create_test_entry(
        "Setting up a table for connection tracking on this node: tracked_connection",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::CONNECTIONS));
    assert!(!labels.contains(LogEntryLabels::CHANNELS));
    assert!(!labels.contains(LogEntryLabels::ACCESS_CONTROL));
}

#[test]
fn test_setting_up_per_vhost_connection_table() {
    let entry = create_test_entry(
        "Setting up a table for per-vhost connection counting on this node: tracked_connection_per_vhost",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::CONNECTIONS));
    assert!(!labels.contains(LogEntryLabels::CHANNELS));
    assert!(labels.contains(LogEntryLabels::VIRTUAL_HOSTS));
}

#[test]
fn test_setting_up_per_user_connection_table() {
    let entry = create_test_entry(
        "Setting up a table for per-user connection counting on this node: tracked_connection_per_user",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::CONNECTIONS));
    assert!(!labels.contains(LogEntryLabels::CHANNELS));
    assert!(labels.contains(LogEntryLabels::ACCESS_CONTROL));
}
