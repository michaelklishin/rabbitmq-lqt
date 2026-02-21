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

#![allow(dead_code)]

use chrono::Utc;
use rabbitmq_lqt_lib::entry_metadata::labels::LogEntryLabels;
use rabbitmq_lqt_lib::entry_metadata::subsystems::Subsystem;
use rabbitmq_lqt_lib::{ParsedLogEntry, Severity};

pub fn create_test_entry(message: &str, severity: Severity) -> ParsedLogEntry {
    ParsedLogEntry {
        sequence_id: 0,
        explicit_id: None,
        timestamp: Utc::now(),
        severity,
        process_id: "<0.208.0>".to_string(),
        message: message.to_string(),
        message_lowercased: message.to_lowercase(),
        subsystem_id: None,
        labels: LogEntryLabels::default(),
        resolution_or_discussion_url_id: None,
        doc_url_id: None,
    }
}

pub fn create_test_entry_with_subsystem(
    message: &str,
    severity: Severity,
    subsystem: Subsystem,
) -> ParsedLogEntry {
    let mut entry = create_test_entry(message, severity);
    entry.subsystem_id = Some(subsystem.to_id());
    entry
}

pub fn create_test_entry_info(message: &str) -> ParsedLogEntry {
    create_test_entry(message, Severity::Info)
}
