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
fn test_setting_up_channel_tracking_table() {
    let entry = create_test_entry(
        "Setting up a table for channel tracking on this node: tracked_channel",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::CHANNELS));
    assert!(!labels.contains(LogEntryLabels::CONNECTIONS));
    assert!(!labels.contains(LogEntryLabels::ACCESS_CONTROL));
}

#[test]
fn test_setting_up_per_user_channel_table() {
    let entry = create_test_entry(
        "Setting up a table for channel tracking on this node: tracked_channel_per_user",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::CHANNELS));
    assert!(!labels.contains(LogEntryLabels::CONNECTIONS));
    assert!(!labels.contains(LogEntryLabels::ACCESS_CONTROL));
}
