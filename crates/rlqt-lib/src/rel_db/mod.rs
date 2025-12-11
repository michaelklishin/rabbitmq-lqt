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
pub mod presets;

pub use file_metadata::FileMetadata;
pub use node_log_entry::{NodeLogEntry, QueryContext};
pub use presets::QueryPreset;

use duckdb::Error as DuckDbError;
use r2d2::{Pool, PooledConnection};
use r2d2_duckdb::DuckDbConnectionManager;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

mod r2d2_duckdb {
    use duckdb::{Connection, Error as DuckDbError};
    use r2d2::ManageConnection;
    use std::path::PathBuf;

    pub struct DuckDbConnectionManager {
        path: PathBuf,
    }

    impl DuckDbConnectionManager {
        pub fn file(path: PathBuf) -> Self {
            Self { path }
        }
    }

    impl ManageConnection for DuckDbConnectionManager {
        type Connection = Connection;
        type Error = DuckDbError;

        fn connect(&self) -> Result<Self::Connection, Self::Error> {
            Connection::open(&self.path)
        }

        fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
            conn.execute_batch("SELECT 1")?;
            Ok(())
        }

        fn has_broken(&self, _conn: &mut Self::Connection) -> bool {
            false
        }
    }
}

pub type DbPool = Pool<DuckDbConnectionManager>;
pub type DbConnection = PooledConnection<DuckDbConnectionManager>;

#[derive(Clone)]
pub struct DatabaseConnection {
    pool: Arc<DbPool>,
}

impl DatabaseConnection {
    pub fn get(&self) -> Result<DbConnection, r2d2::Error> {
        self.pool.get()
    }
}

pub fn create_database(db_path: &Path) -> Result<DatabaseConnection, DuckDbError> {
    create_database_with_options(db_path, false)
}

pub fn create_database_for_bulk_import(db_path: &Path) -> Result<DatabaseConnection, DuckDbError> {
    create_database_with_options(db_path, true)
}

fn create_database_with_options(
    db_path: &Path,
    _fast_import: bool,
) -> Result<DatabaseConnection, DuckDbError> {
    let manager = DuckDbConnectionManager::file(db_path.to_path_buf());
    let pool = Pool::builder()
        .max_size(4)
        .connection_timeout(Duration::from_secs(1))
        .build(manager)
        .map_err(|e| {
            DuckDbError::ToSqlConversionFailure(Box::new(std::io::Error::other(e.to_string())))
        })?;

    let conn = pool.get().map_err(|e| {
        DuckDbError::ToSqlConversionFailure(Box::new(std::io::Error::other(e.to_string())))
    })?;

    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS node_log_entries (
            id BIGINT PRIMARY KEY,
            node VARCHAR NOT NULL,
            timestamp TIMESTAMPTZ NOT NULL,
            severity VARCHAR NOT NULL,
            erlang_pid VARCHAR NOT NULL,
            subsystem_id SMALLINT,
            message VARCHAR NOT NULL,
            labels BIGINT NOT NULL DEFAULT 0,
            resolution_or_discussion_url_id SMALLINT,
            doc_url_id SMALLINT
        );

        CREATE TABLE IF NOT EXISTS file_metadata (
            file_path VARCHAR PRIMARY KEY,
            rabbitmq_versions VARCHAR NOT NULL DEFAULT '[]',
            erlang_versions VARCHAR NOT NULL DEFAULT '[]',
            tls_library VARCHAR,
            oldest_entry_at TIMESTAMPTZ,
            most_recent_entry_at TIMESTAMPTZ,
            total_lines BIGINT NOT NULL,
            total_entries BIGINT NOT NULL,
            nodes VARCHAR NOT NULL DEFAULT '[]',
            subsystems VARCHAR NOT NULL DEFAULT '[]',
            labels VARCHAR NOT NULL DEFAULT '[]',
            enabled_plugins VARCHAR NOT NULL DEFAULT '[]'
        );
        ",
    )?;

    Ok(DatabaseConnection {
        pool: Arc::new(pool),
    })
}

pub fn finalize_bulk_import(_db: &DatabaseConnection) -> Result<(), DuckDbError> {
    Ok(())
}

pub fn post_insertion_operations(db: &DatabaseConnection) -> Result<(), DuckDbError> {
    let conn = db.get().map_err(|e| {
        DuckDbError::ToSqlConversionFailure(Box::new(std::io::Error::other(e.to_string())))
    })?;

    conn.execute_batch(
        "
        CREATE INDEX IF NOT EXISTS idx_node_log_entries_node ON node_log_entries(node);
        CREATE INDEX IF NOT EXISTS idx_node_log_entries_timestamp ON node_log_entries(timestamp);
        CREATE INDEX IF NOT EXISTS idx_node_log_entries_severity ON node_log_entries(severity);
        CREATE INDEX IF NOT EXISTS idx_node_log_entries_erlang_pid ON node_log_entries(erlang_pid);
        CREATE INDEX IF NOT EXISTS idx_node_log_entries_subsystem_id ON node_log_entries(subsystem_id);
        CREATE INDEX IF NOT EXISTS idx_node_timestamp ON node_log_entries(node, timestamp);
        CREATE INDEX IF NOT EXISTS idx_timestamp_severity ON node_log_entries(timestamp, severity);
        CREATE INDEX IF NOT EXISTS idx_timestamp_subsystem_id ON node_log_entries(timestamp, subsystem_id);
        CREATE INDEX IF NOT EXISTS idx_resolution_or_discussion_url_id ON node_log_entries(resolution_or_discussion_url_id);
        CREATE INDEX IF NOT EXISTS idx_doc_url_id ON node_log_entries(doc_url_id);
        CREATE INDEX IF NOT EXISTS idx_timestamp_doc_url_id ON node_log_entries(timestamp, doc_url_id);
        CREATE INDEX IF NOT EXISTS idx_timestamp_resolution_url_id ON node_log_entries(timestamp, resolution_or_discussion_url_id);
        CREATE INDEX IF NOT EXISTS idx_severity_doc_url_id ON node_log_entries(severity, doc_url_id);
        CREATE INDEX IF NOT EXISTS idx_node_timestamp_doc_url_id ON node_log_entries(node, timestamp, doc_url_id);
        ",
    )?;

    Ok(())
}

pub fn open_database(db_path: &Path) -> Result<DatabaseConnection, DuckDbError> {
    let manager = DuckDbConnectionManager::file(db_path.to_path_buf());
    let pool = Pool::builder()
        .max_size(4)
        .connection_timeout(Duration::from_secs(1))
        .build(manager)
        .map_err(|e| {
            DuckDbError::ToSqlConversionFailure(Box::new(std::io::Error::other(e.to_string())))
        })?;

    Ok(DatabaseConnection {
        pool: Arc::new(pool),
    })
}
