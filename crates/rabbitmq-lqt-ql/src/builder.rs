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

//! Builder API for programmatically constructing RLQT queries.
//!
//! This module provides a fluent builder interface for creating queries
//! without parsing query strings. This is useful for:
//! - WebAssembly-based UIs
//! - Programmatic query generation
//! - Type-safe query construction
//!
//! # Example
//!
//! ```
//! use rabbitmq_lqt_ql::builder::QueryBuilder;
//! use rabbitmq_lqt_ql::ast::{Field, SortDirection};
//!
//! let query = QueryBuilder::new()
//!     .last_hours(24)
//!     .severity("error")
//!     .message_contains("connection")
//!     .sort(Field::Timestamp, SortDirection::Desc)
//!     .limit(100)
//!     .build();
//! ```

use crate::ast::{
    Duration, DurationUnit, Field, FilterExpr, LabelMatcher, MatchOp, PipelineStage, Query,
    Selector, SortDirection, SortSpec, Value,
};
use crate::compiler::{CompiledQuery, compile};
use crate::errors::CompileError;
use crate::presets::PresetName;
use chrono::{DateTime, Utc};

/// Builder for constructing RLQT queries programmatically.
#[derive(Debug, Clone, Default)]
pub struct QueryBuilder {
    time_range: Option<Duration>,
    selector_matchers: Vec<LabelMatcher>,
    filters: Vec<FilterExpr>,
    pipeline: Vec<PipelineStage>,
}

impl QueryBuilder {
    /// Creates a new empty query builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the time range to the last N seconds.
    pub fn last_seconds(mut self, seconds: i64) -> Self {
        self.time_range = Some(Duration {
            value: seconds,
            unit: DurationUnit::Seconds,
        });
        self
    }

    /// Sets the time range to the last N minutes.
    pub fn last_minutes(mut self, minutes: i64) -> Self {
        self.time_range = Some(Duration {
            value: minutes,
            unit: DurationUnit::Minutes,
        });
        self
    }

    /// Sets the time range to the last N hours.
    pub fn last_hours(mut self, hours: i64) -> Self {
        self.time_range = Some(Duration {
            value: hours,
            unit: DurationUnit::Hours,
        });
        self
    }

    /// Sets the time range to the last N days.
    pub fn last_days(mut self, days: i64) -> Self {
        self.time_range = Some(Duration {
            value: days,
            unit: DurationUnit::Days,
        });
        self
    }

    /// Sets the time range to the last N weeks.
    pub fn last_weeks(mut self, weeks: i64) -> Self {
        self.time_range = Some(Duration {
            value: weeks,
            unit: DurationUnit::Weeks,
        });
        self
    }

    /// Sets the time range using a custom Duration.
    pub fn time_range(mut self, duration: Duration) -> Self {
        self.time_range = Some(duration);
        self
    }

    /// Filters by severity level (exact match).
    pub fn severity(mut self, level: impl Into<String>) -> Self {
        self.selector_matchers.push(LabelMatcher {
            field: Field::Severity,
            op: MatchOp::Eq,
            value: Value::String(level.into()),
        });
        self
    }

    /// Filters by subsystem (exact match).
    pub fn subsystem(mut self, name: impl Into<String>) -> Self {
        self.selector_matchers.push(LabelMatcher {
            field: Field::Subsystem,
            op: MatchOp::Eq,
            value: Value::String(name.into()),
        });
        self
    }

    /// Filters by node name (exact match).
    pub fn node(mut self, name: impl Into<String>) -> Self {
        self.selector_matchers.push(LabelMatcher {
            field: Field::Node,
            op: MatchOp::Eq,
            value: Value::String(name.into()),
        });
        self
    }

    /// Filters by Erlang PID (exact match).
    pub fn erlang_pid(mut self, pid: impl Into<String>) -> Self {
        self.selector_matchers.push(LabelMatcher {
            field: Field::ErlangPid,
            op: MatchOp::Eq,
            value: Value::String(pid.into()),
        });
        self
    }

