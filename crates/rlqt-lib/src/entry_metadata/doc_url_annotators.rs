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

//! Documentation URL annotators for RabbitMQ log entries.
//!
//! This module contains annotators that associate log entries with relevant
//! RabbitMQ documentation URLs. These annotators mutate entries by setting
//! the `doc_url_id` field, which maps to a URL defined in the constants module.

use crate::constants::{ALARMS_DOC_URL_ID, METADATA_STORE_DOC_URL_ID};
use crate::entry_metadata::annotator::Annotator;
use crate::entry_metadata::subsystems::Subsystem;
use crate::parser::ParsedLogEntry;

pub trait DocUrlAnnotator: Annotator {
    fn annotate(&self, entry: &mut ParsedLogEntry);
}

#[derive(Debug)]
pub struct MetadataStoreDocAnnotator;

impl Annotator for MetadataStoreDocAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.subsystem_id == Some(Subsystem::MetadataStore.to_id())
    }
}

impl DocUrlAnnotator for MetadataStoreDocAnnotator {
    fn annotate(&self, entry: &mut ParsedLogEntry) {
        entry.doc_url_id = Some(METADATA_STORE_DOC_URL_ID);
    }
}

#[derive(Debug)]
pub struct FreeDiskSpaceAlarmDocAnnotator;

impl Annotator for FreeDiskSpaceAlarmDocAnnotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message.starts_with("Disk free limit set to")
    }
}

impl DocUrlAnnotator for FreeDiskSpaceAlarmDocAnnotator {
    fn annotate(&self, entry: &mut ParsedLogEntry) {
        entry.doc_url_id = Some(ALARMS_DOC_URL_ID);
    }
}

#[inline]
pub fn annotate_doc_urls(entry: &mut ParsedLogEntry) -> &mut ParsedLogEntry {
    if entry.doc_url_id.is_some() {
        return entry;
    }

    const ANNOTATORS: &[&dyn DocUrlAnnotator] =
        &[&MetadataStoreDocAnnotator, &FreeDiskSpaceAlarmDocAnnotator];

    for annotator in ANNOTATORS {
        if annotator.does_match(entry) {
            annotator.annotate(entry);
            return entry;
        }
    }

    entry
}
