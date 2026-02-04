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

//! Compiler that translates the RLQT Query Language AST to QueryContext.

use crate::ast::{
    Field, FilterExpr, LabelMatcher, MatchOp, PipelineStage, Query, Selector, SortDirection, Value,
};
use crate::errors::CompileError;
use chrono::Utc;
use chrono_english::{Dialect, parse_date_string};
use rabbitmq_lqt_lib::QueryContext;
use rabbitmq_lqt_lib::entry_metadata::labels::LogEntryLabels;
use rabbitmq_lqt_lib::entry_metadata::subsystems::Subsystem;
use std::mem;
use std::str::FromStr;

#[derive(Debug, Clone, Default)]
pub struct CompiledQuery {
    pub context: QueryContext,
    pub sql_where_fragments: Vec<String>,
    pub sql_order_by: Option<String>,
    pub sql_offset: Option<u64>,
    pub sql_limit_from_end: Option<u64>,
    pub projection: Option<Vec<Field>>,
    pub distinct_fields: Option<Vec<Field>>,
    pub count_by_field: Option<Field>,
    pub has_aggregation: bool,
}

pub fn compile(query: &Query) -> Result<CompiledQuery, CompileError> {
    let mut compiled = CompiledQuery::default();

    if let Some(ref time_range) = query.time_range {
        let since = Utc::now() - time_range.to_chrono_duration();
        compiled.context = compiled.context.since(since);
    }

    if let Some(ref selector) = query.selector {
        compile_selector(selector, &mut compiled)?;
    }

    if let Some(ref filter) = query.filter {
        compile_filter(filter, &mut compiled)?;
    }

    for stage in &query.pipeline {
        compile_pipeline_stage(stage, &mut compiled)?;
    }

    Ok(compiled)
}

fn compile_selector(selector: &Selector, compiled: &mut CompiledQuery) -> Result<(), CompileError> {
    for matcher in &selector.matchers {
        compile_matcher(matcher, compiled)?;
    }
    Ok(())
}

fn compile_matcher(
    matcher: &LabelMatcher,
    compiled: &mut CompiledQuery,
) -> Result<(), CompileError> {
    match matcher.field {
        Field::Severity => {
            if matcher.op == MatchOp::Eq {
                if let Value::String(ref s) = matcher.value {
                    validate_severity(s)?;
                    compiled.context = mem::take(&mut compiled.context).severity(s.clone());
                }
            } else {
                let fragment = compile_comparison_to_sql(matcher)?;
                compiled.sql_where_fragments.push(fragment);
            }
        }
        Field::Subsystem => {
            if matcher.op == MatchOp::Eq {
                if let Value::String(ref s) = matcher.value {
                    compiled.context = mem::take(&mut compiled.context).subsystem(s.clone());
                }
            } else {
                let fragment = compile_comparison_to_sql(matcher)?;
                compiled.sql_where_fragments.push(fragment);
            }
        }
        Field::Node => {
            if matcher.op == MatchOp::Eq {
                if let Value::String(ref s) = matcher.value {
                    compiled.context = mem::take(&mut compiled.context).node(s.clone());
                }
            } else {
                let fragment = compile_comparison_to_sql(matcher)?;
                compiled.sql_where_fragments.push(fragment);
            }
        }
        Field::ErlangPid => {
            if matcher.op == MatchOp::Eq {
                if let Value::String(ref s) = matcher.value {
                    compiled.context = mem::take(&mut compiled.context).erlang_pid(s.clone());
                }
            } else {
                let fragment = compile_comparison_to_sql(matcher)?;
                compiled.sql_where_fragments.push(fragment);
            }
        }
        Field::Labels => {
            if matcher.op == MatchOp::HasLabel {
                if let Value::String(ref label) = matcher.value {
                    compiled.context = mem::take(&mut compiled.context).add_label(label.clone());
                }
            } else if let Value::LabelList(ref labels) = matcher.value {
                for label in labels {
                    compiled.context = mem::take(&mut compiled.context).add_label(label.clone());
                }
            }
        }
        Field::Timestamp => {
            compile_timestamp_matcher(matcher, compiled)?;
        }
        Field::Message => {
            let fragment = compile_comparison_to_sql(matcher)?;
            compiled.sql_where_fragments.push(fragment);
        }
        Field::Id => {
            let fragment = compile_comparison_to_sql(matcher)?;
            compiled.sql_where_fragments.push(fragment);
        }
    }
    Ok(())
}

