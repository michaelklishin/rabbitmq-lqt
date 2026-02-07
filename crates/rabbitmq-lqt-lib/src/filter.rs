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
use crate::entry_metadata::labels::LogEntryLabels;
use crate::entry_metadata::subsystems::Subsystem;
use crate::parser::ParsedLogEntry;
use crate::severity::Severity;
use chrono::{DateTime, Utc};
use std::str::FromStr;

#[derive(Debug, Clone, Default)]
pub struct EntryFilter {
    severity: Option<Severity>,
    subsystem_id: Option<i16>,
    label_mask: u64,
    matching_all_labels: bool,
    erlang_pid: Option<String>,
    since_time: Option<DateTime<Utc>>,
    to_time: Option<DateTime<Utc>>,
    has_resolution_or_discussion_url: bool,
    has_doc_url: bool,
}

impl EntryFilter {
    #[must_use]
    pub fn severity(mut self, sev: &str) -> Self {
        if let Ok(s) = Severity::from_str(sev) {
            self.severity = Some(s);
        }
        self
    }

    #[must_use]
    pub fn subsystem(mut self, sub: &str) -> Self {
        if let Ok(s) = Subsystem::from_str(sub) {
            self.subsystem_id = Some(s.to_id());
        }
        self
    }

    #[must_use]
    pub fn add_label(mut self, label: &str) -> Self {
        if let Some(bit) = LogEntryLabels::bit_for_label(label) {
            self.label_mask |= bit;
        }
        self
    }

    #[must_use]
    pub fn matching_all_labels(mut self, match_all: bool) -> Self {
        self.matching_all_labels = match_all;
        self
    }

    #[must_use]
    pub fn erlang_pid(mut self, pid: &str) -> Self {
        self.erlang_pid = Some(pid.to_string());
        self
    }

    #[must_use]
    pub fn since(mut self, time: DateTime<Utc>) -> Self {
        self.since_time = Some(time);
        self
    }

    #[must_use]
    pub fn to(mut self, time: DateTime<Utc>) -> Self {
        self.to_time = Some(time);
        self
    }

    #[must_use]
    pub fn has_resolution_or_discussion_url(mut self, has: bool) -> Self {
        self.has_resolution_or_discussion_url = has;
        self
    }

    #[must_use]
    pub fn has_doc_url(mut self, has: bool) -> Self {
        self.has_doc_url = has;
        self
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.severity.is_none()
            && self.subsystem_id.is_none()
            && self.label_mask == 0
            && self.erlang_pid.is_none()
            && self.since_time.is_none()
            && self.to_time.is_none()
            && !self.has_resolution_or_discussion_url
            && !self.has_doc_url
    }

    #[inline]
    pub fn matches(&self, entry: &ParsedLogEntry) -> bool {
        if let Some(ref sev) = self.severity
            && entry.severity < *sev
        {
            return false;
        }

        if let Some(sub_id) = self.subsystem_id
            && entry.subsystem_id != Some(sub_id)
        {
            return false;
        }

        if self.label_mask != 0 {
            let entry_bits = entry.labels.bits();
            if self.matching_all_labels {
                if entry_bits & self.label_mask != self.label_mask {
                    return false;
                }
            } else if entry_bits & self.label_mask == 0 {
                return false;
            }
        }

        if let Some(ref pid) = self.erlang_pid
            && entry.process_id != *pid
        {
            return false;
        }

        if let Some(since) = self.since_time
            && entry.timestamp < since
        {
            return false;
        }

        if let Some(to) = self.to_time
            && entry.timestamp > to
        {
            return false;
        }

        if self.has_resolution_or_discussion_url && entry.resolution_or_discussion_url_id.is_none()
        {
            return false;
        }

        if self.has_doc_url && entry.doc_url_id.is_none() {
            return false;
        }

        true
    }

    pub fn filter<'a>(&self, entries: &'a [ParsedLogEntry]) -> Vec<&'a ParsedLogEntry> {
        entries.iter().filter(|e| self.matches(e)).collect()
    }
}
