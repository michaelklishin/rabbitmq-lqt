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

use crate::entry_metadata::labels::{
    LABEL_ACCESS_CONTROL, LABEL_AUTO_DELETE, LABEL_CHANNEL_EXCEPTIONS, LABEL_CONNECTIONS,
    LABEL_CQ_STORES, LABEL_DELETE, LABEL_DISCONNECTS, LABEL_ELECTIONS, LABEL_ERL_PROCESS_CRASH,
    LABEL_EXCLUSIVE, LABEL_FEDERATION, LABEL_PROCESS_STOPS, LABEL_QUEUE_FEDERATION, LABEL_QUEUES,
    LABEL_RAFT, LABEL_SHOVELS, LABEL_UNDEFINED_FN, LABEL_VIRTUAL_HOSTS, LogEntryLabels,
};
use crate::entry_metadata::subsystems::Subsystem;
use crate::parser::ParsedLogEntry;
use crate::rel_db::file_metadata;
use chrono::{DateTime, Utc};
use nom::{
    IResult, Parser,
    bytes::complete::{tag, take_until},
    character::complete::multispace0,
};
use sea_orm::prelude::Json;
use serde_json::Value;
use std::collections::HashSet;

#[derive(Debug, Clone, Default)]
pub struct FileMetadataContext {
    pub rabbitmq_version: Option<String>,
    pub erlang_version: Option<String>,
    pub tls_library: Option<String>,
    pub oldest_entry_at: Option<DateTime<Utc>>,
    pub most_recent_entry_at: Option<DateTime<Utc>>,
    pub total_lines: i64,
    pub total_entries: i64,
    pub nodes: HashSet<String>,
    pub subsystems: HashSet<String>,
    pub labels: HashSet<String>,
    pub enabled_plugins: HashSet<String>,
}

impl FileMetadataContext {
    pub fn to_model(&self, file_path: String) -> file_metadata::Model {
        file_metadata::Model {
            file_path,
            rabbitmq_version: self.rabbitmq_version.clone(),
            erlang_version: self.erlang_version.clone(),
            tls_library: self.tls_library.clone(),
            oldest_entry_at: self.oldest_entry_at,
            most_recent_entry_at: self.most_recent_entry_at,
            total_lines: self.total_lines,
            total_entries: self.total_entries,
            nodes: json_from_hashset(&self.nodes),
            subsystems: json_from_hashset(&self.subsystems),
            labels: json_from_hashset(&self.labels),
            enabled_plugins: json_from_hashset(&self.enabled_plugins),
        }
    }

    pub fn aggregate_from_entries(&mut self, entries: &[ParsedLogEntry]) {
        const LABEL_FLAGS: [(LogEntryLabels, &str); 18] = [
            (LogEntryLabels::ERL_PROCESS_CRASH, LABEL_ERL_PROCESS_CRASH),
            (LogEntryLabels::UNDEFINED_FN, LABEL_UNDEFINED_FN),
            (LogEntryLabels::PROCESS_STOPS, LABEL_PROCESS_STOPS),
            (LogEntryLabels::RAFT, LABEL_RAFT),
            (LogEntryLabels::ELECTIONS, LABEL_ELECTIONS),
            (LogEntryLabels::QUEUES, LABEL_QUEUES),
            (LogEntryLabels::AUTO_DELETE, LABEL_AUTO_DELETE),
            (LogEntryLabels::EXCLUSIVE, LABEL_EXCLUSIVE),
            (LogEntryLabels::CHANNEL_EXCEPTIONS, LABEL_CHANNEL_EXCEPTIONS),
            (LogEntryLabels::DELETE, LABEL_DELETE),
            (LogEntryLabels::QUEUE_FEDERATION, LABEL_QUEUE_FEDERATION),
            (LogEntryLabels::VIRTUAL_HOSTS, LABEL_VIRTUAL_HOSTS),
            (LogEntryLabels::CONNECTIONS, LABEL_CONNECTIONS),
            (LogEntryLabels::ACCESS_CONTROL, LABEL_ACCESS_CONTROL),
            (LogEntryLabels::SHOVELS, LABEL_SHOVELS),
            (LogEntryLabels::CQ_STORES, LABEL_CQ_STORES),
            (LogEntryLabels::DISCONNECTS, LABEL_DISCONNECTS),
            (LogEntryLabels::FEDERATION, LABEL_FEDERATION),
        ];

        for entry in entries {
            if let Some(subsystem_id) = entry.subsystem_id
                && let Some(subsystem) = Subsystem::from_id(subsystem_id)
            {
                self.subsystems.insert(subsystem.to_string());
            }

            for (flag, name) in &LABEL_FLAGS {
                if entry.labels.contains(*flag) {
                    self.labels.insert(name.to_string());
                }
            }
        }
    }
}

