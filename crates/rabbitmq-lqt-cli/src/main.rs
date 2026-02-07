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

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod archive;
mod cli;
mod commands;
mod completions;
mod core;
mod errors;
mod output;

use bel7_cli::ExitCode;
use std::io::stderr;
use tokio::runtime::Runtime;

#[cfg(feature = "web-ui")]
use std::path::PathBuf;

fn main() -> ExitCode {
    if let Err(e) = init_logging() {
        eprintln!("Failed to initialize logging: {}", e);
        return ExitCode::Software;
    }

    let rt = match Runtime::new() {
        Ok(rt) => rt,
        Err(e) => {
            eprintln!("Failed to create Tokio runtime: {}", e);
            return ExitCode::OsErr;
        }
    };

    let matches = cli::clap_parser().get_matches();
    rt.block_on(dispatch_command(&matches))
}

const BIN_NAME: &str = env!("CARGO_BIN_NAME");

async fn dispatch_command(cli: &clap::ArgMatches) -> ExitCode {
    match cli.subcommand() {
        Some(("logs", logs_args)) => match logs_args.subcommand() {
            Some(("parse", args)) => commands::handle_parse_command(args),
            Some(("merge", args)) => commands::handle_merge_command(args),
            Some(("obfuscate", args)) => commands::handle_obfuscate_command(args),
            Some(("query", args)) => commands::handle_query_command(args),
            Some(("overview", args)) => commands::handle_overview_command(args),
            Some(("ql", args)) => commands::handle_ql_command(args),
            Some(("tail", args)) => commands::handle_tail_command(args).await,
            _ => {
                eprintln!(
                    "Unknown logs subcommand. Try '{} logs --help' for available commands.",
                    BIN_NAME
                );
                log::error!("Unknown logs subcommand");
                ExitCode::Usage
            }
        },
        Some(("shell", shell_args)) => match shell_args.subcommand() {
            Some(("completions", args)) => {
                let shell = args
                    .get_one::<cli::CompletionShell>("shell")
                    .copied()
                    .unwrap_or_else(cli::CompletionShell::detect);
                completions::generate_completions(shell);
                ExitCode::Ok
            }
            _ => {
                eprintln!(
                    "Unknown shell subcommand. Try '{} shell --help' for available commands.",
                    BIN_NAME
                );
                log::error!("Unknown shell subcommand");
                ExitCode::Usage
            }
        },
        #[cfg(feature = "web-ui")]
        Some(("web", web_args)) => match web_args.subcommand() {
            Some(("serve", args)) => handle_web_serve_command(args).await,
            _ => {
                eprintln!(
                    "Unknown web subcommand. Try '{} web --help' for available commands.",
                    BIN_NAME
                );
                log::error!("Unknown web subcommand");
                ExitCode::Usage
            }
        },
        _ => {
            eprintln!(
                "Unknown command group. Try '{} --help' for available commands.",
                BIN_NAME
            );
            log::error!("Unknown command group");
            ExitCode::Usage
        }
    }
}

#[cfg(feature = "web-ui")]
async fn handle_web_serve_command(args: &clap::ArgMatches) -> ExitCode {
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

    match rabbitmq_lqt_ui::run_server(&db_path, host, port).await {
        Ok(()) => ExitCode::Ok,
        Err(e) => {
            log::error!("Web UI startup error: {}", e);
            ExitCode::DataErr
        }
    }
}

fn init_logging() -> Result<(), log::SetLoggerError> {
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
}
