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

//! Built-in query presets for common log analysis patterns.
//!
//! Presets are defined in terms of QL queries.

use crate::ast::FilterExpr;
use crate::errors::ParseError;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PresetName {
    Errors,
    Crashes,
    ErrorsOrCrashes,
    Disconnects,
    TlsIssues,
    AccessDenied,
    Timeouts,
    RaftAndQuorumQueues,
}

impl PresetName {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Errors => "errors",
            Self::Crashes => "crashes",
            Self::ErrorsOrCrashes => "errors_or_crashes",
            Self::Disconnects => "disconnects",
            Self::TlsIssues => "tls_issues",
            Self::AccessDenied => "access_denied",
            Self::Timeouts => "timeouts",
            Self::RaftAndQuorumQueues => "raft_and_quorum_queues",
        }
    }

    pub fn query_string(&self) -> &'static str {
        match self {
            Self::Errors => r#"severity == "error""#,
            Self::Crashes => r#"labels any ["erl_process_crash", "exceptions", "undefined_fn"]"#,
            Self::ErrorsOrCrashes => {
                r#"severity == "error" or labels any ["erl_process_crash", "exceptions", "undefined_fn"]"#
            }
            Self::Disconnects => r#"labels any ["disconnects", "connections"]"#,
            Self::TlsIssues => r#"labels any ["tls"]"#,
            Self::AccessDenied => r#"labels any ["access_control"]"#,
            Self::Timeouts => r#"labels any ["timeouts"]"#,
            Self::RaftAndQuorumQueues => {
                r#"labels any ["raft", "quorum_queues", "elections"] or subsystem any ["raft", "metadata_store"]"#
            }
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Errors => "Log entries with severity level 'error'",
            Self::Crashes => "Erlang process crashes, exceptions, and undefined function calls",
            Self::ErrorsOrCrashes => "Errors or crashes (combination of :errors and :crashes)",
            Self::Disconnects => "Client disconnections and connection-related events",
            Self::TlsIssues => "TLS-related issues",
            Self::AccessDenied => "Access control failures and permission denied events",
            Self::Timeouts => "Timeout-related events",
            Self::RaftAndQuorumQueues => {
                "Raft consensus, quorum queues, elections, and metadata store events"
            }
        }
    }

    pub fn to_filter_expr(&self) -> FilterExpr {
        crate::parser::parse_filter_only(self.query_string())
            .expect("preset query strings must be valid")
    }

    pub fn all() -> &'static [Self] {
        &[
            Self::Errors,
            Self::Crashes,
            Self::ErrorsOrCrashes,
            Self::Disconnects,
            Self::TlsIssues,
            Self::AccessDenied,
            Self::Timeouts,
            Self::RaftAndQuorumQueues,
        ]
    }
}

impl FromStr for PresetName {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "errors" => Ok(Self::Errors),
            "crashes" => Ok(Self::Crashes),
            "errors_or_crashes" => Ok(Self::ErrorsOrCrashes),
            "disconnects" => Ok(Self::Disconnects),
            "tls_issues" | "tls" => Ok(Self::TlsIssues),
            "access_denied" | "access_control" => Ok(Self::AccessDenied),
            "timeouts" => Ok(Self::Timeouts),
            "raft_and_quorum_queues" | "raft" | "quorum_queues" => Ok(Self::RaftAndQuorumQueues),
            _ => Err(ParseError::unknown_preset(s)),
        }
    }
}
