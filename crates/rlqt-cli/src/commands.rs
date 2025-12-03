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
use crate::core::Result;
use crate::errors::CommandRunError;
use crate::output;
use chrono::{DateTime, Utc};
use clap::ArgMatches;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use rlqt_lib::file_set_metadata::extract_file_metadata;
use rlqt_lib::rel_db::FileMetadata;
use rlqt_lib::{
    NodeLogEntry, QueryContext, create_database, open_database, parse_log_file,
    post_insertion_operations,
};
use rlqt_obfuscation::{LogObfuscator, ObfuscationStats};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Error as IoError, ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use sysexits::ExitCode;
use tabled::{Table, Tabled, settings::Style};

const ERR_MSG_FILE_NOT_FOUND_HELP: &str = "Make sure:\n\
  • The file path(s) are correct\n\
  • You have read permissions for the file(s)\n\
  • The file(s) exist and are not directories";

const ERR_MSG_PARENT_DIR_HELP: &str = "Create the directory first with:\n\
  mkdir -p";

pub async fn handle_parse_command(args: &ArgMatches) -> ExitCode {
    match parse_logs(args).await {
        Ok(_) => {
            if !args.get_flag("silent") {
                println!("Done");
            }
            ExitCode::Ok
        }
        Err(e) => {
            log::error!("Failed to parse logs: {}", e);
            ExitCode::Software
        }
    }
}

pub async fn handle_query_command(args: &ArgMatches) -> ExitCode {
    match query_logs(args).await {
        Ok(_) => ExitCode::Ok,
        Err(e) => {
            log::error!("Failed to query logs: {}", e);
            ExitCode::Software
        }
    }
}

pub async fn handle_overview_command(args: &ArgMatches) -> ExitCode {
    match overview(args).await {
        Ok(_) => ExitCode::Ok,
        Err(e) => {
            log::error!("Failed to show overview: {}", e);
            ExitCode::Software
        }
    }
}

pub fn handle_obfuscate_command(args: &ArgMatches) -> ExitCode {
    match obfuscate_log(args) {
        Ok(_) => {
            if !args.get_flag("silent") {
                println!("Done");
            }
            ExitCode::Ok
        }
        Err(e) => {
            log::error!("Failed to obfuscate log: {}", e);
            ExitCode::Software
        }
    }
}

fn validate_file_paths(log_paths: &[PathBuf]) -> Result<()> {
    let missing_files: Vec<_> = log_paths.iter().filter(|p| !p.exists()).collect();

    if !missing_files.is_empty() {
        let file_list = missing_files
            .iter()
            .map(|p| format!("  - {}", p.display()))
            .collect::<Vec<_>>()
            .join("\n");

        return Err(CommandRunError::Library(rlqt_lib::Error::Io(IoError::new(
            ErrorKind::NotFound,
            format!(
                "Log file(s) not found:\n{}\n\n{}",
                file_list, ERR_MSG_FILE_NOT_FOUND_HELP
            ),
        ))));
    }

    Ok(())
}

fn validate_database_path(db_path: &Path) -> Result<()> {
    if let Some(parent) = db_path.parent()
        && !parent.exists()
    {
        return Err(CommandRunError::Library(rlqt_lib::Error::Io(IoError::new(
            ErrorKind::NotFound,
            format!(
                "Parent directory does not exist: {}\n\n{}  {}",
                parent.display(),
                ERR_MSG_PARENT_DIR_HELP,
                parent.display()
            ),
        ))));
    }

    if db_path.exists() {
        let metadata = db_path.metadata()?;
        if !metadata.is_file() {
            return Err(CommandRunError::Library(rlqt_lib::Error::Io(IoError::new(
                ErrorKind::InvalidInput,
                format!(
                    "Database path exists but is not a file: {}\n\
                    \n\
                    The path points to a directory or special file.\n\
                    Choose a different path for the database file.",
                    db_path.display()
                ),
            ))));
        }
        log::warn!(
            "Database file already exists and will be overwritten: {}",
            db_path.display()
        );
    }

    Ok(())
}

