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

mod api;
mod errors;
mod server;

use rlqt_ui::cli::clap_parser;
use std::process;

#[tokio::main]
async fn main() {
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
        .level_for("sqlx::query", log::LevelFilter::Warn)
        .chain(std::io::stdout())
        .apply()
        .unwrap();

    let matches = clap_parser().get_matches();

    let exit_code = match matches.subcommand() {
        Some(("web", web_matches)) => match web_matches.subcommand() {
            Some(("serve", serve_matches)) => server::handle_serve_command(serve_matches).await,
            _ => {
                eprintln!("Unknown web subcommand");
                process::exit(1);
            }
        },
        _ => {
            eprintln!("Unknown command");
            process::exit(1);
        }
    };

    process::exit(exit_code as i32);
}
