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
use sea_orm::entity::prelude::*;
use sea_orm::sea_query::Expr;
use sea_orm::{ActiveValue, QueryOrder, QuerySelect, TransactionTrait};
use serde::{Deserialize, Serialize};

pub type NodeLogEntry = Entity;

/// Default maximum number of entries returned by filtering queries.
/// Can be overridden by specifying an explicit limit in the query context.
const DEFAULT_MAX_QUERY_LIMIT: u64 = 10_000;

/// Number of entries to insert in a single database transaction.
/// Batching provides a 10-15% entry insertion speedup.
///
/// Values higher hit the law of diminishing returns.
const DB_INSERT_BATCH_SIZE: usize = 2000;

/// Query context for filtering log entries
#[derive(Debug, Default, Clone)]
pub struct QueryContext {
    pub(crate) since_time: Option<DateTimeUtc>,
    pub(crate) to_time: Option<DateTimeUtc>,
    pub(crate) severity: Option<String>,
    pub(crate) erlang_pid: Option<String>,
    pub(crate) node: Option<String>,
    pub(crate) subsystem: Option<String>,
    pub(crate) labels: Vec<String>,
    pub(crate) matching_all_labels: bool,
    pub(crate) limit: Option<u64>,
    pub(crate) has_resolution_or_discussion_url: bool,
    pub(crate) has_doc_url: bool,
}

impl QueryContext {
    #[must_use]
    pub fn since(mut self, time: DateTimeUtc) -> Self {
        self.since_time = Some(time);
        self
    }

    #[must_use]
    pub fn to(mut self, time: DateTimeUtc) -> Self {
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
}

/// A RabbitMQ log entry stored in the database
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "node_log_entries")]
pub struct Model {
    /// Unique numerical ID (primary key)
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,

    /// Node name
    #[sea_orm(indexed)]
    pub node: String,

    /// Timestamp of the log entry
    #[sea_orm(indexed)]
    pub timestamp: DateTimeUtc,

    /// Log severity level (debug, info, notice, warning, error, critical)
    #[sea_orm(indexed)]
    pub severity: String,

    /// Erlang PID (e.g., "<0.208.0>")
    #[sea_orm(indexed)]
    pub erlang_pid: String,

    /// Subsystem identifier
    #[sea_orm(indexed)]
    pub subsystem_id: Option<i16>,

    /// Log message content (can be multiline)
    pub message: String,

    /// Labels attached to this log entry (stored as bitflags in i64)
    pub labels: i64,

    /// ID of related resolution or discussion URL
    pub resolution_or_discussion_url_id: Option<i16>,

    /// ID of relevant documentation URL
    pub doc_url_id: Option<i16>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    /// Check if this log entry is multiline
    #[inline]
    pub fn is_multiline(&self) -> bool {
        self.message.contains('\n')
    }

    /// Format labels as a newline-separated list of set labels
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

    /// Get labels as LogEntryLabels bitflags
    #[inline]
    pub fn get_labels(&self) -> LogEntryLabels {
        LogEntryLabels::from_bits_i64(self.labels)
    }
}

impl ActiveModel {
    fn from_parsed(entry: &ParsedLogEntry, node: &str) -> Self {
        let id_value = if let Some(explicit_id) = entry.explicit_id {
            ActiveValue::Set(explicit_id)
        } else {
            ActiveValue::NotSet
        };

        Self {
            id: id_value,
            node: ActiveValue::Set(node.to_string()),
            timestamp: ActiveValue::Set(entry.timestamp),
            severity: ActiveValue::Set(entry.severity.to_string()),
            erlang_pid: ActiveValue::Set(entry.process_id.clone()),
            subsystem_id: ActiveValue::Set(entry.subsystem_id),
            message: ActiveValue::Set(entry.message.clone()),
            labels: ActiveValue::Set(entry.labels.to_bits_i64()),
            resolution_or_discussion_url_id: ActiveValue::Set(
                entry.resolution_or_discussion_url_id,
            ),
            doc_url_id: ActiveValue::Set(entry.doc_url_id),
        }
    }
}

impl Entity {
    /// Count all log entries
    pub async fn count_all(db: &DatabaseConnection) -> Result<u64, DbErr> {
        Self::find().count(db).await
    }

    /// Query log entries with optional filters
    ///
    /// Default limit is 10,000 entries to prevent memory exhaustion.
    /// Specify a limit explicitly to override this.
    pub async fn query(db: &DatabaseConnection, ctx: &QueryContext) -> Result<Vec<Model>, DbErr> {
        let mut query = Self::find();

        if let Some(since) = ctx.since_time {
            query = query.filter(Column::Timestamp.gte(since));
        }

        if let Some(to) = ctx.to_time {
            query = query.filter(Column::Timestamp.lte(to));
        }

        if let Some(ref sev) = ctx.severity {
            query = query.filter(Column::Severity.eq(sev));
        }

        if let Some(ref pid) = ctx.erlang_pid {
            query = query.filter(Column::ErlangPid.eq(pid));
        }

        if let Some(ref n) = ctx.node {
            query = query.filter(Column::Node.eq(n));
        }

        if let Some(ref sub) = ctx.subsystem
            && let Ok(subsystem) = sub.parse::<crate::entry_metadata::subsystems::Subsystem>()
        {
            query = query.filter(Column::SubsystemId.eq(subsystem.to_id()));
        }

        if !ctx.labels.is_empty() {
            if ctx.matching_all_labels {
                // AND query: all specified labels must be set
                let mut combined_mask: u64 = 0;
                for label in &ctx.labels {
                    if let Some(bit) = LogEntryLabels::bit_for_label(label) {
                        combined_mask |= bit;
                    }
                }
                if combined_mask != 0 {
                    query = query.filter(Expr::cust_with_values(
                        "(labels & ?) = ?",
                        [combined_mask as i64, combined_mask as i64],
                    ));
                }
            } else {
                // OR query: any of the specified labels must be set
                let mut combined_mask: u64 = 0;
                for label in &ctx.labels {
                    if let Some(bit) = LogEntryLabels::bit_for_label(label) {
                        combined_mask |= bit;
                    }
                }
                if combined_mask != 0 {
                    query = query.filter(Expr::cust_with_values(
                        "(labels & ?) != 0",
                        [combined_mask as i64],
                    ));
                }
            }
        }

        if ctx.has_resolution_or_discussion_url {
            query = query.filter(Column::ResolutionOrDiscussionUrlId.is_not_null());
        }

        if ctx.has_doc_url {
            query = query.filter(Column::DocUrlId.is_not_null());
        }

        query = query.order_by_asc(Column::Timestamp);

        let effective_limit = ctx.limit.unwrap_or(DEFAULT_MAX_QUERY_LIMIT);
        query = query.limit(effective_limit);

        query.all(db).await
    }

    pub async fn insert_parsed_entries(
        db: &DatabaseConnection,
        entries: &[ParsedLogEntry],
        node: &str,
    ) -> Result<(), DbErr> {
        if entries.is_empty() {
            return Ok(());
        }

        let txn = db.begin().await?;
        for chunk in entries.chunks(DB_INSERT_BATCH_SIZE) {
            let active_models: Vec<ActiveModel> = chunk
                .iter()
                .map(|entry| ActiveModel::from_parsed(entry, node))
                .collect();
            Entity::insert_many(active_models).exec(&txn).await?;
        }
        txn.commit().await?;
        Ok(())
    }
}
