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

//! Unit tests for query language presets.

use rabbitmq_lqt_ql::ast::{Field, FilterExpr};
use rabbitmq_lqt_ql::parse_filter_only;
use rabbitmq_lqt_ql::presets::PresetName;
use std::str::FromStr;

#[test]
fn test_all_presets_have_valid_query_strings() {
    for preset in PresetName::all() {
        let query_string = preset.query_string();
        assert!(
            !query_string.is_empty(),
            "Preset {:?} has empty query string",
            preset
        );

        let result = parse_filter_only(query_string);
        assert!(
            result.is_ok(),
            "Preset {:?} query string '{}' failed to parse: {:?}",
            preset,
            query_string,
            result.err()
        );
    }
}

#[test]
fn test_all_presets_have_descriptions() {
    for preset in PresetName::all() {
        let description = preset.description();
        assert!(
            !description.is_empty(),
            "Preset {:?} has empty description",
            preset
        );
        assert!(
            description.len() > 10,
            "Preset {:?} has suspiciously short description: '{}'",
            preset,
            description
        );
    }
}

#[test]
fn test_to_filter_expr_returns_valid_ast() {
    for preset in PresetName::all() {
        let expr = preset.to_filter_expr();
        match expr {
            FilterExpr::Comparison(_)
            | FilterExpr::And(_, _)
            | FilterExpr::Or(_, _)
            | FilterExpr::Not(_)
            | FilterExpr::Grouped(_)
            | FilterExpr::LabelAny(_)
            | FilterExpr::LabelAll(_)
            | FilterExpr::SubsystemAny(_)
            | FilterExpr::Preset(_)
            | FilterExpr::HasDocUrl
            | FilterExpr::HasResolutionUrl
            | FilterExpr::Unlabelled => {}
        }
    }
}

