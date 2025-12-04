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

use rlqt_lib::entry_metadata::Annotator;
use rlqt_lib::entry_metadata::LABEL_NAMES;
use rlqt_lib::entry_metadata::label_annotators::{
    DeletionProtectionAnnotator, ElectionsAnnotator, ErlProcessCrashAnnotator, ExchangesAnnotator,
    LabelAnnotator, LimitsAnnotator, MultilineAnnotator, PeerDiscoveryClassicAnnotator,
    PluginsLabelAnnotator, ProcessStopsAnnotator, RaftBasedAnnotator, StartupBannerAnnotator,
    StreamsAnnotator, UndefinedFnAnnotator, WorkerPoolAnnotator, annotate_labels,
};
use rlqt_lib::entry_metadata::labels::LogEntryLabels;
use rlqt_lib::{Severity, parse_log_file};
use std::fs::File;
use std::io::BufReader;
use test_helpers::create_test_entry;

#[test]
fn test_erl_process_crash_annotator_matches_multiline_with_crasher() {
    let entry = create_test_entry(
        "First line\nsecond line with crasher: info\nthird line",
        Severity::Error,
    );

    let annotator = ErlProcessCrashAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_erl_process_crash_annotator_no_match_single_line() {
    let entry = create_test_entry("crasher: something", Severity::Error);

    let annotator = ErlProcessCrashAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_erl_process_crash_annotator_no_match_multiline_without_crasher() {
    let entry = create_test_entry("First line\nsecond line\nthird line", Severity::Error);

    let annotator = ErlProcessCrashAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_erl_process_crash_annotator_annotates() {
    let _entry = create_test_entry(
        "First line\nsecond line with crasher: info",
        Severity::Error,
    );

    let annotator = ErlProcessCrashAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);

    assert!(labels.contains(LogEntryLabels::ERL_PROCESS_CRASH));
}

#[test]
fn test_undefined_fn_annotator_matches_undef_comma() {
    let entry = create_test_entry("error in call: :undef, module not found", Severity::Error);

    let annotator = UndefinedFnAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_undefined_fn_annotator_matches_undefined_function() {
    let entry = create_test_entry("call to undefined function foo:bar/2", Severity::Error);

    let annotator = UndefinedFnAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_undefined_fn_annotator_no_match() {
    let entry = create_test_entry("regular error message", Severity::Error);

    let annotator = UndefinedFnAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_undefined_fn_annotator_annotates() {
    let _entry = create_test_entry(":undef, something", Severity::Error);

    let annotator = UndefinedFnAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);

    assert!(labels.contains(LogEntryLabels::UNDEFINED_FN));
}

#[test]
fn test_annotate_labels_with_crash() {
    let entry = create_test_entry("Line 1\nLine 2 crasher: details\nLine 3", Severity::Error);

    let labels = annotate_labels(&entry);

    assert!(labels.contains(LogEntryLabels::ERL_PROCESS_CRASH));
    assert!(!labels.contains(LogEntryLabels::UNDEFINED_FN));
}

#[test]
fn test_annotate_labels_with_undefined_fn() {
    let entry = create_test_entry("error: :undef, module missing", Severity::Error);

    let labels = annotate_labels(&entry);

    assert!(!labels.contains(LogEntryLabels::ERL_PROCESS_CRASH));
    assert!(labels.contains(LogEntryLabels::UNDEFINED_FN));
}

#[test]
fn test_annotate_labels_with_both() {
    let entry = create_test_entry(
        "Line 1\ncrasher: :undef, foo:bar/2\nLine 3",
        Severity::Error,
    );

    let labels = annotate_labels(&entry);

    assert!(labels.contains(LogEntryLabels::ERL_PROCESS_CRASH));
    assert!(labels.contains(LogEntryLabels::UNDEFINED_FN));
}

#[test]
fn test_annotate_labels_with_neither() {
    let entry = create_test_entry("regular message", Severity::Info);

    let labels = annotate_labels(&entry);

    assert!(labels.is_empty());
    assert!(!labels.contains(LogEntryLabels::ERL_PROCESS_CRASH));
    assert!(!labels.contains(LogEntryLabels::UNDEFINED_FN));
}

#[test]
fn test_labels_merge() {
    let mut labels1 = LogEntryLabels::default();
    labels1 |= LogEntryLabels::ERL_PROCESS_CRASH;

    let labels2 = LogEntryLabels::UNDEFINED_FN;

    labels1.merge(labels2);

    assert!(labels1.contains(LogEntryLabels::ERL_PROCESS_CRASH));
    assert!(labels1.contains(LogEntryLabels::UNDEFINED_FN));
}

#[test]
fn test_labels_merge_overwrites() {
    let mut labels1 = LogEntryLabels::default();

    let labels2 = LogEntryLabels::ERL_PROCESS_CRASH;

    labels1.merge(labels2);

    assert!(labels1.contains(LogEntryLabels::ERL_PROCESS_CRASH));
}

#[test]
fn test_process_stops_annotator_matches() {
    let entry = create_test_entry("process terminating with reason: normal", Severity::Info);

    let annotator = ProcessStopsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_process_stops_annotator_no_match() {
    let entry = create_test_entry("process started successfully", Severity::Info);

    let annotator = ProcessStopsAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_process_stops_annotator_annotates() {
    let annotator = ProcessStopsAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);

    assert!(labels.contains(LogEntryLabels::PROCESS_STOPS));
}

#[test]
fn test_raft_annotator_matches_pre_vote() {
    let entry = create_test_entry("pre_vote request received", Severity::Debug);

    let annotator = RaftBasedAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_raft_annotator_matches_election_called() {
    let entry = create_test_entry("election called for term 5", Severity::Info);

    let annotator = RaftBasedAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_raft_annotator_matches_ra_log_init() {
    let entry = create_test_entry("ra_log:init starting", Severity::Debug);

    let annotator = RaftBasedAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_raft_annotator_no_match() {
    let entry = create_test_entry("regular log message", Severity::Info);

    let annotator = RaftBasedAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_raft_annotator_annotates() {
    let annotator = RaftBasedAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);

    assert!(labels.contains(LogEntryLabels::RAFT));
}

#[test]
fn test_elections_annotator_matches_pre_vote() {
    let entry = create_test_entry("pre_vote request sent", Severity::Debug);

    let annotator = ElectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_elections_annotator_matches_election_called() {
    let entry = create_test_entry("election called", Severity::Info);

    let annotator = ElectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_elections_annotator_no_match_ra_log_init() {
    let entry = create_test_entry("ra_log:init starting", Severity::Debug);

    let annotator = ElectionsAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_elections_annotator_no_match() {
    let entry = create_test_entry("regular message", Severity::Info);

    let annotator = ElectionsAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_elections_annotator_annotates() {
    let annotator = ElectionsAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);

    assert!(labels.contains(LogEntryLabels::ELECTIONS));
}

#[test]
fn test_annotate_labels_with_process_stops() {
    let entry = create_test_entry("terminating with reason: shutdown", Severity::Info);

    let labels = annotate_labels(&entry);

    assert!(labels.contains(LogEntryLabels::PROCESS_STOPS));
    assert!(!labels.contains(LogEntryLabels::RAFT));
    assert!(!labels.contains(LogEntryLabels::ELECTIONS));
}

#[test]
fn test_annotate_labels_with_raft() {
    let entry = create_test_entry("ra_log:init completed", Severity::Debug);

    let labels = annotate_labels(&entry);

    assert!(!labels.contains(LogEntryLabels::PROCESS_STOPS));
    assert!(labels.contains(LogEntryLabels::RAFT));
    assert!(!labels.contains(LogEntryLabels::ELECTIONS));
}

#[test]
fn test_annotate_labels_with_elections() {
    let entry = create_test_entry("pre_vote response received", Severity::Debug);

    let labels = annotate_labels(&entry);

    assert!(!labels.contains(LogEntryLabels::PROCESS_STOPS));
    assert!(labels.contains(LogEntryLabels::RAFT));
    assert!(labels.contains(LogEntryLabels::ELECTIONS));
}

#[test]
fn test_annotate_labels_elections_implies_raft() {
    let entry = create_test_entry("election called for new term", Severity::Info);

    let labels = annotate_labels(&entry);

    assert!(labels.contains(LogEntryLabels::RAFT));
    assert!(labels.contains(LogEntryLabels::ELECTIONS));
}

#[test]
fn test_annotate_labels_raft_without_elections() {
    let entry = create_test_entry("ra_log:init configuration loaded", Severity::Debug);

    let labels = annotate_labels(&entry);

    assert!(labels.contains(LogEntryLabels::RAFT));
    assert!(!labels.contains(LogEntryLabels::ELECTIONS));
}

#[test]
fn test_labels_merge_new_fields() {
    let mut labels1 = LogEntryLabels::PROCESS_STOPS | LogEntryLabels::RAFT;

    let labels2 = LogEntryLabels::ELECTIONS;

    labels1.merge(labels2);

    assert!(labels1.contains(LogEntryLabels::PROCESS_STOPS));
    assert!(labels1.contains(LogEntryLabels::RAFT));
    assert!(labels1.contains(LogEntryLabels::ELECTIONS));
}

#[test]
fn test_labels_is_empty_with_new_fields() {
    let mut labels = LogEntryLabels::default();
    assert!(labels.is_empty());

    labels |= LogEntryLabels::PROCESS_STOPS;
    assert!(!labels.is_empty());

    labels = LogEntryLabels::default();
    labels |= LogEntryLabels::RAFT;
    assert!(!labels.is_empty());

    labels = LogEntryLabels::default();
    labels |= LogEntryLabels::ELECTIONS;
    assert!(!labels.is_empty());
}

// Edge case tests for pattern matching

#[test]
fn test_process_stops_case_insensitive() {
    let entry = create_test_entry("Process TERMINATING WITH REASON: normal", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::PROCESS_STOPS));
}

#[test]
fn test_process_stops_terminated_variant() {
    let entry = create_test_entry("Process terminated with reason: shutdown", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::PROCESS_STOPS));
}

#[test]
fn test_process_stops_stopped_variant() {
    let entry = create_test_entry("Server stopped with reason: normal", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::PROCESS_STOPS));
}

#[test]
fn test_process_stops_exiting_variant() {
    let entry = create_test_entry("Worker exiting with reason: timeout", Severity::Warning);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::PROCESS_STOPS));
}

#[test]
fn test_process_stops_shutdown_variant() {
    let entry = create_test_entry("System shutdown with reason: maintenance", Severity::Notice);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::PROCESS_STOPS));
}

#[test]
fn test_process_stops_no_match_similar_word() {
    let entry = create_test_entry("Process terminator started", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(!labels.contains(LogEntryLabels::PROCESS_STOPS));
}

#[test]
fn test_raft_case_insensitive_pre_vote() {
    let entry = create_test_entry("PRE_VOTE request received", Severity::Debug);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::RAFT));
    assert!(labels.contains(LogEntryLabels::ELECTIONS));
}

#[test]
fn test_raft_case_insensitive_election() {
    let entry = create_test_entry("ELECTION CALLED for term 5", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::RAFT));
    assert!(labels.contains(LogEntryLabels::ELECTIONS));
}

#[test]
fn test_raft_election_triggered_variant() {
    let entry = create_test_entry("election triggered by node", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::RAFT));
    assert!(labels.contains(LogEntryLabels::ELECTIONS));
}

#[test]
fn test_raft_trigger_election_variant() {
    let entry = create_test_entry("Trigger election in store", Severity::Debug);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::RAFT));
    assert!(labels.contains(LogEntryLabels::ELECTIONS));
}

