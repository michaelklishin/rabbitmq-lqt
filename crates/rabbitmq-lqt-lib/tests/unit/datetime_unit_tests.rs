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

use rabbitmq_lqt_lib::datetime::parse_datetime_flexible;

#[test]
fn date_only_format() {
    let dt = parse_datetime_flexible("2025-10-27").unwrap();
    assert_eq!(dt.format("%Y-%m-%d").to_string(), "2025-10-27");
    assert_eq!(dt.format("%H:%M:%S").to_string(), "00:00:00");
}

#[test]
fn datetime_without_timezone() {
    let dt = parse_datetime_flexible("2025-10-27 18:23:00").unwrap();
    assert_eq!(
        dt.format("%Y-%m-%d %H:%M:%S").to_string(),
        "2025-10-27 18:23:00"
    );
}

#[test]
fn rfc3339_with_utc() {
    let dt = parse_datetime_flexible("2025-10-27T18:23:00Z").unwrap();
    assert_eq!(
        dt.format("%Y-%m-%d %H:%M:%S").to_string(),
        "2025-10-27 18:23:00"
    );
}

#[test]
fn rfc3339_with_offset() {
    let dt = parse_datetime_flexible("2025-10-27T18:23:00+05:00").unwrap();
    assert_eq!(
        dt.format("%Y-%m-%d %H:%M:%S").to_string(),
        "2025-10-27 13:23:00"
    );
}

#[test]
fn natural_language_yesterday() {
    let dt = parse_datetime_flexible("yesterday");
    assert!(dt.is_ok());
}

#[test]
fn natural_language_two_days_ago() {
    let dt = parse_datetime_flexible("2 days ago");
    assert!(dt.is_ok());
}

#[test]
fn invalid_format_returns_error() {
    let result = parse_datetime_flexible("not-a-date-at-all");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Could not parse"));
    assert!(err.contains("Supported formats"));
}

#[test]
fn empty_string_returns_error() {
    assert!(parse_datetime_flexible("").is_err());
}
