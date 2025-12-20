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

//! Autocomplete suggestions for the RLQT Query Language.
//!
//! This module provides lists of valid values for UI autocompletion and validation.

use crate::ast::{Field, MatchOp};
use crate::presets::PresetName;

pub const SEVERITIES: &[&str] = &["debug", "info", "notice", "warning", "error", "critical"];

pub const SUBSYSTEMS: &[&str] = &[
    "access_control",
    "amqp10",
    "boot",
    "channels",
    "classic_queues",
    "clustering",
    "connections",
    "erlang_otp",
    "exchanges",
    "feature_flags",
    "federation",
    "limits",
    "logging",
    "maintenance_mode",
    "management",
    "metadata_store",
    "metrics",
    "mqtt",
    "oauth2",
    "peer_discovery",
    "plugins",
    "policies",
    "queues",
    "raft",
    "runtime_parameters",
    "shovels",
    "shutdown",
    "streams",
    "virtual_hosts",
];

pub const LABELS: &[&str] = &[
    "access_control",
    "amqp10",
    "auto_delete",
    "channels",
    "classic_queues",
    "clustering",
    "connections",
    "consumers",
    "cq_stores",
    "definitions",
    "delete",
    "deletion_protection",
    "deprecated_features",
    "disconnects",
    "elections",
    "erl_process_crash",
    "exceptions",
    "exchanges",
    "exclusive",
    "feature_flags",
    "federation",
    "http",
    "khepri",
    "limits",
    "maintenance_mode",
    "metrics",
    "mnesia",
    "mqtt",
    "multiline",
    "networking",
    "oauth2",
    "peer_discovery:classic",
    "plugins",
    "policies",
    "process_stops",
    "queue_federation",
    "queues",
    "quorum_queues",
    "raft",
    "runtime_parameters",
    "sessions",
    "shovels",
    "shutdown",
    "sql",
    "startup_banner",
    "stomp",
    "streams",
    "timeouts",
    "tls",
    "undefined_fn",
    "unlabelled",
    "virtual_hosts",
    "websockets",
    "worker_pool",
];

pub const FIELDS: &[FieldInfo] = &[
    FieldInfo {
        name: "severity",
        aliases: &["level"],
        description: "Log severity level",
        example_values: &["error", "warning", "info"],
    },
    FieldInfo {
        name: "subsystem",
        aliases: &[],
        description: "RabbitMQ subsystem",
        example_values: &["connections", "queues", "clustering"],
    },
    FieldInfo {
        name: "node",
        aliases: &[],
        description: "RabbitMQ node name",
        example_values: &["rabbit@host1", "rabbit@localhost"],
    },
    FieldInfo {
        name: "erlang_pid",
        aliases: &["pid"],
        description: "Erlang process identifier",
        example_values: &["<0.208.0>", "<0.123.0>"],
    },
    FieldInfo {
        name: "message",
        aliases: &["msg"],
        description: "Log message content",
        example_values: &["connection", "timeout"],
    },
    FieldInfo {
        name: "labels",
        aliases: &["label"],
        description: "Semantic labels applied to log entries",
        example_values: &["connections", "disconnects", "tls"],
    },
    FieldInfo {
        name: "timestamp",
        aliases: &["time", "ts"],
        description: "Log entry timestamp",
        example_values: &["@1h", "@24h", "2024-01-01"],
    },
    FieldInfo {
        name: "id",
        aliases: &[],
        description: "Log entry unique identifier",
        example_values: &["1", "42"],
    },
];

pub const OPERATORS: &[OperatorInfo] = &[
    OperatorInfo {
        symbol: "==",
        aliases: &["="],
        name: "equals",
        description: "Exact equality match",
    },
    OperatorInfo {
        symbol: "!=",
        aliases: &["<>"],
        name: "not equals",
        description: "Not equal comparison",
    },
    OperatorInfo {
        symbol: "<",
        aliases: &[],
        name: "less than",
        description: "Less than comparison",
    },
    OperatorInfo {
        symbol: "<=",
        aliases: &[],
        name: "less than or equal",
        description: "Less than or equal comparison",
    },
    OperatorInfo {
        symbol: ">",
        aliases: &[],
        name: "greater than",
        description: "Greater than comparison",
    },
    OperatorInfo {
        symbol: ">=",
        aliases: &[],
        name: "greater than or equal",
        description: "Greater than or equal comparison",
    },
    OperatorInfo {
        symbol: "=~",
        aliases: &[],
        name: "regex match",
        description: "Regular expression match",
    },
    OperatorInfo {
        symbol: "!~",
        aliases: &[],
        name: "regex not match",
        description: "Regular expression negative match",
    },
    OperatorInfo {
        symbol: "contains",
        aliases: &[],
        name: "contains",
        description: "Case-insensitive substring match",
    },
    OperatorInfo {
        symbol: "icontains",
        aliases: &[],
        name: "icontains",
        description: "Case-insensitive substring match (explicit)",
    },
    OperatorInfo {
        symbol: "~=",
        aliases: &[],
        name: "has label",
        description: "Check if entry has a specific label",
    },
];

