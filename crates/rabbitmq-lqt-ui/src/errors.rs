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

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Database error: {0}")]
    Database(#[from] duckdb::Error),

    #[error("Library error: {0}")]
    Library(#[from] rabbitmq_lqt_lib::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Invalid datetime format: {0}")]
    DateTimeParse(String),

    #[error("Invalid preset: {0}")]
    InvalidPreset(String),

    #[error("Invalid query: {0}")]
    InvalidQuery(String),
}
