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

use rlqt_lib::Severity;
use rlqt_lib::entry_metadata::label_annotators::annotate_labels;
use rlqt_lib::entry_metadata::labels::LogEntryLabels;
use test_helpers::create_test_entry;

#[test]
fn test_inter_node_tls() {
    let entry = create_test_entry("Inter-node TLS not enabled", Severity::Debug);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::TLS));
    assert!(labels.contains(LogEntryLabels::CLUSTERING));
}

#[test]
fn test_tls_options() {
    let entry = create_test_entry("TLS options: verify_peer=true", Severity::Debug);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::TLS));
}

#[test]
fn test_tls_connection() {
    let entry = create_test_entry(
        "TLS connection established from 10.0.0.1:5671",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::TLS));
}

#[test]
fn test_ssl_options() {
    let entry = create_test_entry(
        "SSL options: cacertfile=/etc/rabbitmq/ca.pem",
        Severity::Debug,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::TLS));
}

#[test]
fn test_ssl_connection() {
    let entry = create_test_entry("SSL connection accepted", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::TLS));
}

#[test]
fn test_client_certificates() {
    let entry = create_test_entry("Client certificates verification enabled", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::TLS));
}
