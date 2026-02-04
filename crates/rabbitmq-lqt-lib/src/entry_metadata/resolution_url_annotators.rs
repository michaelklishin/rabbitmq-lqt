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

//! Resolution and discussion URL annotators for RabbitMQ log entries.
//!
//! This module contains annotators that link log entries to specific GitHub issues,
//! pull requests, or discussions that provide context, workarounds, or fixes for
//! the logged conditions. These annotators mutate entries by setting the
//! `resolution_or_discussion_url_id` field.

use crate::constants::{DISCUSSION_14094, ISSUE_14181, ISSUE_14213, PULL_REQUEST_14409};
use crate::entry_metadata::annotator::Annotator;
use crate::parser::ParsedLogEntry;

pub trait ResolutionOrDiscussionUrlAnnotator: Annotator {
    fn annotate(&self, entry: &mut ParsedLogEntry);
}

#[derive(Debug)]
pub struct Issue14181Annotator;

impl Annotator for Issue14181Annotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.contains("reader_pread_parse")
            && entry.message_lowercased.contains("rabbit_msg_store")
            && entry.message_lowercased.contains("eof")
    }
}

impl ResolutionOrDiscussionUrlAnnotator for Issue14181Annotator {
    fn annotate(&self, entry: &mut ParsedLogEntry) {
        entry.resolution_or_discussion_url_id = Some(ISSUE_14181);
    }
}

#[derive(Debug)]
pub struct Issue14213Annotator;

impl Annotator for Issue14213Annotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.contains("to_091")
            && entry.message_lowercased.contains("as_is")
            && entry.message_lowercased.contains("mc_amqpl")
    }
}

impl ResolutionOrDiscussionUrlAnnotator for Issue14213Annotator {
    fn annotate(&self, entry: &mut ParsedLogEntry) {
        entry.resolution_or_discussion_url_id = Some(ISSUE_14213);
    }
}

#[derive(Debug)]
pub struct Discussion14094Annotator;

impl Annotator for Discussion14094Annotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        (entry
            .message_lowercased
            .contains("rabbit_classic_queue_index_v2,new_segment_file,3")
            || entry
                .message_lowercased
                .contains("rabbit_classic_queue_store_v2,'-flush_buffer/2-fun-0-',4"))
            && entry.message_lowercased.contains("eacces")
            && entry.message_lowercased.contains("badmatch")
            && entry.message_lowercased.contains("error")
    }
}

impl ResolutionOrDiscussionUrlAnnotator for Discussion14094Annotator {
    fn annotate(&self, entry: &mut ParsedLogEntry) {
        entry.resolution_or_discussion_url_id = Some(DISCUSSION_14094);
    }
}

#[derive(Debug)]
pub struct PullRequest14409Annotator;

impl Annotator for PullRequest14409Annotator {
    fn does_match(&self, entry: &ParsedLogEntry) -> bool {
        entry.message_lowercased.contains("mismatching_node")
            && entry.message_lowercased.contains("if_node_exists")
            && entry.message_lowercased.contains("khepri")
            && entry.message_lowercased.contains("case_clause")
    }
}

impl ResolutionOrDiscussionUrlAnnotator for PullRequest14409Annotator {
    fn annotate(&self, entry: &mut ParsedLogEntry) {
        entry.resolution_or_discussion_url_id = Some(PULL_REQUEST_14409);
    }
}

#[inline]
pub fn annotate_resolution_or_discussion_urls(entry: &mut ParsedLogEntry) -> &mut ParsedLogEntry {
    if entry.resolution_or_discussion_url_id.is_some() {
        return entry;
    }

    const ANNOTATORS: &[&dyn ResolutionOrDiscussionUrlAnnotator] = &[
        &Issue14181Annotator,
        &Issue14213Annotator,
        &Discussion14094Annotator,
        &PullRequest14409Annotator,
    ];

    for annotator in ANNOTATORS {
        if annotator.does_match(entry) {
            annotator.annotate(entry);
            return entry;
        }
    }

    entry
}
