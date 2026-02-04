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
use rabbitmq_lqt_lib::entry_metadata::Annotator;
use rabbitmq_lqt_lib::entry_metadata::subsystem_annotators::{
    ChannelsAnnotator, SubsystemAnnotator,
};
use rabbitmq_lqt_lib::entry_metadata::subsystems::Subsystem;
use test_helpers::create_test_entry;

#[test]
fn test_channels_annotator_matches_consumer_timed_out() {
    let entry = create_test_entry(
        "Consumer <0.123.0> in queue 'test' timed out",
        Severity::Warning,
    );
    let annotator = ChannelsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_channels_annotator_matches_consumer_timeout_atom() {
    let entry = create_test_entry("Channel exception: consumer_timeout", Severity::Error);
    let annotator = ChannelsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_channels_annotator_case_insensitive() {
    let entry = create_test_entry("CONSUMER timed out waiting for ack", Severity::Warning);
    let annotator = ChannelsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_channels_annotator_no_match_unrelated() {
    let entry = create_test_entry("Channel opened successfully", Severity::Info);
    let annotator = ChannelsAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_channels_annotator_annotate_sets_subsystem() {
    let mut entry = create_test_entry(
        "Consumer in queue 'test' timed out after 30s",
        Severity::Warning,
    );
    let annotator = ChannelsAnnotator;
    annotator.annotate(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Channels.to_id()));
}
