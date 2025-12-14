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

use crate::rel_db::DatabaseConnection;
use chrono::{DateTime, Utc};
use duckdb::types::Value;
use duckdb::{Error as DuckDbError, params};
use serde::{Deserialize, Serialize};
use std::io::Error as IoError;

pub struct FileMetadata;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Model {
    pub file_path: String,
    pub rabbitmq_versions: Vec<String>,
    pub erlang_versions: Vec<String>,
    pub tls_library: Option<String>,
    pub oldest_entry_at: Option<DateTime<Utc>>,
    pub most_recent_entry_at: Option<DateTime<Utc>>,
    pub total_lines: i64,
    pub total_entries: i64,
    pub nodes: Vec<String>,
    pub subsystems: Vec<String>,
    pub labels: Vec<String>,
    pub enabled_plugins: Vec<String>,
}

fn json_to_vec(json_str: &str) -> Vec<String> {
    serde_json::from_str(json_str).unwrap_or_default()
}

fn vec_to_json(vec: &[String]) -> String {
    serde_json::to_string(vec).unwrap_or_else(|_| "[]".to_string())
}

impl FileMetadata {
    pub fn find_all(db: &DatabaseConnection) -> Result<Vec<Model>, DuckDbError> {
        let conn = db.get().map_err(|e| {
            DuckDbError::ToSqlConversionFailure(Box::new(IoError::other(e.to_string())))
        })?;

        let mut stmt = conn.prepare(
            "SELECT file_path, rabbitmq_versions, erlang_versions, tls_library, oldest_entry_at, most_recent_entry_at, total_lines, total_entries, nodes, subsystems, labels, enabled_plugins
             FROM file_metadata",
        )?;

        let rows = stmt.query_map([], |row| {
            let oldest_entry_at: Option<i64> = row.get(4)?;
            let most_recent_entry_at: Option<i64> = row.get(5)?;

            let rabbitmq_versions_json: String = row.get(1)?;
            let erlang_versions_json: String = row.get(2)?;
            let nodes_json: String = row.get(8)?;
            let subsystems_json: String = row.get(9)?;
            let labels_json: String = row.get(10)?;
            let enabled_plugins_json: String = row.get(11)?;

            Ok(Model {
                file_path: row.get(0)?,
                rabbitmq_versions: json_to_vec(&rabbitmq_versions_json),
                erlang_versions: json_to_vec(&erlang_versions_json),
                tls_library: row.get(3)?,
                oldest_entry_at: oldest_entry_at.and_then(DateTime::from_timestamp_micros),
                most_recent_entry_at: most_recent_entry_at
                    .and_then(DateTime::from_timestamp_micros),
                total_lines: row.get(6)?,
                total_entries: row.get(7)?,
                nodes: json_to_vec(&nodes_json),
                subsystems: json_to_vec(&subsystems_json),
                labels: json_to_vec(&labels_json),
                enabled_plugins: json_to_vec(&enabled_plugins_json),
            })
        })?;

        rows.collect()
    }

    pub fn insert_metadata(db: &DatabaseConnection, metadata: Model) -> Result<(), DuckDbError> {
        let conn = db.get().map_err(|e| {
            DuckDbError::ToSqlConversionFailure(Box::new(IoError::other(e.to_string())))
        })?;

        let oldest_entry_at = metadata.oldest_entry_at.map(|dt| {
            Value::Timestamp(duckdb::types::TimeUnit::Microsecond, dt.timestamp_micros())
        });
        let most_recent_entry_at = metadata.most_recent_entry_at.map(|dt| {
            Value::Timestamp(duckdb::types::TimeUnit::Microsecond, dt.timestamp_micros())
        });

        conn.execute(
            "INSERT INTO file_metadata (file_path, rabbitmq_versions, erlang_versions, tls_library, oldest_entry_at, most_recent_entry_at, total_lines, total_entries, nodes, subsystems, labels, enabled_plugins)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                metadata.file_path,
                vec_to_json(&metadata.rabbitmq_versions),
                vec_to_json(&metadata.erlang_versions),
                metadata.tls_library,
                oldest_entry_at,
                most_recent_entry_at,
                metadata.total_lines,
                metadata.total_entries,
                vec_to_json(&metadata.nodes),
                vec_to_json(&metadata.subsystems),
                vec_to_json(&metadata.labels),
                vec_to_json(&metadata.enabled_plugins),
            ],
        )?;

        Ok(())
    }

    pub fn upsert_metadata(db: &DatabaseConnection, metadata: Model) -> Result<(), DuckDbError> {
        let conn = db.get().map_err(|e| {
            DuckDbError::ToSqlConversionFailure(Box::new(IoError::other(e.to_string())))
        })?;

        let oldest_entry_at = metadata.oldest_entry_at.map(|dt| {
            Value::Timestamp(duckdb::types::TimeUnit::Microsecond, dt.timestamp_micros())
        });
        let most_recent_entry_at = metadata.most_recent_entry_at.map(|dt| {
            Value::Timestamp(duckdb::types::TimeUnit::Microsecond, dt.timestamp_micros())
        });

        conn.execute(
            "INSERT OR REPLACE INTO file_metadata (file_path, rabbitmq_versions, erlang_versions, tls_library, oldest_entry_at, most_recent_entry_at, total_lines, total_entries, nodes, subsystems, labels, enabled_plugins)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                metadata.file_path,
                vec_to_json(&metadata.rabbitmq_versions),
                vec_to_json(&metadata.erlang_versions),
                metadata.tls_library,
                oldest_entry_at,
                most_recent_entry_at,
                metadata.total_lines,
                metadata.total_entries,
                vec_to_json(&metadata.nodes),
                vec_to_json(&metadata.subsystems),
                vec_to_json(&metadata.labels),
                vec_to_json(&metadata.enabled_plugins),
            ],
        )?;

        Ok(())
    }
}
