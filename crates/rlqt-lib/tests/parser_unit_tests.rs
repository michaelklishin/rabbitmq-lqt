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

use rlqt_lib::{Severity, parse_log_file};
use std::io::BufReader;

#[test]
fn test_parse_single_line_entry() {
    let input = "2025-10-27 11:23:27.566558-07:00 [notice] <0.208.0> Logging: configured log handlers are now ACTIVE";
    let entries = parse_log_file(BufReader::new(input.as_bytes()))
        .unwrap()
        .entries;
    assert_eq!(entries.len(), 1);

    assert_eq!(entries[0].severity, Severity::Notice);
    assert_eq!(entries[0].process_id, "<0.208.0>");
    assert_eq!(
        entries[0].message,
        "Logging: configured log handlers are now ACTIVE"
    );
}

#[test]
fn test_parse_rabbitmq_multiline_continuation() {
    let input = r#"2025-10-28 08:47:30.773425-07:00 [debug] <0.5470694.0> Inserted a virtual host record {vhost,<<"\"1@58:">>,[],
2025-10-28 08:47:30.773425-07:00 [debug] <0.5470694.0>                                       #{description => <<>>,tags => [],
2025-10-28 08:47:30.773425-07:00 [debug] <0.5470694.0>                                         default_queue_type => <<"classic">>}}
2025-10-28 08:47:30.773500-07:00 [info] <0.208.0> Next message"#;

    let entries = parse_log_file(BufReader::new(input.as_bytes()))
        .unwrap()
        .entries;
    assert_eq!(entries.len(), 2);

    assert_eq!(entries[0].severity, Severity::Debug);
    assert_eq!(entries[0].process_id, "<0.5470694.0>");
    assert!(
        entries[0]
            .message
            .contains("Inserted a virtual host record")
    );
    assert!(
        entries[0]
            .message
            .contains("#{description => <<>>,tags => []")
    );
    assert!(
        entries[0]
            .message
            .contains(r#"default_queue_type => <<"classic">>"#)
    );

    assert_eq!(entries[0].message.matches('\n').count(), 2);

    assert_eq!(entries[1].severity, Severity::Info);
    assert_eq!(entries[1].message, "Next message");
}

#[test]
fn test_parse_multiple_separate_entries_same_timestamp() {
    let input = r#"2025-10-27 11:23:27.568937-07:00 [debug] <0.208.0> First message
2025-10-27 11:23:27.568937-07:00 [debug] <0.208.0> Second message
2025-10-27 11:23:27.568937-07:00 [info] <0.208.0> Third message with different severity"#;

    let entries = parse_log_file(BufReader::new(input.as_bytes()))
        .unwrap()
        .entries;
    assert_eq!(entries.len(), 2);

    assert!(entries[0].message.contains("First message"));
    assert!(entries[0].message.contains("Second message"));

    assert_eq!(entries[1].severity, Severity::Info);
    assert_eq!(entries[1].message, "Third message with different severity");
}

#[test]
fn test_parse_actual_multiline_continuation() {
    let input = r#"2025-10-27 11:23:27.566558-07:00 [notice] <0.208.0> This is a message
that continues on the next line
and the line after that
2025-10-27 11:23:27.566588-07:00 [debug] <0.208.0> Next message"#;

    let entries = parse_log_file(BufReader::new(input.as_bytes()))
        .unwrap()
        .entries;
    assert_eq!(entries.len(), 2);

    assert_eq!(entries[0].severity, Severity::Notice);
    assert!(entries[0].message.contains("This is a message"));
    assert!(entries[0].message.contains("that continues"));
    assert!(entries[0].message.contains("and the line after"));

    assert_eq!(entries[1].severity, Severity::Debug);
    assert_eq!(entries[1].message, "Next message");
}

#[test]
fn test_parse_valid_timestamp() {
    let input = "2025-10-27 11:23:27.566558-07:00 [notice] <0.208.0> Test message";
    let entries = parse_log_file(BufReader::new(input.as_bytes()))
        .unwrap()
        .entries;
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].message, "Test message");
}

#[test]
fn test_parse_invalid_timestamp() {
    let input = "2025-13-32 25:99:99.000000+00:00 [notice] <0.208.0> Invalid timestamp";
    let entries = parse_log_file(BufReader::new(input.as_bytes()))
        .unwrap()
        .entries;
    assert_eq!(entries.len(), 0);
}

