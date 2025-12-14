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

//! Abstract Syntax Tree definitions for the RLQT Query Language.

use crate::presets::PresetName;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Query {
    pub time_range: Option<Duration>,
    pub selector: Option<Selector>,
    pub filter: Option<FilterExpr>,
    pub pipeline: Vec<PipelineStage>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Selector {
    pub matchers: Vec<LabelMatcher>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LabelMatcher {
    pub field: Field,
    pub op: MatchOp,
    pub value: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Field {
    Severity,
    Subsystem,
    Node,
    ErlangPid,
    Message,
    Labels,
    Timestamp,
    Id,
}

impl Field {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Severity => "severity",
            Self::Subsystem => "subsystem",
            Self::Node => "node",
            Self::ErlangPid => "erlang_pid",
            Self::Message => "message",
            Self::Labels => "labels",
            Self::Timestamp => "timestamp",
            Self::Id => "id",
        }
    }

    pub fn sql_column(&self) -> &'static str {
        match self {
            Self::Severity => "severity",
            Self::Subsystem => "subsystem_id",
            Self::Node => "node",
            Self::ErlangPid => "erlang_pid",
            Self::Message => "message",
            Self::Labels => "labels",
            Self::Timestamp => "timestamp",
            Self::Id => "id",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchOp {
    Eq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    Regex,
    NotRegex,
    Contains,
    IContains,
    HasLabel,
}

impl MatchOp {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Eq => "==",
            Self::NotEq => "!=",
            Self::Lt => "<",
            Self::LtEq => "<=",
            Self::Gt => ">",
            Self::GtEq => ">=",
            Self::Regex => "=~",
            Self::NotRegex => "!~",
            Self::Contains => "contains",
            Self::IContains => "icontains",
            Self::HasLabel => "~=",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Regex(String),
    Integer(i64),
    Timestamp(DateTime<Utc>),
    RelativeTime(Duration),
    LabelList(Vec<String>),
    Boolean(bool),
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Duration {
    pub value: i64,
    pub unit: DurationUnit,
}

impl Duration {
    pub fn to_chrono_duration(&self) -> chrono::Duration {
        match self.unit {
            DurationUnit::Seconds => chrono::Duration::seconds(self.value),
            DurationUnit::Minutes => chrono::Duration::minutes(self.value),
            DurationUnit::Hours => chrono::Duration::hours(self.value),
            DurationUnit::Days => chrono::Duration::days(self.value),
            DurationUnit::Weeks => chrono::Duration::weeks(self.value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurationUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Weeks,
}

impl DurationUnit {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Seconds => "s",
            Self::Minutes => "m",
            Self::Hours => "h",
            Self::Days => "d",
            Self::Weeks => "w",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FilterExpr {
    Comparison(Box<LabelMatcher>),
    And(Box<FilterExpr>, Box<FilterExpr>),
    Or(Box<FilterExpr>, Box<FilterExpr>),
    Not(Box<FilterExpr>),
    LabelAny(Vec<String>),
    LabelAll(Vec<String>),
    HasDocUrl,
    HasResolutionUrl,
    Unlabelled,
    Preset(PresetName),
    Grouped(Box<FilterExpr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum PipelineStage {
    Where(FilterExpr),
    Limit(u64),
    Offset(u64),
    Head(u64),
    Tail(u64),
    Sort(SortSpec),
    Project(Vec<Field>),
    CountBy(Option<Field>),
    Distinct(Vec<Field>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SortSpec {
    pub field: Field,
    pub direction: SortDirection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SortDirection {
    #[default]
    Asc,
    Desc,
}

impl SortDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Asc => "asc",
            Self::Desc => "desc",
        }
    }
}
