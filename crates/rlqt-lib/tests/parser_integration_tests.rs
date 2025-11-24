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

use rlqt_lib::entry_metadata::subsystem_annotators::annotate_subsystems;
use rlqt_lib::parser::parse_log_file;
use rlqt_lib::{Severity, Subsystem};
use std::io::BufReader;

#[test]
fn test_parse_real_rabbitmq_log_snippet() {
    let log_content = r#"2025-10-27 11:23:27.566558-07:00 [notice] <0.208.0> Logging: configured log handlers are now ACTIVE
2025-10-27 11:23:27.566588-07:00 [debug] <0.208.0> Starting Ra systems
2025-10-27 11:23:27.568937-07:00 [debug] <0.208.0> Starting Ra system called "coordination" with configuration:
2025-10-27 11:23:27.568937-07:00 [debug] <0.208.0> #{message_queue_data => off_heap,name => coordination,
2025-10-27 11:23:27.568937-07:00 [debug] <0.208.0>   names =>
2025-10-27 11:23:27.568937-07:00 [debug] <0.208.0>       #{directory => ra_coordination_directory,
2025-10-27 11:23:27.569119-07:00 [info] <0.208.0> ra: starting system coordination"#;

    let entries = parse_log_file(BufReader::new(log_content.as_bytes()))
        .unwrap()
        .entries;

    assert_eq!(entries.len(), 4);

    assert_eq!(entries[0].severity, Severity::Notice);
    assert!(
        entries[0]
            .message
            .contains("Logging: configured log handlers")
    );

    assert_eq!(entries[1].severity, Severity::Debug);
    assert_eq!(entries[1].message, "Starting Ra systems");

    assert_eq!(entries[2].severity, Severity::Debug);
    assert!(
        entries[2]
            .message
            .contains("Starting Ra system called \"coordination\"")
    );
    assert!(entries[2].message.contains("#{message_queue_data"));
    assert!(entries[2].message.contains("names =>"));
    assert!(entries[2].message.contains("#{directory =>"));

    assert_eq!(entries[3].severity, Severity::Info);
    assert_eq!(entries[3].message, "ra: starting system coordination");
}

#[test]
fn test_parse_vhost_insertion_multiline() {
    let log_content = r#"2025-10-27 11:23:28.333429-07:00 [debug] <0.208.0> Inserted a virtual host record {vhost,<<"/">>,[],
2025-10-27 11:23:28.333429-07:00 [debug] <0.208.0>                                       #{description =>
2025-10-27 11:23:28.333429-07:00 [debug] <0.208.0>                                             <<"Default virtual host">>,
2025-10-27 11:23:28.333429-07:00 [debug] <0.208.0>                                         tags => [],
2025-10-27 11:23:28.333429-07:00 [debug] <0.208.0>                                         default_queue_type => <<"classic">>}}
2025-10-27 11:23:28.333500-07:00 [info] <0.208.0> Virtual host created successfully"#;

    let entries = parse_log_file(BufReader::new(log_content.as_bytes()))
        .unwrap()
        .entries;

    assert_eq!(entries.len(), 2);

    assert_eq!(entries[0].severity, Severity::Debug);
    assert_eq!(entries[0].process_id, "<0.208.0>");
    assert!(
        entries[0]
            .message
            .contains("Inserted a virtual host record")
    );
    assert!(entries[0].message.contains("Default virtual host"));
    assert!(entries[0].message.contains("default_queue_type"));

    assert!(entries[0].message.matches('\n').count() >= 4);

    assert_eq!(entries[1].severity, Severity::Info);
    assert_eq!(entries[1].message, "Virtual host created successfully");
}

#[test]
fn test_parse_mixed_entries() {
    let log_content = r#"2025-10-27 11:23:27.566558-07:00 [notice] <0.208.0> Single line message
2025-10-27 11:23:27.566600-07:00 [debug] <0.208.0> Multiline starts here {data,
2025-10-27 11:23:27.566600-07:00 [debug] <0.208.0>                            field1 => value1,
2025-10-27 11:23:27.566600-07:00 [debug] <0.208.0>                            field2 => value2}
2025-10-27 11:23:27.566700-07:00 [info] <0.208.0> Another single line
2025-10-27 11:23:27.566800-07:00 [warning] <0.300.0> Different process message"#;

    let entries = parse_log_file(BufReader::new(log_content.as_bytes()))
        .unwrap()
        .entries;

    assert_eq!(entries.len(), 4);

    assert_eq!(entries[0].severity, Severity::Notice);
    assert_eq!(entries[0].message, "Single line message");
    assert!(!entries[0].message.contains('\n'));

    assert_eq!(entries[1].severity, Severity::Debug);
    assert!(entries[1].message.contains("Multiline starts here"));
    assert!(entries[1].message.contains("field1 => value1"));
    assert!(entries[1].message.contains("field2 => value2"));
    assert_eq!(entries[1].message.matches('\n').count(), 2);

    assert_eq!(entries[2].severity, Severity::Info);
    assert_eq!(entries[2].message, "Another single line");

    assert_eq!(entries[3].severity, Severity::Warning);
    assert_eq!(entries[3].process_id, "<0.300.0>");
}