pub const PIPELINE_STAGES: &[PipelineStageInfo] = &[
    PipelineStageInfo {
        name: "where",
        aliases: &[],
        syntax: "where <filter>",
        description: "Filter results with a condition",
    },
    PipelineStageInfo {
        name: "limit",
        aliases: &[],
        syntax: "limit <n>",
        description: "Limit the number of results",
    },
    PipelineStageInfo {
        name: "offset",
        aliases: &["skip"],
        syntax: "offset <n>",
        description: "Skip the first N results",
    },
    PipelineStageInfo {
        name: "head",
        aliases: &["first"],
        syntax: "head <n>",
        description: "Return only the first N results",
    },
    PipelineStageInfo {
        name: "tail",
        aliases: &["last"],
        syntax: "tail <n>",
        description: "Return only the last N results",
    },
    PipelineStageInfo {
        name: "sort",
        aliases: &["order by"],
        syntax: "sort <field> [asc|desc]",
        description: "Sort results by a field",
    },
    PipelineStageInfo {
        name: "count",
        aliases: &[],
        syntax: "count [by <field>]",
        description: "Count entries, optionally grouped by field",
    },
    PipelineStageInfo {
        name: "distinct",
        aliases: &[],
        syntax: "distinct <field>[, <field>...]",
        description: "Return distinct values for fields",
    },
    PipelineStageInfo {
        name: "project",
        aliases: &["select"],
        syntax: "project <field>[, <field>...]",
        description: "Select specific fields to return",
    },
];

pub const SPECIAL_FILTERS: &[&str] = &["has_doc_url", "has_resolution_url", "unlabelled"];

pub const DURATION_UNITS: &[DurationUnitInfo] = &[
    DurationUnitInfo {
        suffix: "s",
        name: "seconds",
        example: "@60s",
    },
    DurationUnitInfo {
        suffix: "m",
        name: "minutes",
        example: "@30m",
    },
    DurationUnitInfo {
        suffix: "h",
        name: "hours",
        example: "@24h",
    },
    DurationUnitInfo {
        suffix: "d",
        name: "days",
        example: "@7d",
    },
    DurationUnitInfo {
        suffix: "w",
        name: "weeks",
        example: "@2w",
    },
];

#[derive(Debug, Clone, Copy)]
pub struct FieldInfo {
    pub name: &'static str,
    pub aliases: &'static [&'static str],
    pub description: &'static str,
    pub example_values: &'static [&'static str],
}

#[derive(Debug, Clone, Copy)]
pub struct OperatorInfo {
    pub symbol: &'static str,
    pub aliases: &'static [&'static str],
    pub name: &'static str,
    pub description: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct PipelineStageInfo {
    pub name: &'static str,
    pub aliases: &'static [&'static str],
    pub syntax: &'static str,
    pub description: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct DurationUnitInfo {
    pub suffix: &'static str,
    pub name: &'static str,
    pub example: &'static str,
}

pub fn field_names() -> Vec<&'static str> {
    FIELDS.iter().map(|f| f.name).collect()
}

pub fn field_names_with_aliases() -> Vec<&'static str> {
    let mut names: Vec<&'static str> = Vec::new();
    for f in FIELDS {
        names.push(f.name);
        names.extend(f.aliases.iter());
    }
    names
}

pub fn preset_names() -> Vec<&'static str> {
    PresetName::all().iter().map(|p| p.as_str()).collect()
}

pub fn severity_names() -> Vec<&'static str> {
    SEVERITIES.to_vec()
}

pub fn label_names() -> Vec<&'static str> {
    LABELS.to_vec()
}

pub fn subsystem_names() -> Vec<&'static str> {
    SUBSYSTEMS.to_vec()
}

