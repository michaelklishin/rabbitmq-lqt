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

//! Core query language parser and autocomplete for RLQT.
//!
//! This crate provides the parser, AST types, autocomplete data, and error types
//! for the RLQT Query Language. It has no dependencies on `rlqt-lib` and can be
//! compiled to WebAssembly.

pub mod ast;
pub mod autocomplete;
pub mod errors;
mod parser;
pub mod presets;

pub use ast::{
    Duration, DurationUnit, Field, FilterExpr, LabelMatcher, MatchOp, PipelineStage, Query,
    Selector, SortDirection, SortSpec, Value,
};
pub use errors::{Diagnostic, ParseError, Span};
pub use parser::{parse, parse_filter_only};
pub use presets::PresetName;
