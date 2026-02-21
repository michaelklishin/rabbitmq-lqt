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

#[test]
fn test_severity_from_str() {
    assert_eq!("debug".parse::<Severity>().unwrap(), Severity::Debug);
    assert_eq!("info".parse::<Severity>().unwrap(), Severity::Info);
    assert_eq!("notice".parse::<Severity>().unwrap(), Severity::Notice);
    assert_eq!("warning".parse::<Severity>().unwrap(), Severity::Warning);
    assert_eq!("error".parse::<Severity>().unwrap(), Severity::Error);
    assert_eq!("critical".parse::<Severity>().unwrap(), Severity::Critical);
    assert!("invalid".parse::<Severity>().is_err());
}

#[test]
fn test_severity_display() {
    assert_eq!(Severity::Debug.to_string(), "debug");
    assert_eq!(Severity::Info.to_string(), "info");
    assert_eq!(Severity::Notice.to_string(), "notice");
    assert_eq!(Severity::Warning.to_string(), "warning");
    assert_eq!(Severity::Error.to_string(), "error");
    assert_eq!(Severity::Critical.to_string(), "critical");
}

#[test]
fn test_severity_as_str() {
    assert_eq!(Severity::Debug.as_str(), "debug");
    assert_eq!(Severity::Info.as_str(), "info");
    assert_eq!(Severity::Notice.as_str(), "notice");
    assert_eq!(Severity::Warning.as_str(), "warning");
    assert_eq!(Severity::Error.as_str(), "error");
    assert_eq!(Severity::Critical.as_str(), "critical");
}