pub fn pipeline_stage_names() -> Vec<&'static str> {
    PIPELINE_STAGES.iter().map(|s| s.name).collect()
}

pub fn is_valid_severity(s: &str) -> bool {
    SEVERITIES.contains(&s.to_lowercase().as_str())
}

pub fn is_valid_label(s: &str) -> bool {
    LABELS.contains(&s)
}

pub fn is_valid_subsystem(s: &str) -> bool {
    SUBSYSTEMS.contains(&s.to_lowercase().as_str())
}

pub fn is_valid_field(s: &str) -> bool {
    let lower = s.to_lowercase();
    FIELDS
        .iter()
        .any(|f| f.name == lower || f.aliases.contains(&lower.as_str()))
}

pub fn is_valid_preset(s: &str) -> bool {
    PresetName::all()
        .iter()
        .any(|p| p.as_str() == s.to_lowercase())
}

pub fn find_similar(input: &str, candidates: &[&str], max_distance: usize) -> Vec<String> {
    let input_lower = input.to_lowercase();
    let mut matches: Vec<(String, usize)> = candidates
        .iter()
        .filter_map(|&candidate| {
            let dist = levenshtein(&input_lower, candidate);
            if dist <= max_distance {
                Some((candidate.to_string(), dist))
            } else {
                None
            }
        })
        .collect();

    matches.sort_by_key(|(_, dist)| *dist);
    matches.into_iter().map(|(s, _)| s).collect()
}

pub fn suggest_severity(input: &str) -> Vec<String> {
    find_similar(input, SEVERITIES, 3)
}

pub fn suggest_label(input: &str) -> Vec<String> {
    find_similar(input, LABELS, 4)
}

pub fn suggest_subsystem(input: &str) -> Vec<String> {
    find_similar(input, SUBSYSTEMS, 4)
}

pub fn suggest_field(input: &str) -> Vec<String> {
    let names: Vec<&str> = field_names_with_aliases();
    find_similar(input, &names, 3)
}

pub fn suggest_preset(input: &str) -> Vec<String> {
    let names: Vec<&str> = preset_names();
    find_similar(input, &names, 3)
}

pub fn filter_by_prefix<'a>(input: &str, candidates: &[&'a str]) -> Vec<&'a str> {
    if input.is_empty() {
        return candidates.to_vec();
    }
    let input_lower = input.to_lowercase();
    candidates
        .iter()
        .filter(|c| c.to_lowercase().starts_with(&input_lower))
        .copied()
        .collect()
}

pub fn complete_severity(prefix: &str) -> Vec<&'static str> {
    filter_by_prefix(prefix, SEVERITIES)
}

pub fn complete_label(prefix: &str) -> Vec<&'static str> {
    filter_by_prefix(prefix, LABELS)
}

pub fn complete_subsystem(prefix: &str) -> Vec<&'static str> {
    filter_by_prefix(prefix, SUBSYSTEMS)
}

pub fn complete_field(prefix: &str) -> Vec<&'static str> {
    let names: Vec<&str> = field_names_with_aliases();
    filter_by_prefix(prefix, &names)
}

pub fn complete_preset(prefix: &str) -> Vec<&'static str> {
    let names: Vec<&str> = preset_names();
    filter_by_prefix(prefix, &names)
}

#[allow(clippy::needless_range_loop)]
fn levenshtein(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let a_len = a_chars.len();
    let b_len = b_chars.len();

    if a_len == 0 {
        return b_len;
    }
    if b_len == 0 {
        return a_len;
    }

    let mut matrix = vec![vec![0; b_len + 1]; a_len + 1];

    for i in 0..=a_len {
        matrix[i][0] = i;
    }
    for j in 0..=b_len {
        matrix[0][j] = j;
    }

    for i in 1..=a_len {
        for j in 1..=b_len {
            let cost = if a_chars[i - 1] == b_chars[j - 1] {
                0
            } else {
                1
            };
            matrix[i][j] = (matrix[i - 1][j] + 1)
                .min(matrix[i][j - 1] + 1)
                .min(matrix[i - 1][j - 1] + cost);
        }
    }

    matrix[a_len][b_len]
}

