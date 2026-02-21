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

use crate::test_helpers::create_test_entry;
use rabbitmq_lqt_lib::Severity;
use rabbitmq_lqt_lib::entry_metadata::Annotator;
use rabbitmq_lqt_lib::entry_metadata::subsystem_annotators::{
    RuntimeParametersAnnotator, SubsystemAnnotator,
};
use rabbitmq_lqt_lib::entry_metadata::subsystems::Subsystem;

#[test]
fn test_asked_to_set_runtime_parameter() {
    let entry = create_test_entry(
        "Asked to set or update runtime parameter 'max-connections' in vhost '/'",
        Severity::Info,
    );
    let annotator = RuntimeParametersAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_asked_to_set_runtime_parameter_case_insensitive() {
    let entry = create_test_entry(
        "ASKED TO SET OR UPDATE RUNTIME PARAMETER 'foo' in vhost 'bar'",
        Severity::Info,
    );
    let annotator = RuntimeParametersAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_asked_to_set_runtime_parameter_mixed_case() {
    let entry = create_test_entry(
        "Asked To Set Or Update Runtime Parameter 'test-param'",
        Severity::Info,
    );
    let annotator = RuntimeParametersAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_asked_to_set_runtime_parameter_in_middle() {
    let entry = create_test_entry(
        "Server asked to set or update runtime parameter 'policy' for component",
        Severity::Info,
    );
    let annotator = RuntimeParametersAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_no_match_runtime_without_pattern() {
    let entry = create_test_entry(
        "Runtime parameter 'max-connections' was updated",
        Severity::Info,
    );
    let annotator = RuntimeParametersAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_no_match_partial_match() {
    let entry = create_test_entry("Asked to set parameter for vhost", Severity::Info);
    let annotator = RuntimeParametersAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_no_match_unrelated_message() {
    let entry = create_test_entry("Connection established successfully", Severity::Info);
    let annotator = RuntimeParametersAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_annotate_sets_subsystem() {
    let mut entry = create_test_entry(
        "Asked to set or update runtime parameter 'test'",
        Severity::Info,
    );
    let annotator = RuntimeParametersAnnotator;
    annotator.annotate(&mut entry);
    assert_eq!(
        entry.subsystem_id,
        Some(Subsystem::RuntimeParameters.to_id())
    );
}
