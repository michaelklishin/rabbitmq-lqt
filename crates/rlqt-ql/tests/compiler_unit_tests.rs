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

use rlqt_ql::{compile, parse};

#[test]
fn test_compile_severity_selector() {
    let query = parse(r#"{severity="error"}"#).unwrap();
    let result = compile(&query);
    assert!(result.is_ok());
}

#[test]
fn test_compile_severity_warning() {
    let query = parse(r#"{severity="warning"}"#).unwrap();
    let result = compile(&query);
    assert!(result.is_ok());
}

#[test]
fn test_compile_severity_notice() {
    let query = parse(r#"{severity="notice"}"#).unwrap();
    let result = compile(&query);
    assert!(result.is_ok());
}

#[test]
fn test_compile_severity_info() {
    let query = parse(r#"{severity="info"}"#).unwrap();
    let result = compile(&query);
    assert!(result.is_ok());
}

#[test]
fn test_compile_severity_debug() {
    let query = parse(r#"{severity="debug"}"#).unwrap();
    let result = compile(&query);
    assert!(result.is_ok());
}

#[test]
fn test_compile_invalid_severity() {
    let query = parse(r#"{severity="invalid"}"#).unwrap();
    let result = compile(&query);
    assert!(result.is_err());
}

#[test]
fn test_compile_subsystem_selector() {
    let query = parse(r#"{subsystem="connections"}"#).unwrap();
    let result = compile(&query);
    assert!(result.is_ok());
}

#[test]
fn test_compile_node_selector() {
    let query = parse(r#"{node="rabbit@host1"}"#).unwrap();
    let result = compile(&query);
    assert!(result.is_ok());
}

#[test]
fn test_compile_erlang_pid_selector() {
    let query = parse(r#"{erlang_pid="<0.208.0>"}"#).unwrap();
    let result = compile(&query);
    assert!(result.is_ok());
}

#[test]
fn test_compile_multiple_selectors() {
    let query = parse(r#"{severity="error", subsystem="connections"}"#).unwrap();
    let result = compile(&query);
    assert!(result.is_ok());
}

#[test]
fn test_compile_time_range() {
    let query = parse("@1h").unwrap();
    let result = compile(&query);
    assert!(result.is_ok());
}

#[test]
fn test_compile_time_range_24h() {
    let query = parse("@24h").unwrap();
    let result = compile(&query);
    assert!(result.is_ok());
}

#[test]
fn test_compile_time_range_7d() {
    let query = parse("@7d").unwrap();
    let result = compile(&query);
    assert!(result.is_ok());
}

#[test]
fn test_compile_limit() {
    let query = parse("* | limit 100").unwrap();
    let result = compile(&query);
    assert!(result.is_ok());
}

#[test]
fn test_compile_limit_50() {
    let query = parse("* | limit 50").unwrap();
    let result = compile(&query);
    assert!(result.is_ok());
}

#[test]
fn test_compile_head() {
    let query = parse("* | head 25").unwrap();
    let result = compile(&query);
    assert!(result.is_ok());
}

#[test]
fn test_compile_message_contains_sql() {
    let query = parse(r#"message contains "accepting AMQP connection""#).unwrap();
    let compiled = compile(&query).unwrap();
    assert!(!compiled.sql_where_fragments.is_empty());
    let sql = &compiled.sql_where_fragments[0];
    assert!(sql.contains("ILIKE"));
    assert!(sql.contains("accepting AMQP connection"));
}

#[test]
fn test_compile_message_contains_http_access_denied() {
    let query = parse(r#"message contains "HTTP access denied""#).unwrap();
    let compiled = compile(&query).unwrap();
    assert!(!compiled.sql_where_fragments.is_empty());
    let sql = &compiled.sql_where_fragments[0];
    assert!(sql.contains("HTTP access denied"));
}

#[test]
fn test_compile_message_regex_sql() {
    let query = parse(r#"message =~ "connection.*vhost""#).unwrap();
    let compiled = compile(&query).unwrap();
    assert!(!compiled.sql_where_fragments.is_empty());
    let sql = &compiled.sql_where_fragments[0];
    assert!(sql.contains("regexp_matches"));
}

#[test]
fn test_compile_message_not_regex_sql() {
    let query = parse(r#"message !~ "startup""#).unwrap();
    let compiled = compile(&query).unwrap();
    assert!(!compiled.sql_where_fragments.is_empty());
    let sql = &compiled.sql_where_fragments[0];
    assert!(sql.contains("NOT regexp_matches"));
}

#[test]
fn test_compile_invalid_regex() {
    let query = parse(r#"message =~ "[invalid""#).unwrap();
    let result = compile(&query);
    assert!(result.is_err());
}

#[test]
fn test_compile_label_any_sql() {
    let query = parse(r#"labels any ["connections", "disconnects"]"#).unwrap();
    let compiled = compile(&query).unwrap();
    assert!(!compiled.sql_where_fragments.is_empty());
    let sql = &compiled.sql_where_fragments[0];
    assert!(sql.contains("labels &"));
    assert!(sql.contains("!= 0"));
}

#[test]
fn test_compile_label_all_sql() {
    let query = parse(r#"labels all ["connections", "networking"]"#).unwrap();
    let compiled = compile(&query).unwrap();
    assert!(!compiled.sql_where_fragments.is_empty());
}

#[test]
fn test_compile_unknown_label() {
    let query = parse(r#"labels any ["nonexistent_label_xyz"]"#).unwrap();
    let result = compile(&query);
    assert!(result.is_err());
}

#[test]
fn test_compile_subsystem_any_sql() {
    let query = parse(r#"subsystem any ["raft", "metadata_store"]"#).unwrap();
    let compiled = compile(&query).unwrap();
    assert!(!compiled.sql_where_fragments.is_empty());
    let sql = &compiled.sql_where_fragments[0];
    assert!(sql.contains("subsystem_id IN"));
    assert!(sql.contains("4")); // Raft = 4
    assert!(sql.contains("1")); // MetadataStore = 1
}

#[test]
fn test_compile_unknown_subsystem() {
    let query = parse(r#"subsystem any ["nonexistent_subsystem"]"#).unwrap();
    let result = compile(&query);
    assert!(result.is_err());
}

#[test]
fn test_compile_has_doc_url() {
    let query = parse("has_doc_url").unwrap();
    let result = compile(&query);
    assert!(result.is_ok());
}

#[test]
fn test_compile_has_resolution_url() {
    let query = parse("has_resolution_url").unwrap();
    let result = compile(&query);
    assert!(result.is_ok());
}

#[test]
fn test_compile_unlabelled() {
    let query = parse("unlabelled").unwrap();
    let result = compile(&query);
    assert!(result.is_ok());
}

#[test]
fn test_compile_or_filter_sql() {
    let query = parse(r#"severity == "error" or severity == "warning""#).unwrap();
    let compiled = compile(&query).unwrap();
    assert!(!compiled.sql_where_fragments.is_empty());
    let sql = &compiled.sql_where_fragments[0];
    assert!(sql.contains("OR"));
}

#[test]
fn test_compile_not_filter_sql() {
    let query = parse(r#"not severity == "debug""#).unwrap();
    let compiled = compile(&query).unwrap();
    assert!(!compiled.sql_where_fragments.is_empty());
    let sql = &compiled.sql_where_fragments[0];
    assert!(sql.contains("NOT"));
}

#[test]
fn test_compile_sort_order_by() {
    let query = parse("* | sort timestamp desc").unwrap();
    let compiled = compile(&query).unwrap();
    assert!(compiled.sql_order_by.is_some());
    let order = compiled.sql_order_by.as_ref().unwrap();
    assert!(order.contains("timestamp"));
    assert!(order.contains("DESC"));
}

#[test]
fn test_compile_sort_severity_asc() {
    let query = parse("* | sort severity asc").unwrap();
    let compiled = compile(&query).unwrap();
    assert!(compiled.sql_order_by.is_some());
    let order = compiled.sql_order_by.as_ref().unwrap();
    assert!(order.contains("severity"));
    assert!(order.contains("ASC"));
}

#[test]
fn test_compile_count_by_aggregation() {
    let query = parse("* | count by severity").unwrap();
    let compiled = compile(&query).unwrap();
    assert!(compiled.has_aggregation);
}

#[test]
fn test_compile_preset_errors() {
    let query = parse(":errors").unwrap();
    let result = compile(&query);
    assert!(result.is_ok());
}

#[test]
fn test_compile_preset_crashes() {
    let query = parse(":crashes").unwrap();
    let compiled = compile(&query).unwrap();
    assert!(!compiled.sql_where_fragments.is_empty());
}

#[test]
fn test_compile_preset_errors_or_crashes() {
    let query = parse(":errors_or_crashes").unwrap();
    let compiled = compile(&query).unwrap();
    assert!(!compiled.sql_where_fragments.is_empty());
}

#[test]
fn test_compile_preset_disconnects() {
    let query = parse(":disconnects").unwrap();
    let compiled = compile(&query).unwrap();
    assert!(!compiled.sql_where_fragments.is_empty());
}

#[test]
fn test_compile_preset_tls_issues() {
    let query = parse(":tls_issues").unwrap();
    let compiled = compile(&query).unwrap();
    assert!(!compiled.sql_where_fragments.is_empty());
}

#[test]
fn test_compile_preset_access_denied() {
    let query = parse(":access_denied").unwrap();
    let compiled = compile(&query).unwrap();
    assert!(!compiled.sql_where_fragments.is_empty());
}

#[test]
fn test_compile_preset_timeouts() {
    let query = parse(":timeouts").unwrap();
    let compiled = compile(&query).unwrap();
    assert!(!compiled.sql_where_fragments.is_empty());
}

#[test]
fn test_compile_comparison_neq_sql() {
    let query = parse(r#"severity != "debug""#).unwrap();
    let compiled = compile(&query).unwrap();
    assert!(!compiled.sql_where_fragments.is_empty());
    let sql = &compiled.sql_where_fragments[0];
    assert!(sql.contains("!="));
}

#[test]
fn test_compile_timestamp_since() {
    let query = parse(r#"timestamp >= @1h"#).unwrap();
    let result = compile(&query);
    assert!(result.is_ok());
}

#[test]
fn test_compile_timestamp_to() {
    let query = parse(r#"timestamp <= @1h"#).unwrap();
    let result = compile(&query);
    assert!(result.is_ok());
}

#[test]
fn test_compile_sql_string_escaping() {
    let query = parse(r#"message contains "user's data""#).unwrap();
    let compiled = compile(&query).unwrap();
    assert!(!compiled.sql_where_fragments.is_empty());
    let sql = &compiled.sql_where_fragments[0];
    assert!(sql.contains("user''s data"));
}

#[test]
fn test_compile_complete_query() {
    let query =
        parse(r#"@24h {severity="warning"} message contains "HTTP access denied" | limit 100"#)
            .unwrap();
    let compiled = compile(&query).unwrap();
    assert!(!compiled.sql_where_fragments.is_empty());
}

#[test]
fn test_compile_wildcard_query() {
    let query = parse("*").unwrap();
    let compiled = compile(&query).unwrap();
    assert!(compiled.sql_where_fragments.is_empty());
}

#[test]
fn test_compile_multiple_pipeline_stages() {
    let query = parse(r#"* | where severity == "error" | sort timestamp desc | limit 50"#).unwrap();
    let compiled = compile(&query).unwrap();
    assert!(compiled.sql_order_by.is_some());
}

#[test]
fn test_compile_label_has() {
    let query = parse(r#"labels ~= "connections""#).unwrap();
    let result = compile(&query);
    assert!(result.is_ok());
}

#[test]
fn test_compile_and_filter() {
    let query = parse(r#"severity == "error" and subsystem == "connections""#).unwrap();
    let result = compile(&query);
    assert!(result.is_ok());
}

#[test]
fn test_compile_message_eq_sql() {
    let query = parse(r#"message == "Logging: configured log handlers are now ACTIVE""#).unwrap();
    let compiled = compile(&query).unwrap();
    assert!(!compiled.sql_where_fragments.is_empty());
    let sql = &compiled.sql_where_fragments[0];
    assert!(sql.contains("message = "));
}

#[test]
fn test_compile_id_gt_sql() {
    let query = parse("id > 100").unwrap();
    let compiled = compile(&query).unwrap();
    assert!(!compiled.sql_where_fragments.is_empty());
    let sql = &compiled.sql_where_fragments[0];
    assert!(sql.contains("id > 100"));
}

#[test]
fn test_compile_id_lt_sql() {
    let query = parse("id < 50").unwrap();
    let compiled = compile(&query).unwrap();
    assert!(!compiled.sql_where_fragments.is_empty());
    let sql = &compiled.sql_where_fragments[0];
    assert!(sql.contains("id < 50"));
}

#[test]
fn test_compile_grouped_or_and_sql() {
    let query = parse(
        r#"(severity == "error" or severity == "warning") and message contains "connection""#,
    )
    .unwrap();
    let compiled = compile(&query).unwrap();
    assert!(!compiled.sql_where_fragments.is_empty());
}

#[test]
fn test_compile_regex_escape_sql() {
    let query = parse(r#"message =~ "rabbit@host[0-9]+""#).unwrap();
    let compiled = compile(&query).unwrap();
    assert!(!compiled.sql_where_fragments.is_empty());
    let sql = &compiled.sql_where_fragments[0];
    assert!(sql.contains("regexp_matches"));
    assert!(sql.contains("rabbit@host[0-9]+"));
}

#[test]
fn test_compile_message_icontains_sql() {
    let query = parse(r#"message icontains "AMQP""#).unwrap();
    let compiled = compile(&query).unwrap();
    assert!(!compiled.sql_where_fragments.is_empty());
    let sql = &compiled.sql_where_fragments[0];
    assert!(sql.contains("ILIKE"));
}

#[test]
fn test_compile_sort_message_desc() {
    let query = parse("* | sort message desc").unwrap();
    let compiled = compile(&query).unwrap();
    assert!(compiled.sql_order_by.is_some());
    let order = compiled.sql_order_by.as_ref().unwrap();
    assert!(order.contains("message"));
    assert!(order.contains("DESC"));
}

#[test]
fn test_compile_sort_node_asc() {
    let query = parse("* | sort node asc").unwrap();
    let compiled = compile(&query).unwrap();
    assert!(compiled.sql_order_by.is_some());
    let order = compiled.sql_order_by.as_ref().unwrap();
    assert!(order.contains("node"));
    assert!(order.contains("ASC"));
}

#[test]
fn test_compile_offset() {
    let query = parse("* | offset 100").unwrap();
    let compiled = compile(&query).unwrap();
    assert_eq!(compiled.sql_offset, Some(100));
}

#[test]
fn test_compile_skip_alias() {
    let query = parse("* | skip 50").unwrap();
    let compiled = compile(&query).unwrap();
    assert_eq!(compiled.sql_offset, Some(50));
}

#[test]
fn test_compile_tail() {
    let query = parse("* | tail 25").unwrap();
    let compiled = compile(&query).unwrap();
    assert_eq!(compiled.sql_limit_from_end, Some(25));
}

#[test]
fn test_compile_project() {
    let query = parse("* | project timestamp, severity, message").unwrap();
    let compiled = compile(&query).unwrap();
    assert!(compiled.projection.is_some());
    let fields = compiled.projection.as_ref().unwrap();
    assert_eq!(fields.len(), 3);
}

#[test]
fn test_compile_select_alias() {
    let query = parse("* | select timestamp, message").unwrap();
    let compiled = compile(&query).unwrap();
    assert!(compiled.projection.is_some());
    let fields = compiled.projection.as_ref().unwrap();
    assert_eq!(fields.len(), 2);
}

#[test]
fn test_compile_distinct() {
    let query = parse("* | distinct severity").unwrap();
    let compiled = compile(&query).unwrap();
    assert!(compiled.distinct_fields.is_some());
    let fields = compiled.distinct_fields.as_ref().unwrap();
    assert_eq!(fields.len(), 1);
}

#[test]
fn test_compile_distinct_multiple_fields() {
    let query = parse("* | distinct severity, subsystem").unwrap();
    let compiled = compile(&query).unwrap();
    assert!(compiled.distinct_fields.is_some());
    let fields = compiled.distinct_fields.as_ref().unwrap();
    assert_eq!(fields.len(), 2);
}

#[test]
fn test_compile_limit_with_offset() {
    let query = parse("* | limit 100 | offset 50").unwrap();
    let compiled = compile(&query).unwrap();
    assert_eq!(compiled.sql_offset, Some(50));
}

#[test]
fn test_compile_count_by_stores_field() {
    let query = parse("* | count by severity").unwrap();
    let compiled = compile(&query).unwrap();
    assert!(compiled.has_aggregation);
    assert!(compiled.count_by_field.is_some());
}

#[test]
fn test_compile_count_without_by() {
    let query = parse("* | count").unwrap();
    let compiled = compile(&query).unwrap();
    assert!(compiled.has_aggregation);
    assert!(compiled.count_by_field.is_none());
}
