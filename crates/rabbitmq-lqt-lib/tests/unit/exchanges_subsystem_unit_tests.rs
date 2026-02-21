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

use crate::test_helpers::create_test_entry;
use rabbitmq_lqt_lib::Severity;
use rabbitmq_lqt_lib::entry_metadata::Annotator;
use rabbitmq_lqt_lib::entry_metadata::subsystem_annotators::{
    ExchangesAnnotator, SubsystemAnnotator,
};
use rabbitmq_lqt_lib::entry_metadata::subsystems::Subsystem;

#[test]
fn test_exchanges_annotator_matches_consistent_hashing() {
    let entry = create_test_entry(
        "Consistent hashing exchange: rebalancing bindings",
        Severity::Info,
    );
    let annotator = ExchangesAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_exchanges_annotator_case_insensitive() {
    let entry = create_test_entry("CONSISTENT HASHING EXCHANGE: something", Severity::Info);
    let annotator = ExchangesAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_exchanges_annotator_no_match_unrelated() {
    let entry = create_test_entry("Exchange declared successfully", Severity::Info);
    let annotator = ExchangesAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_exchanges_annotator_annotate_sets_subsystem() {
    let mut entry = create_test_entry(
        "Consistent hashing exchange: operation completed",
        Severity::Info,
    );
    let annotator = ExchangesAnnotator;
    annotator.annotate(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Exchanges.to_id()));
}
