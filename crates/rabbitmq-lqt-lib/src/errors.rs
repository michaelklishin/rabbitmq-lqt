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
use duckdb::Error as DuckDbError;
use std::io::Error as IoError;
use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("Database error: {0}")]
    Database(#[from] DuckDbError),

    #[error("IO error: {0}")]
    Io(#[from] IoError),

    #[error("Database connection pool error: {0}")]
    ConnectionPool(String),

    #[error("Failed to parse log entry at line {line}: {reason}")]
    ParseEntry { line: usize, reason: String },

    #[error("Failed to parse timestamp: {0}")]
    ParseTimestamp(String),

    #[error("Failed to read log file at line {line}: {source}")]
    ReadLine {
        line: usize,
        #[source]
        source: IoError,
    },
}
