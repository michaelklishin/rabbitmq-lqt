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
use crate::errors::CommandRunError;
use bel7_cli::{responsive_width, should_colorize};
use owo_colors::OwoColorize;
use rabbitmq_lqt_lib::constants::{doc_url_from_id, resolution_or_discussion_url_from_id};
use rabbitmq_lqt_lib::entry_metadata::subsystems::Subsystem;
use rabbitmq_lqt_lib::rel_db::file_metadata;
use rabbitmq_lqt_lib::rel_db::node_log_entry::Model;
use std::borrow::Cow;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use tabled::{
    Table, Tabled,
    settings::{Modify, Style, Width, object::Columns},
};

fn should_use_colors(without_colors_flag: bool) -> bool {
    if without_colors_flag {
        return false;
    }
    should_colorize()
}

const ID_COLUMN_WIDTH: usize = 6;
/// Width for node names (e.g., "rabbit@hostname.local")
const NODE_COLUMN_WIDTH: usize = 18;
/// Width for ISO 8601 timestamp with microseconds and timezone: "2025-10-27 11:23:27.566558-07:00"
const TIMESTAMP_COLUMN_WIDTH: usize = 27;
/// Width for severity levels ("debug", "info", "notice", "warning", "error", "critical")
const SEVERITY_COLUMN_WIDTH: usize = 10;
/// Width for Erlang PIDs (e.g., "<0.208.0>")
const ERLANG_PID_COLUMN_WIDTH: usize = 12;
/// Width for subsystem names (e.g., "classic_queues")
const SUBSYSTEM_COLUMN_WIDTH: usize = 16;
/// Width for GitHub issue/PR URLs
const ISSUE_OR_SCM_URL_COLUMN_WIDTH: usize = 50;
/// Width for documentation URLs
const DOC_URL_COLUMN_WIDTH: usize = 50;
/// Width consumed by table borders and separators
const TABLE_BORDERS_WIDTH: usize = 19;

const FIXED_COLUMNS_WIDTH: usize = ID_COLUMN_WIDTH
    + NODE_COLUMN_WIDTH
    + TIMESTAMP_COLUMN_WIDTH
    + SEVERITY_COLUMN_WIDTH
    + ERLANG_PID_COLUMN_WIDTH
    + SUBSYSTEM_COLUMN_WIDTH
    + ISSUE_OR_SCM_URL_COLUMN_WIDTH
    + DOC_URL_COLUMN_WIDTH
    + TABLE_BORDERS_WIDTH;

const _: () = assert!(
    FIXED_COLUMNS_WIDTH > 0,
    "FIXED_COLUMNS_WIDTH must be positive"
);
const _: () = assert!(
    FIXED_COLUMNS_WIDTH == 208,
    "FIXED_COLUMNS_WIDTH calculation has changed unexpectedly"
);

const MIN_MESSAGE_WIDTH: usize = 60;
const TERMINAL_WIDTH_UTILIZATION: f64 = 0.85;

fn colorize_node_name(node: &str, use_colors: bool) -> Cow<'_, str> {
    if !use_colors {
        return Cow::Borrowed(node);
    }

    let mut hasher = DefaultHasher::new();
    node.hash(&mut hasher);
    let hash = hasher.finish();

    let color_index = (hash as usize) % 9;
    Cow::Owned(match color_index {
        0 => node.red().to_string(),
        1 => node.green().to_string(),
        2 => node.yellow().to_string(),
        3 => node.blue().to_string(),
        4 => node.magenta().to_string(),
        5 => node.cyan().to_string(),
        6 => node.bright_red().to_string(),
        7 => node.bright_green().to_string(),
        _ => node.bright_yellow().to_string(),
    })
}

fn colorize_severity(severity: &str, use_colors: bool) -> Cow<'_, str> {
    if !use_colors {
        return Cow::Borrowed(severity);
    }

    Cow::Owned(match severity {
        "error" | "critical" => severity.red().to_string(),
        "info" => severity.yellow().to_string(),
        "warning" => severity.bright_yellow().to_string(),
        "notice" => severity.cyan().to_string(),
        "debug" => severity.bright_black().to_string(),
        _ => return Cow::Borrowed(severity),
    })
}

/// Display version of a log entry for table output
#[derive(Clone, Debug, Tabled)]
struct DisplayLogEntry {
    #[tabled(rename = "ID")]
    id: i64,

    #[tabled(rename = "Node")]
    node: String,

    #[tabled(rename = "Timestamp")]
    timestamp: String,

    #[tabled(rename = "Severity")]
    severity: String,

