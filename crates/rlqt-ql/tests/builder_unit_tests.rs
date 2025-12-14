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

//! Unit tests for the builder API.

use rlqt_ql::ast::{DurationUnit, Field, FilterExpr, MatchOp, PipelineStage, SortDirection, Value};
use rlqt_ql::builder::{FilterBuilder, QueryBuilder, and, comparison, not, or};
use rlqt_ql::presets::PresetName;

#[test]
fn test_empty_builder() {
    let query = QueryBuilder::new().build();
    assert!(query.time_range.is_none());
    assert!(query.selector.is_none());
    assert!(query.filter.is_none());
    assert!(query.pipeline.is_empty());
}

#[test]
fn test_time_range_hours() {
    let query = QueryBuilder::new().last_hours(24).build();
    let duration = query.time_range.expect("should have time range");
    assert_eq!(duration.value, 24);
    assert_eq!(duration.unit, DurationUnit::Hours);
}

#[test]
fn test_time_range_minutes() {
    let query = QueryBuilder::new().last_minutes(30).build();
    let duration = query.time_range.expect("should have time range");
    assert_eq!(duration.value, 30);
    assert_eq!(duration.unit, DurationUnit::Minutes);
}

#[test]
fn test_time_range_seconds() {
    let query = QueryBuilder::new().last_seconds(60).build();
    let duration = query.time_range.expect("should have time range");
    assert_eq!(duration.value, 60);
    assert_eq!(duration.unit, DurationUnit::Seconds);
}

#[test]
fn test_time_range_days() {
    let query = QueryBuilder::new().last_days(7).build();
    let duration = query.time_range.expect("should have time range");
    assert_eq!(duration.value, 7);
    assert_eq!(duration.unit, DurationUnit::Days);
}

#[test]
fn test_time_range_weeks() {
    let query = QueryBuilder::new().last_weeks(2).build();
    let duration = query.time_range.expect("should have time range");
    assert_eq!(duration.value, 2);
    assert_eq!(duration.unit, DurationUnit::Weeks);
}

#[test]
fn test_severity_selector() {
    let query = QueryBuilder::new().severity("error").build();
    let selector = query.selector.expect("should have selector");
    assert_eq!(selector.matchers.len(), 1);
    assert_eq!(selector.matchers[0].field, Field::Severity);
    assert_eq!(selector.matchers[0].op, MatchOp::Eq);
    assert_eq!(
        selector.matchers[0].value,
        Value::String("error".to_string())
    );
}

#[test]
fn test_subsystem_selector() {
    let query = QueryBuilder::new().subsystem("connections").build();
    let selector = query.selector.expect("should have selector");
    assert_eq!(selector.matchers.len(), 1);
    assert_eq!(selector.matchers[0].field, Field::Subsystem);
}

#[test]
fn test_node_selector() {
    let query = QueryBuilder::new().node("rabbit@host1").build();
    let selector = query.selector.expect("should have selector");
    assert_eq!(selector.matchers.len(), 1);
    assert_eq!(selector.matchers[0].field, Field::Node);
    assert_eq!(
        selector.matchers[0].value,
        Value::String("rabbit@host1".to_string())
    );
}

#[test]
fn test_erlang_pid_selector() {
    let query = QueryBuilder::new().erlang_pid("<0.208.0>").build();
    let selector = query.selector.expect("should have selector");
    assert_eq!(selector.matchers.len(), 1);
    assert_eq!(selector.matchers[0].field, Field::ErlangPid);
}

#[test]
fn test_multiple_selectors() {
    let query = QueryBuilder::new()
        .severity("warning")
        .subsystem("connections")
        .build();
    let selector = query.selector.expect("should have selector");
    assert_eq!(selector.matchers.len(), 2);
}

#[test]
fn test_message_contains_filter() {
    let query = QueryBuilder::new()
        .message_contains("connection refused")
        .build();
    let filter = query.filter.expect("should have filter");
    match filter {
        FilterExpr::Comparison(matcher) => {
            assert_eq!(matcher.field, Field::Message);
            assert_eq!(matcher.op, MatchOp::IContains);
        }
        _ => panic!("expected comparison filter"),
    }
}

#[test]
fn test_message_matches_filter() {
    let query = QueryBuilder::new()
        .message_matches("timeout.*exceeded")
        .build();
    let filter = query.filter.expect("should have filter");
    match filter {
        FilterExpr::Comparison(matcher) => {
            assert_eq!(matcher.field, Field::Message);
            assert_eq!(matcher.op, MatchOp::Regex);
        }
        _ => panic!("expected comparison filter"),
    }
}

