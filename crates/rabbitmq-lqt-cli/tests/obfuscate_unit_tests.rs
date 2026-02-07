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
use std::fs;
use tempfile::NamedTempFile;

mod test_helpers;
use test_helpers::*;

#[test]
fn obfuscate_log_file() -> Result<(), Box<dyn Error>> {
    let log_path = fixture_log_path();
    let output_file = NamedTempFile::new()?;
    let output_path = output_file.path().to_str().unwrap();

    run_succeeds([
        "logs",
        "obfuscate",
        "--input-log-file-path",
        log_path.to_str().unwrap(),
        "--output-log-file-path",
        output_path,
    ]);

    let content = fs::read_to_string(output_file.path())?;
    assert!(!content.is_empty(), "Output file should not be empty");

    Ok(())
}

#[test]
fn obfuscate_nonexistent_file_fails() -> Result<(), Box<dyn Error>> {
    let output_file = NamedTempFile::new()?;
    let output_path = output_file.path().to_str().unwrap();

    run_fails([
        "logs",
        "obfuscate",
        "--input-log-file-path",
        "/nonexistent/file.log",
        "--output-log-file-path",
        output_path,
    ])
    .stderr(output_includes("not found"));

    Ok(())
}

#[test]
fn obfuscate_requires_input_path() -> Result<(), Box<dyn Error>> {
    let output_file = NamedTempFile::new()?;
    let output_path = output_file.path().to_str().unwrap();

    run_fails(["logs", "obfuscate", "--output-log-file-path", output_path])
        .stderr(output_includes("required arguments were not provided"));

    Ok(())
}

#[test]
fn obfuscate_requires_output_path() -> Result<(), Box<dyn Error>> {
    let log_path = fixture_log_path();

    run_fails([
        "logs",
        "obfuscate",
        "--input-log-file-path",
        log_path.to_str().unwrap(),
    ])
    .stderr(output_includes("required arguments were not provided"));

    Ok(())
}

#[test]
fn obfuscate_replaces_hostnames() -> Result<(), Box<dyn Error>> {
    let log_path = fixture_log_path();
    let output_file = NamedTempFile::new()?;
    let output_path = output_file.path().to_str().unwrap();

    run_succeeds([
        "logs",
        "obfuscate",
        "--input-log-file-path",
        log_path.to_str().unwrap(),
        "--output-log-file-path",
        output_path,
    ]);

    let content = fs::read_to_string(output_file.path())?;
    assert!(
        !content.contains("fixture1"),
        "Hostnames should be obfuscated"
    );
    assert!(
        content.contains("host1"),
        "Should contain obfuscated hostname"
    );

    Ok(())
}

#[test]
fn obfuscate_with_silent_flag() -> Result<(), Box<dyn Error>> {
    let log_path = fixture_log_path();
    let output_file = NamedTempFile::new()?;
    let output_path = output_file.path().to_str().unwrap();

    run_succeeds([
        "logs",
        "obfuscate",
        "--input-log-file-path",
        log_path.to_str().unwrap(),
        "--output-log-file-path",
        output_path,
        "--silent",
    ]);

    let content = fs::read_to_string(output_file.path())?;
    assert!(!content.is_empty(), "Output file should not be empty");

    Ok(())
}