    #[tabled(rename = "Erlang PID")]
    erlang_pid: String,

    #[tabled(rename = "Message")]
    message: String,

    #[tabled(rename = "Subsystem")]
    subsystem: String,

    #[tabled(rename = "Labels")]
    labels: String,

    #[tabled(rename = "Doc URL")]
    doc_url: String,

    #[tabled(rename = "Resolution/Discussion URL")]
    resolution_or_discussion_url: String,
}

impl DisplayLogEntry {
    fn from_model(model: Model, use_colors: bool) -> Self {
        let labels_str = model.format_labels();
        let labels_display = if labels_str.is_empty() {
            "-".to_string()
        } else {
            labels_str
        };

        Self {
            id: model.id,
            node: colorize_node_name(&model.node, use_colors).into_owned(),
            timestamp: model.timestamp.to_rfc3339(),
            severity: colorize_severity(&model.severity, use_colors).into_owned(),
            erlang_pid: model.erlang_pid,
            message: model.message,
            subsystem: model
                .subsystem_id
                .and_then(Subsystem::from_id)
                .map(|s| s.to_string())
                .unwrap_or_else(|| "-".into()),
            labels: labels_display,
            doc_url: model
                .doc_url_id
                .and_then(doc_url_from_id)
                .unwrap_or("-")
                .to_string(),
            resolution_or_discussion_url: model
                .resolution_or_discussion_url_id
                .and_then(resolution_or_discussion_url_from_id)
                .unwrap_or("-")
                .to_string(),
        }
    }
}

/// Display log entries as a formatted table
pub fn display_log_entries(
    entries: Vec<Model>,
    without_colors: bool,
) -> Result<(), CommandRunError> {
    if entries.is_empty() {
        println!("No matching log entries found.");
        return Ok(());
    }

    let use_colors = should_use_colors(without_colors);

    let display_entries: Vec<DisplayLogEntry> = entries
        .into_iter()
        .map(|model| DisplayLogEntry::from_model(model, use_colors))
        .collect();

    let mut table = Table::new(&display_entries);
    table.with(Style::modern());

    let target_width = responsive_width(TERMINAL_WIDTH_UTILIZATION);

    let message_width = target_width
        .saturating_sub(FIXED_COLUMNS_WIDTH)
        .max(MIN_MESSAGE_WIDTH);

    table
        .with(Modify::new(Columns::new(5..6)).with(Width::wrap(message_width)))
        .with(Width::increase(target_width));

    println!("{}", table);

    Ok(())
}

pub fn display_file_metadata(
    metadata_entries: Vec<file_metadata::Model>,
    _without_colors: bool,
) -> Result<(), CommandRunError> {
    if metadata_entries.is_empty() {
        println!("No file metadata found in database.");
        return Ok(());
    }

    for (i, metadata) in metadata_entries.iter().enumerate() {
        if i > 0 {
            println!();
        }

        println!("File: {}", metadata.file_path);

        if !metadata.rabbitmq_versions.is_empty() {
            println!(
                "  RabbitMQ Versions: {}",
                metadata.rabbitmq_versions.join(", ")
            );
        }

        if !metadata.erlang_versions.is_empty() {
            println!("  Erlang Versions: {}", metadata.erlang_versions.join(", "));
        }

        if let Some(tls_lib) = &metadata.tls_library {
            println!("  TLS Library: {}", tls_lib);
        }

        if let Some(oldest) = &metadata.oldest_entry_at {
            println!("  Oldest Entry: {}", oldest.format("%Y-%m-%d %H:%M:%S UTC"));
        }

        if let Some(newest) = &metadata.most_recent_entry_at {
            println!(
                "  Most Recent Entry: {}",
                newest.format("%Y-%m-%d %H:%M:%S UTC")
            );
        }

        println!("  Total Lines: {}", metadata.total_lines);
        println!("  Total Entries: {}", metadata.total_entries);

        if !metadata.nodes.is_empty() {
            println!("  Nodes:");
            for node in &metadata.nodes {
                println!("    * {}", node);
            }
        }

        if !metadata.subsystems.is_empty() {
            println!("  Subsystems:");
            for subsystem in &metadata.subsystems {
                println!("    * {}", subsystem);
            }
        }

        if !metadata.labels.is_empty() {
            println!("  Labels:");
            for label in &metadata.labels {
                println!("    * {}", label);
            }
        }

        if !metadata.enabled_plugins.is_empty() {
            println!("  Enabled Plugins ({}):", metadata.enabled_plugins.len());
            for plugin in &metadata.enabled_plugins {
                println!("    * {}", plugin);
            }
        }
    }

    Ok(())
}