#[test]
fn test_parse_malformed_continuation_lines() {
    let log_content = r#"2025-10-27 11:23:27.566558-07:00 [notice] <0.208.0> Exception occurred:
    at some.module.function (some_file.erl:123)
    in some_other_function/2
2025-10-27 11:23:27.566600-07:00 [info] <0.208.0> Recovery completed"#;

    let entries = parse_log_file(BufReader::new(log_content.as_bytes()))
        .unwrap()
        .entries;

    assert_eq!(entries.len(), 2);

    assert_eq!(entries[0].severity, Severity::Notice);
    assert!(entries[0].message.contains("Exception occurred"));
    assert!(entries[0].message.contains("at some.module.function"));
    assert!(entries[0].message.contains("in some_other_function/2"));

    assert_eq!(entries[1].severity, Severity::Info);
    assert_eq!(entries[1].message, "Recovery completed");
}

#[test]
fn test_parse_error_messages() {
    let log_content = r#"2025-10-27 11:23:27.566558-07:00 [error] <0.208.0> Connection failed: timeout
2025-10-27 11:23:27.566600-07:00 [critical] <0.208.0> System shutdown initiated
2025-10-27 11:23:27.566700-07:00 [warning] <0.208.0> Disk space low"#;

    let entries = parse_log_file(BufReader::new(log_content.as_bytes()))
        .unwrap()
        .entries;

    assert_eq!(entries.len(), 3);

    assert_eq!(entries[0].severity, Severity::Error);
    assert_eq!(entries[0].message, "Connection failed: timeout");

    assert_eq!(entries[1].severity, Severity::Critical);
    assert_eq!(entries[1].message, "System shutdown initiated");

    assert_eq!(entries[2].severity, Severity::Warning);
    assert_eq!(entries[2].message, "Disk space low");
}

#[test]
fn test_parse_special_characters() {
    let log_content = r#"2025-10-27 11:23:27.566558-07:00 [debug] <0.208.0> Queue name: "test/queue@123"
2025-10-27 11:23:27.566600-07:00 [info] <0.208.0> Pattern: {queue, <<"name">>, []}
2025-10-27 11:23:27.566700-07:00 [debug] <0.208.0> URL: amqp://user:pass@localhost:5672/%2F"#;

    let entries = parse_log_file(BufReader::new(log_content.as_bytes()))
        .unwrap()
        .entries;

    assert_eq!(entries.len(), 3);

    assert!(entries[0].message.contains("test/queue@123"));
    assert!(entries[1].message.contains("<<\"name\">>"));
    assert!(entries[2].message.contains("amqp://"));
}

#[test]
fn test_parse_long_multiline_message() {
    let log_content = r#"2025-10-27 11:23:27.566558-07:00 [debug] <0.208.0> Configuration map: #{
2025-10-27 11:23:27.566558-07:00 [debug] <0.208.0>     key1 => value1,
2025-10-27 11:23:27.566558-07:00 [debug] <0.208.0>     key2 => value2,
2025-10-27 11:23:27.566558-07:00 [debug] <0.208.0>     key3 => value3,
2025-10-27 11:23:27.566558-07:00 [debug] <0.208.0>     key4 => value4,
2025-10-27 11:23:27.566558-07:00 [debug] <0.208.0>     key5 => value5,
2025-10-27 11:23:27.566558-07:00 [debug] <0.208.0>     key6 => value6,
2025-10-27 11:23:27.566558-07:00 [debug] <0.208.0>     key7 => value7,
2025-10-27 11:23:27.566558-07:00 [debug] <0.208.0>     key8 => value8
2025-10-27 11:23:27.566558-07:00 [debug] <0.208.0> }
2025-10-27 11:23:27.566600-07:00 [info] <0.208.0> Next entry"#;

    let entries = parse_log_file(BufReader::new(log_content.as_bytes()))
        .unwrap()
        .entries;

    assert_eq!(entries.len(), 2);

    assert_eq!(entries[0].severity, Severity::Debug);
    assert!(entries[0].message.contains("Configuration map"));
    assert!(entries[0].message.contains("key1 => value1"));
    assert!(entries[0].message.contains("key8 => value8"));
    assert!(entries[0].message.contains("}"));
    assert_eq!(entries[0].message.matches('\n').count(), 9);

    assert_eq!(entries[1].message, "Next entry");
}

