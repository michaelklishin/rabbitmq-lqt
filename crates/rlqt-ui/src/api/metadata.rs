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
use axum::extract::State;
use rlqt_lib::NodeLogEntry;
use rlqt_lib::Severity;
use rlqt_lib::entry_metadata::labels::LABEL_NAMES;
use rlqt_lib::rel_db::FileMetadata;
use rlqt_lib::rel_db::node_log_entry::Column;
use sea_orm::{ColumnTrait, EntityTrait, FromQueryResult, QueryOrder, QuerySelect};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashSet;

#[derive(Debug, Serialize)]
pub struct MetadataResponse {
    severities: Vec<String>,
    subsystems: Vec<String>,
    labels: Vec<String>,
    nodes: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct StatsResponse {
    total_entries: u64,
    nodes: Vec<NodeStats>,
}

#[derive(Debug, Serialize)]
pub struct NodeStats {
    node: String,
    count: u64,
}

fn json_to_vec(json: &Value) -> Vec<String> {
    match json {
        Value::Array(arr) => arr
            .iter()
            .filter_map(|v| v.as_str().map(str::to_string))
            .collect(),
        _ => Vec::new(),
    }
}

fn hashset_to_sorted_vec(set: HashSet<String>) -> Vec<String> {
    let mut vec: Vec<_> = set.into_iter().collect();
    vec.sort_unstable();
    vec
}

pub async fn get_metadata(
    State(state): State<AppState>,
) -> Result<Json<MetadataResponse>, ServerError> {
    let severities: Vec<String> = Severity::all()
        .iter()
        .map(|s| s.as_str().to_string())
        .collect();

    let file_metadata_list = FileMetadata::find_all(&state.db).await?;

    let mut nodes_set = HashSet::new();
    let mut subsystems_set = HashSet::new();

    for metadata in &file_metadata_list {
        nodes_set.extend(json_to_vec(&metadata.nodes));
        subsystems_set.extend(json_to_vec(&metadata.subsystems));
    }

    let nodes = hashset_to_sorted_vec(nodes_set);
    let subsystems = hashset_to_sorted_vec(subsystems_set);
    let mut labels: Vec<String> = LABEL_NAMES.iter().map(|s| s.to_string()).collect();
    labels.sort();

    Ok(Json(MetadataResponse {
        severities,
        subsystems,
        labels,
        nodes,
    }))
}

pub async fn get_stats(State(state): State<AppState>) -> Result<Json<StatsResponse>, ServerError> {
    #[derive(Debug, FromQueryResult)]
    struct NodeCount {
        node: String,
        count: i64,
    }

    let total = NodeLogEntry::count_all(&state.db).await?;

    let node_counts = NodeLogEntry::find()
        .select_only()
        .column(Column::Node)
        .column_as(Column::Id.count(), "count")
        .group_by(Column::Node)
        .order_by_asc(Column::Node)
        .into_model::<NodeCount>()
        .all(&*state.db)
        .await?;

    let nodes = node_counts
        .into_iter()
        .map(|nc| NodeStats {
            node: nc.node,
            count: nc.count as u64,
        })
        .collect();

    Ok(Json(StatsResponse {
        total_entries: total,
        nodes,
    }))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileMetadataResponse {
    pub file_path: String,
    pub rabbitmq_versions: Vec<String>,
    pub erlang_versions: Vec<String>,
    pub tls_library: Option<String>,
    pub oldest_entry_at: Option<String>,
    pub most_recent_entry_at: Option<String>,
    pub total_lines: i64,
    pub total_entries: i64,
    pub nodes: Vec<String>,
    pub subsystems: Vec<String>,
    pub labels: Vec<String>,
    pub enabled_plugins: Vec<String>,
}

impl From<rlqt_lib::rel_db::file_metadata::Model> for FileMetadataResponse {
    fn from(model: rlqt_lib::rel_db::file_metadata::Model) -> Self {
        Self {
            file_path: model.file_path,
            rabbitmq_versions: json_to_vec(&model.rabbitmq_versions),
            erlang_versions: json_to_vec(&model.erlang_versions),
            tls_library: model.tls_library,
            oldest_entry_at: model.oldest_entry_at.map(|dt| dt.to_rfc3339()),
            most_recent_entry_at: model.most_recent_entry_at.map(|dt| dt.to_rfc3339()),
            total_lines: model.total_lines,
            total_entries: model.total_entries,
            nodes: json_to_vec(&model.nodes),
            subsystems: json_to_vec(&model.subsystems),
            labels: json_to_vec(&model.labels),
            enabled_plugins: json_to_vec(&model.enabled_plugins),
        }
    }
}

pub async fn get_file_metadata(
    State(state): State<AppState>,
) -> Result<Json<Vec<FileMetadataResponse>>, ServerError> {
    let metadata_list = FileMetadata::find_all(&state.db).await?;
    let response = metadata_list
        .into_iter()
        .map(FileMetadataResponse::from)
        .collect();
    Ok(Json(response))
}