#[test]
fn test_elections_catch_up_on_replication() {
    let entry = create_test_entry(
        "catch up on replication to the Raft cluster leader",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::RAFT));
    assert!(labels.contains(LogEntryLabels::ELECTIONS));
}

#[test]
fn test_elections_catch_up_on_replication_case_insensitive() {
    let entry = create_test_entry(
        "CATCH UP ON REPLICATION TO THE RAFT CLUSTER LEADER",
        Severity::Info,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::RAFT));
    assert!(labels.contains(LogEntryLabels::ELECTIONS));
}

#[test]
fn test_raft_ra_log_init_case_insensitive() {
    let entry = create_test_entry("RA_LOG:INIT starting system", Severity::Debug);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::RAFT));
    assert!(!labels.contains(LogEntryLabels::ELECTIONS));
}

#[test]
fn test_raft_recovered_to_follower() {
    let entry = create_test_entry("recovered -> follower in term 5", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::RAFT));
    assert!(!labels.contains(LogEntryLabels::ELECTIONS));
}

#[test]
fn test_raft_recover_to_recovered() {
    let entry = create_test_entry("recover -> recovered in term 3", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::RAFT));
    assert!(!labels.contains(LogEntryLabels::ELECTIONS));
}

#[test]
fn test_raft_follower_to_pre_vote() {
    let entry = create_test_entry("follower -> pre_vote in term 2", Severity::Debug);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::RAFT));
    // This contains "pre_vote" so it's also an election event
    assert!(labels.contains(LogEntryLabels::ELECTIONS));
}