#[test]
fn test_message_not_matches_filter() {
    let query = QueryBuilder::new()
        .message_not_matches("startup_banner")
        .build();
    let filter = query.filter.expect("should have filter");
    match filter {
        FilterExpr::Comparison(matcher) => {
            assert_eq!(matcher.field, Field::Message);
            assert_eq!(matcher.op, MatchOp::NotRegex);
        }
        _ => panic!("expected comparison filter"),
    }
}

#[test]
fn test_labels_any_filter() {
    let query = QueryBuilder::new()
        .labels_any(vec!["connections".to_string(), "disconnects".to_string()])
        .build();
    let filter = query.filter.expect("should have filter");
    match filter {
        FilterExpr::LabelAny(labels) => {
            assert_eq!(labels, vec!["connections", "disconnects"]);
        }
        _ => panic!("expected labels any filter"),
    }
}

#[test]
fn test_labels_all_filter() {
    let query = QueryBuilder::new()
        .labels_all(vec!["connections".to_string(), "networking".to_string()])
        .build();
    let filter = query.filter.expect("should have filter");
    match filter {
        FilterExpr::LabelAll(labels) => {
            assert_eq!(labels, vec!["connections", "networking"]);
        }
        _ => panic!("expected labels all filter"),
    }
}

#[test]
fn test_has_label_filter() {
    let query = QueryBuilder::new().has_label("disconnects").build();
    let filter = query.filter.expect("should have filter");
    match filter {
        FilterExpr::Comparison(matcher) => {
            assert_eq!(matcher.field, Field::Labels);
            assert_eq!(matcher.op, MatchOp::HasLabel);
        }
        _ => panic!("expected comparison filter"),
    }
}

#[test]
fn test_has_doc_url_filter() {
    let query = QueryBuilder::new().has_doc_url().build();
    let filter = query.filter.expect("should have filter");
    assert!(matches!(filter, FilterExpr::HasDocUrl));
}

#[test]
fn test_has_resolution_url_filter() {
    let query = QueryBuilder::new().has_resolution_url().build();
    let filter = query.filter.expect("should have filter");
    assert!(matches!(filter, FilterExpr::HasResolutionUrl));
}

#[test]
fn test_unlabelled_filter() {
    let query = QueryBuilder::new().unlabelled().build();
    let filter = query.filter.expect("should have filter");
    assert!(matches!(filter, FilterExpr::Unlabelled));
}

#[test]
fn test_preset_errors() {
    let query = QueryBuilder::new().errors().build();
    let filter = query.filter.expect("should have filter");
    assert!(matches!(filter, FilterExpr::Preset(PresetName::Errors)));
}

#[test]
fn test_preset_crashes() {
    let query = QueryBuilder::new().crashes().build();
    let filter = query.filter.expect("should have filter");
    assert!(matches!(filter, FilterExpr::Preset(PresetName::Crashes)));
}

#[test]
fn test_preset_errors_or_crashes() {
    let query = QueryBuilder::new().errors_or_crashes().build();
    let filter = query.filter.expect("should have filter");
    assert!(matches!(
        filter,
        FilterExpr::Preset(PresetName::ErrorsOrCrashes)
    ));
}

#[test]
fn test_preset_disconnects() {
    let query = QueryBuilder::new().disconnects().build();
    let filter = query.filter.expect("should have filter");
    assert!(matches!(
        filter,
        FilterExpr::Preset(PresetName::Disconnects)
    ));
}

#[test]
fn test_preset_tls_issues() {
    let query = QueryBuilder::new().tls_issues().build();
    let filter = query.filter.expect("should have filter");
    assert!(matches!(filter, FilterExpr::Preset(PresetName::TlsIssues)));
}

#[test]
fn test_preset_access_denied() {
    let query = QueryBuilder::new().access_denied().build();
    let filter = query.filter.expect("should have filter");
    assert!(matches!(
        filter,
        FilterExpr::Preset(PresetName::AccessDenied)
    ));
}

#[test]
fn test_preset_timeouts() {
    let query = QueryBuilder::new().timeouts().build();
    let filter = query.filter.expect("should have filter");
    assert!(matches!(filter, FilterExpr::Preset(PresetName::Timeouts)));
}

#[test]
fn test_multiple_filters_combined_with_and() {
    let query = QueryBuilder::new()
        .message_contains("connection")
        .has_label("disconnects")
        .build();
    let filter = query.filter.expect("should have filter");
    assert!(matches!(filter, FilterExpr::And(_, _)));
}

