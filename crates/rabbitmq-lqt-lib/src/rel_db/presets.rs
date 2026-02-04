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
use crate::entry_metadata::labels::LogEntryLabels;
use crate::rel_db::node_log_entry::QueryContext;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryPreset {
    ErrorsOrCrashes,
}

impl fmt::Display for QueryPreset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QueryPreset::ErrorsOrCrashes => write!(f, "errors_or_crashes"),
        }
    }
}

impl FromStr for QueryPreset {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "errors_or_crashes" => Ok(QueryPreset::ErrorsOrCrashes),
            _ => Err(format!("Unknown preset: {}", s)),
        }
    }
}

impl QueryPreset {
    pub fn severity(&self) -> Option<&'static str> {
        match self {
            QueryPreset::ErrorsOrCrashes => Some("error"),
        }
    }

    pub fn labels(&self) -> LogEntryLabels {
        match self {
            QueryPreset::ErrorsOrCrashes => {
                LogEntryLabels::ERL_PROCESS_CRASH | LogEntryLabels::EXCEPTIONS
            }
        }
    }
}

impl From<QueryPreset> for QueryContext {
    fn from(preset: QueryPreset) -> Self {
        QueryContext {
            preset: Some(preset),
            ..Default::default()
        }
    }
}