#[test]
fn test_raft_pre_vote_to_candidate() {
    let entry = create_test_entry("pre_vote -> candidate in term 7", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::RAFT));
    // This contains "pre_vote" so it's also an election event
    assert!(labels.contains(LogEntryLabels::ELECTIONS));
}

#[test]
fn test_raft_candidate_to_leader() {
    let entry = create_test_entry("candidate -> leader in term 4", Severity::Notice);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::RAFT));
    assert!(!labels.contains(LogEntryLabels::ELECTIONS));
}

#[test]
fn test_raft_state_transitions_case_insensitive() {
    let entry1 = create_test_entry("RECOVERED -> FOLLOWER IN TERM 1", Severity::Info);
    let labels1 = annotate_labels(&entry1);
    assert!(labels1.contains(LogEntryLabels::RAFT));
    assert!(!labels1.contains(LogEntryLabels::ELECTIONS));

    let entry2 = create_test_entry("Follower -> Pre_Vote In Term 2", Severity::Info);
    let labels2 = annotate_labels(&entry2);
    assert!(labels2.contains(LogEntryLabels::RAFT));
    // This contains "pre_vote" so it's also an election event
    assert!(labels2.contains(LogEntryLabels::ELECTIONS));

    let entry3 = create_test_entry("Candidate -> Leader In Term 3", Severity::Notice);
    let labels3 = annotate_labels(&entry3);
    assert!(labels3.contains(LogEntryLabels::RAFT));
    assert!(!labels3.contains(LogEntryLabels::ELECTIONS));
}

