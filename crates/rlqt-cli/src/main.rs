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
mod cli;
mod commands;
mod core;
mod errors;
mod output;

use std::io::stderr;
use std::path::Path;
use std::process::{Command, exit};

#[tokio::main]
async fn main() {
    init_logging();

    let matches = cli::clap_parser().get_matches();
    let exit_code = dispatch_command(&matches).await;

    if exit_code != sysexits::ExitCode::Ok {
        exit(exit_code as i32);
    }
}

async fn dispatch_command(cli: &clap::ArgMatches) -> sysexits::ExitCode {
    match cli.subcommand() {
        Some(("logs", logs_args)) => match logs_args.subcommand() {
            Some(("parse", args)) => commands::handle_parse_command(args).await,
            Some(("query", args)) => commands::handle_query_command(args).await,
            Some(("overview", args)) => commands::handle_overview_command(args).await,
            _ => {
                eprintln!("Unknown logs subcommand. Try 'rlq logs --help' for available commands.");
                log::error!("Unknown logs subcommand");
                sysexits::ExitCode::Usage
            }
        },
        Some(("web", web_args)) => match web_args.subcommand() {
            Some(("serve", args)) => handle_web_serve_command(args),
            _ => {
                eprintln!("Unknown web subcommand. Try 'rlq web --help' for available commands.");
                log::error!("Unknown web subcommand");
                sysexits::ExitCode::Usage
            }
        },
        _ => {
            eprintln!("Unknown command group. Try 'rlq --help' for available commands.");
            log::error!("Unknown command group");
            sysexits::ExitCode::Usage
        }
    }
}

fn handle_web_serve_command(args: &clap::ArgMatches) -> sysexits::ExitCode {
    let db_path = args
        .get_one::<String>("input_db_file_path")
        .expect("input_db_file_path is required");
    let host = args
        .get_one::<String>("host")
        .expect("host has a default value");
    let port = args
        .get_one::<String>("port")
        .expect("port has a default value");

    let db_path_buf = Path::new(db_path);
    if !db_path_buf.exists() {
        eprintln!("Database file does not exist: {}", db_path);
        log::error!("Database file does not exist: {}", db_path);
        return sysexits::ExitCode::NoInput;
    }

    if !db_path_buf.is_file() {
        eprintln!("Database path is not a file: {}", db_path);
        log::error!("Database path is not a file: {}", db_path);
        return sysexits::ExitCode::NoInput;
    }

    let rlqt_ui_binary = if cfg!(debug_assertions) {
        "target/debug/rlqt-ui"
    } else {
        "target/release/rlqt-ui"
    };

    log::info!("Starting web server using {}", rlqt_ui_binary);

    let status = Command::new(rlqt_ui_binary)
        .arg("web")
        .arg("serve")
        .arg("--input-db-file-path")
        .arg(db_path)
        .arg("--host")
        .arg(host)
        .arg("--port")
        .arg(port)
        .status();

    match status {
        Ok(exit_status) => {
            if exit_status.success() {
                sysexits::ExitCode::Ok
            } else {
                log::error!("Web server exited with status: {}", exit_status);
                sysexits::ExitCode::Software
            }
        }
        Err(e) => {
            eprintln!("Failed to start HTTP server: {}", e);
            eprintln!("\nMake sure the rlqt-ui binary is built:\n  cargo build --package rlqt-ui");
            log::error!("Failed to start HTTP server: {}", e);
            sysexits::ExitCode::Software
        }
    }
}

fn init_logging() {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}] {}",
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .level_for("sqlx::query", log::LevelFilter::Off)
        .level_for("sqlx::pool::acquire", log::LevelFilter::Off)
        .chain(stderr())
        .apply()
        .unwrap_or_else(|e| {
            eprintln!("Failed to initialize logging: {}", e);
            exit(1);
        });
}
