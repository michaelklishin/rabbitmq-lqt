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
use std::fs;
use std::fs::metadata;
use tempfile::NamedTempFile;

use crate::test_helpers::*;

#[test]
fn merge_into_existing_database() -> Result<(), Box<dyn Error>> {
    let log_path1 = fixture_log_path();
    let log_path2 = fixture_log_path_hare();
    let db_file = NamedTempFile::new()?;
    let db_path = db_file.path().to_str().unwrap();

    run_succeeds([
        "logs",
        "parse",
        "--input-log-file-path",
        log_path1.to_str().unwrap(),
        "--output-db-file-path",
        db_path,
    ])
    .stderr(output_includes("465 log entries"));

    run_succeeds([
        "logs",
        "merge",
        "--input-log-file-path",
        log_path2.to_str().unwrap(),
        "--db-file-path",
        db_path,
    ])
    .stderr(output_includes("Merged 465 new log entries"))
    .stderr(output_includes("total: 930"));

    Ok(())
}

#[test]
fn merge_nonexistent_log_file_fails() -> Result<(), Box<dyn Error>> {
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
    ]);

    run_fails([
        "logs",
        "merge",
        "--input-log-file-path",
        "/nonexistent/file.log",
        "--db-file-path",
        db_path,
    ])
    .stderr(
        output_includes("Log file(s) not found")
            .or(output_includes("not found"))
            .or(output_includes("No such file or directory")),
    );

    Ok(())
}

#[test]
fn merge_into_nonexistent_database_fails() -> Result<(), Box<dyn Error>> {
    let log_path = fixture_log_path();

    run_fails([
        "logs",
        "merge",
        "--input-log-file-path",
        log_path.to_str().unwrap(),
        "--db-file-path",
        "/nonexistent/database.db",
    ])
    .stderr(output_includes("Database file not found").or(output_includes("not found")));

    Ok(())
}

#[test]
fn merge_into_directory_instead_of_file_fails() -> Result<(), Box<dyn Error>> {
    let log_path = fixture_log_path();
    let dir = tempfile::tempdir()?;

    run_fails([
        "logs",
        "merge",
        "--input-log-file-path",
        log_path.to_str().unwrap(),
        "--db-file-path",
        dir.path().to_str().unwrap(),
    ])
    .stderr(output_includes("not a file").or(output_includes("Database path is not a file")));

    Ok(())
}

#[test]
fn merge_requires_input_path() -> Result<(), Box<dyn Error>> {
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
    ]);

    run_fails(["logs", "merge", "--db-file-path", db_path])
        .stderr(output_includes("required arguments were not provided"));

    Ok(())
}

#[test]
fn merge_requires_database_path() -> Result<(), Box<dyn Error>> {
    let log_path = fixture_log_path();

    run_fails([
        "logs",
        "merge",
        "--input-log-file-path",
        log_path.to_str().unwrap(),
    ])
    .stderr(output_includes("required arguments were not provided"));

    Ok(())
}

#[test]
fn merge_additional_file_into_existing_database() -> Result<(), Box<dyn Error>> {
    let log_path = fixture_log_path();
    let log_path3 = fixture_log_path_cottontail();
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
    .stderr(output_includes("465 log entries"));

    run_succeeds([
        "logs",
        "merge",
        "--input-log-file-path",
        log_path3.to_str().unwrap(),
        "--db-file-path",
        db_path,
    ])
    .stderr(output_includes("Merged 224 new log entries"))
    .stderr(output_includes("total: 689"));

    Ok(())
}

#[test]
fn merge_incremental_growth_with_four_fixtures() -> Result<(), Box<dyn Error>> {
    let log_path1 = fixture_log_path();
    let log_path2 = fixture_log_path_hare();
    let log_path3 = fixture_log_path_cottontail();
    let log_path4 = fixture_log_path_flopsy();

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
    .stderr(output_includes("930 log entries"));

    let size_after_first_two = metadata(db_file.path())?.len();

    run_succeeds([
        "logs",
        "merge",
        "--input-log-file-path",
        log_path3.to_str().unwrap(),
        "--db-file-path",
        db_path,
    ])
    .stderr(output_includes("Merged 224 new log entries"))
    .stderr(output_includes("total: 1154"));

    let size_after_third = metadata(db_file.path())?.len();
    assert!(
        size_after_third > size_after_first_two,
        "Database should grow after merging third fixture: {} vs {}",
        size_after_third,
        size_after_first_two
    );

    run_succeeds([
        "logs",
        "merge",
        "--input-log-file-path",
        log_path4.to_str().unwrap(),
        "--db-file-path",
        db_path,
    ])
    .stderr(output_includes("Merged 993 new log entries"))
    .stderr(output_includes("total: 2147"));

    let size_after_fourth = metadata(db_file.path())?.len();
    assert!(
        size_after_fourth > size_after_third,
        "Database should grow after merging fourth fixture: {} vs {}",
        size_after_fourth,
        size_after_third
    );

    Ok(())
}

#[test]
fn merge_same_file_twice_adds_entries_and_upserts_metadata() -> Result<(), Box<dyn Error>> {
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
    .stderr(output_includes("465 log entries"));

    run_succeeds([
        "logs",
        "merge",
        "--input-log-file-path",
        log_path.to_str().unwrap(),
        "--db-file-path",
        db_path,
    ])
    .stderr(output_includes("Merged 465 new log entries"))
    .stderr(output_includes("total: 930"));

    Ok(())
}

#[test]
fn merge_multiple_files_at_once() -> Result<(), Box<dyn Error>> {
    let log_path1 = fixture_log_path();
    let log_path2 = fixture_log_path_hare();
    let log_path3 = fixture_log_path_cottontail();
    let db_file = NamedTempFile::new()?;
    let db_path = db_file.path().to_str().unwrap();

    run_succeeds([
        "logs",
        "parse",
        "--input-log-file-path",
        log_path1.to_str().unwrap(),
        "--output-db-file-path",
        db_path,
    ])
    .stderr(output_includes("465 log entries"));

    run_succeeds([
        "logs",
        "merge",
        "--input-log-file-path",
        log_path2.to_str().unwrap(),
        "--input-log-file-path",
        log_path3.to_str().unwrap(),
        "--db-file-path",
        db_path,
    ])
    .stderr(output_includes("Merged 689 new log entries"))
    .stderr(output_includes("total: 1154"));

    Ok(())
}

#[test]
fn merge_directory_into_existing_database() -> Result<(), Box<dyn Error>> {
    let dir = tempfile::tempdir()?;

    fs::copy(
        fixture_log_path_cottontail(),
        dir.path().join("rabbit@cottontail.log"),
    )?;
    fs::copy(
        fixture_log_path_flopsy(),
        dir.path().join("rabbit@flopsy.log"),
    )?;

    let log_path1 = fixture_log_path();
    let log_path2 = fixture_log_path_hare();
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
    .stderr(output_includes("930 log entries"));

    run_succeeds([
        "logs",
        "merge",
        "--input-log-dir-path",
        dir.path().to_str().unwrap(),
        "--db-file-path",
        db_path,
    ])
    .stderr(output_includes("Merged 1217 new log entries"))
    .stderr(output_includes("total: 2147"));

    Ok(())
}
