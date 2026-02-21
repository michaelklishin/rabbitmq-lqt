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
fn test_oauth2_downloading_signing_keys_from_jwks() {
    let entry = create_test_entry(
        "Downloading signing keys from https://auth.example.com/.well-known/jwks.json (TLS options: [])",
        Severity::Debug,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::OAUTH2));
}

#[test]
fn test_oauth2_decoding_token_with_provider_id() {
    let entry = create_test_entry(
        "Decoding token for resource_server: rabbitmq using oauth_provider_id: keycloak",
        Severity::Debug,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::OAUTH2));
}

#[test]
fn test_oauth2_jwk_from_pem_error() {
    let entry = create_test_entry(
        "Error parsing jwk from pem: invalid_format",
        Severity::Warning,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::OAUTH2));
}

#[test]
fn test_oauth2_no_false_positive() {
    let entry = create_test_entry("Connection from 192.168.1.1 established", Severity::Debug);
    let labels = annotate_labels(&entry);
    assert!(!labels.contains(LogEntryLabels::OAUTH2));
}

#[test]
fn test_oauth2_client_plugin() {
    let entry = create_test_entry("    oauth2_client", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::OAUTH2));
}
