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
use clap::{Arg, ArgAction, Command};

pub fn clap_parser() -> Command {
    let logs_group = Command::new("logs")
        .about("Log file operations")
        .subcommand_required(true)
        .subcommands(logs_subcommands());

    let about = format!(
        "RabbitMQ Log Query Toolkit (RLQT) {}: parse, query, analyze, obfuscate RabbitMQ log files",
        env!("CARGO_PKG_VERSION")
    );
    let cmd = Command::new("rabbitmq-lqt")
        .version(env!("CARGO_PKG_VERSION"))
        .long_about(&about)
        .about(&about)
        .subcommand_required(true)
        .subcommand(logs_group);

    #[cfg(feature = "web-ui")]
    let cmd = {
        let web_group = Command::new("web")
            .about("Web UI operations")
            .subcommand_required(true)
            .subcommands(web_subcommands());
        cmd.subcommand(web_group)
    };

    cmd
}

#[cfg(feature = "web-ui")]
fn web_subcommands() -> Vec<Command> {
    let serve_cmd = Command::new("serve")
        .about("Start the web server for log querying")
        .arg(
            Arg::new("input_db_file_path")
                .long("input-db-file-path")
                .short('i')
                .required(true)
                .value_name("PATH")
                .help("Path to the SQLite database file"),
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

    vec![serve_cmd]
}

fn logs_subcommands() -> Vec<Command> {
    let parse_cmd = Command::new("parse")
        .about("Parses and annotates RabbitMQ log files")
        .arg(
            Arg::new("input_log_file_path")
                .long("input-log-file-path")
                .value_name("PATH")
                .action(ArgAction::Append)
                .num_args(1..)
                .help("Path to an input RabbitMQ log file. Can be provided multiple times"),
        )
        .arg(
            Arg::new("input_log_dir_path")
                .long("input-log-dir-path")
                .short('d')
                .value_name("DIRECTORY")
                .action(ArgAction::Append)
                .num_args(1..)
                .help("Path(s) to a directory containing RabbitMQ log files (*.log)"),
        )
        .group(
            clap::ArgGroup::new("input_log_files")
                .args(["input_log_file_path", "input_log_dir_path"])
                .required(true)
                .multiple(true),
        )
        .arg(
            Arg::new("output_db_file_path")
                .long("output-db-file-path")
                .short('o')
                .required(true)
                .value_name("PATH")
                .help("Path to the output SQLite database file"),
        )
        .arg(
            Arg::new("silent")
                .long("silent")
                .action(ArgAction::SetTrue)
                .help("Suppress output messages"),
        );

    let merge_cmd = Command::new("merge")
        .about("Merges additional log files into an existing database")
        .arg(
            Arg::new("input_log_file_path")
                .long("input-log-file-path")
                .value_name("PATH")
                .action(ArgAction::Append)
                .num_args(1..)
                .help("Path to an input RabbitMQ log file. Can be provided multiple times"),
        )
        .arg(
            Arg::new("input_log_dir_path")
                .long("input-log-dir-path")
                .short('d')
                .value_name("DIRECTORY")
                .action(ArgAction::Append)
                .num_args(1..)
                .help("Path(s) to a directory containing RabbitMQ log files (*.log)"),
        )
        .group(
            clap::ArgGroup::new("input_log_files")
                .args(["input_log_file_path", "input_log_dir_path"])
                .required(true)
                .multiple(true),
        )
        .arg(
            Arg::new("db_file_path")
                .long("db-file-path")
                .short('i')
                .required(true)
                .value_name("PATH")
                .help("Path to the existing database file to merge into"),
        )
        .arg(
            Arg::new("silent")
                .long("silent")
                .action(ArgAction::SetTrue)
                .help("Suppress output messages"),
        );

    let obfuscate_cmd = Command::new("obfuscate")
        .about("Obfuscates sensitive information in RabbitMQ log files")
        .arg(
            Arg::new("input_log_file_path")
                .long("input-log-file-path")
                .short('i')
                .required(true)
                .value_name("PATH")
                .help("Path to the input RabbitMQ log file"),
        )
        .arg(
            Arg::new("output_log_file_path")
                .long("output-log-file-path")
                .short('o')
                .required(true)
                .value_name("PATH")
                .help("Path to the output obfuscated log file"),
        )
        .arg(
            Arg::new("silent")
                .long("silent")
                .action(ArgAction::SetTrue)
                .help("Suppress output messages"),
        );

    let query_cmd = Command::new("query")
        .about("Query log entries from a SQLite database")
        .arg(
            Arg::new("input_db_file_path")
                .long("input-db-file-path")
                .short('i')
                .required(true)
                .value_name("PATH")
                .help("Path to the SQLite database file"),
        )
        .arg(
            Arg::new("since_time")
                .long("since-time")
                .value_name("DATETIME")
                .help("Lower bound of datetime range. Accepts: dates (2025-10-27), datetime (2025-10-27 18:23:00), RFC 3339, or human formats ('yesterday', '2 days ago', 'last Monday')"),
        )
        .arg(
            Arg::new("to_time")
                .long("to-time")
                .value_name("DATETIME")
                .help("Upper bound of datetime range (defaults to now). Accepts: dates (2025-10-27), datetime (2025-10-27 18:23:00), RFC 3339, or human formats ('yesterday', '2 days ago', 'now')"),
        )
        .arg(
            Arg::new("severity")
                .long("severity")
                .value_name("SEVERITY")
                .value_parser(["debug", "info", "notice", "warning", "error", "critical"])
                .help("Filter by severity level"),
        )
        .arg(
            Arg::new("erlang_pid")
                .long("erlang-pid")
                .value_name("PID")
                .help("Filter by Erlang process ID (e.g., <0.208.0>)"),
        )
        .arg(
            Arg::new("node")
                .long("node")
                .value_name("NODE")
                .help("Filter by node name (e.g., rabbit@sunnyside)"),
        )
        .arg(
            Arg::new("subsystem")
                .long("subsystem")
                .value_name("SUBSYSTEM")
                .value_parser(["access_control", "boot", "channels", "classic_queues", "clustering", "connections", "erlang_otp", "exchanges", "feature_flags", "federation", "limits", "logging", "maintenance_mode", "metadata_store", "mqtt", "peer_discovery", "plugins", "policies", "queues", "raft", "runtime_parameters", "shovels", "shutdown", "streams", "virtual_hosts"])
                .help("Filter by subsystem"),
        )
        .arg(
            Arg::new("label")
                .long("label")
                .value_name("LABEL")
                .action(ArgAction::Append)
                .value_parser(["access_control", "amqp1_0", "auto_delete", "channels", "classic_queues", "clustering", "connections", "consumers", "cq_stores", "definitions", "delete", "deletion_protection", "deprecated_features", "disconnects", "election", "elections", "erl_process_crash", "exceptions", "exchanges", "exclusive", "feature_flags", "federation", "handshake", "http", "khepri", "limits", "maintenance_mode", "metrics", "mqtt", "multiline", "networking", "peer_discovery:classic", "plugins", "policies", "process_stops", "queue_federation", "queues", "quorum_queues", "raft", "runtime_parameters", "sessions", "shovels", "shutdown", "startup_banner", "stomp", "streams", "timeouts", "tls", "undefined_fn", "unlabelled", "virtual_hosts", "wal", "websockets", "worker_pool"])
                .help("Filter by label (can be specified multiple times). Matches entries with ANY of the specified labels set to true. Note: 'election' is an alias for 'elections', and 'unlabelled' matches entries with no labels."),
        )
        .arg(
            Arg::new("matching_all_labels")
                .long("matching-all-labels")
                .action(ArgAction::SetTrue)
                .help("When multiple --label filters are specified, require ALL labels to be set (logical AND) instead of ANY (logical OR)"),
        )
        .arg(
            Arg::new("limit")
                .long("limit")
                .value_name("N")
                .help("Limit the number of results")
                .value_parser(clap::value_parser!(usize)),
        )
        .arg(
            Arg::new("without_colors")
                .long("without-colors")
                .action(ArgAction::SetTrue)
                .help("Disable colored output"),
        )
        .arg(
            Arg::new("has_resolution_or_discussion_url")
                .long("has-resolution-or-discussion-url")
                .action(ArgAction::SetTrue)
                .help("Filter entries that have a resolution or discussion URL"),
        )
        .arg(
            Arg::new("has_doc_url")
                .long("has-doc-url")
                .action(ArgAction::SetTrue)
                .help("Filter entries that have a documentation URL"),
        )
        .arg(
            Arg::new("unlabelled")
                .long("unlabelled")
                .action(ArgAction::SetTrue)
                .help("Filter entries that have no labels set (equivalent to --label unlabelled)"),
        );

    let overview_cmd = Command::new("overview")
        .about("Show file-level metadata for all log files in the database")
        .arg(
            Arg::new("input_db_file_path")
                .long("input-db-file-path")
                .short('i')
                .required(true)
                .value_name("PATH")
                .help("Path to the SQLite database file"),
        )
        .arg(
            Arg::new("without_colors")
                .long("without-colors")
                .action(ArgAction::SetTrue)
                .help("Disable colored output"),
        );

    let ql_cmd = Command::new("ql")
        .about("Query log entries using RQL (RLQT Query Language)")
        .arg(
            Arg::new("input_db_file_path")
                .long("input-db-file-path")
                .short('i')
                .required(true)
                .value_name("PATH")
                .help("Path to the SQLite database file"),
        )
        .arg(
            Arg::new("query")
                .long("query")
                .short('q')
                .required(true)
                .value_name("QUERY")
                .help("RQL query string (e.g., 'severity == \"error\" | limit 100')"),
        )
        .arg(
            Arg::new("without_colors")
                .long("without-colors")
                .action(ArgAction::SetTrue)
                .help("Disable colored output"),
        );

    vec![
        parse_cmd,
        merge_cmd,
        obfuscate_cmd,
        query_cmd,
        overview_cmd,
        ql_cmd,
    ]
}
