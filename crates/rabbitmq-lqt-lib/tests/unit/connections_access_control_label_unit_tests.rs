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
use rabbitmq_lqt_lib::entry_metadata::label_annotators::annotate_labels;
use rabbitmq_lqt_lib::entry_metadata::labels::LogEntryLabels;

#[test]
fn test_authenticated_successfully_by_backend() {
    let entry = create_test_entry("User authenticated successfully by backend", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::CONNECTIONS));
    assert!(labels.contains(LogEntryLabels::ACCESS_CONTROL));
}

#[test]
fn test_authenticated_successfully_by_backend_uppercase() {
    let entry = create_test_entry("USER AUTHENTICATED SUCCESSFULLY BY BACKEND", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::CONNECTIONS));
    assert!(labels.contains(LogEntryLabels::ACCESS_CONTROL));
}

#[test]
fn test_client_address_during_authn_phase() {
    let entry = create_test_entry(
        "Client address during authN phase: 192.168.1.100",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::CONNECTIONS));
    assert!(labels.contains(LogEntryLabels::ACCESS_CONTROL));
}

#[test]
fn test_client_address_during_authn_phase_uppercase() {
    let entry = create_test_entry(
        "CLIENT ADDRESS DURING AUTHN PHASE: 10.0.0.5",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::CONNECTIONS));
    assert!(labels.contains(LogEntryLabels::ACCESS_CONTROL));
}

#[test]
fn test_authenticated_and_granted_access() {
    let entry = create_test_entry(
        "User authenticated and granted access to vhost '/'",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::CONNECTIONS));
    assert!(labels.contains(LogEntryLabels::ACCESS_CONTROL));
}

#[test]
fn test_authenticated_and_granted_access_uppercase() {
    let entry = create_test_entry(
        "USER AUTHENTICATED AND GRANTED ACCESS TO VHOST 'production'",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::CONNECTIONS));
    assert!(labels.contains(LogEntryLabels::ACCESS_CONTROL));
}

#[test]
fn test_authenticated_and_granted_access_mixed_case() {
    let entry = create_test_entry(
        "User Authenticated and Granted Access to VHost 'staging'",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::CONNECTIONS));
    assert!(labels.contains(LogEntryLabels::ACCESS_CONTROL));
}

#[test]
fn test_failed_to_authenticate() {
    let entry = create_test_entry(
        "Connection failed to authenticate: invalid credentials",
        Severity::Warning,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::CONNECTIONS));
    assert!(labels.contains(LogEntryLabels::ACCESS_CONTROL));
}

#[test]
fn test_failed_to_authenticate_uppercase() {
    let entry = create_test_entry(
        "USER FAILED TO AUTHENTICATE DUE TO INCORRECT PASSWORD",
        Severity::Warning,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::CONNECTIONS));
    assert!(labels.contains(LogEntryLabels::ACCESS_CONTROL));
}

#[test]
fn test_failed_to_authenticate_mixed_case() {
    let entry = create_test_entry(
        "Connection (<0.1234.0>) Failed to Authenticate: User 'guest' denied",
        Severity::Warning,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::CONNECTIONS));
    assert!(labels.contains(LogEntryLabels::ACCESS_CONTROL));
}

#[test]
fn test_no_match_unrelated() {
    let entry = create_test_entry("Queue created successfully", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(!labels.contains(LogEntryLabels::CONNECTIONS));
    assert!(!labels.contains(LogEntryLabels::ACCESS_CONTROL));
}
