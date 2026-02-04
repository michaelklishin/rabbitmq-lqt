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
use crate::entry_metadata::labels::{LABEL_NAMES, LogEntryLabels};
use crate::parser::ParsedLogEntry;
use crate::rel_db::DatabaseConnection;
use crate::rel_db::presets::QueryPreset;
use chrono::{DateTime, Utc};
use duckdb::types::Value;
use duckdb::{Error as DuckDbError, params};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::io::Error as IoError;

fn pool_error_to_duckdb(e: impl Display) -> DuckDbError {
    DuckDbError::ToSqlConversionFailure(Box::new(IoError::other(format!(
        "Connection pool error: {}",
        e
    ))))
}

pub struct NodeLogEntry;

const DEFAULT_MAX_QUERY_LIMIT: u64 = 10_000;
const DB_INSERT_BATCH_SIZE: usize = 3000;

#[derive(Debug, Default, Clone)]
pub struct QueryContext {
    pub(crate) since_time: Option<DateTime<Utc>>,
    pub(crate) to_time: Option<DateTime<Utc>>,
    pub(crate) severity: Option<String>,
    pub(crate) erlang_pid: Option<String>,
    pub(crate) node: Option<String>,
    pub(crate) subsystem: Option<String>,
    pub(crate) labels: Vec<String>,
    pub(crate) matching_all_labels: bool,
    pub(crate) limit: Option<u64>,
    pub(crate) has_resolution_or_discussion_url: bool,
    pub(crate) has_doc_url: bool,
    pub(crate) preset: Option<QueryPreset>,
    pub(crate) raw_where_clauses: Vec<String>,
}

impl QueryContext {
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
    pub fn severity(mut self, sev: impl Into<String>) -> Self {
        self.severity = Some(sev.into());
        self
    }

    #[must_use]
    pub fn erlang_pid(mut self, pid: impl Into<String>) -> Self {
        self.erlang_pid = Some(pid.into());
        self
    }

    #[must_use]
    pub fn node(mut self, n: impl Into<String>) -> Self {
        self.node = Some(n.into());
        self
    }

    #[must_use]
    pub fn subsystem(mut self, sub: impl Into<String>) -> Self {
        self.subsystem = Some(sub.into());
        self
    }

    #[must_use]
    pub fn labels(mut self, labels: Vec<String>) -> Self {
        self.labels = labels;
        self
    }

    #[must_use]
    pub fn add_label(mut self, label: impl Into<String>) -> Self {
        self.labels.push(label.into());
        self
    }

    #[must_use]
    pub fn matching_all_labels(mut self, match_all: bool) -> Self {
        self.matching_all_labels = match_all;
        self
    }