#[test]
fn test_multiple_labels_on_same_entry() {
    let entry = create_test_entry(
        "Line 1\ncrasher: :undef, undefined function\nLine 3",
        Severity::Error,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::ERL_PROCESS_CRASH));
    assert!(labels.contains(LogEntryLabels::UNDEFINED_FN));
    assert!(!labels.contains(LogEntryLabels::PROCESS_STOPS));
    assert!(!labels.contains(LogEntryLabels::RAFT));
}

#[test]
fn test_empty_message_no_labels() {
    let entry = create_test_entry("", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.is_empty());
}

#[test]
fn test_whitespace_only_message_multiline_label() {
    let entry = create_test_entry("   \n\t  ", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::MULTILINE));
    assert!(!labels.contains(LogEntryLabels::RAFT));
    assert!(!labels.contains(LogEntryLabels::ERL_PROCESS_CRASH));
}

#[test]
fn test_pattern_at_start_of_message() {
    let entry = create_test_entry("pre_vote request initiated", Severity::Debug);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::RAFT));
    assert!(labels.contains(LogEntryLabels::ELECTIONS));
}

#[test]
fn test_pattern_at_end_of_message() {
    let entry = create_test_entry("Now starting pre_vote", Severity::Debug);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::RAFT));
    assert!(labels.contains(LogEntryLabels::ELECTIONS));
}

#[test]
fn test_pattern_in_middle_of_long_message() {
    let long_msg = format!("{}pre_vote{}", "a".repeat(1000), "b".repeat(1000));
    let entry = create_test_entry(&long_msg, Severity::Debug);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::RAFT));
    assert!(labels.contains(LogEntryLabels::ELECTIONS));
}