    /// Filters messages containing the given text (case-insensitive).
    pub fn message_contains(mut self, text: impl Into<String>) -> Self {
        self.filters
            .push(FilterExpr::Comparison(Box::new(LabelMatcher {
                field: Field::Message,
                op: MatchOp::IContains,
                value: Value::String(text.into()),
            })));
        self
    }

    /// Filters messages matching a regex pattern.
    pub fn message_matches(mut self, pattern: impl Into<String>) -> Self {
        self.filters
            .push(FilterExpr::Comparison(Box::new(LabelMatcher {
                field: Field::Message,
                op: MatchOp::Regex,
                value: Value::Regex(pattern.into()),
            })));
        self
    }

    /// Filters messages NOT matching a regex pattern.
    pub fn message_not_matches(mut self, pattern: impl Into<String>) -> Self {
        self.filters
            .push(FilterExpr::Comparison(Box::new(LabelMatcher {
                field: Field::Message,
                op: MatchOp::NotRegex,
                value: Value::Regex(pattern.into()),
            })));
        self
    }

    /// Filters entries having any of the specified labels.
    pub fn labels_any(mut self, labels: Vec<String>) -> Self {
        self.filters.push(FilterExpr::LabelAny(labels));
        self
    }

    /// Filters entries having all of the specified labels.
    pub fn labels_all(mut self, labels: Vec<String>) -> Self {
        self.filters.push(FilterExpr::LabelAll(labels));
        self
    }

    /// Filters entries having a specific label.
    pub fn has_label(mut self, label: impl Into<String>) -> Self {
        self.filters
            .push(FilterExpr::Comparison(Box::new(LabelMatcher {
                field: Field::Labels,
                op: MatchOp::HasLabel,
                value: Value::String(label.into()),
            })));
        self
    }

    /// Filters entries that have a documentation URL.
    pub fn has_doc_url(mut self) -> Self {
        self.filters.push(FilterExpr::HasDocUrl);
        self
    }

    /// Filters entries that have a resolution/discussion URL.
    pub fn has_resolution_url(mut self) -> Self {
        self.filters.push(FilterExpr::HasResolutionUrl);
        self
    }

    /// Filters entries that are unlabelled.
    pub fn unlabelled(mut self) -> Self {
        self.filters.push(FilterExpr::Unlabelled);
        self
    }

    /// Filters by timestamp greater than or equal to the given time.
    pub fn since(mut self, timestamp: DateTime<Utc>) -> Self {
        self.filters
            .push(FilterExpr::Comparison(Box::new(LabelMatcher {
                field: Field::Timestamp,
                op: MatchOp::GtEq,
                value: Value::Timestamp(timestamp),
            })));
        self
    }

    /// Filters by timestamp less than or equal to the given time.
    pub fn until(mut self, timestamp: DateTime<Utc>) -> Self {
        self.filters
            .push(FilterExpr::Comparison(Box::new(LabelMatcher {
                field: Field::Timestamp,
                op: MatchOp::LtEq,
                value: Value::Timestamp(timestamp),
            })));
        self
    }

    /// Applies a preset filter (e.g., :errors, :crashes).
    pub fn preset(mut self, preset: PresetName) -> Self {
        self.filters.push(FilterExpr::Preset(preset));
        self
    }

    /// Applies the :errors preset.
    pub fn errors(self) -> Self {
        self.preset(PresetName::Errors)
    }

    /// Applies the :crashes preset.
    pub fn crashes(self) -> Self {
        self.preset(PresetName::Crashes)
    }

    /// Applies the :errors_or_crashes preset.
    pub fn errors_or_crashes(self) -> Self {
        self.preset(PresetName::ErrorsOrCrashes)
    }

    /// Applies the :disconnects preset.
    pub fn disconnects(self) -> Self {
        self.preset(PresetName::Disconnects)
    }