    #[must_use]
    pub fn limit(mut self, l: u64) -> Self {
        self.limit = Some(l);
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

    #[must_use]
    pub fn preset(mut self, preset: QueryPreset) -> Self {
        self.preset = Some(preset);
        self
    }

    /// Adds a raw SQL WHERE clause fragment.
    ///
    /// ## Safety
    ///
    /// Only use with trusted, pre-validated input such as output from the QL compiler.
    /// User input must be properly escaped before being included in the clause.
    ///
    /// The QL compiler (`rabbitmq_lqt_ql::compiler`) uses `escape_sql_string()` and
    /// `escape_like_pattern()` to sanitize user-provided values before generating
    /// SQL fragments that are passed to this method.
    #[must_use]
    pub fn raw_where(mut self, clause: impl Into<String>) -> Self {
        self.raw_where_clauses.push(clause.into());
        self
    }

    /// Sets multiple raw SQL WHERE clause fragments.
    ///
    /// ## Safety
    ///
    /// **This method accepts raw SQL that is concatenated directly into the query.**
    /// See [`raw_where`](Self::raw_where).
    #[must_use]
    pub fn raw_where_clauses(mut self, clauses: Vec<String>) -> Self {
        self.raw_where_clauses = clauses;
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Model {
    pub id: i64,
    pub node: String,
    pub timestamp: DateTime<Utc>,
    pub severity: String,
    pub erlang_pid: String,
    pub subsystem_id: Option<i16>,
    pub message: String,
    pub labels: i64,
    pub resolution_or_discussion_url_id: Option<i16>,
    pub doc_url_id: Option<i16>,
}

impl Model {
    #[inline]
    pub fn is_multiline(&self) -> bool {
        self.message.contains('\n')
    }

    #[inline]
    pub fn format_labels(&self) -> String {
        let labels = LogEntryLabels::from_bits_i64(self.labels);
        let mut result = String::new();
        for (i, label_name) in LABEL_NAMES.iter().enumerate() {
            if labels.bits() & (1u64 << i) != 0 {
                if !result.is_empty() {
                    result.push('\n');
                }
                result.push_str(label_name);
            }
        }
        result
    }

    #[inline]
    pub fn get_labels(&self) -> LogEntryLabels {
        LogEntryLabels::from_bits_i64(self.labels)
    }
}

impl NodeLogEntry {
    pub fn count_all(db: &DatabaseConnection) -> Result<u64, DuckDbError> {
        let conn = db.get().map_err(pool_error_to_duckdb)?;

        let mut stmt = conn.prepare("SELECT COUNT(*) FROM node_log_entries")?;
        let count: i64 = stmt.query_row([], |row| row.get(0))?;
        Ok(count as u64)
    }

    pub fn max_entry_id(db: &DatabaseConnection) -> Result<i64, DuckDbError> {
        let conn = db.get().map_err(pool_error_to_duckdb)?;

        let max_id: Option<i64> = conn
            .query_row("SELECT MAX(id) FROM node_log_entries", [], |row| row.get(0))
            .ok()
            .flatten();
        Ok(max_id.unwrap_or(0))
    }

    pub fn query(db: &DatabaseConnection, ctx: &QueryContext) -> Result<Vec<Model>, DuckDbError> {
        let conn = db.get().map_err(pool_error_to_duckdb)?;

        let mut conditions = Vec::new();
        let mut params: Vec<Value> = Vec::new();

        if let Some(since) = ctx.since_time {
            conditions.push("timestamp >= ?".to_string());
            params.push(Value::Timestamp(
                duckdb::types::TimeUnit::Microsecond,
                since.timestamp_micros(),
            ));
        }

        if let Some(to) = ctx.to_time {
            conditions.push("timestamp <= ?".to_string());
            params.push(Value::Timestamp(
                duckdb::types::TimeUnit::Microsecond,
                to.timestamp_micros(),
            ));
        }

        if let Some(ref preset) = ctx.preset {
            let mut or_parts = Vec::new();

            if let Some(sev) = preset.severity() {
                or_parts.push("severity = ?".to_string());
                params.push(Value::Text(sev.to_string()));
            }

            let label_mask = preset.labels().bits();
            if label_mask != 0 {
                or_parts.push("(labels & ?) != 0".to_string());
                params.push(Value::BigInt(label_mask as i64));
            }

            if !or_parts.is_empty() {
                conditions.push(format!("({})", or_parts.join(" OR ")));
            }
        } else {
            if let Some(ref sev) = ctx.severity {
                conditions.push("severity = ?".to_string());
                params.push(Value::Text(sev.clone()));
            }

            if !ctx.labels.is_empty() {
                let mut combined_mask: u64 = 0;
                for label in &ctx.labels {
                    if let Some(bit) = LogEntryLabels::bit_for_label(label) {
                        combined_mask |= bit;
                    }
                }
                if combined_mask != 0 {
                    if ctx.matching_all_labels {
                        conditions.push("(labels & ?) = ?".to_string());
                        params.push(Value::BigInt(combined_mask as i64));
                        params.push(Value::BigInt(combined_mask as i64));
                    } else {
                        conditions.push("(labels & ?) != 0".to_string());
                        params.push(Value::BigInt(combined_mask as i64));
                    }
                }
            }
        }

        if let Some(ref pid) = ctx.erlang_pid {
            conditions.push("erlang_pid = ?".to_string());
            params.push(Value::Text(pid.clone()));
        }

        if let Some(ref n) = ctx.node {
            conditions.push("node = ?".to_string());
            params.push(Value::Text(n.clone()));
        }

        if let Some(ref sub) = ctx.subsystem
            && let Ok(subsystem) = sub.parse::<crate::entry_metadata::subsystems::Subsystem>()
        {
            conditions.push("subsystem_id = ?".to_string());
            params.push(Value::SmallInt(subsystem.to_id()));
        }

        if ctx.has_resolution_or_discussion_url {
            conditions.push("resolution_or_discussion_url_id IS NOT NULL".to_string());
        }

        if ctx.has_doc_url {
            conditions.push("doc_url_id IS NOT NULL".to_string());
        }

        for clause in &ctx.raw_where_clauses {
            conditions.push(clause.clone());
        }

        let effective_limit = ctx.limit.unwrap_or(DEFAULT_MAX_QUERY_LIMIT);

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let sql = format!(
            "SELECT id, node, timestamp, severity, erlang_pid, subsystem_id, message, labels, resolution_or_discussion_url_id, doc_url_id
             FROM node_log_entries
             {}
             ORDER BY timestamp ASC
             LIMIT {}",
            where_clause, effective_limit
        );

        let mut stmt = conn.prepare(&sql)?;
        let params_slice: Vec<&dyn duckdb::ToSql> =
            params.iter().map(|p| p as &dyn duckdb::ToSql).collect();

        let rows = stmt.query_map(params_slice.as_slice(), |row| {
            let timestamp_micros: i64 = row.get(2)?;
            let timestamp = DateTime::from_timestamp_micros(timestamp_micros)
                .unwrap_or_else(|| DateTime::from_timestamp(0, 0).unwrap());

            Ok(Model {
                id: row.get(0)?,
                node: row.get(1)?,
                timestamp,
                severity: row.get(3)?,
                erlang_pid: row.get(4)?,
                subsystem_id: row.get(5)?,
                message: row.get(6)?,
                labels: row.get(7)?,
                resolution_or_discussion_url_id: row.get(8)?,
                doc_url_id: row.get(9)?,
            })
        })?;

        let mut results = Vec::new();
        for row_result in rows {
            results.push(row_result?);
        }
        Ok(results)
    }

    pub fn insert_parsed_entries(
        db: &DatabaseConnection,
        entries: &[ParsedLogEntry],
        node: &str,
    ) -> Result<(), DuckDbError> {
        if entries.is_empty() {
            return Ok(());
        }

        let conn = db.get().map_err(pool_error_to_duckdb)?;

        let needs_auto_id = entries.iter().any(|e| e.explicit_id.is_none());
        let mut running_id = if needs_auto_id {
            let max_id: Option<i64> = conn
                .query_row("SELECT MAX(id) FROM node_log_entries", [], |row| row.get(0))
                .ok()
                .flatten();
            max_id.unwrap_or(0) + 1
        } else {
            0
        };

        for chunk in entries.chunks(DB_INSERT_BATCH_SIZE) {
            let mut appender = conn.appender("node_log_entries")?;

            for entry in chunk {
                let id = entry.explicit_id.unwrap_or_else(|| {
                    let id = running_id;
                    running_id += 1;
                    id
                });
                let timestamp_micros = entry.timestamp.timestamp_micros();

                appender.append_row(params![
                    id,
                    node,
                    Value::Timestamp(duckdb::types::TimeUnit::Microsecond, timestamp_micros),
                    entry.severity.to_string(),
                    entry.process_id,
                    entry.subsystem_id,
                    entry.message,
                    entry.labels.to_bits_i64(),
                    entry.resolution_or_discussion_url_id,
                    entry.doc_url_id,
                ])?;
            }

            appender.flush()?;
        }

        Ok(())
    }

    pub fn find_all(db: &DatabaseConnection) -> Result<Vec<Model>, DuckDbError> {
        let conn = db.get().map_err(pool_error_to_duckdb)?;

        let mut stmt = conn.prepare(
            "SELECT id, node, timestamp, severity, erlang_pid, subsystem_id, message, labels, resolution_or_discussion_url_id, doc_url_id
             FROM node_log_entries
             ORDER BY timestamp ASC",
        )?;

        let rows = stmt.query_map([], |row| {
            let timestamp_micros: i64 = row.get(2)?;
            let timestamp = DateTime::from_timestamp_micros(timestamp_micros)
                .unwrap_or_else(|| DateTime::from_timestamp(0, 0).unwrap());

            Ok(Model {
                id: row.get(0)?,
                node: row.get(1)?,
                timestamp,
                severity: row.get(3)?,
                erlang_pid: row.get(4)?,
                subsystem_id: row.get(5)?,
                message: row.get(6)?,
                labels: row.get(7)?,
                resolution_or_discussion_url_id: row.get(8)?,
                doc_url_id: row.get(9)?,
            })
        })?;

        let mut results = Vec::new();
        for row_result in rows {
            results.push(row_result?);
        }
        Ok(results)
    }

    pub fn get_node_counts(db: &DatabaseConnection) -> Result<Vec<(String, i64)>, DuckDbError> {
        let conn = db.get().map_err(pool_error_to_duckdb)?;

        let mut stmt = conn.prepare(
            "SELECT node, COUNT(*) as count FROM node_log_entries GROUP BY node ORDER BY node ASC",
        )?;

        let rows = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?;

        let mut results = Vec::new();
        for row_result in rows {
            results.push(row_result?);
        }
        Ok(results)
    }
}