#[test]
fn test_annotate_metadata_store_entries() {
    let log_content = r#"2025-10-27 11:23:27.632069-07:00 [debug] <0.246.0> RabbitMQ metadata store: ra_log:init recovered last_index_term {0,0}
2025-10-27 11:23:27.639169-07:00 [debug] <0.246.0> RabbitMQ metadata store: post_init -> recover in term: 0
2025-10-27 11:23:27.642388-07:00 [notice] <0.246.0> RabbitMQ metadata store: candidate -> leader in term: 1
2025-10-27 11:23:27.650000-07:00 [info] <0.208.0> Regular log message"#;

    let mut entries = parse_log_file(BufReader::new(log_content.as_bytes()))
        .unwrap()
        .entries;
    assert_eq!(entries.len(), 4);

    for entry in &mut entries {
        annotate_subsystems(entry);
    }

    assert_eq!(
        entries[0].subsystem_id,
        Some(Subsystem::MetadataStore.to_id())
    );
    assert!(entries[0].message.starts_with("RabbitMQ metadata store:"));

    assert_eq!(
        entries[1].subsystem_id,
        Some(Subsystem::MetadataStore.to_id())
    );
    assert!(entries[1].message.starts_with("RabbitMQ metadata store:"));

    assert_eq!(
        entries[2].subsystem_id,
        Some(Subsystem::MetadataStore.to_id())
    );
    assert!(entries[2].message.starts_with("RabbitMQ metadata store:"));

    assert_eq!(entries[3].subsystem_id, None);
    assert_eq!(entries[3].message, "Regular log message");
}

#[test]
fn test_annotate_feature_flags_entries() {
    let log_content = r#"2025-10-27 11:23:27.693368-07:00 [debug] <0.264.0> Feature flags: controller standing by
2025-10-27 11:23:27.744063-07:00 [debug] <0.208.0> Feature flags: REFRESHING after applications load...
2025-10-27 11:23:27.744089-07:00 [debug] <0.264.0> Feature flags: registering controller globally
2025-10-27 11:23:27.750000-07:00 [info] <0.208.0> Some other message"#;

    let mut entries = parse_log_file(BufReader::new(log_content.as_bytes()))
        .unwrap()
        .entries;
    assert_eq!(entries.len(), 4);

    for entry in &mut entries {
        annotate_subsystems(entry);
    }

    assert_eq!(
        entries[0].subsystem_id,
        Some(Subsystem::FeatureFlags.to_id())
    );
    assert!(entries[0].message.starts_with("Feature flags:"));

    assert_eq!(
        entries[1].subsystem_id,
        Some(Subsystem::FeatureFlags.to_id())
    );
    assert!(entries[1].message.starts_with("Feature flags:"));

    assert_eq!(
        entries[2].subsystem_id,
        Some(Subsystem::FeatureFlags.to_id())
    );
    assert!(entries[2].message.starts_with("Feature flags:"));

    assert_eq!(entries[3].subsystem_id, None);
    assert_eq!(entries[3].message, "Some other message");
}

#[test]
fn test_annotate_mixed_subsystems() {
    let log_content = r#"2025-10-27 11:23:27.632069-07:00 [debug] <0.246.0> RabbitMQ metadata store: initializing
2025-10-27 11:23:27.693368-07:00 [debug] <0.264.0> Feature flags: controller standing by
2025-10-27 11:23:27.700000-07:00 [info] <0.208.0> Application started
2025-10-27 11:23:27.744063-07:00 [debug] <0.208.0> Feature flags: REFRESHING"#;

    let mut entries = parse_log_file(BufReader::new(log_content.as_bytes()))
        .unwrap()
        .entries;
    assert_eq!(entries.len(), 4);

    for entry in &mut entries {
        annotate_subsystems(entry);
    }

    assert_eq!(
        entries[0].subsystem_id,
        Some(Subsystem::MetadataStore.to_id())
    );
    assert_eq!(
        entries[1].subsystem_id,
        Some(Subsystem::FeatureFlags.to_id())
    );
    assert_eq!(entries[2].subsystem_id, None);
    assert_eq!(
        entries[3].subsystem_id,
        Some(Subsystem::FeatureFlags.to_id())
    );
}