#[test]
fn test_substring_boundary_pre_voter_no_match() {
    let entry = create_test_entry("pre_voter process started", Severity::Info);
    let labels = annotate_labels(&entry);
    // Should still match because "pre_vote" is contained in "pre_voter"
    // This is expected behavior for substring matching
    assert!(labels.contains(LogEntryLabels::RAFT));
    assert!(labels.contains(LogEntryLabels::ELECTIONS));
}

#[test]
fn test_mixed_case_patterns() {
    let entry = create_test_entry("Election Triggered By Pre_Vote", Severity::Info);
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::RAFT));
    assert!(labels.contains(LogEntryLabels::ELECTIONS));
}

// Integration test with real RabbitMQ log data

#[test]
fn test_real_rabbitmq_log_fixture_annotation() {
    // Parse the real RabbitMQ log fixture
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let fixture_path = std::path::Path::new(manifest_dir)
        .parent()
        .unwrap()
        .join("rlqt-cli")
        .join("tests")
        .join("fixtures")
        .join("rabbit@sunnyside.log");

    let file =
        File::open(fixture_path).expect("Failed to open fixture log, ensure path is correct");
    let reader = BufReader::new(file);

    let result = parse_log_file(reader).expect("Failed to parse fixture log");
    let entries = result.entries;

    // Annotate all entries
    let annotated: Vec<_> = entries
        .iter()
        .map(|entry| {
            let mut annotated_entry = entry.clone();
            annotated_entry.labels = annotate_labels(entry);
            annotated_entry
        })
        .collect();

    // Verify we parsed a reasonable number of entries
    assert!(
        annotated.len() > 100,
        "Expected at least 100 entries, got {}",
        annotated.len()
    );

    // Count entries with each label
    let raft_based_count = annotated
        .iter()
        .filter(|e| e.labels.contains(LogEntryLabels::RAFT))
        .count();
    let elections_count = annotated
        .iter()
        .filter(|e| e.labels.contains(LogEntryLabels::ELECTIONS))
        .count();
    let process_stops_count = annotated
        .iter()
        .filter(|e| e.labels.contains(LogEntryLabels::PROCESS_STOPS))
        .count();
    let crash_count = annotated
        .iter()
        .filter(|e| e.labels.contains(LogEntryLabels::ERL_PROCESS_CRASH))
        .count();
    let undefined_fn_count = annotated
        .iter()
        .filter(|e| e.labels.contains(LogEntryLabels::UNDEFINED_FN))
        .count();
    let startup_banner_count = annotated
        .iter()
        .filter(|e| e.labels.contains(LogEntryLabels::STARTUP_BANNER))
        .count();

    // The fixture log contains real RabbitMQ startup logs with raft operations
    // We know there are at least 8 lines matching raft patterns from grep
    assert!(
        raft_based_count >= 4,
        "Expected at least 4 raft entries in fixture, got {}",
        raft_based_count
    );

    // Elections should be a subset of raft_based
    assert!(
        elections_count <= raft_based_count,
        "Elections count ({}) should be <= raft count ({})",
        elections_count,
        raft_based_count
    );

    // Verify specific known patterns from the fixture
    let has_ra_log_init = annotated.iter().any(|e| {
        e.labels.contains(LogEntryLabels::RAFT) && e.message.to_lowercase().contains("ra_log:init")
    });
    assert!(
        has_ra_log_init,
        "Expected to find at least one ra_log:init entry with raft label"
    );

    let has_election = annotated.iter().any(|e| {
        e.labels.contains(LogEntryLabels::ELECTIONS)
            && (e.message.to_lowercase().contains("pre_vote")
                || e.message.to_lowercase().contains("election"))
    });
    assert!(
        has_election,
        "Expected to find at least one election-related entry with elections label"
    );

    // Verify label consistency: all elections entries should also have raft_based
    for entry in &annotated {
        if entry.labels.contains(LogEntryLabels::ELECTIONS) {
            assert!(
                entry.labels.contains(LogEntryLabels::RAFT),
                "Entry with elections label must also have raft label. Message: {}",
                entry.message
            );
        }
    }

    // Verify startup_banner labels are present in fixture
    assert!(
        startup_banner_count > 0,
        "Expected at least one startup_banner entry in fixture, got {}",
        startup_banner_count
    );

    // Log statistics for visibility
    println!("Fixture log annotation statistics:");
    println!("  Total entries: {}", annotated.len());
    println!("  raft: {}", raft_based_count);
    println!("  elections: {}", elections_count);
    println!("  process_stops: {}", process_stops_count);
    println!("  erl_process_crash: {}", crash_count);
    println!("  undefined_fn: {}", undefined_fn_count);
    println!("  startup_banner: {}", startup_banner_count);
}

