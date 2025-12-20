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

use rlqt_ql::ast::{DurationUnit, Field, FilterExpr, MatchOp, PipelineStage, SortDirection, Value};
use rlqt_ql::parse;
use rlqt_ql::presets::PresetName;

#[test]
fn test_parse_empty_query() {
    let result = parse("");
    assert!(result.is_err());
}

#[test]
fn test_parse_wildcard_query() {
    let query = parse("*").unwrap();
    assert!(query.selector.is_none());
    assert!(query.filter.is_none());
    assert!(query.pipeline.is_empty());
}

#[test]
fn test_parse_severity_selector() {
    let query = parse(r#"{severity="error"}"#).unwrap();
    let selector = query.selector.as_ref().unwrap();
    assert_eq!(selector.matchers.len(), 1);
    assert_eq!(selector.matchers[0].field, Field::Severity);
    assert_eq!(selector.matchers[0].op, MatchOp::Eq);
    assert_eq!(
        selector.matchers[0].value,
        Value::String("error".to_string())
    );
}

#[test]
fn test_parse_severity_warning() {
    let query = parse(r#"{severity="warning"}"#).unwrap();
    let selector = query.selector.as_ref().unwrap();
    assert_eq!(selector.matchers[0].field, Field::Severity);
    assert_eq!(
        selector.matchers[0].value,
        Value::String("warning".to_string())
    );
}

#[test]
fn test_parse_severity_notice() {
    let query = parse(r#"{severity="notice"}"#).unwrap();
    let selector = query.selector.as_ref().unwrap();
    assert_eq!(
        selector.matchers[0].value,
        Value::String("notice".to_string())
    );
}

#[test]
fn test_parse_severity_info() {
    let query = parse(r#"{severity="info"}"#).unwrap();
    let selector = query.selector.as_ref().unwrap();
    assert_eq!(
        selector.matchers[0].value,
        Value::String("info".to_string())
    );
}

#[test]
fn test_parse_severity_debug() {
    let query = parse(r#"{severity="debug"}"#).unwrap();
    let selector = query.selector.as_ref().unwrap();
    assert_eq!(
        selector.matchers[0].value,
        Value::String("debug".to_string())
    );
}

#[test]
fn test_parse_multiple_matchers() {
    let query = parse(r#"{severity="error", subsystem="connections"}"#).unwrap();
    let selector = query.selector.as_ref().unwrap();
    assert_eq!(selector.matchers.len(), 2);
    assert_eq!(selector.matchers[0].field, Field::Severity);
    assert_eq!(selector.matchers[1].field, Field::Subsystem);
}

#[test]
fn test_parse_node_selector() {
    let query = parse(r#"{node="rabbit@host1"}"#).unwrap();
    let selector = query.selector.as_ref().unwrap();
    assert_eq!(selector.matchers[0].field, Field::Node);
    assert_eq!(
        selector.matchers[0].value,
        Value::String("rabbit@host1".to_string())
    );
}

#[test]
fn test_parse_erlang_pid_selector() {
    let query = parse(r#"{erlang_pid="<0.208.0>"}"#).unwrap();
    let selector = query.selector.as_ref().unwrap();
    assert_eq!(selector.matchers[0].field, Field::ErlangPid);
    assert_eq!(
        selector.matchers[0].value,
        Value::String("<0.208.0>".to_string())
    );
}

#[test]
fn test_parse_message_contains() {
    let query = parse(r#"message contains "accepting AMQP connection""#).unwrap();
    let filter = query.filter.as_ref().unwrap();
    if let FilterExpr::Comparison(matcher) = filter {
        assert_eq!(matcher.field, Field::Message);
        assert_eq!(matcher.op, MatchOp::Contains);
        assert_eq!(
            matcher.value,
            Value::String("accepting AMQP connection".to_string())
        );
    } else {
        panic!("Expected Comparison filter");
    }
}

#[test]
fn test_parse_message_contains_http_access_denied() {
    let query = parse(r#"message contains "HTTP access denied""#).unwrap();
    let filter = query.filter.as_ref().unwrap();
    if let FilterExpr::Comparison(matcher) = filter {
        assert_eq!(
            matcher.value,
            Value::String("HTTP access denied".to_string())
        );
    } else {
        panic!("Expected Comparison filter");
    }
}

#[test]
fn test_parse_message_contains_ra_system() {
    let query = parse(r#"message contains "ra: starting system coordination""#).unwrap();
    let filter = query.filter.as_ref().unwrap();
    if let FilterExpr::Comparison(matcher) = filter {
        assert_eq!(
            matcher.value,
            Value::String("ra: starting system coordination".to_string())
        );
    } else {
        panic!("Expected Comparison filter");
    }
}

#[test]
fn test_parse_message_regex() {
    let query = parse(r#"message =~ "connection.*vhost""#).unwrap();
    let filter = query.filter.as_ref().unwrap();
    if let FilterExpr::Comparison(matcher) = filter {
        assert_eq!(matcher.field, Field::Message);
        assert_eq!(matcher.op, MatchOp::Regex);
    } else {
        panic!("Expected Comparison filter");
    }
}

#[test]
fn test_parse_message_not_regex() {
    let query = parse(r#"message !~ "startup""#).unwrap();
    let filter = query.filter.as_ref().unwrap();
    if let FilterExpr::Comparison(matcher) = filter {
        assert_eq!(matcher.op, MatchOp::NotRegex);
    } else {
        panic!("Expected Comparison filter");
    }
}

#[test]
fn test_parse_time_range_hours() {
    let query = parse("@1h").unwrap();
    let time_range = query.time_range.as_ref().unwrap();
    assert_eq!(time_range.value, 1);
    assert_eq!(time_range.unit, DurationUnit::Hours);
}

#[test]
fn test_parse_time_range_minutes() {
    let query = parse("@30m").unwrap();
    let time_range = query.time_range.as_ref().unwrap();
    assert_eq!(time_range.value, 30);
    assert_eq!(time_range.unit, DurationUnit::Minutes);
}

#[test]
fn test_parse_time_range_days() {
    let query = parse("@7d").unwrap();
    let time_range = query.time_range.as_ref().unwrap();
    assert_eq!(time_range.value, 7);
    assert_eq!(time_range.unit, DurationUnit::Days);
}

#[test]
fn test_parse_time_range_weeks() {
    let query = parse("@2w").unwrap();
    let time_range = query.time_range.as_ref().unwrap();
    assert_eq!(time_range.value, 2);
    assert_eq!(time_range.unit, DurationUnit::Weeks);
}

#[test]
fn test_parse_time_range_seconds() {
    let query = parse("@60s").unwrap();
    let time_range = query.time_range.as_ref().unwrap();
    assert_eq!(time_range.value, 60);
    assert_eq!(time_range.unit, DurationUnit::Seconds);
}

#[test]
fn test_parse_time_range_with_selector() {
    let query = parse(r#"@24h {severity="error"}"#).unwrap();
    assert!(query.time_range.is_some());
    assert!(query.selector.is_some());
}

#[test]
fn test_parse_preset_errors() {
    let query = parse(":errors").unwrap();
    let filter = query.filter.as_ref().unwrap();
    if let FilterExpr::Preset(preset) = filter {
        assert_eq!(*preset, PresetName::Errors);
    } else {
        panic!("Expected Preset filter");
    }
}

#[test]
fn test_parse_preset_crashes() {
    let query = parse(":crashes").unwrap();
    let filter = query.filter.as_ref().unwrap();
    if let FilterExpr::Preset(preset) = filter {
        assert_eq!(*preset, PresetName::Crashes);
    } else {
        panic!("Expected Preset filter");
    }
}

#[test]
fn test_parse_preset_errors_or_crashes() {
    let query = parse(":errors_or_crashes").unwrap();
    let filter = query.filter.as_ref().unwrap();
    if let FilterExpr::Preset(preset) = filter {
        assert_eq!(*preset, PresetName::ErrorsOrCrashes);
    } else {
        panic!("Expected Preset filter");
    }
}

#[test]
fn test_parse_preset_disconnects() {
    let query = parse(":disconnects").unwrap();
    let filter = query.filter.as_ref().unwrap();
    if let FilterExpr::Preset(preset) = filter {
        assert_eq!(*preset, PresetName::Disconnects);
    } else {
        panic!("Expected Preset filter");
    }
}

#[test]
fn test_parse_preset_tls_issues() {
    let query = parse(":tls_issues").unwrap();
    let filter = query.filter.as_ref().unwrap();
    if let FilterExpr::Preset(preset) = filter {
        assert_eq!(*preset, PresetName::TlsIssues);
    } else {
        panic!("Expected Preset filter");
    }
}

#[test]
fn test_parse_preset_access_denied() {
    let query = parse(":access_denied").unwrap();
    let filter = query.filter.as_ref().unwrap();
    if let FilterExpr::Preset(preset) = filter {
        assert_eq!(*preset, PresetName::AccessDenied);
    } else {
        panic!("Expected Preset filter");
    }
}

#[test]
fn test_parse_preset_timeouts() {
    let query = parse(":timeouts").unwrap();
    let filter = query.filter.as_ref().unwrap();
    if let FilterExpr::Preset(preset) = filter {
        assert_eq!(*preset, PresetName::Timeouts);
    } else {
        panic!("Expected Preset filter");
    }
}

#[test]
fn test_parse_unknown_preset() {
    let result = parse(":nonexistent");
    assert!(result.is_err());
}

#[test]
fn test_parse_label_has() {
    let query = parse(r#"labels ~= "connections""#).unwrap();
    let filter = query.filter.as_ref().unwrap();
    if let FilterExpr::Comparison(matcher) = filter {
        assert_eq!(matcher.field, Field::Labels);
        assert_eq!(matcher.op, MatchOp::HasLabel);
        assert_eq!(matcher.value, Value::String("connections".to_string()));
    } else {
        panic!("Expected Comparison filter");
    }
}

#[test]
fn test_parse_label_any() {
    let query = parse(r#"labels any ["connections", "disconnects"]"#).unwrap();
    let filter = query.filter.as_ref().unwrap();
    if let FilterExpr::LabelAny(labels) = filter {
        assert_eq!(labels.len(), 2);
        assert!(labels.contains(&"connections".to_string()));
        assert!(labels.contains(&"disconnects".to_string()));
    } else {
        panic!("Expected LabelAny filter");
    }
}

#[test]
fn test_parse_label_all() {
    let query = parse(r#"labels all ["connections", "networking"]"#).unwrap();
    let filter = query.filter.as_ref().unwrap();
    if let FilterExpr::LabelAll(labels) = filter {
        assert_eq!(labels.len(), 2);
    } else {
        panic!("Expected LabelAll filter");
    }
}

#[test]
fn test_parse_subsystem_any() {
    let query = parse(r#"subsystem any ["raft", "metadata_store"]"#).unwrap();
    let filter = query.filter.as_ref().unwrap();
    if let FilterExpr::SubsystemAny(subsystems) = filter {
        assert_eq!(subsystems.len(), 2);
        assert!(subsystems.contains(&"raft".to_string()));
        assert!(subsystems.contains(&"metadata_store".to_string()));
    } else {
        panic!("Expected SubsystemAny filter");
    }
}

#[test]
fn test_parse_subsystem_any_case_insensitive() {
    let query = parse(r#"SUBSYSTEM ANY ["raft"]"#).unwrap();
    let filter = query.filter.as_ref().unwrap();
    assert!(matches!(filter, FilterExpr::SubsystemAny(_)));
}

#[test]
fn test_parse_has_doc_url() {
    let query = parse("has_doc_url").unwrap();
    let filter = query.filter.as_ref().unwrap();
    assert!(matches!(filter, FilterExpr::HasDocUrl));
}

#[test]
fn test_parse_has_resolution_url() {
    let query = parse("has_resolution_url").unwrap();
    let filter = query.filter.as_ref().unwrap();
    assert!(matches!(filter, FilterExpr::HasResolutionUrl));
}

#[test]
fn test_parse_unlabelled() {
    let query = parse("unlabelled").unwrap();
    let filter = query.filter.as_ref().unwrap();
    assert!(matches!(filter, FilterExpr::Unlabelled));
}

#[test]
fn test_parse_and_filter() {
    let query = parse(r#"severity == "error" and subsystem == "connections""#).unwrap();
    let filter = query.filter.as_ref().unwrap();
    assert!(matches!(filter, FilterExpr::And(_, _)));
}

#[test]
fn test_parse_or_filter() {
    let query = parse(r#"severity == "error" or severity == "warning""#).unwrap();
    let filter = query.filter.as_ref().unwrap();
    assert!(matches!(filter, FilterExpr::Or(_, _)));
}

#[test]
fn test_parse_not_filter() {
    let query = parse(r#"not severity == "debug""#).unwrap();
    let filter = query.filter.as_ref().unwrap();
    assert!(matches!(filter, FilterExpr::Not(_)));
}

#[test]
fn test_parse_grouped_filter() {
    let query =
        parse(r#"(severity == "error" or severity == "warning") and subsystem == "connections""#)
            .unwrap();
    let filter = query.filter.as_ref().unwrap();
    assert!(matches!(filter, FilterExpr::And(_, _)));
}

#[test]
fn test_parse_pipeline_limit() {
    let query = parse("* | limit 100").unwrap();
    assert_eq!(query.pipeline.len(), 1);
    assert!(matches!(query.pipeline[0], PipelineStage::Limit(100)));
}

#[test]
fn test_parse_pipeline_head() {
    let query = parse("* | head 50").unwrap();
    assert_eq!(query.pipeline.len(), 1);
    assert!(matches!(query.pipeline[0], PipelineStage::Head(50)));
}

#[test]
fn test_parse_pipeline_tail() {
    let query = parse("* | tail 25").unwrap();
    assert_eq!(query.pipeline.len(), 1);
    assert!(matches!(query.pipeline[0], PipelineStage::Tail(25)));
}

#[test]
fn test_parse_pipeline_sort_timestamp_desc() {
    let query = parse("* | sort timestamp desc").unwrap();
    if let PipelineStage::Sort(spec) = &query.pipeline[0] {
        assert_eq!(spec.field, Field::Timestamp);
        assert_eq!(spec.direction, SortDirection::Desc);
    } else {
        panic!("Expected Sort stage");
    }
}

#[test]
fn test_parse_pipeline_sort_severity_asc() {
    let query = parse("* | sort severity asc").unwrap();
    if let PipelineStage::Sort(spec) = &query.pipeline[0] {
        assert_eq!(spec.field, Field::Severity);
        assert_eq!(spec.direction, SortDirection::Asc);
    } else {
        panic!("Expected Sort stage");
    }
}

#[test]
fn test_parse_pipeline_where() {
    let query = parse(r#"* | where message contains "AMQP""#).unwrap();
    assert!(matches!(query.pipeline[0], PipelineStage::Where(_)));
}

#[test]
fn test_parse_multiple_pipeline_stages() {
    let query = parse(r#"* | where severity == "error" | sort timestamp desc | limit 50"#).unwrap();
    assert_eq!(query.pipeline.len(), 3);
    assert!(matches!(query.pipeline[0], PipelineStage::Where(_)));
    assert!(matches!(query.pipeline[1], PipelineStage::Sort(_)));
    assert!(matches!(query.pipeline[2], PipelineStage::Limit(50)));
}

#[test]
fn test_parse_complete_query() {
    let query =
        parse(r#"@24h {severity="warning"} message contains "HTTP access denied" | limit 100"#)
            .unwrap();
    assert!(query.time_range.is_some());
    assert!(query.selector.is_some());
    assert!(query.filter.is_some());
    assert!(!query.pipeline.is_empty());
}

#[test]
fn test_parse_comparison_operators_eq() {
    let query = parse(r#"severity == "error""#).unwrap();
    if let Some(FilterExpr::Comparison(m)) = query.filter {
        assert_eq!(m.op, MatchOp::Eq);
    } else {
        panic!("Expected comparison");
    }
}

#[test]
fn test_parse_comparison_operators_neq() {
    let query = parse(r#"severity != "debug""#).unwrap();
    if let Some(FilterExpr::Comparison(m)) = query.filter {
        assert_eq!(m.op, MatchOp::NotEq);
    } else {
        panic!("Expected comparison");
    }
}

#[test]
fn test_parse_timestamp_gt() {
    let query = parse(r#"timestamp > @1h"#).unwrap();
    if let Some(FilterExpr::Comparison(m)) = query.filter {
        assert_eq!(m.field, Field::Timestamp);
        assert_eq!(m.op, MatchOp::Gt);
    } else {
        panic!("Expected comparison");
    }
}

#[test]
fn test_parse_timestamp_gte() {
    let query = parse(r#"timestamp >= @24h"#).unwrap();
    if let Some(FilterExpr::Comparison(m)) = query.filter {
        assert_eq!(m.field, Field::Timestamp);
        assert_eq!(m.op, MatchOp::GtEq);
    } else {
        panic!("Expected comparison");
    }
}

#[test]
fn test_parse_timestamp_lt() {
    let query = parse(r#"timestamp < @1h"#).unwrap();
    if let Some(FilterExpr::Comparison(m)) = query.filter {
        assert_eq!(m.op, MatchOp::Lt);
    } else {
        panic!("Expected comparison");
    }
}

#[test]
fn test_parse_timestamp_lte() {
    let query = parse(r#"timestamp <= @30m"#).unwrap();
    if let Some(FilterExpr::Comparison(m)) = query.filter {
        assert_eq!(m.op, MatchOp::LtEq);
    } else {
        panic!("Expected comparison");
    }
}

#[test]
fn test_parse_id_field() {
    let query = parse(r#"id == 42"#).unwrap();
    if let Some(FilterExpr::Comparison(m)) = query.filter {
        assert_eq!(m.field, Field::Id);
        assert_eq!(m.value, Value::Integer(42));
    } else {
        panic!("Expected comparison");
    }
}

#[test]
fn test_parse_count_by() {
    let query = parse("* | count by severity").unwrap();
    assert!(matches!(
        query.pipeline[0],
        PipelineStage::CountBy(Some(Field::Severity))
    ));
}

#[test]
fn test_parse_count_by_subsystem() {
    let query = parse("* | count by subsystem").unwrap();
    assert!(matches!(
        query.pipeline[0],
        PipelineStage::CountBy(Some(Field::Subsystem))
    ));
}

#[test]
fn test_parse_distinct() {
    let query = parse("* | distinct severity").unwrap();
    if let PipelineStage::Distinct(fields) = &query.pipeline[0] {
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0], Field::Severity);
    } else {
        panic!("Expected Distinct stage");
    }
}

#[test]
fn test_parse_project() {
    let query = parse("* | project timestamp, severity, message").unwrap();
    if let PipelineStage::Project(fields) = &query.pipeline[0] {
        assert_eq!(fields.len(), 3);
        assert_eq!(fields[0], Field::Timestamp);
        assert_eq!(fields[1], Field::Severity);
        assert_eq!(fields[2], Field::Message);
    } else {
        panic!("Expected Project stage");
    }
}

#[test]
fn test_parse_offset() {
    let query = parse("* | offset 10").unwrap();
    assert!(matches!(query.pipeline[0], PipelineStage::Offset(10)));
}

#[test]
fn test_parse_invalid_field() {
    let result = parse(r#"{invalid_field="value"}"#);
    assert!(result.is_err());
}

#[test]
fn test_parse_unclosed_string() {
    let result = parse(r#"message contains "unclosed"#);
    assert!(result.is_err());
}

#[test]
fn test_parse_unclosed_selector() {
    let result = parse(r#"{severity="error""#);
    assert!(result.is_err());
}

#[test]
fn test_parse_single_quoted_string() {
    let query = parse(r#"message contains 'accepting AMQP connection'"#).unwrap();
    let filter = query.filter.as_ref().unwrap();
    if let FilterExpr::Comparison(matcher) = filter {
        assert_eq!(
            matcher.value,
            Value::String("accepting AMQP connection".to_string())
        );
    } else {
        panic!("Expected Comparison filter");
    }
}

#[test]
fn test_parse_escaped_characters_double_quotes() {
    let query = parse(r#"message contains "line1\nline2""#).unwrap();
    let filter = query.filter.as_ref().unwrap();
    if let FilterExpr::Comparison(matcher) = filter {
        assert_eq!(matcher.value, Value::String("line1\nline2".to_string()));
    } else {
        panic!("Expected Comparison filter");
    }
}

#[test]
fn test_parse_escaped_tab_character() {
    let query = parse(r#"message contains "col1\tcol2""#).unwrap();
    let filter = query.filter.as_ref().unwrap();
    if let FilterExpr::Comparison(matcher) = filter {
        assert_eq!(matcher.value, Value::String("col1\tcol2".to_string()));
    } else {
        panic!("Expected Comparison filter");
    }
}

#[test]
fn test_parse_empty_string_value() {
    let query = parse(r#"message == """#).unwrap();
    let filter = query.filter.as_ref().unwrap();
    if let FilterExpr::Comparison(matcher) = filter {
        assert_eq!(matcher.value, Value::String("".to_string()));
    } else {
        panic!("Expected Comparison filter");
    }
}

#[test]
fn test_parse_negative_integer() {
    let query = parse("id > -100").unwrap();
    let filter = query.filter.as_ref().unwrap();
    if let FilterExpr::Comparison(matcher) = filter {
        assert_eq!(matcher.value, Value::Integer(-100));
    } else {
        panic!("Expected Comparison filter");
    }
}

#[test]
fn test_parse_regex_literal_syntax() {
    let query = parse(r#"message =~ /connection.*timeout/"#).unwrap();
    let filter = query.filter.as_ref().unwrap();
    if let FilterExpr::Comparison(matcher) = filter {
        assert_eq!(matcher.op, MatchOp::Regex);
        assert_eq!(
            matcher.value,
            Value::Regex("connection.*timeout".to_string())
        );
    } else {
        panic!("Expected Comparison filter");
    }
}

#[test]
fn test_parse_boolean_true_value() {
    let query = parse(r#"{severity="true"}"#).unwrap();
    let selector = query.selector.as_ref().unwrap();
    assert_eq!(
        selector.matchers[0].value,
        Value::String("true".to_string())
    );
}

#[test]
fn test_parse_null_value() {
    let query = parse("message == null").unwrap();
    let filter = query.filter.as_ref().unwrap();
    if let FilterExpr::Comparison(matcher) = filter {
        assert_eq!(matcher.value, Value::Null);
    } else {
        panic!("Expected Comparison filter");
    }
}

#[test]
fn test_parse_escaped_quote_in_string() {
    let query = parse(r#"message contains "user\"s data""#).unwrap();
    let filter = query.filter.as_ref().unwrap();
    if let FilterExpr::Comparison(matcher) = filter {
        assert_eq!(matcher.value, Value::String("user\"s data".to_string()));
    } else {
        panic!("Expected Comparison filter");
    }
}

#[test]
fn test_parse_field_alias_level() {
    let query = parse(r#"{level="error"}"#).unwrap();
    let selector = query.selector.as_ref().unwrap();
    assert_eq!(selector.matchers[0].field, Field::Severity);
}

#[test]
fn test_parse_field_alias_msg() {
    let query = parse(r#"msg contains "AMQP""#).unwrap();
    let filter = query.filter.as_ref().unwrap();
    if let FilterExpr::Comparison(matcher) = filter {
        assert_eq!(matcher.field, Field::Message);
    } else {
        panic!("Expected Comparison filter");
    }
}

#[test]
fn test_parse_field_alias_ts() {
    let query = parse(r#"ts >= @1h"#).unwrap();
    let filter = query.filter.as_ref().unwrap();
    if let FilterExpr::Comparison(matcher) = filter {
        assert_eq!(matcher.field, Field::Timestamp);
    } else {
        panic!("Expected Comparison filter");
    }
}

#[test]
fn test_parse_unlabeled_american_spelling() {
    let query = parse("unlabeled").unwrap();
    let filter = query.filter.as_ref().unwrap();
    assert!(matches!(filter, FilterExpr::Unlabelled));
}

#[test]
fn test_parse_alternative_operator_not_equals() {
    let query = parse(r#"severity <> "debug""#).unwrap();
    let filter = query.filter.as_ref().unwrap();
    if let FilterExpr::Comparison(matcher) = filter {
        assert_eq!(matcher.op, MatchOp::NotEq);
    } else {
        panic!("Expected Comparison filter");
    }
}

#[test]
fn test_parse_double_pipe_or() {
    let query = parse(r#"severity == "error" || severity == "warning""#).unwrap();
    let filter = query.filter.as_ref().unwrap();
    assert!(matches!(filter, FilterExpr::Or(_, _)));
}

#[test]
fn test_parse_double_ampersand_and() {
    let query = parse(r#"severity == "error" && subsystem == "connections""#).unwrap();
    let filter = query.filter.as_ref().unwrap();
    assert!(matches!(filter, FilterExpr::And(_, _)));
}

#[test]
fn test_parse_exclamation_not() {
    let query = parse(r#"! severity == "debug""#).unwrap();
    let filter = query.filter.as_ref().unwrap();
    assert!(matches!(filter, FilterExpr::Not(_)));
}

#[test]
fn test_parse_skip_alias_for_offset() {
    let query = parse("* | skip 20").unwrap();
    assert!(matches!(query.pipeline[0], PipelineStage::Offset(20)));
}

#[test]
fn test_parse_select_alias_for_project() {
    let query = parse("* | select timestamp, message").unwrap();
    if let PipelineStage::Project(fields) = &query.pipeline[0] {
        assert_eq!(fields.len(), 2);
    } else {
        panic!("Expected Project stage");
    }
}

#[test]
fn test_parse_preset_tls_alias() {
    let query = parse(":tls").unwrap();
    let filter = query.filter.as_ref().unwrap();
    if let FilterExpr::Preset(preset) = filter {
        assert_eq!(*preset, PresetName::TlsIssues);
    } else {
        panic!("Expected Preset filter");
    }
}

#[test]
fn test_parse_preset_access_control_alias() {
    let query = parse(":access_control").unwrap();
    let filter = query.filter.as_ref().unwrap();
    if let FilterExpr::Preset(preset) = filter {
        assert_eq!(*preset, PresetName::AccessDenied);
    } else {
        panic!("Expected Preset filter");
    }
}

#[test]
fn test_parse_uppercase_or() {
    let query = parse(r#"severity == "error" OR severity == "warning""#).unwrap();
    let filter = query.filter.as_ref().unwrap();
    assert!(matches!(filter, FilterExpr::Or(_, _)));
}

#[test]
fn test_parse_uppercase_and() {
    let query = parse(r#"severity == "error" AND subsystem == "connections""#).unwrap();
    let filter = query.filter.as_ref().unwrap();
    assert!(matches!(filter, FilterExpr::And(_, _)));
}

#[test]
fn test_parse_uppercase_not() {
    let query = parse(r#"NOT severity == "debug""#).unwrap();
    let filter = query.filter.as_ref().unwrap();
    assert!(matches!(filter, FilterExpr::Not(_)));
}

#[test]
fn test_parse_mixed_case_or() {
    let query = parse(r#"severity == "error" Or severity == "warning""#).unwrap();
    let filter = query.filter.as_ref().unwrap();
    assert!(matches!(filter, FilterExpr::Or(_, _)));
}

#[test]
fn test_parse_uppercase_field_name() {
    let query = parse(r#"SEVERITY == "error""#).unwrap();
    let filter = query.filter.as_ref().unwrap();
    if let FilterExpr::Comparison(matcher) = filter {
        assert_eq!(matcher.field, Field::Severity);
    } else {
        panic!("Expected Comparison filter");
    }
}

#[test]
fn test_parse_mixed_case_field_name() {
    let query = parse(r#"Timestamp >= @1h"#).unwrap();
    let filter = query.filter.as_ref().unwrap();
    if let FilterExpr::Comparison(matcher) = filter {
        assert_eq!(matcher.field, Field::Timestamp);
    } else {
        panic!("Expected Comparison filter");
    }
}

#[test]
fn test_parse_first_alias_for_head() {
    let query = parse("* | first 10").unwrap();
    assert!(matches!(query.pipeline[0], PipelineStage::Head(10)));
}

#[test]
fn test_parse_last_alias_for_tail() {
    let query = parse("* | last 5").unwrap();
    assert!(matches!(query.pipeline[0], PipelineStage::Tail(5)));
}

#[test]
fn test_parse_order_by_alias_for_sort() {
    let query = parse("* | order by timestamp desc").unwrap();
    if let PipelineStage::Sort(spec) = &query.pipeline[0] {
        assert_eq!(spec.field, Field::Timestamp);
        assert_eq!(spec.direction, SortDirection::Desc);
    } else {
        panic!("Expected Sort stage");
    }
}

#[test]
fn test_parse_order_by_case_insensitive() {
    let query = parse("* | ORDER BY severity ASC").unwrap();
    if let PipelineStage::Sort(spec) = &query.pipeline[0] {
        assert_eq!(spec.field, Field::Severity);
        assert_eq!(spec.direction, SortDirection::Asc);
    } else {
        panic!("Expected Sort stage");
    }
}
