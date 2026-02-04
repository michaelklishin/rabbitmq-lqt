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

//! Generic annotator infrastructure.
//!
//! This module provides a unified trait for all annotator types.
//! All annotators follow a common pattern: check if an entry matches, then annotate it.

use crate::parser::ParsedLogEntry;

/// Generic trait for all annotators.
///
/// Annotators examine log entries and either:
/// * Return metadata to be applied (label annotators)
/// * Mutate the entry directly (subsystem, URL annotators)
pub trait Annotator {
    /// Check if this annotator matches the given log entry
    fn does_match(&self, entry: &ParsedLogEntry) -> bool;
}
