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

use clap::{Arg, Command};

pub fn clap_parser() -> Command {
    let serve_cmd = Command::new("serve")
        .about("Start the web server for log querying")
        .arg(
            Arg::new("input_db_file_path")
                .long("input-db-file-path")
                .short('d')
                .required(true)
                .value_name("PATH")
                .help("Path to the database file"),
        )
        .arg(
            Arg::new("host")
                .long("host")
                .default_value("127.0.0.1")
                .value_name("HOST")
                .help("Host address to bind to"),
        )
        .arg(
            Arg::new("port")
                .long("port")
                .short('p')
                .default_value("15692")
                .value_name("PORT")
                .help("Port to listen on"),
        );

    let web_group = Command::new("web")
        .about("Web UI operations")
        .subcommand_required(true)
        .subcommand(serve_cmd);

    Command::new("rabbitmq-lqt-ui")
        .version(env!("CARGO_PKG_VERSION"))
        .about("RabbitMQ Log Query Tools - Web UI")
        .subcommand_required(true)
        .subcommand(web_group)
}
