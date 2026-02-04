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

mod test_helpers;

use bel7_cli::CommandShellExt;
use std::error::Error;
use std::ffi::OsStr;
use std::process::Command;
use test_helpers::{output_includes, run_fails, run_succeeds};

fn run_with_shell_env<I, S>(args: I, shell_path: &str) -> assert_cmd::assert::Assert
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("rabbitmq-lqt"));
    cmd.clear_shell_detection_env();
    cmd.env("SHELL", shell_path);
    cmd.args(args);
    assert_cmd::assert::Assert::new(cmd.output().unwrap())
}

fn run_without_shell_env<I, S>(args: I) -> assert_cmd::assert::Assert
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("rabbitmq-lqt"));
    cmd.clear_shell_detection_env();
    cmd.args(args);
    assert_cmd::assert::Assert::new(cmd.output().unwrap())
}

#[test]
fn shell_completions_bash() -> Result<(), Box<dyn Error>> {
    run_succeeds(["shell", "completions", "--shell", "bash"])
        .stdout(output_includes("_rabbitmq-lqt"));
    Ok(())
}

#[test]
fn shell_completions_zsh() -> Result<(), Box<dyn Error>> {
    run_succeeds(["shell", "completions", "--shell", "zsh"])
        .stdout(output_includes("#compdef rabbitmq-lqt"));
    Ok(())
}

#[test]
fn shell_completions_fish() -> Result<(), Box<dyn Error>> {
    run_succeeds(["shell", "completions", "--shell", "fish"])
        .stdout(output_includes("complete -c rabbitmq-lqt"));
    Ok(())
}

#[test]
fn shell_completions_elvish() -> Result<(), Box<dyn Error>> {
    run_succeeds(["shell", "completions", "--shell", "elvish"]).stdout(output_includes(
        "set edit:completion:arg-completer[rabbitmq-lqt]",
    ));
    Ok(())
}

#[test]
fn shell_completions_nushell() -> Result<(), Box<dyn Error>> {
    run_succeeds(["shell", "completions", "--shell", "nushell"])
        .stdout(output_includes("module completions"));
    Ok(())
}

#[test]
fn shell_completions_nu_alias() -> Result<(), Box<dyn Error>> {
    run_succeeds(["shell", "completions", "--shell", "nu"])
        .stdout(output_includes("module completions"));
    Ok(())
}

#[test]
fn shell_completions_detects_bash() -> Result<(), Box<dyn Error>> {
    run_with_shell_env(["shell", "completions"], "/bin/bash")
        .success()
        .stdout(output_includes("_rabbitmq-lqt"));
    Ok(())
}

#[test]
fn shell_completions_detects_zsh() -> Result<(), Box<dyn Error>> {
    run_with_shell_env(["shell", "completions"], "/bin/zsh")
        .success()
        .stdout(output_includes("#compdef rabbitmq-lqt"));
    Ok(())
}

#[test]
fn shell_completions_detects_fish() -> Result<(), Box<dyn Error>> {
    run_with_shell_env(["shell", "completions"], "/usr/bin/fish")
        .success()
        .stdout(output_includes("complete -c rabbitmq-lqt"));
    Ok(())
}

#[test]
fn shell_completions_detects_elvish() -> Result<(), Box<dyn Error>> {
    run_with_shell_env(["shell", "completions"], "/usr/local/bin/elvish")
        .success()
        .stdout(output_includes(
            "set edit:completion:arg-completer[rabbitmq-lqt]",
        ));
    Ok(())
}

#[test]
fn shell_completions_detects_nushell_short_name() -> Result<(), Box<dyn Error>> {
    run_with_shell_env(["shell", "completions"], "/opt/homebrew/bin/nu")
        .success()
        .stdout(output_includes("module completions"));
    Ok(())
}

#[test]
fn shell_completions_detects_nushell_long_name() -> Result<(), Box<dyn Error>> {
    run_with_shell_env(["shell", "completions"], "/usr/local/bin/nushell")
        .success()
        .stdout(output_includes("module completions"));
    Ok(())
}

#[test]
fn shell_completions_defaults_to_bash_for_unknown_shell() -> Result<(), Box<dyn Error>> {
    run_with_shell_env(["shell", "completions"], "/unknown/shell")
        .success()
        .stdout(output_includes("_rabbitmq-lqt"));
    Ok(())
}