    /// Applies the :tls_issues preset.
    pub fn tls_issues(self) -> Self {
        self.preset(PresetName::TlsIssues)
    }

    /// Applies the :access_denied preset.
    pub fn access_denied(self) -> Self {
        self.preset(PresetName::AccessDenied)
    }

    /// Applies the :timeouts preset.
    pub fn timeouts(self) -> Self {
        self.preset(PresetName::Timeouts)
    }

    /// Adds a custom filter expression.
    pub fn filter(mut self, expr: FilterExpr) -> Self {
        self.filters.push(expr);
        self
    }

    /// Adds a filter built using a FilterBuilder.
    pub fn filter_with<F>(self, f: F) -> Self
    where
        F: FnOnce(FilterBuilder) -> FilterBuilder,
    {
        let builder = f(FilterBuilder::new());
        if let Some(expr) = builder.build() {
            self.filter(expr)
        } else {
            self
        }
    }

    /// Adds a WHERE pipeline stage with a filter expression.
    pub fn where_filter(mut self, expr: FilterExpr) -> Self {
        self.pipeline.push(PipelineStage::Where(expr));
        self
    }

    /// Limits the number of results.
    pub fn limit(mut self, n: u64) -> Self {
        self.pipeline.push(PipelineStage::Limit(n));
        self
    }

    /// Sets the offset (skip N entries).
    pub fn offset(mut self, n: u64) -> Self {
        self.pipeline.push(PipelineStage::Offset(n));
        self
    }

    /// Returns only the first N entries.
    pub fn head(mut self, n: u64) -> Self {
        self.pipeline.push(PipelineStage::Head(n));
        self
    }

    /// Returns only the last N entries.
    pub fn tail(mut self, n: u64) -> Self {
        self.pipeline.push(PipelineStage::Tail(n));
        self
    }

    /// Sorts results by the given field and direction.
    pub fn sort(mut self, field: Field, direction: SortDirection) -> Self {
        self.pipeline
            .push(PipelineStage::Sort(SortSpec { field, direction }));
        self
    }

    /// Sorts results by timestamp in descending order (newest first).
    pub fn sort_by_timestamp_desc(self) -> Self {
        self.sort(Field::Timestamp, SortDirection::Desc)
    }

    /// Sorts results by timestamp in ascending order (oldest first).
    pub fn sort_by_timestamp_asc(self) -> Self {
        self.sort(Field::Timestamp, SortDirection::Asc)
    }

    /// Projects only the specified fields.
    pub fn project(mut self, fields: Vec<Field>) -> Self {
        self.pipeline.push(PipelineStage::Project(fields));
        self
    }

    /// Counts entries, optionally grouped by a field.
    pub fn count_by(mut self, field: Option<Field>) -> Self {
        self.pipeline.push(PipelineStage::CountBy(field));
        self
    }

    /// Counts all entries.
    pub fn count(self) -> Self {
        self.count_by(None)
    }

    /// Returns distinct values for the specified fields.
    pub fn distinct(mut self, fields: Vec<Field>) -> Self {
        self.pipeline.push(PipelineStage::Distinct(fields));
        self
    }

    /// Builds the Query AST from the builder state.
    pub fn build(self) -> Query {
        let selector = if self.selector_matchers.is_empty() {
            None
        } else {
            Some(Selector {
                matchers: self.selector_matchers,
            })
        };

        let filter = combine_filters(self.filters);

        Query {
            time_range: self.time_range,
            selector,
            filter,
            pipeline: self.pipeline,
        }
    }

    /// Builds and compiles the query, returning a CompiledQuery.
    pub fn compile(self) -> Result<CompiledQuery, CompileError> {
        let query = self.build();
        compile(&query)
    }
}

/// Builder for constructing complex filter expressions.
#[derive(Debug, Clone, Default)]
pub struct FilterBuilder {
    expressions: Vec<FilterExpr>,
}

