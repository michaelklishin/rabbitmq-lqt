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

use bel7_cli::CommandShellExt;
use proptest::prelude::*;
use std::ffi::OsStr;
use std::process::Command;

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

proptest! {
    #[test]
    fn unknown_shells_default_to_bash(random_path in "[a-z0-9/]{1,50}") {
        let path = format!("/some/path/{}", random_path);
        let result = run_with_shell_env(["shell", "completions"], &path);
        let output = result.get_output();
        prop_assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        prop_assert!(
            stdout.contains("_rabbitmq-lqt")
                || stdout.contains("#compdef")
                || stdout.contains("complete -c")
                || stdout.contains("module completions")
                || stdout.contains("edit:completion:arg-completer")
        );
    }

    #[test]
    fn bash_detected_with_any_path_prefix(prefix in "[a-z0-9/]{0,30}") {
        let path = format!("{}/bash", prefix);
        let result = run_with_shell_env(["shell", "completions"], &path);
        let output = result.get_output();
        prop_assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        prop_assert!(stdout.contains("_rabbitmq-lqt"));
    }

    #[test]
    fn zsh_detected_with_any_path_prefix(prefix in "[a-z0-9/]{0,30}") {
        let path = format!("{}/zsh", prefix);
        let result = run_with_shell_env(["shell", "completions"], &path);
        let output = result.get_output();
        prop_assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        prop_assert!(stdout.contains("#compdef rabbitmq-lqt"));
    }

    #[test]
    fn fish_detected_with_any_path_prefix(prefix in "[a-z0-9/]{0,30}") {
        let path = format!("{}/fish", prefix);
        let result = run_with_shell_env(["shell", "completions"], &path);
        let output = result.get_output();
        prop_assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        prop_assert!(stdout.contains("complete -c rabbitmq-lqt"));
    }

    #[test]
    fn elvish_detected_with_any_path_prefix(prefix in "[a-z0-9/]{0,30}") {
        let path = format!("{}/elvish", prefix);
        let result = run_with_shell_env(["shell", "completions"], &path);
        let output = result.get_output();
        prop_assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        prop_assert!(stdout.contains("set edit:completion:arg-completer[rabbitmq-lqt]"));
    }

    #[test]
    fn nushell_short_detected_with_any_path_prefix(prefix in "[a-z0-9/]{0,30}") {
        let path = format!("{}/nu", prefix);
        let result = run_with_shell_env(["shell", "completions"], &path);
        let output = result.get_output();
        prop_assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        prop_assert!(stdout.contains("module completions"));
    }

    #[test]
    fn nushell_long_detected_with_any_path_prefix(prefix in "[a-z0-9/]{0,30}") {
        let path = format!("{}/nushell", prefix);
        let result = run_with_shell_env(["shell", "completions"], &path);
        let output = result.get_output();
        prop_assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        prop_assert!(stdout.contains("module completions"));
    }
}