#[test]
fn shell_completions_defaults_to_bash_when_shell_env_unset() -> Result<(), Box<dyn Error>> {
    run_without_shell_env(["shell", "completions"])
        .success()
        .stdout(output_includes("_rabbitmq-lqt"));
    Ok(())
}

#[test]
fn shell_completions_detects_shell_name_without_path() -> Result<(), Box<dyn Error>> {
    run_with_shell_env(["shell", "completions"], "zsh")
        .success()
        .stdout(output_includes("#compdef rabbitmq-lqt"));
    Ok(())
}

#[test]
fn shell_completions_includes_logs_subcommands() -> Result<(), Box<dyn Error>> {
    let output = run_succeeds(["shell", "completions", "--shell", "bash"]);
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(stdout.contains("logs"));
    assert!(stdout.contains("parse"));
    assert!(stdout.contains("query"));
    assert!(stdout.contains("merge"));
    assert!(stdout.contains("obfuscate"));
    assert!(stdout.contains("overview"));
    assert!(stdout.contains("ql"));
    Ok(())
}

#[test]
fn shell_completions_includes_shell_subcommand() -> Result<(), Box<dyn Error>> {
    let output = run_succeeds(["shell", "completions", "--shell", "bash"]);
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(stdout.contains("shell"));
    assert!(stdout.contains("completions"));
    Ok(())
}

#[test]
fn shell_help_shows_completions_subcommand() -> Result<(), Box<dyn Error>> {
    run_succeeds(["shell", "--help"]).stdout(output_includes("completions"));
    Ok(())
}

#[test]
fn shell_requires_subcommand() {
    run_fails(["shell"]).stderr(output_includes("requires a subcommand"));
}

#[test]
fn main_help_shows_shell_subcommand() -> Result<(), Box<dyn Error>> {
    run_succeeds(["--help"])
        .stdout(output_includes("shell"))
        .stdout(output_includes("Shell-related operations"));
    Ok(())
}

#[test]
fn shell_completions_help() -> Result<(), Box<dyn Error>> {
    run_succeeds(["shell", "completions", "--help"])
        .stdout(output_includes("shell completion scripts"));
    Ok(())
}

#[test]
fn shell_completions_rejects_invalid_shell() {
    run_fails(["shell", "completions", "--shell", "invalid"])
        .stderr(output_includes("invalid value 'invalid'"));
}

#[test]
fn shell_completions_help_lists_supported_shells() -> Result<(), Box<dyn Error>> {
    run_succeeds(["shell", "completions", "--help"])
        .stdout(output_includes("bash"))
        .stdout(output_includes("zsh"))
        .stdout(output_includes("fish"))
        .stdout(output_includes("elvish"))
        .stdout(output_includes("nushell"));
    Ok(())
}

#[test]
fn shell_completions_zsh_includes_severity_values() -> Result<(), Box<dyn Error>> {
    let output = run_succeeds(["shell", "completions", "--shell", "zsh"]);
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(stdout.contains("debug"));
    assert!(stdout.contains("info"));
    assert!(stdout.contains("warning"));
    assert!(stdout.contains("error"));
    Ok(())
}

#[test]
fn shell_completions_bash_includes_long_options() -> Result<(), Box<dyn Error>> {
    let output = run_succeeds(["shell", "completions", "--shell", "bash"]);
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(stdout.contains("--input-log-file-path"));
    assert!(stdout.contains("--output-db-file-path"));
    assert!(stdout.contains("--input-db-file-path"));
    assert!(stdout.contains("--severity"));
    assert!(stdout.contains("--subsystem"));
    Ok(())
}

#[test]
fn shell_completions_fish_includes_descriptions() -> Result<(), Box<dyn Error>> {
    let output = run_succeeds(["shell", "completions", "--shell", "fish"]);
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(stdout.contains("Log file operations"));
    Ok(())
}

#[test]
fn shell_completions_nushell_has_valid_syntax() -> Result<(), Box<dyn Error>> {
    let output = run_succeeds(["shell", "completions", "--shell", "nushell"]);
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(stdout.contains("export extern"));
    assert!(stdout.contains("def"));
    Ok(())
}
