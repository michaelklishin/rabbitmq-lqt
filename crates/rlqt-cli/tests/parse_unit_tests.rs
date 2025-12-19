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

mod test_helpers;
use test_helpers::*;

#[test]
fn parse_log_file_creates_database() -> Result<(), Box<dyn Error>> {
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
    .stderr(output_includes("Parsed"))
    .stderr(output_includes("log entries"));

    assert!(db_file.path().exists());
    let file_metadata = metadata(db_file.path())?;
    assert!(file_metadata.len() > 0, "Database file should not be empty");

    Ok(())
}

#[test]
fn parse_nonexistent_log_file_fails() -> Result<(), Box<dyn Error>> {
    let db_file = NamedTempFile::new()?;
    let db_path = db_file.path().to_str().unwrap();

    run_fails([
        "logs",
        "parse",
        "--input-log-file-path",
        "/nonexistent/file.log",
        "--output-db-file-path",
        db_path,
    ])
    .stderr(
        output_includes("Log file(s) not found")
            .or(output_includes("Log file not found"))
            .or(output_includes("No such file or directory"))
            .or(output_includes("cannot find the path")),
    );

    Ok(())
}

#[test]
fn parse_requires_input_path() -> Result<(), Box<dyn Error>> {
    let db_file = NamedTempFile::new()?;
    let db_path = db_file.path().to_str().unwrap();

    run_fails(["logs", "parse", "--output-db-file-path", db_path])
        .stderr(output_includes("required arguments were not provided"));

    Ok(())
}

#[test]
fn parse_requires_output_path() -> Result<(), Box<dyn Error>> {
    let log_path = fixture_log_path();

    run_fails([
        "logs",
        "parse",
        "--input-log-file-path",
        log_path.to_str().unwrap(),
    ])
    .stderr(output_includes("required arguments were not provided"));

    Ok(())
}

#[test]
fn parse_log_directory_creates_database() -> Result<(), Box<dyn Error>> {
    let dir_path = fixture_directory_path();
    let db_file = NamedTempFile::new()?;
    let db_path = db_file.path().to_str().unwrap();

    run_succeeds([
        "logs",
        "parse",
        "--input-log-dir-path",
        dir_path.to_str().unwrap(),
        "--output-db-file-path",
        db_path,
    ])
    .stderr(output_includes("Parsed"))
    .stderr(output_includes("log entries"));

    assert!(db_file.path().exists());
    let file_metadata = metadata(db_file.path())?;
    assert!(file_metadata.len() > 0, "Database file should not be empty");

    Ok(())
}

#[test]
fn parse_nonexistent_directory_fails() -> Result<(), Box<dyn Error>> {
    let db_file = NamedTempFile::new()?;
    let db_path = db_file.path().to_str().unwrap();

    run_fails([
        "logs",
        "parse",
        "--input-log-dir-path",
        "/nonexistent/directory",
        "--output-db-file-path",
        db_path,
    ])
    .stderr(output_includes("Directory not found").or(output_includes("not found")));

    Ok(())
}

#[test]
fn parse_directory_with_no_log_files_fails() -> Result<(), Box<dyn Error>> {
    let empty_dir = tempfile::tempdir()?;
    let db_file = NamedTempFile::new()?;
    let db_path = db_file.path().to_str().unwrap();

    run_fails([
        "logs",
        "parse",
        "--input-log-dir-path",
        empty_dir.path().to_str().unwrap(),
        "--output-db-file-path",
        db_path,
    ])
    .stderr(output_includes("No .log files found"));

    Ok(())
}

#[test]
fn parse_file_as_directory_fails() -> Result<(), Box<dyn Error>> {
    let log_path = fixture_log_path();
    let db_file = NamedTempFile::new()?;
    let db_path = db_file.path().to_str().unwrap();

    run_fails([
        "logs",
        "parse",
        "--input-log-dir-path",
        log_path.to_str().unwrap(),
        "--output-db-file-path",
        db_path,
    ])
    .stderr(output_includes("not a directory"));

    Ok(())
}

#[test]
fn parse_both_file_and_directory() -> Result<(), Box<dyn Error>> {
    let log_path = fixture_log_path();
    let dir_path = fixture_directory_path();
    let db_file = NamedTempFile::new()?;
    let db_path = db_file.path().to_str().unwrap();

    run_succeeds([
        "logs",
        "parse",
        "--input-log-file-path",
        log_path.to_str().unwrap(),
        "--input-log-dir-path",
        dir_path.to_str().unwrap(),
        "--output-db-file-path",
        db_path,
    ])
    .stderr(output_includes("Parsed"))
    .stderr(output_includes("log entries"));

    assert!(db_file.path().exists());
    let file_metadata = metadata(db_file.path())?;
    assert!(file_metadata.len() > 0, "Database file should not be empty");

    Ok(())
}

#[test]
fn parse_duplicate_file_paths_deduplicates() -> Result<(), Box<dyn Error>> {
    let log_path = fixture_log_path();
    let db_file = NamedTempFile::new()?;
    let db_path = db_file.path().to_str().unwrap();

    run_succeeds([
        "logs",
        "parse",
        "--input-log-file-path",
        log_path.to_str().unwrap(),
        "--input-log-file-path",
        log_path.to_str().unwrap(),
        "--output-db-file-path",
        db_path,
    ])
    .stderr(output_includes("465 log entries"));

    Ok(())
}