fn deduplicate_paths(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut seen = HashSet::new();
    paths
        .into_iter()
        .filter(|path| {
            let key = path.canonicalize().unwrap_or_else(|_| path.to_owned());
            seen.insert(key)
        })
        .collect()
}

fn collect_log_files_from_directory(dir_path: &str) -> Result<Vec<PathBuf>> {
    let dir = Path::new(dir_path);

    if !dir.exists() {
        return Err(CommandRunError::Library(rlqt_lib::Error::Io(IoError::new(
            ErrorKind::NotFound,
            format!(
                "Directory not found: {}\n\
                \n\
                Make sure:\n\
                • The directory path is correct\n\
                • You have read permissions for the directory\n\
                • Use --input-log-file-path to specify individual files",
                dir.display()
            ),
        ))));
    }

    if !dir.is_dir() {
        return Err(CommandRunError::Library(rlqt_lib::Error::Io(IoError::new(
            ErrorKind::InvalidInput,
            format!(
                "Path is not a directory: {}\n\
                \n\
                The path points to a file, not a directory.\n\
                Use --input-log-file-path for individual files.",
                dir.display()
            ),
        ))));
    }

    let mut log_files = Vec::new();
    let entries = std::fs::read_dir(dir).map_err(|e| {
        CommandRunError::Library(rlqt_lib::Error::Io(IoError::new(
            e.kind(),
            format!(
                "Failed to read directory '{}': {}\n\
                \n\
                Possible causes:\n\
                • Directory permissions - try: chmod +rx {}\n\
                • Disk I/O error",
                dir.display(),
                e,
                dir.display()
            ),
        )))
    })?;

    for entry in entries {
        let entry = entry.map_err(|e| {
            CommandRunError::Library(rlqt_lib::Error::Io(IoError::new(
                e.kind(),
                format!("Failed to read directory entry: {}", e),
            )))
        })?;

        let path = entry.path();
        if path.is_file()
            && path
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("log"))
        {
            log_files.push(path);
        }
    }

    if log_files.is_empty() {
        return Err(CommandRunError::Library(rlqt_lib::Error::Io(IoError::new(
            ErrorKind::NotFound,
            format!(
                "No .log files found in directory: {}\n\
                \n\
                Make sure:\n\
                • The directory contains files with .log extension\n\
                • Use --input-log-file-path to specify individual non-.log files",
                dir.display()
            ),
        ))));
    }

    log_files.sort();

    Ok(log_files)
}

