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

use bel7_cli::{CompletionShell, SHELL_DETECTION_ENV_VARS};
use proptest::prelude::*;

/// Clears all shell detection env vars, then sets SHELL to the given path.
///
/// SAFETY: nextest runs each test case in its own process,
/// so there is no concurrent access to environment variables.
fn set_shell_env(shell_path: &str) {
    for var in SHELL_DETECTION_ENV_VARS {
        unsafe { std::env::remove_var(var) };
    }
    unsafe { std::env::set_var("SHELL", shell_path) };
}

proptest! {
    #[test]
    fn unknown_shells_default_to_bash(random_path in "[a-z0-9/]{1,50}") {
        let path = format!("/some/path/{}", random_path);
        set_shell_env(&path);
        let shell = CompletionShell::detect();
        prop_assert_eq!(shell, CompletionShell::Bash);
    }

    #[test]
    fn bash_detected_with_any_path_prefix(prefix in "[a-z0-9/]{0,30}") {
        let path = format!("{}/bash", prefix);
        set_shell_env(&path);
        let shell = CompletionShell::detect();
        prop_assert_eq!(shell, CompletionShell::Bash);
    }

    #[test]
    fn zsh_detected_with_any_path_prefix(prefix in "[a-z0-9/]{0,30}") {
        let path = format!("{}/zsh", prefix);
        set_shell_env(&path);
        let shell = CompletionShell::detect();
        prop_assert_eq!(shell, CompletionShell::Zsh);
    }

    #[test]
    fn fish_detected_with_any_path_prefix(prefix in "[a-z0-9/]{0,30}") {
        let path = format!("{}/fish", prefix);
        set_shell_env(&path);
        let shell = CompletionShell::detect();
        prop_assert_eq!(shell, CompletionShell::Fish);
    }

    #[test]
    fn elvish_detected_with_any_path_prefix(prefix in "[a-z0-9/]{0,30}") {
        let path = format!("{}/elvish", prefix);
        set_shell_env(&path);
        let shell = CompletionShell::detect();
        prop_assert_eq!(shell, CompletionShell::Elvish);
    }

    #[test]
    fn nushell_short_detected_with_any_path_prefix(prefix in "[a-z0-9/]{0,30}") {
        let path = format!("{}/nu", prefix);
        set_shell_env(&path);
        let shell = CompletionShell::detect();
        prop_assert_eq!(shell, CompletionShell::Nushell);
    }

    #[test]
    fn nushell_long_detected_with_any_path_prefix(prefix in "[a-z0-9/]{0,30}") {
        let path = format!("{}/nushell", prefix);
        set_shell_env(&path);
        let shell = CompletionShell::detect();
        prop_assert_eq!(shell, CompletionShell::Nushell);
    }
}
