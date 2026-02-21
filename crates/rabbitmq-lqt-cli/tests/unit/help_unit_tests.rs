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

use predicates::prelude::*;
use std::error::Error;

use crate::test_helpers::*;

#[test]
fn show_help_with_no_arguments() -> Result<(), Box<dyn Error>> {
    let args: [&str; 0] = [];
    run_fails(args).stderr(output_includes("requires a subcommand"));

    Ok(())
}

#[test]
fn show_help_with_help_flag() -> Result<(), Box<dyn Error>> {
    run_succeeds(["--help"]).stdout(output_includes("Usage:"));

    Ok(())
}

#[test]
fn show_version() -> Result<(), Box<dyn Error>> {
    run_succeeds(["--version"]).stdout(output_includes("rabbitmq-lqt"));

    Ok(())
}

#[test]
fn show_logs_subcommand_help() -> Result<(), Box<dyn Error>> {
    run_succeeds(["logs", "--help"])
        .stdout(output_includes("Log file operations"))
        .stdout(output_includes("Commands:"));

    Ok(())
}

#[test]
fn show_parse_command_help() -> Result<(), Box<dyn Error>> {
    run_succeeds(["logs", "parse", "--help"])
        .stdout(output_includes("Parses and annotates RabbitMQ log files"))
        .stdout(output_includes("--input-log-file-path"))
        .stdout(output_includes("--output-db-file-path"));

    Ok(())
}

#[test]
fn show_query_command_help() -> Result<(), Box<dyn Error>> {
    run_succeeds(["logs", "query", "--help"])
        .stdout(output_includes("Query log entries"))
        .stdout(output_includes("--input-db-file-path"))
        .stdout(output_includes("--since-time"))
        .stdout(output_includes("--to-time"))
        .stdout(output_includes("--severity"))
        .stdout(output_includes("--limit"));

    Ok(())
}

#[test]
fn query_help_shows_datetime_formats() -> Result<(), Box<dyn Error>> {
    run_succeeds(["logs", "query", "--help"])
        .stdout(output_includes("dates (2025-10-27)"))
        .stdout(output_includes("datetime (2025-10-27 18:23:00)"))
        .stdout(output_includes("RFC 3339"))
        .stdout(output_includes("human formats"));

    Ok(())
}

#[test]
fn invalid_subcommand_fails() -> Result<(), Box<dyn Error>> {
    run_fails(["invalid-subcommand"]).stderr(
        output_includes("unrecognized subcommand").or(output_includes("invalid subcommand")),
    );

    Ok(())
}

#[test]
fn logs_parse_requires_subcommand() -> Result<(), Box<dyn Error>> {
    run_fails(["logs"]).stderr(output_includes("requires a subcommand"));

    Ok(())
}

#[test]
fn show_merge_command_help() -> Result<(), Box<dyn Error>> {
    run_succeeds(["logs", "merge", "--help"])
        .stdout(output_includes("Merges additional log files"))
        .stdout(output_includes("--input-log-file-path"))
        .stdout(output_includes("--input-log-dir-path"))
        .stdout(output_includes("--db-file-path"));

    Ok(())
}

#[test]
fn show_obfuscate_command_help() -> Result<(), Box<dyn Error>> {
    run_succeeds(["logs", "obfuscate", "--help"])
        .stdout(output_includes("--input-log-file-path"))
        .stdout(output_includes("--output-log-file-path"));

    Ok(())
}

#[test]
fn show_overview_command_help() -> Result<(), Box<dyn Error>> {
    run_succeeds(["logs", "overview", "--help"])
        .stdout(output_includes("file-level metadata"))
        .stdout(output_includes("--input-db-file-path"));

    Ok(())
}

#[test]
fn show_ql_command_help() -> Result<(), Box<dyn Error>> {
    run_succeeds(["logs", "ql", "--help"])
        .stdout(output_includes("Query log entries using RQL"))
        .stdout(output_includes("--query"))
        .stdout(output_includes("#tls"));

    Ok(())
}

#[test]
fn show_tail_command_help() -> Result<(), Box<dyn Error>> {
    run_succeeds(["logs", "tail", "--help"])
        .stdout(output_includes("--input-log-file-path"))
        .stdout(output_includes("--follow"))
        .stdout(output_includes("--lines"))
        .stdout(output_includes("--without-colors"));

    Ok(())
}
