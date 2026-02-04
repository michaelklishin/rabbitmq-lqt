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
use bel7_cli::{ExitCode, ExitCodeProvider};
use duckdb::Error as DuckDbError;
use rabbitmq_lqt_lib::Error as LibError;
use rabbitmq_lqt_ql::errors::Error as QlError;
use std::io::{Error as IoError, ErrorKind};
use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum CommandRunError {
    #[error(transparent)]
    Library(#[from] LibError),

    #[error("DateTime parse error: {0}")]
    DateTimeParse(String),

    #[error("Query language error: {0}")]
    QueryLanguage(#[from] QlError),
}

impl ExitCodeProvider for CommandRunError {
    fn exit_code(&self) -> ExitCode {
        match self {
            CommandRunError::Library(lib_err) => match lib_err {
                LibError::Io(io_err) => match io_err.kind() {
                    ErrorKind::NotFound => ExitCode::NoInput,
                    ErrorKind::PermissionDenied => ExitCode::NoPerm,
                    _ => ExitCode::IoErr,
                },
                LibError::Database(_) | LibError::ConnectionPool(_) => ExitCode::Software,
                LibError::ParseEntry { .. }
                | LibError::ParseTimestamp(_)
                | LibError::ReadLine { .. } => ExitCode::DataErr,
                _ => ExitCode::Software,
            },
            CommandRunError::DateTimeParse(_) => ExitCode::DataErr,
            CommandRunError::QueryLanguage(_) => ExitCode::DataErr,
        }
    }
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
