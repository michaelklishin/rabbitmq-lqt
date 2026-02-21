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
fn test_memory_high_watermark_mib() {
    let entry = create_test_entry(
        "Memory high watermark set to 39321 MiB (41231686041 bytes) of 65536 MiB (68719476736 bytes) total",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::LIMITS));
}

#[test]
fn test_memory_high_watermark_bytes() {
    let entry = create_test_entry(
        "Memory high watermark set to 4768 MiB (5000000000 bytes) of 65536 MiB (68719476736 bytes) total",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::LIMITS));
}

#[test]
fn test_disk_free_limit() {
    let entry = create_test_entry("Disk free limit set to 50MB", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::LIMITS));
}

#[test]
fn test_enabling_free_disk_space_monitoring() {
    let entry = create_test_entry(
        "Enabling free disk space monitoring (disk free space: 35302490112, total memory: 68719476736)",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::LIMITS));
}

#[test]
fn test_no_match_unrelated() {
    let entry = create_test_entry("Some other log message", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(!labels.contains(LogEntryLabels::LIMITS));
}
