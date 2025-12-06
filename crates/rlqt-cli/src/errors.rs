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
use rlqt_lib::Error as LibError;
use std::io::Error as IoError;
use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum CommandRunError {
    #[error(transparent)]
    Library(#[from] LibError),

    #[error("DateTime parse error: {0}")]
    DateTimeParse(String),
}

impl From<IoError> for CommandRunError {
    fn from(err: IoError) -> Self {
        CommandRunError::Library(LibError::Io(err))
    }
}

impl From<DuckDbError> for CommandRunError {
    fn from(err: DuckDbError) -> Self {
        CommandRunError::Library(LibError::Database(err))
    }
}