impl FilterBuilder {
    /// Creates a new empty filter builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a comparison filter (field op value).
    pub fn comparison(mut self, field: Field, op: MatchOp, value: Value) -> Self {
        self.expressions
            .push(FilterExpr::Comparison(Box::new(LabelMatcher {
                field,
                op,
                value,
            })));
        self
    }

    /// Adds an equality comparison.
    pub fn eq(self, field: Field, value: impl Into<String>) -> Self {
        self.comparison(field, MatchOp::Eq, Value::String(value.into()))
    }

    /// Adds a not-equal comparison.
    pub fn not_eq(self, field: Field, value: impl Into<String>) -> Self {
        self.comparison(field, MatchOp::NotEq, Value::String(value.into()))
    }

    /// Adds a contains comparison (case-insensitive).
    pub fn contains(self, field: Field, value: impl Into<String>) -> Self {
        self.comparison(field, MatchOp::IContains, Value::String(value.into()))
    }

    /// Adds a regex match comparison.
    pub fn matches(self, field: Field, pattern: impl Into<String>) -> Self {
        self.comparison(field, MatchOp::Regex, Value::Regex(pattern.into()))
    }

    /// Adds a "labels any" filter.
    pub fn labels_any(mut self, labels: Vec<String>) -> Self {
        self.expressions.push(FilterExpr::LabelAny(labels));
        self
    }

    /// Adds a "labels all" filter.
    pub fn labels_all(mut self, labels: Vec<String>) -> Self {
        self.expressions.push(FilterExpr::LabelAll(labels));
        self
    }

    /// Adds a preset filter.
    pub fn preset(mut self, preset: PresetName) -> Self {
        self.expressions.push(FilterExpr::Preset(preset));
        self
    }

    /// Adds a negated filter expression.
    pub fn not(mut self, expr: FilterExpr) -> Self {
        self.expressions.push(FilterExpr::Not(Box::new(expr)));
        self
    }

    /// Adds an OR expression combining two filters.
    pub fn or(mut self, left: FilterExpr, right: FilterExpr) -> Self {
        self.expressions
            .push(FilterExpr::Or(Box::new(left), Box::new(right)));
        self
    }

    /// Adds an AND expression combining two filters.
    pub fn and(mut self, left: FilterExpr, right: FilterExpr) -> Self {
        self.expressions
            .push(FilterExpr::And(Box::new(left), Box::new(right)));
        self
    }

    /// Adds a grouped (parenthesized) expression.
    pub fn grouped(mut self, expr: FilterExpr) -> Self {
        self.expressions.push(FilterExpr::Grouped(Box::new(expr)));
        self
    }

    /// Adds a raw filter expression.
    pub fn expr(mut self, expr: FilterExpr) -> Self {
        self.expressions.push(expr);
        self
    }

    /// Builds the filter expression, combining multiple filters with AND.
    pub fn build(self) -> Option<FilterExpr> {
        combine_filters(self.expressions)
    }
}

fn combine_filters(filters: Vec<FilterExpr>) -> Option<FilterExpr> {
    if filters.is_empty() {
        return None;
    }

    let mut iter = filters.into_iter();
    let first = iter.next().unwrap();

    Some(iter.fold(first, |acc, expr| {
        FilterExpr::And(Box::new(acc), Box::new(expr))
    }))
}

/// Helper function to create a simple comparison expression.
pub fn comparison(field: Field, op: MatchOp, value: Value) -> FilterExpr {
    FilterExpr::Comparison(Box::new(LabelMatcher { field, op, value }))
}

/// Helper function to create an OR expression.
pub fn or(left: FilterExpr, right: FilterExpr) -> FilterExpr {
    FilterExpr::Or(Box::new(left), Box::new(right))
}

/// Helper function to create an AND expression.
pub fn and(left: FilterExpr, right: FilterExpr) -> FilterExpr {
    FilterExpr::And(Box::new(left), Box::new(right))
}

/// Helper function to create a NOT expression.
pub fn not(expr: FilterExpr) -> FilterExpr {
    FilterExpr::Not(Box::new(expr))
}
