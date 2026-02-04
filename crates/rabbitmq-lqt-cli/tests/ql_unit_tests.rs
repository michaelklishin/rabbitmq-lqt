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
fn ql_basic_query() -> Result<(), Box<dyn Error>> {
    let db_file = setup_test_db()?;
    let db_path = db_file.path().to_str().unwrap();

    run_succeeds([
        "logs",
        "ql",
        "--input-db-file-path",
        db_path,
        "--query",
        "*",
    ])
    .stderr(output_includes("Found"))
    .stderr(output_includes("matching entries"));

    Ok(())
}

#[test]
fn ql_query_with_severity_filter() -> Result<(), Box<dyn Error>> {
    let db_file = setup_test_db()?;
    let db_path = db_file.path().to_str().unwrap();

    run_succeeds([
        "logs",
        "ql",
        "--input-db-file-path",
        db_path,
        "--query",
        r#"severity == "error""#,
    ])
    .stderr(output_includes("Found"));

    Ok(())
}

#[test]
fn ql_query_with_limit() -> Result<(), Box<dyn Error>> {
    let db_file = setup_test_db()?;
    let db_path = db_file.path().to_str().unwrap();

    run_succeeds([
        "logs",
        "ql",
        "--input-db-file-path",
        db_path,
        "--query",
        "* | limit 10",
    ])
    .stderr(output_includes("Found"));

    Ok(())
}

#[test]
fn ql_query_with_severity_and_limit() -> Result<(), Box<dyn Error>> {
    let db_file = setup_test_db()?;
    let db_path = db_file.path().to_str().unwrap();

    run_succeeds([
        "logs",
        "ql",
        "--input-db-file-path",
        db_path,
        "--query",
        r#"severity == "info" | limit 5"#,
    ])
    .stderr(output_includes("Found"));

    Ok(())
}

#[test]
fn ql_query_with_message_contains() -> Result<(), Box<dyn Error>> {
    let db_file = setup_test_db()?;
    let db_path = db_file.path().to_str().unwrap();

    run_succeeds([
        "logs",
        "ql",
        "--input-db-file-path",
        db_path,
        "--query",
        r#"message contains "connection""#,
    ])
    .stderr(output_includes("Found"));

    Ok(())
}

#[test]
fn ql_query_with_subsystem_filter() -> Result<(), Box<dyn Error>> {
    let db_file = setup_test_db()?;
    let db_path = db_file.path().to_str().unwrap();

    run_succeeds([
        "logs",
        "ql",
        "--input-db-file-path",
        db_path,
        "--query",
        r#"subsystem == "boot""#,
    ])
    .stderr(output_includes("Found"));

    Ok(())
}

#[test]
fn ql_query_requires_database_path() -> Result<(), Box<dyn Error>> {
    run_fails(["logs", "ql", "--query", "*"])
        .stderr(output_includes("required arguments were not provided"));

    Ok(())
}

#[test]
fn ql_query_requires_query_argument() -> Result<(), Box<dyn Error>> {
    let db_file = setup_test_db()?;
    let db_path = db_file.path().to_str().unwrap();

    run_fails(["logs", "ql", "--input-db-file-path", db_path])
        .stderr(output_includes("required arguments were not provided"));

    Ok(())
}

#[test]
fn ql_query_with_invalid_syntax_fails() -> Result<(), Box<dyn Error>> {
    let db_file = setup_test_db()?;
    let db_path = db_file.path().to_str().unwrap();

    run_fails([
        "logs",
        "ql",
        "--input-db-file-path",
        db_path,
        "--query",
        "invalid query syntax",
    ])
    .stderr(output_includes("Query language error"));

    Ok(())
}

#[test]
fn ql_query_with_invalid_severity_fails() -> Result<(), Box<dyn Error>> {
    let db_file = setup_test_db()?;
    let db_path = db_file.path().to_str().unwrap();

    run_fails([
        "logs",
        "ql",
        "--input-db-file-path",
        db_path,
        "--query",
        r#"severity == "invalid""#,
    ])
    .stderr(output_includes("Invalid severity"));

    Ok(())
}

#[test]
fn ql_query_nonexistent_database_fails() -> Result<(), Box<dyn Error>> {
    run_fails([
        "logs",
        "ql",
        "--input-db-file-path",
        "/nonexistent/database.db",
        "--query",
        "*",
    ])
    .stderr(output_includes("unable to open database file").or(output_includes("Database error")));

    Ok(())
}

#[test]
fn ql_query_with_short_options() -> Result<(), Box<dyn Error>> {
    let db_file = setup_test_db()?;
    let db_path = db_file.path().to_str().unwrap();

    run_succeeds(["logs", "ql", "-i", db_path, "-q", "* | limit 5"])
        .stderr(output_includes("Found"));

    Ok(())
}

#[test]
fn ql_query_with_preset() -> Result<(), Box<dyn Error>> {
    let db_file = setup_test_db()?;
    let db_path = db_file.path().to_str().unwrap();

    run_succeeds([
        "logs",
        "ql",
        "--input-db-file-path",
        db_path,
        "--query",
        ":errors",
    ])
    .stderr(output_includes("Found"));

    Ok(())
}

#[test]
fn ql_query_with_time_range() -> Result<(), Box<dyn Error>> {
    let db_file = setup_test_db()?;
    let db_path = db_file.path().to_str().unwrap();

    run_succeeds([
        "logs",
        "ql",
        "--input-db-file-path",
        db_path,
        "--query",
        "@24h",
    ])
    .stderr(output_includes("Found"));

    Ok(())
}

#[test]
fn ql_query_with_boolean_logic() -> Result<(), Box<dyn Error>> {
    let db_file = setup_test_db()?;
    let db_path = db_file.path().to_str().unwrap();

    run_succeeds([
        "logs",
        "ql",
        "--input-db-file-path",
        db_path,
        "--query",
        r#"severity == "warning" or severity == "error""#,
    ])
    .stderr(output_includes("Found"));

    Ok(())
}

#[test]
fn ql_query_without_colors() -> Result<(), Box<dyn Error>> {
    let db_file = setup_test_db()?;
    let db_path = db_file.path().to_str().unwrap();

    run_succeeds([
        "logs",
        "ql",
        "--input-db-file-path",
        db_path,
        "--query",
        "* | limit 5",
        "--without-colors",
    ])
    .stderr(output_includes("Found"));

    Ok(())
}

#[test]
fn ql_query_with_hashtag_label() -> Result<(), Box<dyn Error>> {
    let db_file = setup_test_db()?;
    let db_path = db_file.path().to_str().unwrap();

    run_succeeds([
        "logs",
        "ql",
        "--input-db-file-path",
        db_path,
        "--query",
        "#connections",
    ])
    .stderr(output_includes("Found"));

    Ok(())
}

#[test]
fn ql_query_with_negated_hashtag_label() -> Result<(), Box<dyn Error>> {
    let db_file = setup_test_db()?;
    let db_path = db_file.path().to_str().unwrap();

    run_succeeds([
        "logs",
        "ql",
        "--input-db-file-path",
        db_path,
        "--query=-#timeouts",
    ])
    .stderr(output_includes("Found"));

    Ok(())
}

#[test]
fn ql_query_with_hashtag_label_combination() -> Result<(), Box<dyn Error>> {
    let db_file = setup_test_db()?;
    let db_path = db_file.path().to_str().unwrap();

    run_succeeds([
        "logs",
        "ql",
        "--input-db-file-path",
        db_path,
        "--query=#connections and -#timeouts",
    ])
    .stderr(output_includes("Found"));

    Ok(())
}
