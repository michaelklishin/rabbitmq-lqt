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

use rabbitmq_lqt_lib::Severity;
use rabbitmq_lqt_lib::parser::IncrementalParser;

#[test]
fn single_line_entry_returned_on_flush() {
    let mut parser = IncrementalParser::new(0);
    let result =
        parser.feed_line("2025-10-27 11:23:27.566558-07:00 [notice] <0.208.0> Logging: configured log handlers are now ACTIVE");
    assert!(result.is_none());

    let entry = parser.flush().expect("should have a buffered entry");
    assert_eq!(entry.severity, Severity::Notice);
    assert_eq!(
        entry.message,
        "Logging: configured log handlers are now ACTIVE"
    );
    assert_eq!(entry.sequence_id, 0);
}

#[test]
fn two_entries_first_returned_on_second_feed() {
    let mut parser = IncrementalParser::new(0);
    let r1 =
        parser.feed_line("2025-10-27 11:23:27.566558-07:00 [notice] <0.208.0> Logging: configured log handlers are now ACTIVE");
    assert!(r1.is_none());

    let r2 =
        parser.feed_line("2025-10-27 11:23:27.566588-07:00 [debug] <0.208.0> Starting Ra systems");
    let entry1 = r2.expect("first entry should be returned");
    assert_eq!(entry1.severity, Severity::Notice);
    assert_eq!(entry1.sequence_id, 0);

    let entry2 = parser.flush().expect("second entry should be in buffer");
    assert_eq!(entry2.severity, Severity::Debug);
    assert_eq!(entry2.message, "Starting Ra systems");
    assert_eq!(entry2.sequence_id, 1);
}

#[test]
fn multiline_continuation_merged() {
    let mut parser = IncrementalParser::new(0);
    parser.feed_line("2025-10-27 11:23:27.568937-07:00 [debug] <0.208.0> Starting Ra system called \"coordination\" with configuration:");
    parser.feed_line("#{message_queue_data => off_heap,name => coordination,");
    parser.feed_line("  names => ra_coordination}");

    let r =
        parser.feed_line("2025-10-27 11:23:28.566588-07:00 [debug] <0.208.0> Starting Ra systems");
    let entry = r.expect("first multiline entry should be returned");
    assert!(entry.message.contains("Starting Ra system called"));
    assert!(entry.message.contains("message_queue_data"));
    assert!(entry.message.contains("ra_coordination"));
    assert_eq!(entry.message.matches('\n').count(), 2);
}

#[test]
fn same_timestamp_and_pid_entries_are_continuation() {
    let mut parser = IncrementalParser::new(0);
    parser.feed_line("2025-10-27 11:23:27.568937-07:00 [debug] <0.208.0> Starting Ra system called \"coordination\" with configuration:");
    parser.feed_line("2025-10-27 11:23:27.568937-07:00 [debug] <0.208.0> #{message_queue_data => off_heap,name => coordination,");

    let r = parser.feed_line(
        "2025-10-27 11:23:27.566588-07:00 [info] <0.208.0> accepting AMQP connection <0.301.0>",
    );
    let entry = r.expect("merged entry should be returned");
    assert!(entry.message.contains("Starting Ra system"));
    assert!(entry.message.contains("message_queue_data"));
}

#[test]
fn flush_on_empty_parser_returns_none() {
    let mut parser = IncrementalParser::new(0);
    assert!(parser.flush().is_none());
}

#[test]
fn flush_after_flush_returns_none() {
    let mut parser = IncrementalParser::new(0);
    parser.feed_line("2025-10-27 11:23:27.566558-07:00 [notice] <0.208.0> Logging: configured log handlers are now ACTIVE");
    parser.flush();
    assert!(parser.flush().is_none());
}

#[test]
fn orphaned_continuation_lines_are_ignored() {
    let mut parser = IncrementalParser::new(0);
    let r = parser.feed_line("    orphaned line without header");
    assert!(r.is_none());
    assert!(parser.flush().is_none());
}

#[test]
fn sequence_ids_start_from_given_value() {
    let mut parser = IncrementalParser::new(42);
    parser.feed_line("2025-10-27 11:23:27.566558-07:00 [notice] <0.208.0> Logging: configured log handlers are now ACTIVE");

    let entry1 =
        parser.feed_line("2025-10-27 11:23:28.566588-07:00 [debug] <0.208.0> Starting Ra systems");
    assert_eq!(entry1.unwrap().sequence_id, 42);

    let entry2 = parser.feed_line(
        "2025-10-27 11:23:29.566600-07:00 [info] <0.301.0> accepting AMQP connection <0.301.0>",
    );
    assert_eq!(entry2.unwrap().sequence_id, 43);

    let entry3 = parser.flush().unwrap();
    assert_eq!(entry3.sequence_id, 44);
}

