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
fn test_handshake_timeout() {
    let entry = create_test_entry(
        "AMQP 0-9-1 connection 10.0.0.1:5672 handshake_timeout after 10000 ms",
        Severity::Warning,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::TIMEOUTS));
}

#[test]
fn test_delivery_timeout() {
    let entry = create_test_entry(
        "Consumer delivery timeout exceeded for queue 'my-queue'",
        Severity::Warning,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::TIMEOUTS));
}

#[test]
fn test_connection_timeout() {
    let entry = create_test_entry("Connection timeout exceeded", Severity::Error);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::TIMEOUTS));
}

#[test]
fn test_timed_out_variant() {
    let entry = create_test_entry(
        "Consumer 'consumer-tag-998754663370' on channel 1 and queue 'qq.1' in vhost '/' has timed out \
         waiting for a consumer acknowledgement of a delivery with delivery tag = 10. Timeout used: 180000 ms. \
         This timeout value can be configured, see consumers doc guide to learn more",
        Severity::Warning,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::TIMEOUTS));
}