fn compile_timestamp_matcher(
    matcher: &LabelMatcher,
    compiled: &mut CompiledQuery,
) -> Result<(), CompileError> {
    match &matcher.value {
        Value::RelativeTime(duration) => {
            let ts = Utc::now() - duration.to_chrono_duration();
            match matcher.op {
                MatchOp::GtEq | MatchOp::Gt => {
                    compiled.context = mem::take(&mut compiled.context).since(ts);
                }
                MatchOp::LtEq | MatchOp::Lt => {
                    compiled.context = mem::take(&mut compiled.context).to(ts);
                }
                _ => {
                    let fragment = compile_comparison_to_sql(matcher)?;
                    compiled.sql_where_fragments.push(fragment);
                }
            }
        }
        Value::Timestamp(ts) => match matcher.op {
            MatchOp::GtEq | MatchOp::Gt => {
                compiled.context = mem::take(&mut compiled.context).since(*ts);
            }
            MatchOp::LtEq | MatchOp::Lt => {
                compiled.context = mem::take(&mut compiled.context).to(*ts);
            }
            _ => {
                let fragment = compile_comparison_to_sql(matcher)?;
                compiled.sql_where_fragments.push(fragment);
            }
        },
        Value::String(s) => {
            let ts = parse_timestamp_string(s)?;
            let mut modified_matcher = matcher.clone();
            modified_matcher.value = Value::Timestamp(ts);
            compile_timestamp_matcher(&modified_matcher, compiled)?;
        }
        _ => {
            return Err(CompileError::InvalidTimestamp {
                reason: "expected timestamp value".to_string(),
            });
        }
    }
    Ok(())
}

fn parse_timestamp_string(s: &str) -> Result<chrono::DateTime<Utc>, CompileError> {
    parse_date_string(s, Utc::now(), Dialect::Us).map_err(|e| CompileError::InvalidTimestamp {
        reason: e.to_string(),
    })
}

fn compile_filter(filter: &FilterExpr, compiled: &mut CompiledQuery) -> Result<(), CompileError> {
    match filter {
        FilterExpr::Comparison(matcher) => {
            compile_matcher(matcher, compiled)?;
        }
        FilterExpr::And(left, right) => {
            compile_filter(left, compiled)?;
            compile_filter(right, compiled)?;
        }
        FilterExpr::Or(left, right) => {
            let left_sql = compile_filter_to_sql(left)?;
            let right_sql = compile_filter_to_sql(right)?;
            compiled
                .sql_where_fragments
                .push(format!("({} OR {})", left_sql, right_sql));
        }
        FilterExpr::Not(inner) => {
            let inner_sql = compile_filter_to_sql(inner)?;
            compiled
                .sql_where_fragments
                .push(format!("NOT ({})", inner_sql));
        }
        FilterExpr::LabelAny(labels) => {
            let mask = compute_label_mask(labels)?;
            compiled
                .sql_where_fragments
                .push(format!("(labels & {}) != 0", mask));
        }
        FilterExpr::LabelAll(labels) => {
            let mask = compute_label_mask(labels)?;
            compiled
                .sql_where_fragments
                .push(format!("(labels & {}) = {}", mask, mask));
        }
        FilterExpr::SubsystemAny(subsystems) => {
            compiled
                .sql_where_fragments
                .push(format_subsystem_any_sql(subsystems)?);
        }
        FilterExpr::HasDocUrl => {
            compiled.context = mem::take(&mut compiled.context).has_doc_url(true);
        }
        FilterExpr::HasResolutionUrl => {
            compiled.context =
                mem::take(&mut compiled.context).has_resolution_or_discussion_url(true);
        }
        FilterExpr::Unlabelled => {
            compiled.context = mem::take(&mut compiled.context).add_label("unlabelled".to_string());
        }
        FilterExpr::Preset(preset) => {
            let expanded = preset.to_filter_expr();
            compile_filter(&expanded, compiled)?;
        }
        FilterExpr::Grouped(inner) => {
            compile_filter(inner, compiled)?;
        }
    }
    Ok(())
}