async fn parse_logs(args: &ArgMatches) -> Result<()> {
    let start_time = Instant::now();

    let mut log_paths: Vec<PathBuf> = Vec::new();

    if let Some(file_paths) = args.get_many::<String>("input_log_file_path") {
        log_paths.extend(file_paths.map(PathBuf::from));
    }

    if let Some(dir_paths) = args.get_many::<String>("input_log_dir_path") {
        for dir_path in dir_paths {
            let files = collect_log_files_from_directory(dir_path)?;
            log_paths.extend(files);
        }
    }

    log_paths = deduplicate_paths(log_paths);

    let db_path: PathBuf = args
        .get_one::<String>("output_db_file_path")
        .expect("output_db_file_path is a required argument")
        .into();

    validate_file_paths(&log_paths)?;
    validate_database_path(&db_path)?;

    let db = create_database(&db_path).await?;

    let silent = args.get_flag("silent");

    let multi_progress = if !silent {
        Some(indicatif::MultiProgress::new())
    } else {
        None
    };

    let parsed_files: Vec<_> = log_paths
        .par_iter()
        .map(|log_path| {
            let file_progress = if let Some(mp) = &multi_progress {
                let pb = mp.add(ProgressBar::new_spinner());
                pb.set_style(
                    ProgressStyle::default_spinner()
                        .template("{spinner:.green} {msg}")
                        .unwrap()
                        .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
                );
                pb.set_message(format!("Parsing {}", log_path.display()));
                pb.enable_steady_tick(Duration::from_millis(100));
                Some(pb)
            } else {
                None
            };

            let node_name = extract_node_name(log_path)?;

            if let Some(pb) = &file_progress {
                pb.set_message(format!("Reading {}", log_path.display()));
            }

            let file = File::open(log_path).map_err(|e| {
                CommandRunError::Library(rlqt_lib::Error::Io(IoError::new(
                    e.kind(),
                    format!(
                        "Failed to open log file '{}': {}\n\
                        \n\
                        Possible causes:\n\
                        • File permissions - try: chmod +r {}\n\
                        • File is locked by another process\n\
                        • Disk I/O error",
                        log_path.display(),
                        e,
                        log_path.display()
                    ),
                )))
            })?;
            let reader = BufReader::new(file);

            if let Some(pb) = &file_progress {
                pb.set_message(format!("Parsing {}", log_path.display()));
            }

            let parse_result = parse_log_file(reader)?;
            let total_lines = parse_result.total_lines;
            let mut parsed_entries = parse_result.entries;

            if let Some(pb) = &file_progress {
                pb.set_message(format!("Annotating {} entries", parsed_entries.len()));
            }

            parsed_entries
                .par_iter_mut()
                .for_each(rlqt_lib::entry_metadata::annotate_entry);

            parsed_entries.sort_by_key(|e| e.sequence_id);

            let doc_urls_count = parsed_entries
                .iter()
                .filter(|e| e.doc_url_id.is_some())
                .count();
            let issue_urls_count = parsed_entries
                .iter()
                .filter(|e| e.resolution_or_discussion_url_id.is_some())
                .count();
            log::debug!(
                "Annotated {} entries with doc URLs, {} entries with issue/SCM URLs",
                doc_urls_count,
                issue_urls_count
            );

            if let Some(pb) = &file_progress {
                pb.finish_with_message(format!("✓ Parsed {}", log_path.display()));
            }

            Ok((node_name, parsed_entries, total_lines, log_path.clone()))
        })
        .collect::<Result<Vec<_>>>()?;

    let insert_progress = if !silent {
        let pb = ProgressBar::new(parsed_files.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{bar:40.cyan/blue}] Inserting entries into database")
                .unwrap()
                .progress_chars("#>-"),
        );
        Some(pb)
    } else {
        None
    };

    let mut next_id = 1i64;
    for (node_name, mut parsed_entries, total_lines, log_path) in parsed_files {
        for (i, entry) in parsed_entries.iter_mut().enumerate() {
            entry.explicit_id = Some(next_id + i as i64);
        }
        next_id += parsed_entries.len() as i64;

        NodeLogEntry::insert_parsed_entries(&db, &parsed_entries, &node_name).await?;

        let file_path_str = log_path.to_string_lossy().to_string();
        let file_metadata = extract_file_metadata(
            &parsed_entries,
            file_path_str,
            &node_name,
            total_lines as i64,
        );
        FileMetadata::insert_metadata(&db, file_metadata).await?;

        if let Some(pb) = &insert_progress {
            pb.inc(1);
        }
    }

    if let Some(pb) = insert_progress {
        pb.finish_with_message("✓ All entries inserted");
    }

    let index_progress = if !silent {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap(),
        );
        pb.set_message("Creating database indexes...");
        pb.enable_steady_tick(Duration::from_millis(100));
        Some(pb)
    } else {
        None
    };

    post_insertion_operations(&db).await?;

    if let Some(pb) = index_progress {
        pb.finish_with_message("✓ Indexes created");
    }

    let total = NodeLogEntry::count_all(&db).await?;
    let elapsed = start_time.elapsed();
    let elapsed_secs = elapsed.as_secs_f64();

    log::info!(
        "Parsed, annotated and stored {} log entries in {:.2}s",
        total,
        elapsed_secs
    );

    Ok(())
}

fn extract_node_name(log_path: &Path) -> Result<String> {
    let file_name = log_path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| {
            CommandRunError::Library(rlqt_lib::Error::Io(IoError::new(
                ErrorKind::InvalidInput,
                format!("Invalid log file name: {}", log_path.display()),
            )))
        })?;

    let node_name = file_name
        .strip_suffix(".log")
        .unwrap_or(file_name)
        .to_string();

    Ok(node_name)
}

