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
fn test_failed_authentication_by_backend() {
    let entry = create_test_entry(
        "User 'sdf98sd7f' failed authentication by backend rabbit_auth_backend_internal",
        Severity::Warning,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::ACCESS_CONTROL));
}

#[test]
fn test_failed_to_add_user() {
    let entry = create_test_entry(
        "Failed to add user 'guest': the user already exists",
        Severity::Warning,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::ACCESS_CONTROL));
}

#[test]
fn test_asked_to_create_user() {
    let entry = create_test_entry(
        "Asked to create a new user 'guest', password length in bytes: 40",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::ACCESS_CONTROL));
}

#[test]
fn test_asked_to_create_user_with_hash() {
    let entry = create_test_entry(
        "Asked to create a new user 'test_create_user_using_sha256_for_hashing.1' with password hash",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::ACCESS_CONTROL));
}

#[test]
fn test_asked_to_delete_user() {
    let entry = create_test_entry("Asked to delete user 'test_list_users'", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::ACCESS_CONTROL));
}

#[test]
fn test_created_user() {
    let entry = create_test_entry("Created user 'guest'", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::ACCESS_CONTROL));
}

#[test]
fn test_deleted_user() {
    let entry = create_test_entry(
        "Deleted user 'user_from_combined_integration_test4'",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::ACCESS_CONTROL));
}

#[test]
fn test_asked_to_set_user_tags() {
    let entry = create_test_entry(
        "Asked to set user tags for user 'guest' to [administrator]",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::ACCESS_CONTROL));
}

#[test]
fn test_asked_to_clear_permissions() {
    let entry = create_test_entry(
        "Asked to clear permissions for user 'user_with_permissions' in virtual host '/'",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::ACCESS_CONTROL));
}

#[test]
fn test_no_match_unrelated() {
    let entry = create_test_entry("Some other log message", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(!labels.contains(LogEntryLabels::ACCESS_CONTROL));
}

#[test]
fn test_ldap_searching_for_dn() {
    let entry = create_test_entry(
        "Searching for DN for americas\\svc_prdrabbitmqadm, got back []",
        Severity::Warning,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::ACCESS_CONTROL));
    assert!(labels.contains(LogEntryLabels::PLUGINS));
}

#[test]
fn test_successfully_set_user_tags() {
    let entry = create_test_entry(
        "Successfully set user tags for user 'guest' to [administrator]",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::ACCESS_CONTROL));
}
