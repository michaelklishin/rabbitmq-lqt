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

use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use tempfile::NamedTempFile;

use crate::test_helpers::*;

#[test]
fn tail_default_shows_last_10_entries() -> Result<(), Box<dyn Error>> {
    let log_path = fixture_log_path();

    run_succeeds([
        "logs",
        "tail",
        "--input-log-file-path",
        log_path.to_str().unwrap(),
        "--without-colors",
    ])
    .stdout(output_includes("ID"));

    Ok(())
}

#[test]
fn tail_with_custom_line_count() -> Result<(), Box<dyn Error>> {
    let log_path = fixture_log_path();

    run_succeeds([
        "logs",
        "tail",
        "--input-log-file-path",
        log_path.to_str().unwrap(),
        "-n",
        "5",
        "--without-colors",
    ])
    .stdout(output_includes("ID"));

    Ok(())
}

#[test]
fn tail_with_n_larger_than_file() -> Result<(), Box<dyn Error>> {
    let log_path = fixture_log_path();

    run_succeeds([
        "logs",
        "tail",
        "--input-log-file-path",
        log_path.to_str().unwrap(),
        "-n",
        "100000",
        "--without-colors",
    ])
    .stdout(output_includes("ID"));

    Ok(())
}

#[test]
fn tail_with_n_zero_produces_no_table() -> Result<(), Box<dyn Error>> {
    let log_path = fixture_log_path();

    let assert = run_succeeds([
        "logs",
        "tail",
        "--input-log-file-path",
        log_path.to_str().unwrap(),
        "-n",
        "0",
        "--without-colors",
    ]);
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    assert!(
        !stdout.contains("ID"),
        "No table should be printed with -n 0"
    );

    Ok(())
}

#[test]
fn tail_nonexistent_file_fails() -> Result<(), Box<dyn Error>> {
    run_fails([
        "logs",
        "tail",
        "--input-log-file-path",
        "/nonexistent/file.log",
    ])
    .stderr(output_includes("not found"));

    Ok(())
}

#[test]
fn tail_requires_input_path() -> Result<(), Box<dyn Error>> {
    run_fails(["logs", "tail"]).stderr(output_includes("required arguments were not provided"));

    Ok(())
}

#[test]
fn tail_output_includes_expected_columns() -> Result<(), Box<dyn Error>> {
    let log_path = fixture_log_path();

    run_succeeds([
        "logs",
        "tail",
        "--input-log-file-path",
        log_path.to_str().unwrap(),
        "--without-colors",
    ])
    .stdout(output_includes("ID"))
    .stdout(output_includes("Node"))
    .stdout(output_includes("Timestamp"))
    .stdout(output_includes("Severity"))
    .stdout(output_includes("Message"))
    .stdout(output_includes("Subsystem"));

    Ok(())
}

#[test]
fn tail_output_includes_annotations() -> Result<(), Box<dyn Error>> {
    let log_path = fixture_log_path();

    run_succeeds([
        "logs",
        "tail",
        "--input-log-file-path",
        log_path.to_str().unwrap(),
        "-n",
        "100",
        "--without-colors",
    ])
    .stdout(output_includes("Subsystem"))
    .stdout(output_includes("Labels"));

    Ok(())
}

#[test]
fn tail_handles_multiline_entries() -> Result<(), Box<dyn Error>> {
    let mut tmp = NamedTempFile::with_prefix("rabbit@tail-test.log")?;
    writeln!(
        tmp,
        "2025-10-27 11:23:27.566558-07:00 [notice] <0.208.0> Logging: configured log handlers are now ACTIVE"
    )?;
    writeln!(
        tmp,
        "2025-10-27 11:23:27.568937-07:00 [debug] <0.208.0> Starting Ra system called \"coordination\" with configuration:"
    )?;
    writeln!(
        tmp,
        "2025-10-27 11:23:27.568937-07:00 [debug] <0.208.0> #{{message_queue_data => off_heap,name => coordination}}"
    )?;
    writeln!(
        tmp,
        "2025-10-27 11:23:28.000000-07:00 [info] <0.301.0> accepting AMQP connection <0.301.0>"
    )?;
    tmp.flush()?;

    run_succeeds([
        "logs",
        "tail",
        "--input-log-file-path",
        tmp.path().to_str().unwrap(),
        "-n",
        "10",
        "--without-colors",
    ])
    .stdout(output_includes("Starting Ra system"));

    Ok(())
}

#[test]
fn tail_help_text() -> Result<(), Box<dyn Error>> {
    run_succeeds(["logs", "tail", "--help"])
        .stdout(output_includes("--input-log-file-path"))
        .stdout(output_includes("--follow"))
        .stdout(output_includes("--lines"))
        .stdout(output_includes("--without-colors"));

    Ok(())
}

#[test]
fn tail_follow_with_appended_entries() -> Result<(), Box<dyn Error>> {
    let mut tmp = NamedTempFile::with_prefix("rabbit@follow-test.log")?;
    writeln!(
        tmp,
        "2025-10-27 11:23:27.566558-07:00 [notice] <0.208.0> Logging: configured log handlers are now ACTIVE"
    )?;
    writeln!(
        tmp,
        "2025-10-27 11:23:27.566588-07:00 [debug] <0.208.0> Starting Ra systems"
    )?;
    tmp.flush()?;

    let tmp_path = tmp.path().to_string_lossy().to_string();

    let child = Command::new(assert_cmd::cargo::cargo_bin!("rabbitmq-lqt"))
        .args([
            "logs",
            "tail",
            "--input-log-file-path",
            &tmp_path,
            "-f",
            "--without-colors",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    thread::sleep(Duration::from_millis(500));

    {
        let mut file = OpenOptions::new().append(true).open(&tmp_path)?;
        writeln!(
            file,
            "2025-10-27 11:23:28.000000-07:00 [warning] <0.300.0> Connection attempt from disallowed protocol AMQP 0-9-1"
        )?;
        writeln!(
            file,
            "2025-10-27 11:23:29.000000-07:00 [info] <0.301.0> accepting AMQP connection <0.301.0>"
        )?;
        file.flush()?;
    }

    thread::sleep(Duration::from_millis(800));

    // Send SIGINT to stop the follow mode
    Command::new("kill")
        .args(["-s", "INT", &child.id().to_string()])
        .status()?;

    let output = child.wait_with_output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(
        stdout.contains("Connection attempt from disallowed protocol"),
        "Follow mode should have picked up the new warning entry. Stdout: {}",
        stdout
    );

    Ok(())
}