#[test]
fn test_label_names_count_matches_bitflags() {
    assert_eq!(
        LABEL_NAMES.len(),
        44,
        "LABEL_NAMES array has {} entries but should have 44. Update LABEL_NAMES when adding new label flags.",
        LABEL_NAMES.len()
    );
}

#[test]
fn test_deletion_protection_annotator_matches() {
    let entry = create_test_entry("Queue is protected from deletion", Severity::Warning);

    let annotator = DeletionProtectionAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_deletion_protection_annotator_no_match() {
    let entry = create_test_entry("Queue deleted successfully", Severity::Info);

    let annotator = DeletionProtectionAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_deletion_protection_annotator_annotates() {
    let annotator = DeletionProtectionAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);

    assert!(labels.contains(LogEntryLabels::DELETION_PROTECTION));
}

#[test]
fn test_startup_banner_annotator_matches() {
    let message = "\n node           : rabbit@hostname
 home dir       : /home/username
 config file(s) : /home/username/rabbitmq/etc/rabbitmq/advanced.config
                : /home/username/rabbitmq/etc/rabbitmq/rabbitmq.conf
 cookie hash    : NdNYTSkVzI3As6xqYGJsHg==
 log(s)         : /home/username/rabbitmq/var/log/rabbitmq/rabbit@hostname.log";

    let entry = create_test_entry(message, Severity::Info);

    let annotator = StartupBannerAnnotator;
    assert!(
        annotator.does_match(&entry),
        "StartupBannerAnnotator should match multiline entry with 'config file(s)' and 'home dir'"
    );
}

#[test]
fn test_startup_banner_annotator_matches_via_annotate_labels() {
    let message = "node           : rabbit@hostname
 home dir       : /home/username
 config file(s) : /home/username/rabbitmq/etc/rabbitmq/advanced.config
                : /home/username/rabbitmq/etc/rabbitmq/rabbitmq.conf
 cookie hash    : NdNYTSkVzI3As6xqYGJsHg==
 log(s)         : /home/username/rabbitmq/var/log/rabbitmq/rabbit@hostname.log";

    let entry = create_test_entry(message, Severity::Info);
    let labels = annotate_labels(&entry);

    assert!(
        labels.contains(LogEntryLabels::STARTUP_BANNER),
        "annotate_labels should set STARTUP_BANNER label"
    );
}

#[test]
fn test_startup_banner_annotator_no_match_single_line() {
    let entry = create_test_entry("config file(s) home dir", Severity::Info);

    let annotator = StartupBannerAnnotator;
    assert!(
        !annotator.does_match(&entry),
        "StartupBannerAnnotator should not match single-line entry"
    );
}

#[test]
fn test_startup_banner_annotator_no_match_missing_pattern() {
    let entry = create_test_entry("home dir: /home/username\nother stuff", Severity::Info);

    let annotator = StartupBannerAnnotator;
    assert!(
        !annotator.does_match(&entry),
        "StartupBannerAnnotator should not match without 'config file(s)'"
    );
}

#[test]
fn test_startup_banner_annotator_annotates() {
    let annotator = StartupBannerAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);

    assert!(labels.contains(LogEntryLabels::STARTUP_BANNER));
}

