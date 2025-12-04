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
use rlqt_lib::entry_metadata::Annotator;
use rlqt_lib::entry_metadata::subsystem_annotators::{ErlangOtpAnnotator, SubsystemAnnotator};
use rlqt_lib::entry_metadata::subsystems::Subsystem;
use test_helpers::create_test_entry;

#[test]
fn test_erlang_otp_annotator_matches_supervisor_unexpected_message() {
    let entry = create_test_entry(
        "Supervisor received unexpected message: {timeout, ref}",
        Severity::Warning,
    );
    let annotator = ErlangOtpAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_erlang_otp_annotator_case_insensitive() {
    let entry = create_test_entry(
        "SUPERVISOR RECEIVED UNEXPECTED MESSAGE: something",
        Severity::Warning,
    );
    let annotator = ErlangOtpAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_erlang_otp_annotator_no_match_unrelated() {
    let entry = create_test_entry("Some other Erlang message", Severity::Info);
    let annotator = ErlangOtpAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_erlang_otp_annotator_annotate_sets_subsystem() {
    let mut entry = create_test_entry(
        "Supervisor received unexpected message: test",
        Severity::Warning,
    );
    let annotator = ErlangOtpAnnotator;
    annotator.annotate(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::ErlangOtp.to_id()));
}
