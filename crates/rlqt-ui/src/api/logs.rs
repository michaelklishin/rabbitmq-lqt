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

use crate::errors::ServerError;
use crate::server::AppState;
use axum::Json;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use chrono::{DateTime, Utc};
use rlqt_lib::rel_db::node_log_entry::Model;
use rlqt_lib::{NodeLogEntry, QueryContext};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct LogQueryParams {
    since_time: Option<String>,
    to_time: Option<String>,
    severity: Option<String>,
    erlang_pid: Option<String>,
    node: Option<String>,
    subsystem: Option<String>,
    labels: Option<String>,
    matching_all_labels: Option<bool>,
    limit: Option<u64>,
    has_resolution_or_discussion_url: Option<bool>,
    has_doc_url: Option<bool>,
    unlabelled: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct LogQueryResponse {
    entries: Vec<LogEntry>,
    total: usize,
}

#[derive(Debug, Serialize)]
pub struct LogEntry {
    id: i64,
    node: String,
    timestamp: String,
    severity: String,
    erlang_pid: String,
    message: String,
    subsystem: Option<String>,
    labels: HashMap<String, bool>,
    doc_url: Option<String>,
    resolution_or_discussion_url: Option<String>,
}

impl From<Model> for LogEntry {
    fn from(model: Model) -> Self {
        let labels = if let Some(obj) = model.labels.as_object() {
            obj.iter()
                .filter_map(|(k, v)| v.as_bool().filter(|&b| b).map(|b| (k.clone(), b)))
                .collect()
        } else {
            HashMap::new()
        };

        let subsystem = model
            .subsystem_id
            .and_then(rlqt_lib::entry_metadata::subsystems::Subsystem::from_id)
            .map(|s| s.to_string());

        let doc_url = model
            .doc_url_id
            .and_then(rlqt_lib::constants::doc_url_from_id);

        let resolution_or_discussion_url = model
            .resolution_or_discussion_url_id
            .and_then(rlqt_lib::constants::resolution_or_discussion_url_from_id);

        Self {
            id: model.id,
            node: model.node,
            timestamp: model.timestamp.to_rfc3339(),
            severity: model.severity,
            erlang_pid: model.erlang_pid,
            message: model.message,
            subsystem,
            labels,
            doc_url: doc_url.map(String::from),
            resolution_or_discussion_url: resolution_or_discussion_url.map(String::from),
        }
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ServerError::Database(ref e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            ServerError::Library(ref e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            ServerError::Io(ref e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            ServerError::Serialization(ref e) => (StatusCode::BAD_REQUEST, e.to_string()),
            ServerError::DateTimeParse(ref e) => (StatusCode::BAD_REQUEST, e.clone()),
        };

        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}

pub async fn query_logs(
    State(state): State<AppState>,
    Query(params): Query<LogQueryParams>,
) -> Result<Json<LogQueryResponse>, ServerError> {
    let mut ctx = QueryContext::default();

    if let Some(since) = params
        .since_time
        .as_ref()
        .map(|s| parse_datetime_flexible(s))
        .transpose()?
    {
        ctx = ctx.since(since);
    }

    if let Some(to) = params
        .to_time
        .as_ref()
        .map(|s| parse_datetime_flexible(s))
        .transpose()?
    {
        ctx = ctx.to(to);
    }

    if let Some(sev) = params.severity.as_ref() {
        ctx = ctx.severity(sev);
    }

    if let Some(pid) = params.erlang_pid.as_ref() {
        ctx = ctx.erlang_pid(pid);
    }

    if let Some(n) = params.node.as_ref() {
        ctx = ctx.node(n);
    }

    if let Some(sub) = params.subsystem.as_ref() {
        ctx = ctx.subsystem(sub);
    }

    if let Some(labels_str) = params.labels.as_ref() {
        let labels: Vec<String> = labels_str
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
        for label in labels {
            let normalized_label = if label == "election" {
                "elections"
            } else {
                label.as_str()
            };
            ctx = ctx.add_label(normalized_label);
        }
    }

    if params.matching_all_labels.unwrap_or(false) {
        ctx = ctx.matching_all_labels(true);
    }

    if let Some(l) = params.limit
        && l > 0
    {
        ctx = ctx.limit(l);
    }

    if params.has_resolution_or_discussion_url.unwrap_or(false) {
        ctx = ctx.has_resolution_or_discussion_url(true);
    }

    if params.has_doc_url.unwrap_or(false) {
        ctx = ctx.has_doc_url(true);
    }

    if params.unlabelled.unwrap_or(false) {
        ctx = ctx.add_label("unlabelled");
    }

    let models = NodeLogEntry::query(&state.db, &ctx).await?;
    let total = models.len();
    let entries: Vec<LogEntry> = models.into_iter().map(LogEntry::from).collect();

    Ok(Json(LogQueryResponse { entries, total }))
}

fn parse_datetime_flexible(s: &str) -> Result<DateTime<Utc>, ServerError> {
    rlqt_lib::datetime::parse_datetime_flexible(s).map_err(ServerError::DateTimeParse)
}