async fn overview(args: &ArgMatches) -> Result<()> {
    let db_path: PathBuf = args
        .get_one::<String>("input_db_file_path")
        .expect("input_db_file_path is a required argument")
        .into();

    let db = open_database(&db_path).await?;
    let metadata_entries = FileMetadata::find_all(&db).await?;
    log::info!("Found {} log files in database", metadata_entries.len());

    let without_colors = args.get_flag("without_colors");
    output::display_file_metadata(metadata_entries, without_colors)?;

    Ok(())
}

async fn query_logs(args: &ArgMatches) -> Result<()> {
    let db_path: PathBuf = args
        .get_one::<String>("input_db_file_path")
        .expect("input_db_file_path is a required argument")
        .into();

    let mut ctx = QueryContext::default();

    if let Some(since) = args
        .get_one::<String>("since_time")
        .map(|s| parse_datetime_flexible(s))
        .transpose()?
    {
        ctx = ctx.since(since);
    }

    if let Some(to) = args
        .get_one::<String>("to_time")
        .map(|s| parse_datetime_flexible(s))
        .transpose()?
    {
        ctx = ctx.to(to);
    }

    if let Some(sev) = args.get_one::<String>("severity") {
        ctx = ctx.severity(sev);
    }

    if let Some(pid) = args.get_one::<String>("erlang_pid") {
        ctx = ctx.erlang_pid(pid);
    }

    if let Some(n) = args.get_one::<String>("node") {
        ctx = ctx.node(n);
    }

    if let Some(sub) = args.get_one::<String>("subsystem") {
        ctx = ctx.subsystem(sub);
    }

    if let Some(labels) = args.get_many::<String>("label") {
        for label in labels {
            let normalized_label = if label == "election" {
                "elections"
            } else {
                label.as_str()
            };
            ctx = ctx.add_label(normalized_label);
        }
    }

    if args.get_flag("matching_all_labels") {
        ctx = ctx.matching_all_labels(true);
    }

    if let Some(l) = args.get_one::<usize>("limit").copied() {
        ctx = ctx.limit(l as u64);
    }

    if args.get_flag("has_resolution_or_discussion_url") {
        ctx = ctx.has_resolution_or_discussion_url(true);
    }

    if args.get_flag("has_doc_url") {
        ctx = ctx.has_doc_url(true);
    }

    if args.get_flag("unlabelled") {
        ctx = ctx.add_label("unlabelled");
    }

    let db = open_database(&db_path).await?;
    let entries = NodeLogEntry::query(&db, &ctx).await?;
    log::info!("Found {} matching entries", entries.len());

    let without_colors = args.get_flag("without_colors");
    output::display_log_entries(entries, without_colors)?;

    Ok(())
}

fn parse_datetime_flexible(s: &str) -> Result<DateTime<Utc>> {
    rlqt_lib::datetime::parse_datetime_flexible(s).map_err(CommandRunError::DateTimeParse)
}

