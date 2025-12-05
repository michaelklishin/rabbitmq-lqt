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

use rlqt_lib::parse_log_file;
use std::io::BufReader;

#[test]
fn test_parse_invalid_month_13() {
    let input = "2025-13-01 11:23:27.566558-07:00 [notice] <0.208.0> Invalid month";
    let entries = parse_log_file(BufReader::new(input.as_bytes()))
        .unwrap()
        .entries;
    assert_eq!(entries.len(), 0);
}

#[test]
fn test_parse_invalid_month_00() {
    let input = "2025-00-01 11:23:27.566558-07:00 [notice] <0.208.0> Invalid month";
    let entries = parse_log_file(BufReader::new(input.as_bytes()))
        .unwrap()
        .entries;
    assert_eq!(entries.len(), 0);
}

#[test]
fn test_parse_invalid_day_32() {
    let input = "2025-01-32 11:23:27.566558-07:00 [notice] <0.208.0> Invalid day";
    let entries = parse_log_file(BufReader::new(input.as_bytes()))
        .unwrap()
        .entries;
    assert_eq!(entries.len(), 0);
}

#[test]
fn test_parse_invalid_day_00() {
    let input = "2025-01-00 11:23:27.566558-07:00 [notice] <0.208.0> Invalid day";
    let entries = parse_log_file(BufReader::new(input.as_bytes()))
        .unwrap()
        .entries;
    assert_eq!(entries.len(), 0);
}

#[test]
fn test_parse_invalid_february_30() {
    let input = "2025-02-30 11:23:27.566558-07:00 [notice] <0.208.0> Invalid February date";
    let entries = parse_log_file(BufReader::new(input.as_bytes()))
        .unwrap()
        .entries;
    assert_eq!(entries.len(), 0);
}

#[test]
fn test_parse_valid_february_28_non_leap() {
    let input = "2025-02-28 11:23:27.566558-07:00 [notice] <0.208.0> Valid date";
    let entries = parse_log_file(BufReader::new(input.as_bytes()))
        .unwrap()
        .entries;
    assert_eq!(entries.len(), 1);
}

#[test]
fn test_parse_valid_february_29_leap_year() {
    let input = "2024-02-29 11:23:27.566558-07:00 [notice] <0.208.0> Leap year date";
    let entries = parse_log_file(BufReader::new(input.as_bytes()))
        .unwrap()
        .entries;
    assert_eq!(entries.len(), 1);
}

#[test]
fn test_parse_invalid_february_29_non_leap() {
    let input = "2025-02-29 11:23:27.566558-07:00 [notice] <0.208.0> Not leap year";
    let entries = parse_log_file(BufReader::new(input.as_bytes()))
        .unwrap()
        .entries;
    assert_eq!(entries.len(), 0);
}

#[test]
fn test_parse_invalid_hour_24() {
    let input = "2025-01-15 24:00:00.000000-07:00 [notice] <0.208.0> Invalid hour";
    let entries = parse_log_file(BufReader::new(input.as_bytes()))
        .unwrap()
        .entries;
    assert_eq!(entries.len(), 0);
}

#[test]
fn test_parse_invalid_minute_60() {
    let input = "2025-01-15 11:60:00.000000-07:00 [notice] <0.208.0> Invalid minute";
    let entries = parse_log_file(BufReader::new(input.as_bytes()))
        .unwrap()
        .entries;
    assert_eq!(entries.len(), 0);
}

#[test]
fn test_parse_invalid_second_60() {
    let input = "2025-01-15 11:23:60.000000-07:00 [notice] <0.208.0> Invalid second";
    let entries = parse_log_file(BufReader::new(input.as_bytes()))
        .unwrap()
        .entries;
    assert_eq!(entries.len(), 0);
}

#[test]
fn test_parse_valid_boundary_time_23_59_59() {
    let input = "2025-01-15 23:59:59.999999-07:00 [notice] <0.208.0> Valid boundary";
    let entries = parse_log_file(BufReader::new(input.as_bytes()))
        .unwrap()
        .entries;
    assert_eq!(entries.len(), 1);
}

#[test]
fn test_parse_valid_midnight() {
    let input = "2025-01-15 00:00:00.000000-07:00 [notice] <0.208.0> Midnight";
    let entries = parse_log_file(BufReader::new(input.as_bytes()))
        .unwrap()
        .entries;
    assert_eq!(entries.len(), 1);
}