#[test]
fn parse_file_and_directory_containing_same_file_deduplicates() -> Result<(), Box<dyn Error>> {
    let log_path = fixture_log_path();
    let dir_path = fixture_directory_path();
    let db_file = NamedTempFile::new()?;
    let db_path = db_file.path().to_str().unwrap();

    run_succeeds([
        "logs",
        "parse",
        "--input-log-file-path",
        log_path.to_str().unwrap(),
        "--input-log-dir-path",
        dir_path.to_str().unwrap(),
        "--output-db-file-path",
        db_path,
    ])
    .stderr(output_includes("2147 log entries"));

    Ok(())
}

#[test]
fn parse_multiple_distinct_files() -> Result<(), Box<dyn Error>> {
    let log_path_rabbit = fixture_log_path();
    let log_path_hare = fixture_log_path_hare();
    let db_file = NamedTempFile::new()?;
    let db_path = db_file.path().to_str().unwrap();

    run_succeeds([
        "logs",
        "parse",
        "--input-log-file-path",
        log_path_rabbit.to_str().unwrap(),
        "--input-log-file-path",
        log_path_hare.to_str().unwrap(),
        "--output-db-file-path",
        db_path,
    ])
    .stderr(output_includes("930 log entries"));

    assert!(db_file.path().exists());
    let file_metadata = metadata(db_file.path())?;
    assert!(file_metadata.len() > 0, "Database file should not be empty");

    Ok(())
}

#[test]
fn parse_multiple_directories() -> Result<(), Box<dyn Error>> {
    let dir1 = tempfile::tempdir()?;
    let dir2 = tempfile::tempdir()?;

    fs::copy(fixture_log_path(), dir1.path().join("rabbit@node1.log"))?;
    fs::copy(fixture_log_path_hare(), dir2.path().join("hare@node2.log"))?;

    let db_file = NamedTempFile::new()?;
    let db_path = db_file.path().to_str().unwrap();

    run_succeeds([
        "logs",
        "parse",
        "--input-log-dir-path",
        dir1.path().to_str().unwrap(),
        "--input-log-dir-path",
        dir2.path().to_str().unwrap(),
        "--output-db-file-path",
        db_path,
    ])
    .stderr(output_includes("930 log entries"));

    Ok(())
}

#[test]
fn parse_multiple_directories_fails_if_one_is_invalid() -> Result<(), Box<dyn Error>> {
    let dir1 = tempfile::tempdir()?;
    fs::copy(fixture_log_path(), dir1.path().join("rabbit@node1.log"))?;

    let db_file = NamedTempFile::new()?;
    let db_path = db_file.path().to_str().unwrap();

    run_fails([
        "logs",
        "parse",
        "--input-log-dir-path",
        dir1.path().to_str().unwrap(),
        "--input-log-dir-path",
        "/nonexistent/directory",
        "--output-db-file-path",
        db_path,
    ])
    .stderr(output_includes("Directory not found").or(output_includes("not found")));

    Ok(())
}

#[test]
fn parse_overwrites_existing_database() -> Result<(), Box<dyn Error>> {
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

    let first_size = metadata(db_file.path())?.len();

    run_succeeds([
        "logs",
        "parse",
        "--input-log-file-path",
        log_path.to_str().unwrap(),
        "--output-db-file-path",
        db_path,
    ])
    .stderr(output_includes("465 log entries"));

    let second_size = metadata(db_file.path())?.len();
    assert!(
        (first_size as i64 - second_size as i64).abs() < 1000,
        "Database sizes should be comparable after this run"
    );

    Ok(())
}

#[test]
fn parse_fixture3_log_file() -> Result<(), Box<dyn Error>> {
    let log_path = fixture_log_path_cottontail();
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
    .stderr(output_includes("224 log entries"));

    assert!(db_file.path().exists());
    let file_metadata = metadata(db_file.path())?;
    assert!(file_metadata.len() > 0, "Database file should not be empty");

    Ok(())
}

#[test]
fn parse_fixture4_log_file() -> Result<(), Box<dyn Error>> {
    let log_path = fixture_log_path_flopsy();
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
    .stderr(output_includes("993 log entries"));

    assert!(db_file.path().exists());
    let file_metadata = metadata(db_file.path())?;
    assert!(file_metadata.len() > 0, "Database file should not be empty");

    Ok(())
}

#[test]
fn parse_creates_parent_directory_if_missing() -> Result<(), Box<dyn Error>> {
    let log_path = fixture_log_path();
    let temp_dir = tempfile::tempdir()?;
    let nested_db_path = temp_dir
        .path()
        .join("nested")
        .join("deep")
        .join("output.db");

    run_succeeds([
        "logs",
        "parse",
        "--input-log-file-path",
        log_path.to_str().unwrap(),
        "--output-db-file-path",
        nested_db_path.to_str().unwrap(),
    ])
    .stderr(output_includes("465 log entries"));

    assert!(nested_db_path.exists());
    let file_metadata = metadata(&nested_db_path)?;
    assert!(file_metadata.len() > 0, "Database file should not be empty");

    Ok(())
}