fn obfuscate_log(args: &ArgMatches) -> Result<()> {
    let start_time = Instant::now();

    let input_path: PathBuf = args
        .get_one::<String>("input_log_file_path")
        .expect("input_log_file_path is a required argument")
        .into();

    let output_path: PathBuf = args
        .get_one::<String>("output_log_file_path")
        .expect("output_log_file_path is a required argument")
        .into();

    if !input_path.exists() {
        return Err(CommandRunError::Library(rlqt_lib::Error::Io(IoError::new(
            ErrorKind::NotFound,
            format!(
                "Input log file not found: {}\n\n{}",
                input_path.display(),
                ERR_MSG_FILE_NOT_FOUND_HELP
            ),
        ))));
    }

    if let Some(parent) = output_path.parent()
        && !parent.as_os_str().is_empty()
        && !parent.exists()
    {
        return Err(CommandRunError::Library(rlqt_lib::Error::Io(IoError::new(
            ErrorKind::NotFound,
            format!(
                "Parent directory does not exist: {}\n\n{}  {}",
                parent.display(),
                ERR_MSG_PARENT_DIR_HELP,
                parent.display()
            ),
        ))));
    }

    let silent = args.get_flag("silent");

    let progress = if !silent {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
        );
        pb.set_message(format!("Obfuscating {}", input_path.display()));
        pb.enable_steady_tick(Duration::from_millis(100));
        Some(pb)
    } else {
        None
    };

    let input_file = File::open(&input_path).map_err(|e| {
        CommandRunError::Library(rlqt_lib::Error::Io(IoError::new(
            e.kind(),
            format!(
                "Failed to open input log file '{}': {}",
                input_path.display(),
                e
            ),
        )))
    })?;
    let reader = BufReader::new(input_file);

    let output_file = File::create(&output_path).map_err(|e| {
        CommandRunError::Library(rlqt_lib::Error::Io(IoError::new(
            e.kind(),
            format!(
                "Failed to create output file '{}': {}",
                output_path.display(),
                e
            ),
        )))
    })?;
    let mut writer = BufWriter::new(output_file);

    let mut obfuscator = LogObfuscator::new();
    let mut line_count = 0usize;

    for line_result in reader.lines() {
        let line = line_result.map_err(|e| {
            CommandRunError::Library(rlqt_lib::Error::Io(IoError::new(
                e.kind(),
                format!("Failed to read line {}: {}", line_count + 1, e),
            )))
        })?;

        let obfuscated = obfuscator.obfuscate_line(&line);
        writeln!(writer, "{}", obfuscated).map_err(|e| {
            CommandRunError::Library(rlqt_lib::Error::Io(IoError::new(
                e.kind(),
                format!("Failed to write line {}: {}", line_count + 1, e),
            )))
        })?;

        line_count += 1;
    }

    writer.flush().map_err(|e| {
        CommandRunError::Library(rlqt_lib::Error::Io(IoError::new(
            e.kind(),
            format!("Failed to flush output file: {}", e),
        )))
    })?;

    if let Some(pb) = progress {
        pb.finish_and_clear();
    }

    let stats = obfuscator.stats();
    let elapsed = start_time.elapsed();

    if !silent {
        let table =
            build_obfuscation_stats_table(&input_path, &output_path, line_count, elapsed, stats);
        println!("{}", table);
    }

    Ok(())
}

#[derive(Tabled)]
struct ObfuscationStatsRow<'a> {
    #[tabled(rename = "Metric")]
    metric: &'a str,
    #[tabled(rename = "Value")]
    value: String,
}

fn build_obfuscation_stats_table(
    input_path: &Path,
    output_path: &Path,
    line_count: usize,
    elapsed: Duration,
    stats: &ObfuscationStats,
) -> Table {
    let data = vec![
        ObfuscationStatsRow {
            metric: "Input file",
            value: input_path.display().to_string(),
        },
        ObfuscationStatsRow {
            metric: "Output file",
            value: output_path.display().to_string(),
        },
        ObfuscationStatsRow {
            metric: "Lines processed",
            value: line_count.to_string(),
        },
        ObfuscationStatsRow {
            metric: "Time elapsed",
            value: format!("{:.2}s", elapsed.as_secs_f64()),
        },
        ObfuscationStatsRow {
            metric: "Unique hostnames obfuscated",
            value: stats.hostnames_obfuscated.to_string(),
        },
        ObfuscationStatsRow {
            metric: "Unique directories obfuscated",
            value: stats.directories_obfuscated.to_string(),
        },
        ObfuscationStatsRow {
            metric: "Unique usernames obfuscated",
            value: stats.usernames_obfuscated.to_string(),
        },
        ObfuscationStatsRow {
            metric: "Unique virtual hosts obfuscated",
            value: stats.vhosts_obfuscated.to_string(),
        },
        ObfuscationStatsRow {
            metric: "Unique IPv4 addresses obfuscated",
            value: stats.ipv4_addresses_obfuscated.to_string(),
        },
        ObfuscationStatsRow {
            metric: "Unique IPv6 addresses obfuscated",
            value: stats.ipv6_addresses_obfuscated.to_string(),
        },
    ];

    let mut table = Table::new(data);
    table.with(Style::rounded());
    table
}