#[test]
fn test_limit_pipeline() {
    let query = QueryBuilder::new().limit(100).build();
    assert_eq!(query.pipeline.len(), 1);
    assert!(matches!(query.pipeline[0], PipelineStage::Limit(100)));
}

#[test]
fn test_offset_pipeline() {
    let query = QueryBuilder::new().offset(50).build();
    assert_eq!(query.pipeline.len(), 1);
    assert!(matches!(query.pipeline[0], PipelineStage::Offset(50)));
}

#[test]
fn test_head_pipeline() {
    let query = QueryBuilder::new().head(10).build();
    assert_eq!(query.pipeline.len(), 1);
    assert!(matches!(query.pipeline[0], PipelineStage::Head(10)));
}

#[test]
fn test_tail_pipeline() {
    let query = QueryBuilder::new().tail(25).build();
    assert_eq!(query.pipeline.len(), 1);
    assert!(matches!(query.pipeline[0], PipelineStage::Tail(25)));
}

#[test]
fn test_sort_pipeline() {
    let query = QueryBuilder::new()
        .sort(Field::Timestamp, SortDirection::Desc)
        .build();
    assert_eq!(query.pipeline.len(), 1);
    match &query.pipeline[0] {
        PipelineStage::Sort(spec) => {
            assert_eq!(spec.field, Field::Timestamp);
            assert_eq!(spec.direction, SortDirection::Desc);
        }
        _ => panic!("expected sort stage"),
    }
}

#[test]
fn test_sort_by_timestamp_desc() {
    let query = QueryBuilder::new().sort_by_timestamp_desc().build();
    match &query.pipeline[0] {
        PipelineStage::Sort(spec) => {
            assert_eq!(spec.field, Field::Timestamp);
            assert_eq!(spec.direction, SortDirection::Desc);
        }
        _ => panic!("expected sort stage"),
    }
}

#[test]
fn test_sort_by_timestamp_asc() {
    let query = QueryBuilder::new().sort_by_timestamp_asc().build();
    match &query.pipeline[0] {
        PipelineStage::Sort(spec) => {
            assert_eq!(spec.field, Field::Timestamp);
            assert_eq!(spec.direction, SortDirection::Asc);
        }
        _ => panic!("expected sort stage"),
    }
}

#[test]
fn test_project_pipeline() {
    let query = QueryBuilder::new()
        .project(vec![Field::Timestamp, Field::Severity, Field::Message])
        .build();
    assert_eq!(query.pipeline.len(), 1);
    match &query.pipeline[0] {
        PipelineStage::Project(fields) => {
            assert_eq!(fields.len(), 3);
        }
        _ => panic!("expected project stage"),
    }
}

#[test]
fn test_count_by_pipeline() {
    let query = QueryBuilder::new().count_by(Some(Field::Severity)).build();
    assert_eq!(query.pipeline.len(), 1);
    match &query.pipeline[0] {
        PipelineStage::CountBy(Some(field)) => {
            assert_eq!(*field, Field::Severity);
        }
        _ => panic!("expected count by stage"),
    }
}

#[test]
fn test_count_pipeline() {
    let query = QueryBuilder::new().count().build();
    assert_eq!(query.pipeline.len(), 1);
    assert!(matches!(query.pipeline[0], PipelineStage::CountBy(None)));
}

#[test]
fn test_distinct_pipeline() {
    let query = QueryBuilder::new()
        .distinct(vec![Field::Severity, Field::Subsystem])
        .build();
    assert_eq!(query.pipeline.len(), 1);
    match &query.pipeline[0] {
        PipelineStage::Distinct(fields) => {
            assert_eq!(fields.len(), 2);
        }
        _ => panic!("expected distinct stage"),
    }
}

#[test]
fn test_multiple_pipeline_stages() {
    let query = QueryBuilder::new()
        .sort_by_timestamp_desc()
        .offset(10)
        .limit(100)
        .build();
    assert_eq!(query.pipeline.len(), 3);
}

#[test]
fn test_complex_query() {
    let query = QueryBuilder::new()
        .last_hours(24)
        .severity("error")
        .message_contains("connection refused")
        .sort_by_timestamp_desc()
        .limit(100)
        .build();

    assert!(query.time_range.is_some());
    assert!(query.selector.is_some());
    assert!(query.filter.is_some());
    assert_eq!(query.pipeline.len(), 2);
}

#[test]
fn test_compile_simple_query() {
    // Just verify it compiles without error
    let _compiled = QueryBuilder::new()
        .severity("error")
        .limit(100)
        .compile()
        .expect("should compile");
}

