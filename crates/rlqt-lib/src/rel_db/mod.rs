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
pub mod file_metadata;
pub mod node_log_entry;

pub use file_metadata::FileMetadata;
pub use node_log_entry::{NodeLogEntry, QueryContext};

use crate::entry_metadata::labels::LABEL_NAMES;
use sea_orm::sea_query::Index;
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbErr, Schema};
use std::path::Path;

pub async fn create_database(db_path: &Path) -> Result<DatabaseConnection, DbErr> {
    let db_url = format!("sqlite:{}?mode=rwc", db_path.display());
    let db = Database::connect(&db_url).await?;

    db.execute_unprepared("PRAGMA journal_mode=WAL;").await?;
    db.execute_unprepared("PRAGMA synchronous=NORMAL;").await?;
    db.execute_unprepared("PRAGMA cache_size=10000;").await?;
    db.execute_unprepared("PRAGMA temp_store=MEMORY;").await?;

    let schema = Schema::new(sea_orm::DatabaseBackend::Sqlite);

    let create_table_stmt = schema.create_table_from_entity(node_log_entry::Entity);
    db.execute(&create_table_stmt).await?;

    let create_metadata_table_stmt = schema.create_table_from_entity(file_metadata::Entity);
    db.execute(&create_metadata_table_stmt).await?;

    Ok(db)
}

pub async fn post_insertion_operations(db: &DatabaseConnection) -> Result<(), DbErr> {
    let indexes = [
        (
            "idx_node_log_entries_node",
            vec![node_log_entry::Column::Node],
        ),
        (
            "idx_node_log_entries_timestamp",
            vec![node_log_entry::Column::Timestamp],
        ),
        (
            "idx_node_log_entries_severity",
            vec![node_log_entry::Column::Severity],
        ),
        (
            "idx_node_log_entries_erlang_pid",
            vec![node_log_entry::Column::ErlangPid],
        ),
        (
            "idx_node_log_entries_subsystem_id",
            vec![node_log_entry::Column::SubsystemId],
        ),
        (
            "idx_node_timestamp",
            vec![
                node_log_entry::Column::Node,
                node_log_entry::Column::Timestamp,
            ],
        ),
        (
            "idx_timestamp_severity",
            vec![
                node_log_entry::Column::Timestamp,
                node_log_entry::Column::Severity,
            ],
        ),
        (
            "idx_timestamp_subsystem_id",
            vec![
                node_log_entry::Column::Timestamp,
                node_log_entry::Column::SubsystemId,
            ],
        ),
    ];

    let json_indices: Vec<String> = LABEL_NAMES
        .iter()
        .map(|label| {
            let index_name = label.replace(':', "_");
            format!(
                "CREATE INDEX IF NOT EXISTS idx_label_{} ON node_log_entries(json_extract(labels, '$.{}'))",
                index_name, label
            )
        })
        .collect();

    let url_indices = [
        (
            "idx_resolution_or_discussion_url_id",
            vec![node_log_entry::Column::ResolutionOrDiscussionUrlId],
        ),
        ("idx_doc_url_id", vec![node_log_entry::Column::DocUrlId]),
        (
            "idx_timestamp_doc_url_id",
            vec![
                node_log_entry::Column::Timestamp,
                node_log_entry::Column::DocUrlId,
            ],
        ),
        (
            "idx_timestamp_resolution_url_id",
            vec![
                node_log_entry::Column::Timestamp,
                node_log_entry::Column::ResolutionOrDiscussionUrlId,
            ],
        ),
        (
            "idx_severity_doc_url_id",
            vec![
                node_log_entry::Column::Severity,
                node_log_entry::Column::DocUrlId,
            ],
        ),
        (
            "idx_node_timestamp_doc_url_id",
            vec![
                node_log_entry::Column::Node,
                node_log_entry::Column::Timestamp,
                node_log_entry::Column::DocUrlId,
            ],
        ),
    ];

    for (name, cols) in indexes {
        log::debug!("Creating index {}", name);
        let mut idx = Index::create()
            .if_not_exists()
            .name(name)
            .table(node_log_entry::Entity)
            .to_owned();

        for col in cols {
            idx = idx.col(col).to_owned();
        }

        db.execute(&idx).await?;
    }

    for idx_sql in json_indices {
        if let Some(name_start) = idx_sql.find("idx_")
            && let Some(name_end) = idx_sql[name_start..].find(" ON ")
        {
            let name = &idx_sql[name_start..name_start + name_end];
            log::debug!("Creating index {}", name);
        }
        db.execute_unprepared(&idx_sql).await?;
    }

    for (name, cols) in url_indices {
        log::debug!("Creating index {}", name);
        let mut idx = Index::create()
            .if_not_exists()
            .name(name)
            .table(node_log_entry::Entity)
            .to_owned();

        for col in cols {
            idx = idx.col(col).to_owned();
        }

        db.execute(&idx).await?;
    }

    db.execute_unprepared("ANALYZE;").await?;

    Ok(())
}

/// Open an existing database
pub async fn open_database(db_path: &Path) -> Result<DatabaseConnection, DbErr> {
    let db_url = format!("sqlite:{}", db_path.display());
    Database::connect(&db_url).await
}