#[test]
fn test_parse_positive_timezone_offset() {
    let input = "2025-01-15 11:23:27.566558+05:30 [notice] <0.208.0> Positive offset";
    let entries = parse_log_file(BufReader::new(input.as_bytes()))
        .unwrap()
        .entries;
    assert_eq!(entries.len(), 1);
}

#[test]
fn test_parse_zero_timezone_offset() {
    let input = "2025-01-15 11:23:27.566558+00:00 [notice] <0.208.0> UTC";
    let entries = parse_log_file(BufReader::new(input.as_bytes()))
        .unwrap()
        .entries;
    assert_eq!(entries.len(), 1);
}

#[test]
fn test_parse_negative_timezone_offset() {
    let input = "2025-01-15 11:23:27.566558-12:00 [notice] <0.208.0> Far west";
    let entries = parse_log_file(BufReader::new(input.as_bytes()))
        .unwrap()
        .entries;
    assert_eq!(entries.len(), 1);
}

#[test]
fn test_parse_large_process_id() {
    let input = "2025-01-15 11:23:27.566558-07:00 [notice] <999.999999999.999> Large PID";
    let entries = parse_log_file(BufReader::new(input.as_bytes()))
        .unwrap()
        .entries;
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].process_id, "<999.999999999.999>");
}

#[test]
fn test_parse_preserves_message_with_special_chars() {
    let input =
        "2025-01-15 11:23:27.566558-07:00 [notice] <0.208.0> Message with <brackets> and [more]";
    let entries = parse_log_file(BufReader::new(input.as_bytes()))
        .unwrap()
        .entries;
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].message, "Message with <brackets> and [more]");
}

#[test]
fn test_parse_message_lowercased_field() {
    let input = "2025-01-15 11:23:27.566558-07:00 [notice] <0.208.0> MiXeD CaSe MESSAGE";
    let entries = parse_log_file(BufReader::new(input.as_bytes()))
        .unwrap()
        .entries;
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].message, "MiXeD CaSe MESSAGE");
    assert_eq!(entries[0].message_lowercased, "mixed case message");
}

#[test]
fn test_parse_multiline_preserves_lowercased() {
    let input = "2025-01-15 11:23:27.566558-07:00 [notice] <0.208.0> First LINE\nSecond LINE";
    let entries = parse_log_file(BufReader::new(input.as_bytes()))
        .unwrap()
        .entries;
    assert_eq!(entries.len(), 1);
    assert!(entries[0].message.contains("First LINE"));
    assert!(entries[0].message.contains("Second LINE"));
    assert!(entries[0].message_lowercased.contains("first line"));
    assert!(entries[0].message_lowercased.contains("second line"));
}

#[test]
fn test_parse_trims_trailing_whitespace() {
    let input = "2025-01-15 11:23:27.566558-07:00 [notice] <0.208.0> Message with trailing   ";
    let entries = parse_log_file(BufReader::new(input.as_bytes()))
        .unwrap()
        .entries;
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].message, "Message with trailing");
}

#[test]
fn test_parse_sequence_ids_assigned() {
    let input = r#"2025-01-15 11:23:27.000000-07:00 [notice] <0.208.0> First
2025-01-15 11:23:28.000000-07:00 [notice] <0.208.0> Second
2025-01-15 11:23:29.000000-07:00 [notice] <0.208.0> Third"#;
    let entries = parse_log_file(BufReader::new(input.as_bytes()))
        .unwrap()
        .entries;
    assert_eq!(entries.len(), 3);
    assert_eq!(entries[0].sequence_id, 0);
    assert_eq!(entries[1].sequence_id, 1);
    assert_eq!(entries[2].sequence_id, 2);
}

#[test]
fn test_parse_year_boundary_low() {
    let input = "0001-01-01 00:00:00.000000+00:00 [notice] <0.208.0> Ancient log";
    let entries = parse_log_file(BufReader::new(input.as_bytes()))
        .unwrap()
        .entries;
    assert_eq!(entries.len(), 1);
}

#[test]
fn test_parse_whitespace_only_lines_as_continuation() {
    let input = "2025-01-15 11:23:27.566558-07:00 [notice] <0.208.0> Message\n   \n  ";
    let entries = parse_log_file(BufReader::new(input.as_bytes()))
        .unwrap()
        .entries;
    assert_eq!(entries.len(), 1);
    assert!(entries[0].is_multiline());
}
