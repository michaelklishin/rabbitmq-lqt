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
fn test_set_policy() {
    let entry = create_test_entry(
        "Successfully set policy 'ha-all' matching queue names in virtual host '/' using pattern '.*'",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::POLICIES));
}

#[test]
fn test_default_operator_policy() {
    let entry = create_test_entry(
        "Applied default operator policy to vhost '/': {quorum_group_size, 3}",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::POLICIES));
}

#[test]
fn test_policy_apply_warning() {
    let entry = create_test_entry(
        "rabbit_quorum_queue: policy may not have been successfully applied. Error: timeout",
        Severity::Warning,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::POLICIES));
}