#[test]
fn test_compile_message_contains() {
    let compiled = QueryBuilder::new()
        .message_contains("accepting AMQP connection")
        .compile()
        .expect("should compile");

    assert!(!compiled.sql_where_fragments.is_empty());
}

#[test]
fn test_compile_labels_any() {
    let compiled = QueryBuilder::new()
        .labels_any(vec!["connections".to_string(), "disconnects".to_string()])
        .compile()
        .expect("should compile");

    assert!(!compiled.sql_where_fragments.is_empty());
}

#[test]
fn test_compile_with_sort() {
    let compiled = QueryBuilder::new()
        .sort_by_timestamp_desc()
        .compile()
        .expect("should compile");

    assert!(compiled.sql_order_by.is_some());
}

#[test]
fn test_filter_builder_empty() {
    let filter = FilterBuilder::new().build();
    assert!(filter.is_none());
}

#[test]
fn test_filter_builder_comparison() {
    let filter = FilterBuilder::new()
        .eq(Field::Severity, "error")
        .build()
        .expect("should have filter");
    match filter {
        FilterExpr::Comparison(m) => {
            assert_eq!(m.field, Field::Severity);
            assert_eq!(m.op, MatchOp::Eq);
        }
        _ => panic!("expected comparison"),
    }
}

#[test]
fn test_filter_builder_not_eq() {
    let filter = FilterBuilder::new()
        .not_eq(Field::Severity, "debug")
        .build()
        .expect("should have filter");
    match filter {
        FilterExpr::Comparison(m) => {
            assert_eq!(m.op, MatchOp::NotEq);
        }
        _ => panic!("expected comparison"),
    }
}

#[test]
fn test_filter_builder_contains() {
    let filter = FilterBuilder::new()
        .contains(Field::Message, "connection")
        .build()
        .expect("should have filter");
    match filter {
        FilterExpr::Comparison(m) => {
            assert_eq!(m.op, MatchOp::IContains);
        }
        _ => panic!("expected comparison"),
    }
}

#[test]
fn test_filter_builder_matches() {
    let filter = FilterBuilder::new()
        .matches(Field::Message, "timeout.*")
        .build()
        .expect("should have filter");
    match filter {
        FilterExpr::Comparison(m) => {
            assert_eq!(m.op, MatchOp::Regex);
        }
        _ => panic!("expected comparison"),
    }
}

#[test]
fn test_filter_builder_multiple_combined_with_and() {
    let filter = FilterBuilder::new()
        .eq(Field::Severity, "error")
        .contains(Field::Message, "connection")
        .build()
        .expect("should have filter");
    assert!(matches!(filter, FilterExpr::And(_, _)));
}

#[test]
fn test_filter_builder_preset() {
    let filter = FilterBuilder::new()
        .preset(PresetName::Errors)
        .build()
        .expect("should have filter");
    assert!(matches!(filter, FilterExpr::Preset(PresetName::Errors)));
}

#[test]
fn test_filter_builder_not() {
    let inner = comparison(
        Field::Severity,
        MatchOp::Eq,
        Value::String("debug".to_string()),
    );
    let filter = FilterBuilder::new()
        .not(inner)
        .build()
        .expect("should have filter");
    assert!(matches!(filter, FilterExpr::Not(_)));
}

#[test]
fn test_filter_builder_or() {
    let left = comparison(
        Field::Severity,
        MatchOp::Eq,
        Value::String("error".to_string()),
    );
    let right = comparison(
        Field::Severity,
        MatchOp::Eq,
        Value::String("warning".to_string()),
    );
    let filter = FilterBuilder::new()
        .or(left, right)
        .build()
        .expect("should have filter");
    assert!(matches!(filter, FilterExpr::Or(_, _)));
}

#[test]
fn test_filter_builder_and() {
    let left = comparison(
        Field::Severity,
        MatchOp::Eq,
        Value::String("error".to_string()),
    );
    let right = comparison(
        Field::Message,
        MatchOp::IContains,
        Value::String("connection".to_string()),
    );
    let filter = FilterBuilder::new()
        .and(left, right)
        .build()
        .expect("should have filter");
    assert!(matches!(filter, FilterExpr::And(_, _)));
}

#[test]
fn test_filter_builder_grouped() {
    let inner = comparison(
        Field::Severity,
        MatchOp::Eq,
        Value::String("error".to_string()),
    );
    let filter = FilterBuilder::new()
        .grouped(inner)
        .build()
        .expect("should have filter");
    assert!(matches!(filter, FilterExpr::Grouped(_)));
}

