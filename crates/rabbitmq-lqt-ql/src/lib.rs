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

//! Query language parser for RabbitMQ Log Querying Tools (RLQT).
//!
//! This crate implements RQL (RLQT Query Language), a domain-specific query language
//! for filtering and analyzing RabbitMQ log entries.
//!
//! # Query String Parsing
//!
//! Use [`parse_and_compile`] to parse a query string:
//!
//! ```
//! let compiled = rabbitmq_lqt_ql::parse_and_compile(r#"{severity="error"} | limit 100"#).unwrap();
//! ```
//!
//! # Builder API
//!
//! Use [`builder::QueryBuilder`] for programmatic query construction:
//!
//! ```
//! use rabbitmq_lqt_ql::builder::QueryBuilder;
//! use rabbitmq_lqt_ql::ast::{Field, SortDirection};
//!
//! let compiled = QueryBuilder::new()
//!     .last_hours(24)
//!     .severity("error")
//!     .message_contains("connection")
//!     .sort(Field::Timestamp, SortDirection::Desc)
//!     .limit(100)
//!     .compile()
//!     .unwrap();
//! ```

pub mod builder;
pub mod compiler;
pub mod errors;

use std::result::Result as StdResult;

// Re-export everything from rabbitmq-lqt-ql-core
pub use rabbitmq_lqt_ql_core::PresetName;
pub use rabbitmq_lqt_ql_core::ast;
pub use rabbitmq_lqt_ql_core::autocomplete;
pub use rabbitmq_lqt_ql_core::presets;
pub use rabbitmq_lqt_ql_core::{Diagnostic, ParseError, Span};
pub use rabbitmq_lqt_ql_core::{
    Duration, DurationUnit, Field, FilterExpr, LabelMatcher, MatchOp, PipelineStage, Query,
    Selector, SortDirection, SortSpec, Value,
};
pub use rabbitmq_lqt_ql_core::{parse, parse_filter_only};

pub use builder::{FilterBuilder, QueryBuilder};
pub use compiler::{CompiledQuery, compile};
pub use errors::CompileError;

pub type Result<T, E = errors::Error> = StdResult<T, E>;

/// Parses a query string and compiles it to a QueryContext.
pub fn parse_and_compile(input: &str) -> Result<CompiledQuery> {
    let query = parse(input)?;
    let compiled = compile(&query)?;
    Ok(compiled)
}

/// Parses a query string and returns a QueryContext for use with rabbitmq-lqt-lib.
pub fn to_query_context(input: &str) -> Result<rabbitmq_lqt_lib::QueryContext> {
    let compiled = parse_and_compile(input)?;
    let ctx = if compiled.sql_where_fragments.is_empty() {
        compiled.context
    } else {
        compiled
            .context
            .raw_where_clauses(compiled.sql_where_fragments)
    };
    Ok(ctx)
}