#[test]
fn ansi_codes_stripped() {
    let mut parser = IncrementalParser::new(0);
    parser.feed_line("\x1b[38;5;214m2025-10-19 07:31:54.382157+00:00 [warning] <0.5734745.0> HTTP access denied: user 'default_user_dOLaJvyryUn2w047Ds8' - invalid credentials\x1b[0m");
    let entry = parser.flush().unwrap();
    assert_eq!(entry.severity, Severity::Warning);
    assert_eq!(
        entry.message,
        "HTTP access denied: user 'default_user_dOLaJvyryUn2w047Ds8' - invalid credentials"
    );
}

#[test]
fn sasl_report_header_parsed() {
    let mut parser = IncrementalParser::new(0);
    parser.feed_line("=INFO REPORT==== 4-Dec-2025::19:22:30.888840 ===");
    parser.feed_line("    alarm_handler: {set,{system_memory_high_watermark,[]}}");

    let r =
        parser.feed_line("2025-12-04 19:22:32.792199+00:00 [warning] <0.153.0> Both disk_free_limit.absolute and disk_free_limit.relative are configured");
    let entry = r.expect("SASL entry should be returned");
    assert_eq!(entry.severity, Severity::Info);
    assert!(entry.message.contains("alarm_handler"));
}

#[test]
fn empty_lines_appended_as_continuation() {
    let mut parser = IncrementalParser::new(0);
    parser.feed_line("2025-10-27 11:23:27.566558-07:00 [notice] <0.208.0> Logging: configured log handlers are now ACTIVE");
    parser.feed_line("");
    let r =
        parser.feed_line("2025-10-27 11:23:27.566588-07:00 [debug] <0.208.0> Starting Ra systems");
    let entry = r.unwrap();
    assert!(
        entry
            .message
            .contains("Logging: configured log handlers are now ACTIVE")
    );
    assert_eq!(entry.message.matches('\n').count(), 1);
}

#[test]
fn multiple_sasl_entries_parsed_separately() {
    let mut parser = IncrementalParser::new(0);
    parser.feed_line("=INFO REPORT==== 4-Dec-2025::19:22:30.888840 ===");
    parser.feed_line("    alarm_handler: {set,{system_memory_high_watermark,[]}}");

    let r = parser.feed_line("=WARNING REPORT==== 4-Dec-2025::19:22:31.000000 ===");
    let entry1 = r.expect("first SASL entry should be returned");
    assert_eq!(entry1.severity, Severity::Info);
    assert!(entry1.message.contains("alarm_handler"));

    parser.feed_line("    Something went wrong");
    let entry2 = parser
        .flush()
        .expect("second SASL entry should be in buffer");
    assert_eq!(entry2.severity, Severity::Warning);
    assert!(entry2.message.contains("Something went wrong"));
}

#[test]
fn mixed_sasl_and_standard_entries() {
    let mut parser = IncrementalParser::new(0);
    parser.feed_line("=INFO REPORT==== 4-Dec-2025::19:22:30.888840 ===");
    parser.feed_line("    alarm_handler: {set,{system_memory_high_watermark,[]}}");

    let r = parser.feed_line(
        "2025-12-04 19:22:32.792199+00:00 [warning] <0.153.0> Both disk_free_limit.absolute and disk_free_limit.relative are configured",
    );
    let sasl_entry = r.expect("SASL entry should be returned");
    assert_eq!(sasl_entry.severity, Severity::Info);

    let standard_entry = parser.flush().expect("standard entry in buffer");
    assert_eq!(standard_entry.severity, Severity::Warning);
    assert!(standard_entry.message.contains("disk_free_limit"));
}

#[test]
fn feed_after_flush_starts_fresh() {
    let mut parser = IncrementalParser::new(0);
    parser.feed_line("2025-10-27 11:23:27.566558-07:00 [notice] <0.208.0> Logging: configured log handlers are now ACTIVE");
    let entry1 = parser.flush().unwrap();
    assert_eq!(entry1.sequence_id, 0);

    parser.feed_line(
        "2025-10-27 11:23:28.566588-07:00 [info] <0.301.0> accepting AMQP connection <0.301.0>",
    );
    let entry2 = parser.flush().unwrap();
    assert_eq!(entry2.sequence_id, 1);
    assert!(entry2.message.contains("accepting AMQP connection"));
}
