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
pub mod constants;
pub mod datetime;
pub mod entry_metadata;
pub mod errors;
pub mod file_set_metadata;
pub mod parser;
pub mod rel_db;
pub mod severity;

use std::result::Result as StdResult;

pub use constants::{doc_url_from_id, resolution_or_discussion_url_from_id};
pub use entry_metadata::label_annotators::annotate_labels;
pub use entry_metadata::labels::LogEntryLabels;
pub use entry_metadata::subsystem_annotators::annotate_subsystems;
pub use entry_metadata::subsystems::Subsystem;
pub use errors::Error;
pub use parser::{ParseResult, ParsedLogEntry, parse_log_file};
pub use rel_db::node_log_entry::Entity as NodeLogEntryEntity;
pub use rel_db::{
    NodeLogEntry, QueryContext, create_database, create_database_for_bulk_import,
    finalize_bulk_import, open_database, post_insertion_operations,
};
pub use severity::Severity;

pub type Result<T, E = Error> = StdResult<T, E>;