impl Field {
    pub fn all() -> &'static [Field] {
        &[
            Field::Severity,
            Field::Subsystem,
            Field::Node,
            Field::ErlangPid,
            Field::Message,
            Field::Labels,
            Field::Timestamp,
            Field::Id,
        ]
    }

    pub fn aliases(&self) -> &'static [&'static str] {
        match self {
            Field::Severity => &["level"],
            Field::ErlangPid => &["pid"],
            Field::Message => &["msg"],
            Field::Labels => &["label"],
            Field::Timestamp => &["time", "ts"],
            _ => &[],
        }
    }
}

impl MatchOp {
    pub fn all() -> &'static [MatchOp] {
        &[
            MatchOp::Eq,
            MatchOp::NotEq,
            MatchOp::Lt,
            MatchOp::LtEq,
            MatchOp::Gt,
            MatchOp::GtEq,
            MatchOp::Regex,
            MatchOp::NotRegex,
            MatchOp::Contains,
            MatchOp::IContains,
            MatchOp::HasLabel,
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompletionContext {
    Field,
    FieldValue(Field),
    Operator,
    Label,
    Severity,
    Subsystem,
    Preset,
    PipelineStage,
    Duration,
}

#[derive(Debug, Clone)]
pub struct Suggestion {
    pub text: String,
    pub display: String,
    pub description: Option<String>,
    pub kind: SuggestionKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SuggestionKind {
    Field,
    Operator,
    Value,
    Keyword,
    Preset,
    Label,
    PipelineStage,
}

pub fn completions_for_context(context: CompletionContext) -> Vec<Suggestion> {
    match context {
        CompletionContext::Field => FIELDS
            .iter()
            .map(|f| Suggestion {
                text: f.name.to_string(),
                display: f.name.to_string(),
                description: Some(f.description.to_string()),
                kind: SuggestionKind::Field,
            })
            .collect(),
        CompletionContext::FieldValue(field) => match field {
            Field::Severity => SEVERITIES
                .iter()
                .map(|&s| Suggestion {
                    text: format!("\"{}\"", s),
                    display: s.to_string(),
                    description: None,
                    kind: SuggestionKind::Value,
                })
                .collect(),
            Field::Subsystem => SUBSYSTEMS
                .iter()
                .map(|&s| Suggestion {
                    text: format!("\"{}\"", s),
                    display: s.to_string(),
                    description: None,
                    kind: SuggestionKind::Value,
                })
                .collect(),
            Field::Labels => LABELS
                .iter()
                .map(|&s| Suggestion {
                    text: format!("\"{}\"", s),
                    display: s.to_string(),
                    description: None,
                    kind: SuggestionKind::Label,
                })
                .collect(),
            _ => vec![],
        },
        CompletionContext::Operator => OPERATORS
            .iter()
            .map(|op| Suggestion {
                text: op.symbol.to_string(),
                display: format!("{} ({})", op.symbol, op.name),
                description: Some(op.description.to_string()),
                kind: SuggestionKind::Operator,
            })
            .collect(),
        CompletionContext::Label => LABELS
            .iter()
            .map(|&s| Suggestion {
                text: s.to_string(),
                display: s.to_string(),
                description: None,
                kind: SuggestionKind::Label,
            })
            .collect(),
        CompletionContext::Severity => SEVERITIES
            .iter()
            .map(|&s| Suggestion {
                text: s.to_string(),
                display: s.to_string(),
                description: None,
                kind: SuggestionKind::Value,
            })
            .collect(),
        CompletionContext::Subsystem => SUBSYSTEMS
            .iter()
            .map(|&s| Suggestion {
                text: s.to_string(),
                display: s.to_string(),
                description: None,
                kind: SuggestionKind::Value,
            })
            .collect(),
        CompletionContext::Preset => PresetName::all()
            .iter()
            .map(|p| Suggestion {
                text: format!(":{}", p.as_str()),
                display: format!(":{}", p.as_str()),
                description: Some(p.description().to_string()),
                kind: SuggestionKind::Preset,
            })
            .collect(),
        CompletionContext::PipelineStage => PIPELINE_STAGES
            .iter()
            .map(|s| Suggestion {
                text: s.name.to_string(),
                display: s.syntax.to_string(),
                description: Some(s.description.to_string()),
                kind: SuggestionKind::PipelineStage,
            })
            .collect(),
        CompletionContext::Duration => DURATION_UNITS
            .iter()
            .map(|d| Suggestion {
                text: d.example.to_string(),
                display: format!("{} ({})", d.example, d.name),
                description: None,
                kind: SuggestionKind::Value,
            })
            .collect(),
    }
}