fn compile_filter_to_sql(filter: &FilterExpr) -> Result<String, CompileError> {
    match filter {
        FilterExpr::Comparison(matcher) => compile_comparison_to_sql(matcher),
        FilterExpr::And(left, right) => {
            let left_sql = compile_filter_to_sql(left)?;
            let right_sql = compile_filter_to_sql(right)?;
            Ok(format!("({} AND {})", left_sql, right_sql))
        }
        FilterExpr::Or(left, right) => {
            let left_sql = compile_filter_to_sql(left)?;
            let right_sql = compile_filter_to_sql(right)?;
            Ok(format!("({} OR {})", left_sql, right_sql))
        }
        FilterExpr::Not(inner) => {
            let inner_sql = compile_filter_to_sql(inner)?;
            Ok(format!("NOT ({})", inner_sql))
        }
        FilterExpr::LabelAny(labels) => {
            let mask = compute_label_mask(labels)?;
            Ok(format!("(labels & {}) != 0", mask))
        }
        FilterExpr::LabelAll(labels) => {
            let mask = compute_label_mask(labels)?;
            Ok(format!("(labels & {}) = {}", mask, mask))
        }
        FilterExpr::SubsystemAny(subsystems) => format_subsystem_any_sql(subsystems),
        FilterExpr::HasDocUrl => Ok("doc_url_id IS NOT NULL".to_string()),
        FilterExpr::HasResolutionUrl => {
            Ok("resolution_or_discussion_url_id IS NOT NULL".to_string())
        }
        FilterExpr::Unlabelled => {
            let unlabelled_bit = LogEntryLabels::UNLABELLED.bits();
            Ok(format!("(labels & {}) != 0", unlabelled_bit))
        }
        FilterExpr::Preset(preset) => {
            let expanded = preset.to_filter_expr();
            compile_filter_to_sql(&expanded)
        }
        FilterExpr::Grouped(inner) => {
            let inner_sql = compile_filter_to_sql(inner)?;
            Ok(format!("({})", inner_sql))
        }
    }
}

fn compile_comparison_to_sql(matcher: &LabelMatcher) -> Result<String, CompileError> {
    let column = matcher.field.sql_column();
    let value_sql = compile_value_to_sql(&matcher.value)?;

    match matcher.op {
        MatchOp::Eq => Ok(format!("{} = {}", column, value_sql)),
        MatchOp::NotEq => Ok(format!("{} != {}", column, value_sql)),
        MatchOp::Lt => Ok(format!("{} < {}", column, value_sql)),
        MatchOp::LtEq => Ok(format!("{} <= {}", column, value_sql)),
        MatchOp::Gt => Ok(format!("{} > {}", column, value_sql)),
        MatchOp::GtEq => Ok(format!("{} >= {}", column, value_sql)),
        MatchOp::Contains => {
            if let Value::String(ref s) = matcher.value {
                Ok(format!(
                    "{} ILIKE '%{}%' ESCAPE '\\'",
                    column,
                    escape_like_pattern(s)
                ))
            } else {
                Ok(format!("{} ILIKE '%' || {} || '%'", column, value_sql))
            }
        }
        MatchOp::IContains => {
            if let Value::String(ref s) = matcher.value {
                Ok(format!(
                    "{} ILIKE '%{}%' ESCAPE '\\'",
                    column,
                    escape_like_pattern(s)
                ))
            } else {
                Ok(format!("{} ILIKE '%' || {} || '%'", column, value_sql))
            }
        }
        MatchOp::Regex => {
            if let Value::Regex(ref pattern) = matcher.value {
                validate_regex(pattern)?;
                Ok(format!(
                    "regexp_matches({}, '{}')",
                    column,
                    escape_sql_string(pattern)
                ))
            } else if let Value::String(ref pattern) = matcher.value {
                validate_regex(pattern)?;
                Ok(format!(
                    "regexp_matches({}, '{}')",
                    column,
                    escape_sql_string(pattern)
                ))
            } else {
                Err(CompileError::UnsupportedOperation {
                    operation: "regex requires string pattern".to_string(),
                })
            }
        }
        MatchOp::NotRegex => {
            if let Value::Regex(ref pattern) = matcher.value {
                validate_regex(pattern)?;
                Ok(format!(
                    "NOT regexp_matches({}, '{}')",
                    column,
                    escape_sql_string(pattern)
                ))
            } else if let Value::String(ref pattern) = matcher.value {
                validate_regex(pattern)?;
                Ok(format!(
                    "NOT regexp_matches({}, '{}')",
                    column,
                    escape_sql_string(pattern)
                ))
            } else {
                Err(CompileError::UnsupportedOperation {
                    operation: "regex requires string pattern".to_string(),
                })
            }
        }
        MatchOp::HasLabel => {
            if let Value::String(ref label) = matcher.value {
                let bit = LogEntryLabels::bit_for_label(label)
                    .ok_or_else(|| CompileError::unknown_label(label))?;
                Ok(format!("(labels & {}) != 0", bit))
            } else {
                Err(CompileError::UnsupportedOperation {
                    operation: "~= requires string label".to_string(),
                })
            }
        }
    }
}