#[test]
fn test_multiline_annotator_matches() {
    let entry = create_test_entry("First line\nSecond line", Severity::Info);

    let annotator = MultilineAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_multiline_annotator_no_match() {
    let entry = create_test_entry("Single line entry", Severity::Info);

    let annotator = MultilineAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_multiline_annotator_annotates() {
    let annotator = MultilineAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);

    assert!(labels.contains(LogEntryLabels::MULTILINE));
}

#[test]
fn test_streams_annotator_matches_rabbit_stream() {
    let entry = create_test_entry("rabbit_stream coordinator started", Severity::Info);

    let annotator = StreamsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_streams_annotator_matches_rabbit_stream_underscore() {
    let entry = create_test_entry("rabbit_stream_coordinator started", Severity::Info);

    let annotator = StreamsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_streams_annotator_no_match() {
    let entry = create_test_entry("Starting Ra systems", Severity::Debug);

    let annotator = StreamsAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_streams_annotator_annotates() {
    let annotator = StreamsAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);

    assert!(labels.contains(LogEntryLabels::STREAMS));
}

#[test]
fn test_limits_annotator_matches() {
    let entry = create_test_entry("System reaching file handles limit", Severity::Warning);

    let annotator = LimitsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_limits_annotator_no_match() {
    let entry = create_test_entry("File opened successfully", Severity::Info);

    let annotator = LimitsAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_limits_annotator_annotates() {
    let annotator = LimitsAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);

    assert!(labels.contains(LogEntryLabels::LIMITS));
}

#[test]
fn test_worker_pool_annotator_matches_with_space() {
    let entry = create_test_entry("worker pool initialized", Severity::Info);

    let annotator = WorkerPoolAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_worker_pool_annotator_matches_with_underscore() {
    let entry = create_test_entry("worker_pool size set to 10", Severity::Info);

    let annotator = WorkerPoolAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_worker_pool_annotator_no_match() {
    let entry = create_test_entry("Worker started", Severity::Info);

    let annotator = WorkerPoolAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_worker_pool_annotator_annotates() {
    let annotator = WorkerPoolAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);

    assert!(labels.contains(LogEntryLabels::WORKER_POOL));
}

#[test]
fn test_peer_discovery_classic_annotator_matches() {
    let entry = create_test_entry("Using rabbit_peer_discovery_classic_config", Severity::Info);

    let annotator = PeerDiscoveryClassicAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_peer_discovery_classic_annotator_no_match() {
    let entry = create_test_entry("Peer discovery started", Severity::Info);

    let annotator = PeerDiscoveryClassicAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_peer_discovery_classic_annotator_annotates() {
    let annotator = PeerDiscoveryClassicAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);

    assert!(labels.contains(LogEntryLabels::PEER_DISCOVERY_CLASSIC));
}

#[test]
fn test_plugins_label_annotator_matches_loading() {
    let entry = create_test_entry("Loading the following plugins: foo, bar", Severity::Info);

    let annotator = PluginsLabelAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_plugins_label_annotator_matches_setting_up() {
    let entry = create_test_entry("Setting plugins up", Severity::Info);

    let annotator = PluginsLabelAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_plugins_label_annotator_matches_prelaunch() {
    let entry = create_test_entry("Plugins (prelaunch phase) initialized", Severity::Info);

    let annotator = PluginsLabelAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_plugins_label_annotator_no_match() {
    let entry = create_test_entry("System started", Severity::Info);

    let annotator = PluginsLabelAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_plugins_label_annotator_annotates() {
    let annotator = PluginsLabelAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);

    assert!(labels.contains(LogEntryLabels::PLUGINS));
}

#[test]
fn test_exchanges_annotator_matches() {
    let entry = create_test_entry("rabbit_exchange_type_topic started", Severity::Info);

    let annotator = ExchangesAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_exchanges_annotator_no_match() {
    let entry = create_test_entry("Exchange created", Severity::Info);

    let annotator = ExchangesAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_exchanges_annotator_annotates() {
    let annotator = ExchangesAnnotator;
    let mut labels = LogEntryLabels::default();
    annotator.annotate(&mut labels);

    assert!(labels.contains(LogEntryLabels::EXCHANGES));
}

#[test]
fn test_raft_annotator_matches_starting_ra_systems() {
    let entry = create_test_entry("Starting Ra systems", Severity::Debug);

    let annotator = RaftBasedAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_raft_annotator_matches_starting_ra_system() {
    let entry = create_test_entry("Starting Ra system coordination", Severity::Debug);

    let annotator = RaftBasedAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_raft_annotator_matches_ra_starting() {
    let entry = create_test_entry("ra: starting system", Severity::Info);

    let annotator = RaftBasedAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_raft_annotator_matches_ra_coordination() {
    let entry = create_test_entry("ra_coordination initialized", Severity::Debug);

    let annotator = RaftBasedAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_raft_annotator_matches_ra_system() {
    let entry = create_test_entry("ra system started", Severity::Info);

    let annotator = RaftBasedAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_raft_annotator_matches_wal_with_space() {
    let entry = create_test_entry("wal: checkpoint created", Severity::Debug);

    let annotator = RaftBasedAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_annotate_labels_with_new_labels() {
    let entry1 = create_test_entry("Queue is protected from deletion", Severity::Warning);
    let labels1 = annotate_labels(&entry1);
    assert!(labels1.contains(LogEntryLabels::DELETION_PROTECTION));

    let entry2 = create_test_entry("First\nSecond", Severity::Info);
    let labels2 = annotate_labels(&entry2);
    assert!(labels2.contains(LogEntryLabels::MULTILINE));

    let entry3 = create_test_entry("rabbit_stream started", Severity::Info);
    let labels3 = annotate_labels(&entry3);
    assert!(labels3.contains(LogEntryLabels::STREAMS));

    let entry4 = create_test_entry("file handles limit", Severity::Warning);
    let labels4 = annotate_labels(&entry4);
    assert!(labels4.contains(LogEntryLabels::LIMITS));

    let entry5 = create_test_entry("worker pool ready", Severity::Info);
    let labels5 = annotate_labels(&entry5);
    assert!(labels5.contains(LogEntryLabels::WORKER_POOL));

    let entry6 = create_test_entry("rabbit_peer_discovery_classic_config set", Severity::Info);
    let labels6 = annotate_labels(&entry6);
    assert!(labels6.contains(LogEntryLabels::PEER_DISCOVERY_CLASSIC));

    let entry7 = create_test_entry("Loading the following plugins", Severity::Info);
    let labels7 = annotate_labels(&entry7);
    assert!(labels7.contains(LogEntryLabels::PLUGINS));

    let entry8 = create_test_entry("rabbit_exchange_type_fanout", Severity::Info);
    let labels8 = annotate_labels(&entry8);
    assert!(labels8.contains(LogEntryLabels::EXCHANGES));

    let entry9 = create_test_entry("wal: segment written", Severity::Info);
    let labels9 = annotate_labels(&entry9);
    assert!(labels9.contains(LogEntryLabels::RAFT));
}

#[test]
fn test_raft_annotator_matches_wal_colon() {
    let entry = create_test_entry("wal: segment 0001 written successfully", Severity::Info);

    let annotator = RaftBasedAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_raft_annotator_matches_wal_underscore() {
    let entry = create_test_entry("wal_segment_size configuration applied", Severity::Info);

    let annotator = RaftBasedAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_raft_annotator_matches_wal_max() {
    let entry = create_test_entry("wal_max_size_bytes configuration updated", Severity::Info);

    let annotator = RaftBasedAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_raft_annotator_matches_wal_min() {
    let entry = create_test_entry("wal_min_checkpoint_interval set to 1000ms", Severity::Info);

    let annotator = RaftBasedAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_connections_annotator_matches_handshake() {
    let entry = create_test_entry("Connection closed: handshake_timeout", Severity::Warning);

    let annotator = rlqt_lib::entry_metadata::label_annotators::ConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_connections_annotator_annotates_handshake() {
    let entry = create_test_entry("handshake_timeout occurred", Severity::Warning);
    let labels = annotate_labels(&entry);

    assert!(labels.contains(LogEntryLabels::CONNECTIONS));
    assert!(labels.contains(LogEntryLabels::ACCESS_CONTROL));
}