#[test]
fn test_helper_comparison() {
    let expr = comparison(
        Field::Severity,
        MatchOp::Eq,
        Value::String("error".to_string()),
    );
    assert!(matches!(expr, FilterExpr::Comparison(_)));
}

#[test]
fn test_helper_or() {
    let left = comparison(
        Field::Severity,
        MatchOp::Eq,
        Value::String("error".to_string()),
    );
    let right = comparison(
        Field::Severity,
        MatchOp::Eq,
        Value::String("warning".to_string()),
    );
    let expr = or(left, right);
    assert!(matches!(expr, FilterExpr::Or(_, _)));
}

#[test]
fn test_helper_and() {
    let left = comparison(
        Field::Severity,
        MatchOp::Eq,
        Value::String("error".to_string()),
    );
    let right = comparison(
        Field::Message,
        MatchOp::IContains,
        Value::String("test".to_string()),
    );
    let expr = and(left, right);
    assert!(matches!(expr, FilterExpr::And(_, _)));
}

#[test]
fn test_helper_not() {
    let inner = comparison(
        Field::Severity,
        MatchOp::Eq,
        Value::String("debug".to_string()),
    );
    let expr = not(inner);
    assert!(matches!(expr, FilterExpr::Not(_)));
}

#[test]
fn test_filter_with_closure() {
    let query = QueryBuilder::new()
        .filter_with(|f| f.eq(Field::Severity, "error"))
        .build();
    assert!(query.filter.is_some());
}

#[test]
fn test_where_filter() {
    let filter_expr = comparison(
        Field::Severity,
        MatchOp::Eq,
        Value::String("error".to_string()),
    );
    let query = QueryBuilder::new().where_filter(filter_expr).build();
    assert_eq!(query.pipeline.len(), 1);
    assert!(matches!(query.pipeline[0], PipelineStage::Where(_)));
}

#[test]
fn test_since_filter() {
    let timestamp = chrono::Utc::now() - chrono::Duration::hours(1);
    let query = QueryBuilder::new().since(timestamp).build();
    assert!(query.filter.is_some());
}

#[test]
fn test_until_filter() {
    let timestamp = chrono::Utc::now();
    let query = QueryBuilder::new().until(timestamp).build();
    assert!(query.filter.is_some());
}

#[test]
fn test_builder_equivalence_to_parsed_query() {
    let built_query = QueryBuilder::new().severity("error").limit(100).build();

    let parsed_query = rlqt_ql::parse(r#"{severity="error"} | limit 100"#).expect("should parse");

    assert!(built_query.selector.is_some());
    assert!(parsed_query.selector.is_some());
    assert_eq!(
        built_query.selector.unwrap().matchers.len(),
        parsed_query.selector.unwrap().matchers.len()
    );
    assert_eq!(built_query.pipeline.len(), parsed_query.pipeline.len());
}

#[test]
fn test_realistic_error_investigation_query() {
    let compiled = QueryBuilder::new()
        .last_hours(24)
        .errors()
        .sort_by_timestamp_desc()
        .head(100)
        .compile()
        .expect("should compile");

    assert!(compiled.sql_order_by.is_some());
}

#[test]
fn test_realistic_connection_analysis_query() {
    let compiled = QueryBuilder::new()
        .last_hours(24)
        .message_contains("accepting AMQP connection")
        .sort_by_timestamp_desc()
        .limit(500)
        .compile()
        .expect("should compile");

    assert!(!compiled.sql_where_fragments.is_empty());
}

#[test]
fn test_realistic_access_audit_query() {
    let access_denied = comparison(
        Field::Message,
        MatchOp::IContains,
        Value::String("access denied".to_string()),
    );
    let invalid_creds = comparison(
        Field::Message,
        MatchOp::IContains,
        Value::String("invalid credentials".to_string()),
    );

    let compiled = QueryBuilder::new()
        .filter(or(access_denied, invalid_creds))
        .limit(1000)
        .compile()
        .expect("should compile");

    assert!(!compiled.sql_where_fragments.is_empty());
}

#[test]
fn test_filter_builder_labels_any() {
    let filter = FilterBuilder::new()
        .labels_any(vec!["connections".to_string()])
        .build()
        .expect("should have filter");
    assert!(matches!(filter, FilterExpr::LabelAny(_)));
}

#[test]
fn test_filter_builder_labels_all() {
    let filter = FilterBuilder::new()
        .labels_all(vec!["connections".to_string(), "networking".to_string()])
        .build()
        .expect("should have filter");
    assert!(matches!(filter, FilterExpr::LabelAll(_)));
}