fn compile_value_to_sql(value: &Value) -> Result<String, CompileError> {
    match value {
        Value::String(s) => Ok(format!("'{}'", escape_sql_string(s))),
        Value::Regex(s) => Ok(format!("'{}'", escape_sql_string(s))),
        Value::Integer(n) => Ok(n.to_string()),
        Value::Timestamp(ts) => Ok(format!("'{}'", ts.format("%Y-%m-%d %H:%M:%S%.6f"))),
        Value::RelativeTime(duration) => {
            let ts = Utc::now() - duration.to_chrono_duration();
            Ok(format!("'{}'", ts.format("%Y-%m-%d %H:%M:%S%.6f")))
        }
        Value::LabelList(_) => Err(CompileError::UnsupportedOperation {
            operation: "label list in SQL comparison".to_string(),
        }),
        Value::Boolean(b) => Ok(if *b { "true" } else { "false" }.to_string()),
        Value::Null => Ok("NULL".to_string()),
    }
}

fn compile_pipeline_stage(
    stage: &PipelineStage,
    compiled: &mut CompiledQuery,
) -> Result<(), CompileError> {
    match stage {
        PipelineStage::Where(filter) => {
            compile_filter(filter, compiled)?;
        }
        PipelineStage::Limit(n) => {
            compiled.context = mem::take(&mut compiled.context).limit(*n);
        }
        PipelineStage::Offset(n) => {
            compiled.sql_offset = Some(*n);
        }
        PipelineStage::Head(n) => {
            compiled.context = mem::take(&mut compiled.context).limit(*n);
        }
        PipelineStage::Tail(n) => {
            compiled.sql_limit_from_end = Some(*n);
        }
        PipelineStage::Sort(spec) => {
            let direction = match spec.direction {
                SortDirection::Asc => "ASC",
                SortDirection::Desc => "DESC",
            };
            compiled.sql_order_by = Some(format!("{} {}", spec.field.sql_column(), direction));
        }
        PipelineStage::Project(fields) => {
            compiled.projection = Some(fields.clone());
        }
        PipelineStage::CountBy(field) => {
            compiled.has_aggregation = true;
            compiled.count_by_field = *field;
        }
        PipelineStage::Distinct(fields) => {
            compiled.distinct_fields = Some(fields.clone());
        }
    }
    Ok(())
}

fn compute_label_mask(labels: &[String]) -> Result<u64, CompileError> {
    let mut mask: u64 = 0;
    for label in labels {
        let bit = LogEntryLabels::bit_for_label(label)
            .ok_or_else(|| CompileError::unknown_label(label))?;
        mask |= bit;
    }
    Ok(mask)
}

fn compute_subsystem_ids(subsystems: &[String]) -> Result<Vec<i16>, CompileError> {
    subsystems
        .iter()
        .map(|name| {
            Subsystem::from_str(name)
                .map(|s| s.to_id())
                .map_err(|_| CompileError::invalid_subsystem(name))
        })
        .collect()
}

fn format_subsystem_any_sql(subsystems: &[String]) -> Result<String, CompileError> {
    let ids = compute_subsystem_ids(subsystems)?;
    let id_list = ids
        .iter()
        .map(|id| id.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    Ok(format!("subsystem_id IN ({})", id_list))
}

fn validate_severity(s: &str) -> Result<(), CompileError> {
    match s.to_lowercase().as_str() {
        "debug" | "info" | "notice" | "warning" | "error" | "critical" => Ok(()),
        _ => Err(CompileError::invalid_severity(s)),
    }
}

fn validate_regex(pattern: &str) -> Result<(), CompileError> {
    regex::Regex::new(pattern).map_err(|e| CompileError::RegexCompilation {
        pattern: pattern.to_string(),
        reason: e.to_string(),
    })?;
    Ok(())
}

/// Escapes a string for use in SQL single-quoted string literals.
///
/// # Safety
///
/// This function provides protection against SQL injection by:
/// - Doubling single quotes (standard SQL escaping)
/// - Escaping backslashes (for databases that interpret them as escape chars)
///
/// All user input that reaches this function has already been validated by the
/// QL parser, which restricts the character set and structure of queries.
fn escape_sql_string(s: &str) -> String {
    s.replace('\\', "\\\\").replace('\'', "''")
}

/// Escapes a string for use in SQL LIKE patterns.
///
/// In addition to standard string escaping, this escapes LIKE wildcards
/// (`%` and `_`) to prevent unintended pattern matching.
fn escape_like_pattern(s: &str) -> String {
    escape_sql_string(s).replace('%', "\\%").replace('_', "\\_")
}