#[test]
fn test_errors_preset_query_string() {
    let preset = PresetName::Errors;
    assert_eq!(preset.query_string(), r#"severity == "error""#);
}

#[test]
fn test_crashes_preset_query_string() {
    let preset = PresetName::Crashes;
    let query = preset.query_string();
    assert!(query.contains("erl_process_crash"));
    assert!(query.contains("exceptions"));
    assert!(query.contains("undefined_fn"));
    assert!(query.contains("labels any"));
}

#[test]
fn test_errors_or_crashes_preset_query_string() {
    let preset = PresetName::ErrorsOrCrashes;
    let query = preset.query_string();
    assert!(query.contains(r#"severity == "error""#));
    assert!(query.contains(" or "));
    assert!(query.contains("labels any"));
}

#[test]
fn test_disconnects_preset_query_string() {
    let preset = PresetName::Disconnects;
    let query = preset.query_string();
    assert!(query.contains("disconnects"));
    assert!(query.contains("connections"));
}

#[test]
fn test_tls_issues_preset_query_string() {
    let preset = PresetName::TlsIssues;
    assert!(preset.query_string().contains("tls"));
}

#[test]
fn test_access_denied_preset_query_string() {
    let preset = PresetName::AccessDenied;
    assert!(preset.query_string().contains("access_control"));
}

#[test]
fn test_timeouts_preset_query_string() {
    let preset = PresetName::Timeouts;
    assert!(preset.query_string().contains("timeouts"));
}

#[test]
fn test_preset_from_str() {
    assert_eq!(PresetName::from_str("errors").unwrap(), PresetName::Errors);
    assert_eq!(
        PresetName::from_str("crashes").unwrap(),
        PresetName::Crashes
    );
    assert_eq!(
        PresetName::from_str("errors_or_crashes").unwrap(),
        PresetName::ErrorsOrCrashes
    );
    assert_eq!(
        PresetName::from_str("disconnects").unwrap(),
        PresetName::Disconnects
    );
    assert_eq!(
        PresetName::from_str("tls_issues").unwrap(),
        PresetName::TlsIssues
    );
    assert_eq!(PresetName::from_str("tls").unwrap(), PresetName::TlsIssues);
    assert_eq!(
        PresetName::from_str("access_denied").unwrap(),
        PresetName::AccessDenied
    );
    assert_eq!(
        PresetName::from_str("access_control").unwrap(),
        PresetName::AccessDenied
    );
    assert_eq!(
        PresetName::from_str("timeouts").unwrap(),
        PresetName::Timeouts
    );
}

#[test]
fn test_preset_from_str_unknown() {
    let result = PresetName::from_str("unknown_preset");
    assert!(result.is_err());
}

#[test]
fn test_preset_as_str_roundtrip() {
    for preset in PresetName::all() {
        let name = preset.as_str();
        let parsed = PresetName::from_str(name).unwrap();
        assert_eq!(*preset, parsed);
    }
}

#[test]
fn test_preset_all_returns_all_variants() {
    let all = PresetName::all();
    assert_eq!(all.len(), 8);
    assert!(all.contains(&PresetName::Errors));
    assert!(all.contains(&PresetName::Crashes));
    assert!(all.contains(&PresetName::ErrorsOrCrashes));
    assert!(all.contains(&PresetName::Disconnects));
    assert!(all.contains(&PresetName::TlsIssues));
    assert!(all.contains(&PresetName::AccessDenied));
    assert!(all.contains(&PresetName::Timeouts));
    assert!(all.contains(&PresetName::RaftAndQuorumQueues));
}

#[test]
fn test_errors_preset_produces_comparison() {
    let expr = PresetName::Errors.to_filter_expr();
    match expr {
        FilterExpr::Comparison(matcher) => {
            assert_eq!(matcher.field, Field::Severity);
        }
        _ => panic!("Expected Comparison for :errors preset"),
    }
}

#[test]
fn test_crashes_preset_produces_label_any() {
    let expr = PresetName::Crashes.to_filter_expr();
    match expr {
        FilterExpr::LabelAny(labels) => {
            assert!(labels.contains(&"erl_process_crash".to_string()));
            assert!(labels.contains(&"exceptions".to_string()));
            assert!(labels.contains(&"undefined_fn".to_string()));
        }
        _ => panic!("Expected LabelAny for :crashes preset"),
    }
}

#[test]
fn test_errors_or_crashes_preset_produces_or() {
    let expr = PresetName::ErrorsOrCrashes.to_filter_expr();
    match expr {
        FilterExpr::Or(_, _) => {}
        _ => panic!("Expected Or for :errors_or_crashes preset"),
    }
}

#[test]
fn test_disconnects_preset_produces_label_any() {
    let expr = PresetName::Disconnects.to_filter_expr();
    match expr {
        FilterExpr::LabelAny(labels) => {
            assert!(labels.contains(&"disconnects".to_string()));
            assert!(labels.contains(&"connections".to_string()));
        }
        _ => panic!("Expected LabelAny for :disconnects preset"),
    }
}

#[test]
fn test_raft_and_quorum_queues_preset_query_string() {
    let preset = PresetName::RaftAndQuorumQueues;
    let query = preset.query_string();
    assert!(query.contains("raft"));
    assert!(query.contains("quorum_queues"));
    assert!(query.contains("elections"));
    assert!(query.contains("metadata_store"));
    assert!(query.contains("labels any"));
    assert!(query.contains("subsystem any"));
}

#[test]
fn test_raft_and_quorum_queues_preset_produces_or() {
    let expr = PresetName::RaftAndQuorumQueues.to_filter_expr();
    match expr {
        FilterExpr::Or(_, _) => {}
        _ => panic!("Expected Or for :raft_and_quorum_queues preset"),
    }
}

#[test]
fn test_raft_and_quorum_queues_preset_from_str() {
    assert_eq!(
        PresetName::from_str("raft_and_quorum_queues").unwrap(),
        PresetName::RaftAndQuorumQueues
    );
    assert_eq!(
        PresetName::from_str("raft").unwrap(),
        PresetName::RaftAndQuorumQueues
    );
    assert_eq!(
        PresetName::from_str("quorum_queues").unwrap(),
        PresetName::RaftAndQuorumQueues
    );
}
