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
pub mod annotator;
pub mod doc_url_annotators;
pub mod label_annotators;
pub mod labels;
pub mod resolution_url_annotators;
pub mod shared;
pub mod subsystem_annotators;
pub mod subsystems;

pub use annotator::Annotator;
pub use doc_url_annotators::annotate_doc_urls;
pub use label_annotators::{
    AccessControlAnnotator, AutoDeleteAnnotator, ChannelExceptionsAnnotator, ConnectionsAnnotator,
    DeleteAnnotator, ElectionsAnnotator, ErlProcessCrashAnnotator, ExclusiveAnnotator,
    FederationAnnotator, LabelAnnotator, ProcessStopsAnnotator, QueueFederationAnnotator,
    QueuesAnnotator, RaftBasedAnnotator, ShovelsAnnotator, UndefinedFnAnnotator,
    VirtualHostsAnnotator, annotate_labels,
};
pub use labels::{LABEL_NAMES, LogEntryLabels};
pub use resolution_url_annotators::annotate_resolution_or_discussion_urls;
pub use subsystem_annotators::{SubsystemAnnotator, annotate_subsystems};
pub use subsystems::Subsystem;

use crate::parser::ParsedLogEntry;

/// Annotate a log entry with all metadata in a single pass.
/// This function consolidates subsystem, label, doc URL, and resolution URL annotation
/// for better cache locality and performance.
#[inline]
pub fn annotate_entry(entry: &mut ParsedLogEntry) {
    annotate_subsystems(entry);
    entry.labels = annotate_labels(entry);
    if entry.labels.is_empty() {
        entry.labels.insert(LogEntryLabels::UNLABELLED);
    }
    annotate_doc_urls(entry);
    annotate_resolution_or_discussion_urls(entry);
}
