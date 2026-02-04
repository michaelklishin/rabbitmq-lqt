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

use rabbitmq_lqt_lib::Severity;
use rabbitmq_lqt_lib::entry_metadata::label_annotators::annotate_labels;
use rabbitmq_lqt_lib::entry_metadata::labels::LogEntryLabels;
use test_helpers::create_test_entry;

#[test]
fn test_statistics_database() {
    let entry = create_test_entry("Statistics database started successfully", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::METRICS));
    assert!(labels.contains(LogEntryLabels::PLUGINS));
}

#[test]
fn test_management_plugin() {
    let entry = create_test_entry(
        "Management plugin: HTTP API started on port 15672",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::METRICS));
    assert!(labels.contains(LogEntryLabels::PLUGINS));
}

#[test]
fn test_prometheus_metrics() {
    let entry = create_test_entry(
        "Prometheus metrics: endpoint enabled at /metrics",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::METRICS));
    assert!(labels.contains(LogEntryLabels::PLUGINS));
}

#[test]
fn test_sysmon_handler_busy_dist_port() {
    let entry = create_test_entry(
        "rabbit_sysmon_handler busy_dist_port <0.28731.0> [{name,delegate_management_2},{initial_call,{delegate,init,1}},{gen_server2,process_next_msg,1},{message_queue_len,0}] {#Port<0.84554>,unknown}",
        Severity::Warning,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::METRICS));
    assert!(labels.contains(LogEntryLabels::CLUSTERING));
}
