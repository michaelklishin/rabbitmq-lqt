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
use std::path::PathBuf;
use std::process::exit;

#[tokio::main]
async fn main() {
    init_logging();

    let matches = cli::clap_parser().get_matches();
    let exit_code = dispatch_command(&matches).await;

    if exit_code != sysexits::ExitCode::Ok {
        exit(exit_code as i32);
    }
}

const BIN_NAME: &str = env!("CARGO_BIN_NAME");

async fn dispatch_command(cli: &clap::ArgMatches) -> sysexits::ExitCode {
    match cli.subcommand() {
        Some(("logs", logs_args)) => match logs_args.subcommand() {
            Some(("parse", args)) => commands::handle_parse_command(args).await,
            Some(("query", args)) => commands::handle_query_command(args).await,
            Some(("overview", args)) => commands::handle_overview_command(args).await,
            _ => {
                eprintln!(
                    "Unknown logs subcommand. Try '{} logs --help' for available commands.",
                    BIN_NAME
                );
                log::error!("Unknown logs subcommand");
                sysexits::ExitCode::Usage
            }
        },
        Some(("web", web_args)) => match web_args.subcommand() {
            Some(("serve", args)) => handle_web_serve_command(args).await,
            _ => {
                eprintln!(
                    "Unknown web subcommand. Try '{} web --help' for available commands.",
                    BIN_NAME
                );
                log::error!("Unknown web subcommand");
                sysexits::ExitCode::Usage
            }
        },
        _ => {
            eprintln!(
                "Unknown command group. Try '{} --help' for available commands.",
                BIN_NAME
            );
            log::error!("Unknown command group");
            sysexits::ExitCode::Usage
        }
    }
}

async fn handle_web_serve_command(args: &clap::ArgMatches) -> sysexits::ExitCode {
    let db_path: PathBuf = args
        .get_one::<String>("input_db_file_path")
        .expect("input_db_file_path is required")
        .into();
    let host = args
        .get_one::<String>("host")
        .expect("host has a default value");
    let port: u16 = args
        .get_one::<String>("port")
        .expect("port has a default value")
        .parse()
        .expect("port must be a valid number");

    match rlqt_ui::run_server(&db_path, host, port).await {
        Ok(()) => sysexits::ExitCode::Ok,
        Err(e) => {
            log::error!("Web UI startup error: {}", e);
            sysexits::ExitCode::DataErr
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
