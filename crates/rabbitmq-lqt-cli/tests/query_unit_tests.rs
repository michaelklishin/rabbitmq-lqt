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
use tempfile::NamedTempFile;

mod test_helpers;
use test_helpers::*;

fn setup_test_db() -> Result<NamedTempFile, Box<dyn Error>> {
    let log_path = fixture_log_path();
    let db_file = NamedTempFile::new()?;
    let db_path = db_file.path().to_str().unwrap();

    parse_log_to_db(log_path.to_str().unwrap(), db_path)?;

    Ok(db_file)
}

#[test]
fn query_all_logs() -> Result<(), Box<dyn Error>> {
    let db_file = setup_test_db()?;
    let db_path = db_file.path().to_str().unwrap();

    run_succeeds(["logs", "query", "--input-db-file-path", db_path])
        .stderr(output_includes("Found"))
        .stderr(output_includes("matching entries"));

    Ok(())
}

#[test]
fn query_with_limit() -> Result<(), Box<dyn Error>> {
    let db_file = setup_test_db()?;
    let db_path = db_file.path().to_str().unwrap();

    run_succeeds([
        "logs",
        "query",
        "--input-db-file-path",
        db_path,
        "--limit",
        "10",
    ])
    .stderr(output_includes("Found"));

    Ok(())
}

#[test]
fn query_with_severity_filter() -> Result<(), Box<dyn Error>> {
    let db_file = setup_test_db()?;
    let db_path = db_file.path().to_str().unwrap();

    run_succeeds([
        "logs",
        "query",
        "--input-db-file-path",
        db_path,
        "--severity",
        "error",
    ])
    .stderr(output_includes("Found"));

    Ok(())
}

#[test]
fn query_with_date_only_format() -> Result<(), Box<dyn Error>> {
    let db_file = setup_test_db()?;
    let db_path = db_file.path().to_str().unwrap();

    run_succeeds([
        "logs",
        "query",
        "--input-db-file-path",
        db_path,
        "--since-time",
        "2025-10-27",
    ])
    .stderr(output_includes("Found"));

    Ok(())
}

#[test]
fn query_with_datetime_format() -> Result<(), Box<dyn Error>> {
    let db_file = setup_test_db()?;
    let db_path = db_file.path().to_str().unwrap();

    run_succeeds([
        "logs",
        "query",
        "--input-db-file-path",
        db_path,
        "--since-time",
        "2025-10-27 11:23:00",
    ])
    .stderr(output_includes("Found"));

    Ok(())
}

#[test]
fn query_with_rfc3339_format() -> Result<(), Box<dyn Error>> {
    let db_file = setup_test_db()?;
    let db_path = db_file.path().to_str().unwrap();

    run_succeeds([
        "logs",
        "query",
        "--input-db-file-path",
        db_path,
        "--since-time",
        "2025-10-27T11:23:00+00:00",
    ])
    .stderr(output_includes("Found"));

    Ok(())
}

#[test]
fn query_with_human_format_yesterday() -> Result<(), Box<dyn Error>> {
    let db_file = setup_test_db()?;
    let db_path = db_file.path().to_str().unwrap();

    run_succeeds([
        "logs",
        "query",
        "--input-db-file-path",
        db_path,
        "--since-time",
        "yesterday",
    ])
    .stderr(output_includes("Found"));

    Ok(())
}

#[test]
fn query_with_human_format_days_ago() -> Result<(), Box<dyn Error>> {
    let db_file = setup_test_db()?;
    let db_path = db_file.path().to_str().unwrap();

    run_succeeds([
        "logs",
        "query",
        "--input-db-file-path",
        db_path,
        "--since-time",
        "2 days ago",
    ])
    .stderr(output_includes("Found"));

    Ok(())
}

#[test]
fn query_with_human_format_now() -> Result<(), Box<dyn Error>> {
    let db_file = setup_test_db()?;
    let db_path = db_file.path().to_str().unwrap();

    run_succeeds([
        "logs",
        "query",
        "--input-db-file-path",
        db_path,
        "--since-time",
        "now",
    ])
    .stderr(output_includes("Found"));

    Ok(())
}

#[test]
fn query_with_time_range() -> Result<(), Box<dyn Error>> {
    let db_file = setup_test_db()?;
    let db_path = db_file.path().to_str().unwrap();

    run_succeeds([
        "logs",
        "query",
        "--input-db-file-path",
        db_path,
        "--since-time",
        "2025-10-27",
        "--to-time",
        "2025-10-28",
    ])
    .stderr(output_includes("Found"));

    Ok(())
}

#[test]
fn query_with_invalid_datetime_fails() -> Result<(), Box<dyn Error>> {
    let db_file = setup_test_db()?;
    let db_path = db_file.path().to_str().unwrap();

    run_fails([
        "logs",
        "query",
        "--input-db-file-path",
        db_path,
        "--since-time",
        "invalid-datetime-format",
    ])
    .stderr(output_includes("Could not parse"))
    .stderr(output_includes("Supported formats"));

    Ok(())
}

#[test]
fn query_nonexistent_database_fails() -> Result<(), Box<dyn Error>> {
    run_fails([
        "logs",
        "query",
        "--input-db-file-path",
        "/nonexistent/database.db",
    ])
    .stderr(output_includes("unable to open database file").or(output_includes("Database error")));

    Ok(())
}

#[test]
fn query_requires_database_path() -> Result<(), Box<dyn Error>> {
    run_fails(["logs", "query"]).stderr(output_includes("required arguments were not provided"));

    Ok(())
}

#[test]
fn query_with_has_doc_url_filter() -> Result<(), Box<dyn Error>> {
    let db_file = setup_test_db()?;
    let db_path = db_file.path().to_str().unwrap();

    run_succeeds([
        "logs",
        "query",
        "--input-db-file-path",
        db_path,
        "--has-doc-url",
    ])
    .stderr(output_includes("Found"));

    Ok(())
}

#[test]
fn query_with_has_issue_or_scm_url_filter() -> Result<(), Box<dyn Error>> {
    let db_file = setup_test_db()?;
    let db_path = db_file.path().to_str().unwrap();

    run_succeeds([
        "logs",
        "query",
        "--input-db-file-path",
        db_path,
        "--has-resolution-or-discussion-url",
    ])
    .stderr(output_includes("Found"));

    Ok(())
}

#[test]
fn query_with_both_url_filters() -> Result<(), Box<dyn Error>> {
    let db_file = setup_test_db()?;
    let db_path = db_file.path().to_str().unwrap();

    run_succeeds([
        "logs",
        "query",
        "--input-db-file-path",
        db_path,
        "--has-doc-url",
        "--has-resolution-or-discussion-url",
    ])
    .stderr(output_includes("Found"));

    Ok(())
}
