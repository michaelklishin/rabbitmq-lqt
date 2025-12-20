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

#![allow(dead_code)]

use assert_cmd::assert::Assert;
use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::predicate;
use predicates::str::ContainsPredicate;
use std::error::Error;
use std::ffi::OsStr;
use std::path::PathBuf;

pub fn fixture_log_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("rabbit@fixture1.log")
}

pub fn fixture_directory_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
}

pub fn fixture_log_path_hare() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("rabbit@fixture2.log")
}

pub fn fixture_log_path_cottontail() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("rabbit@fixture3.log")
}

pub fn fixture_log_path_flopsy() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("rabbit@fixture4.log")
}

pub fn fixture_log_path_gzip() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("compressed")
        .join("rabbit@fixture3.log.gz")
}

pub fn fixture_log_path_xz() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("compressed")
        .join("rabbit@hostname010.eng.megacorp.local.log.xz")
}

pub fn fixture_tar_gz_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("compressed")
        .join("fixtures12.log.tar.gz")
}

pub fn fixture_tar_xz_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("compressed")
        .join("fixtures34.log.tar.xz")
}

pub fn parse_log_to_db(log_path: &str, db_path: &str) -> Result<(), Box<dyn Error>> {
    cargo_bin_cmd!("rabbitmq-lqt")
        .args([
            "logs",
            "parse",
            "--input-log-file-path",
            log_path,
            "--output-db-file-path",
            db_path,
            "--silent",
        ])
        .assert()
        .success();
    Ok(())
}

pub fn run_succeeds<I, S>(args: I) -> Assert
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    cargo_bin_cmd!("rabbitmq-lqt").args(args).assert().success()
}

pub fn run_fails<I, S>(args: I) -> Assert
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    cargo_bin_cmd!("rabbitmq-lqt").args(args).assert().failure()
}

pub fn output_includes(content: &str) -> ContainsPredicate {
    predicate::str::contains(content)
}
