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
use std::fs::metadata;
use tempfile::NamedTempFile;

mod test_helpers;
use test_helpers::*;

#[test]
fn parse_with_silent_flag_suppresses_done_message() -> Result<(), Box<dyn Error>> {
    let log_path = fixture_log_path();
    let db_file = NamedTempFile::new()?;
    let db_path = db_file.path().to_str().unwrap();

    run_succeeds([
        "logs",
        "parse",
        "--input-log-file-path",
        log_path.to_str().unwrap(),
        "--output-db-file-path",
        db_path,
        "--silent",
    ])
    .stdout(predicate::str::is_empty());

    assert!(db_file.path().exists());
    let file_metadata = metadata(db_file.path())?;
    assert!(file_metadata.len() > 0, "Database file should not be empty");

    Ok(())
}

#[test]
fn parse_without_silent_flag_shows_done_message() -> Result<(), Box<dyn Error>> {
    let log_path = fixture_log_path();
    let db_file = NamedTempFile::new()?;
    let db_path = db_file.path().to_str().unwrap();

    run_succeeds([
        "logs",
        "parse",
        "--input-log-file-path",
        log_path.to_str().unwrap(),
        "--output-db-file-path",
        db_path,
    ])
    .stdout(output_includes("Done"));

    Ok(())
}

#[test]
fn parse_multiple_files_processes_all() -> Result<(), Box<dyn Error>> {
    let log_path1 = fixture_log_path();
    let log_path2 = fixture_log_path();
    let db_file = NamedTempFile::new()?;
    let db_path = db_file.path().to_str().unwrap();

    run_succeeds([
        "logs",
        "parse",
        "--input-log-file-path",
        log_path1.to_str().unwrap(),
        "--input-log-file-path",
        log_path2.to_str().unwrap(),
        "--output-db-file-path",
        db_path,
    ])
    .stderr(output_includes("Parsed"))
    .stderr(output_includes("log entries"));

    assert!(db_file.path().exists());

    Ok(())
}