#[test]
fn test_parse_all_severity_levels() {
    let input = r#"2025-10-27 11:23:27.566558-07:00 [notice] <0.208.0> Notice message
2025-10-27 11:23:27.566558-07:00 [debug] <0.208.0> Debug message
2025-10-27 11:23:27.566558-07:00 [info] <0.208.0> Info message
2025-10-27 11:23:27.566558-07:00 [warning] <0.208.0> Warning message
2025-10-27 11:23:27.566558-07:00 [error] <0.208.0> Error message
2025-10-27 11:23:27.566558-07:00 [critical] <0.208.0> Critical message"#;

    let entries = parse_log_file(BufReader::new(input.as_bytes()))
        .unwrap()
        .entries;
    assert_eq!(entries.len(), 6);

    assert_eq!(entries[0].severity, Severity::Notice);
    assert_eq!(entries[1].severity, Severity::Debug);
    assert_eq!(entries[2].severity, Severity::Info);
    assert_eq!(entries[3].severity, Severity::Warning);
    assert_eq!(entries[4].severity, Severity::Error);
    assert_eq!(entries[5].severity, Severity::Critical);
}

#[test]
fn test_parse_invalid_severity() {
    let input = "2025-10-27 11:23:27.566558-07:00 [unknown] <0.208.0> Invalid severity";
    let entries = parse_log_file(BufReader::new(input.as_bytes()))
        .unwrap()
        .entries;
    assert_eq!(entries.len(), 0);
}

#[test]
fn test_parse_valid_process_id() {
    let input = "2025-10-27 11:23:27.566558-07:00 [notice] <0.208.0> Test message";
    let entries = parse_log_file(BufReader::new(input.as_bytes()))
        .unwrap()
        .entries;
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].process_id, "<0.208.0>");
}

#[test]
fn test_parse_invalid_process_id() {
    let input1 = "2025-10-27 11:23:27.566558-07:00 [notice] <invalid> Invalid PID";
    let entries1 = parse_log_file(BufReader::new(input1.as_bytes()))
        .unwrap()
        .entries;
    assert_eq!(entries1.len(), 0);

    let input2 = "2025-10-27 11:23:27.566558-07:00 [notice] 0.208.0 Missing brackets";
    let entries2 = parse_log_file(BufReader::new(input2.as_bytes()))
        .unwrap()
        .entries;
    assert_eq!(entries2.len(), 0);

    let input3 = "2025-10-27 11:23:27.566558-07:00 [notice] <0.208> Incomplete PID";
    let entries3 = parse_log_file(BufReader::new(input3.as_bytes()))
        .unwrap()
        .entries;
    assert_eq!(entries3.len(), 0);
}

#[test]
fn test_parse_empty_input() {
    let result = parse_log_file(BufReader::new("".as_bytes())).unwrap();
    assert_eq!(result.entries.len(), 0);
    assert_eq!(result.total_lines, 0);
}

#[test]
fn test_parse_orphaned_continuation() {
    let input = "    orphaned line without header\n    another orphaned line";
    let entries = parse_log_file(BufReader::new(input.as_bytes()))
        .unwrap()
        .entries;
    assert_eq!(entries.len(), 0);
}

#[test]
fn test_parse_counts_lines_with_single_line_entry() {
    let input = "2025-10-27 11:23:27.566558-07:00 [notice] <0.208.0> Message";
    let result = parse_log_file(BufReader::new(input.as_bytes())).unwrap();
    assert_eq!(result.entries.len(), 1);
    assert_eq!(result.total_lines, 1);
}

#[test]
fn test_parse_counts_all_lines_in_multiline_entry() {
    let input = "2025-10-27 11:23:27.566558-07:00 [notice] <0.208.0> First line\ncontinuation\nanother line";
    let result = parse_log_file(BufReader::new(input.as_bytes())).unwrap();
    assert_eq!(result.entries.len(), 1);
    assert_eq!(result.total_lines, 3);
}

#[test]
fn test_parse_counts_lines_correctly_with_multiple_entries() {
    let input = "2025-10-27 11:23:27.566558-07:00 [notice] <0.208.0> First\n2025-10-27 11:23:27.566559-07:00 [info] <0.208.0> Second\n2025-10-27 11:23:27.566560-07:00 [debug] <0.208.0> Third";
    let result = parse_log_file(BufReader::new(input.as_bytes())).unwrap();
    assert_eq!(result.entries.len(), 3);
    assert_eq!(result.total_lines, 3);
}