fn json_from_hashset(set: &HashSet<String>) -> Json {
    let mut vec: Vec<String> = set.iter().cloned().collect();
    vec.sort();
    Json::Array(vec.into_iter().map(Value::String).collect())
}

fn parse_rabbitmq_version(input: &str) -> IResult<&str, &str> {
    let (rest, (_, version, _)) = (
        tag("Starting RabbitMQ "),
        take_until(" on Erlang"),
        multispace0,
    )
        .parse(input)?;
    Ok((rest, version))
}

fn parse_erlang_version(input: &str) -> IResult<&str, &str> {
    let (rest, _) = tag("on Erlang ").parse(input)?;
    let remaining = rest.trim();
    let version = if let Some(newline_pos) = remaining.find('\n') {
        &remaining[..newline_pos]
    } else {
        remaining
    };
    Ok(("", version.trim()))
}

fn parse_startup_banner_line(line: &str) -> Option<(Option<String>, Option<String>)> {
    if !line.contains("Starting RabbitMQ") {
        return None;
    }

    let trimmed_line = line.trim_start();
    let (rmq_ver, erl_ver) = match parse_rabbitmq_version(trimmed_line) {
        Ok((rest, rmq_version)) => {
            let erl_version = parse_erlang_version(rest).ok().map(|(_, v)| v.to_string());
            (Some(rmq_version.to_string()), erl_version)
        }
        Err(_) => (None, None),
    };

    Some((rmq_ver, erl_ver))
}

fn parse_tls_library_line(line: &str) -> Option<String> {
    let pos = line.find("TLS/DTLS")?;
    let after_tls = &line[pos + "TLS/DTLS".len()..];
    let colon_pos = after_tls.find(':')?;
    let lib_part = after_tls[colon_pos + 1..].trim();
    let lib_name = lib_part.split_whitespace().next()?;
    Some(lib_name.to_string())
}

fn parse_plugin_list_start(line: &str) -> Option<usize> {
    if !line.contains("Server startup complete;") || !line.contains("plugins started") {
        return None;
    }

    for part in line.split_whitespace() {
        if part.chars().all(|c| c.is_ascii_digit())
            && let Ok(count) = part.parse::<usize>()
        {
            return Some(count);
        }
    }

    None
}

fn parse_plugin_name(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if !trimmed.starts_with('*') {
        return None;
    }

    let after_star = trimmed[1..].trim();
    let name = after_star.split_whitespace().next()?;
    Some(name.to_string())
}

pub fn extract_file_metadata(
    entries: &[ParsedLogEntry],
    file_path: String,
    node: &str,
    total_lines: i64,
) -> file_metadata::Model {
    let mut ctx = FileMetadataContext::default();
    ctx.nodes.insert(node.to_string());

    ctx.total_lines = total_lines;
    ctx.total_entries = entries.len() as i64;

    if !entries.is_empty() {
        ctx.oldest_entry_at = Some(entries[0].timestamp);
        ctx.most_recent_entry_at = entries.last().map(|e| e.timestamp);
    }

    for entry in entries {
        if let Some((rmq_ver, erl_ver)) = parse_startup_banner_line(&entry.message) {
            ctx.rabbitmq_version = rmq_ver;
            ctx.erlang_version = erl_ver;
        }

        if let Some(tls_lib) = parse_tls_library_line(&entry.message) {
            ctx.tls_library = Some(tls_lib);
        }

        if let Some(expected_count) = parse_plugin_list_start(&entry.message) {
            let mut actual_count = 0;
            for line in entry.message.lines().skip(1) {
                if let Some(plugin_name) = parse_plugin_name(line) {
                    ctx.enabled_plugins.insert(plugin_name);
                    actual_count += 1;
                }
            }
            if actual_count != expected_count {
                log::warn!(
                    "Plugin count mismatch: expected {}, found {}",
                    expected_count,
                    actual_count
                );
            }
        }
    }

    ctx.aggregate_from_entries(entries);
    ctx.to_model(file_path)
}
