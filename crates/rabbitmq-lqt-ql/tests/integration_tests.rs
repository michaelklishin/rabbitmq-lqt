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

//! Integration tests for the RLQT Query Language.
//!
//! These tests use real log entry patterns from the fixture files:
//! - crates/rabbitmq-lqt-cli/tests/fixtures/rabbit@fixture1.log
//! - crates/rabbitmq-lqt-cli/tests/fixtures/rabbit@fixture2.log
//! - crates/rabbitmq-lqt-cli/tests/fixtures/rabbit@fixture3.log
//! - crates/rabbitmq-lqt-cli/tests/fixtures/rabbit@fixture4.log

use rabbitmq_lqt_ql::{parse, parse_and_compile};

#[test]
fn test_parse_and_compile_severity_error() {
    let result = parse_and_compile(r#"{severity="error"}"#);
    assert!(result.is_ok());
}

#[test]
fn test_parse_and_compile_severity_warning() {
    let result = parse_and_compile(r#"{severity="warning"}"#);
    assert!(result.is_ok());
}

#[test]
fn test_parse_and_compile_severity_notice() {
    let result = parse_and_compile(r#"{severity="notice"}"#);
    assert!(result.is_ok());
}

#[test]
fn test_parse_and_compile_severity_info() {
    let result = parse_and_compile(r#"{severity="info"}"#);
    assert!(result.is_ok());
}

#[test]
fn test_parse_and_compile_severity_debug() {
    let result = parse_and_compile(r#"{severity="debug"}"#);
    assert!(result.is_ok());
}

#[test]
fn test_full_query_http_access_denied() {
    let query = r#"@24h {severity="warning"} message contains "HTTP access denied" | limit 100"#;
    let result = parse_and_compile(query);
    assert!(result.is_ok());
    let compiled = result.unwrap();
    assert!(!compiled.sql_where_fragments.is_empty());
}

#[test]
fn test_full_query_amqp_connection() {
    let query = r#"message contains "accepting AMQP connection" | sort timestamp desc | limit 50"#;
    let result = parse_and_compile(query);
    assert!(result.is_ok());
    let compiled = result.unwrap();
    assert!(compiled.sql_order_by.is_some());
}

#[test]
fn test_full_query_ra_system() {
    let query = r#"message contains "ra: starting system coordination""#;
    let result = parse_and_compile(query);
    assert!(result.is_ok());
}

#[test]
fn test_full_query_logging_handlers() {
    let query = r#"message contains "Logging: configured log handlers are now ACTIVE""#;
    let result = parse_and_compile(query);
    assert!(result.is_ok());
}

#[test]
fn test_full_query_disk_free_limit() {
    let query = r#"message contains "disk_free_limit""#;
    let result = parse_and_compile(query);
    assert!(result.is_ok());
}

#[test]
fn test_full_query_deprecated_features() {
    let query = r#"message contains "Deprecated features""#;
    let result = parse_and_compile(query);
    assert!(result.is_ok());
}

#[test]
fn test_full_query_feature_flags() {
    let query = r#"message contains "Feature flags""#;
    let result = parse_and_compile(query);
    assert!(result.is_ok());
}

#[test]
fn test_full_query_vhost_access() {
    let query = r#"message contains "granted access to vhost""#;
    let result = parse_and_compile(query);
    assert!(result.is_ok());
}

#[test]
fn test_full_query_closing_amqp() {
    let query = r#"message contains "closing AMQP connection""#;
    let result = parse_and_compile(query);
    assert!(result.is_ok());
}

#[test]
fn test_full_query_invalid_credentials() {
    let query = r#"{severity="warning"} message contains "invalid credentials""#;
    let result = parse_and_compile(query);
    assert!(result.is_ok());
}

#[test]
fn test_full_query_wal_init() {
    let query = r#"message contains "WAL:" and message contains "init""#;
    let result = parse_and_compile(query);
    assert!(result.is_ok());
}

#[test]
fn test_full_query_rabbitmq_starting() {
    let query = r#"message contains "Starting VMware RabbitMQ""#;
    let result = parse_and_compile(query);
    assert!(result.is_ok());
}

#[test]
fn test_preset_errors_compile() {
    let result = parse_and_compile(":errors");
    assert!(result.is_ok());
}

#[test]
fn test_preset_crashes_compile() {
    let result = parse_and_compile(":crashes");
    assert!(result.is_ok());
}

#[test]
fn test_preset_errors_or_crashes_compile() {
    let result = parse_and_compile(":errors_or_crashes");
    assert!(result.is_ok());
}

#[test]
fn test_preset_disconnects_compile() {
    let result = parse_and_compile(":disconnects");
    assert!(result.is_ok());
}

#[test]
fn test_preset_tls_issues_compile() {
    let result = parse_and_compile(":tls_issues");
    assert!(result.is_ok());
}

#[test]
fn test_preset_access_denied_compile() {
    let result = parse_and_compile(":access_denied");
    assert!(result.is_ok());
}

#[test]
fn test_preset_timeouts_compile() {
    let result = parse_and_compile(":timeouts");
    assert!(result.is_ok());
}

#[test]
fn test_time_range_queries() {
    assert!(parse_and_compile("@1h").is_ok());
    assert!(parse_and_compile("@24h").is_ok());
    assert!(parse_and_compile("@7d").is_ok());
    assert!(parse_and_compile("@30m").is_ok());
    assert!(parse_and_compile("@60s").is_ok());
    assert!(parse_and_compile("@2w").is_ok());
}

#[test]
fn test_time_range_with_filter() {
    let query = r#"@24h {severity="warning"} message contains "access denied""#;
    let result = parse_and_compile(query);
    assert!(result.is_ok());
}

#[test]
fn test_complex_query_with_or() {
    let query = r#"severity == "error" or severity == "warning""#;
    let result = parse_and_compile(query);
    assert!(result.is_ok());
    let compiled = result.unwrap();
    assert!(!compiled.sql_where_fragments.is_empty());
}

#[test]
fn test_complex_query_with_and() {
    let query = r#"severity == "warning" and message contains "denied""#;
    let result = parse_and_compile(query);
    assert!(result.is_ok());
}

#[test]
fn test_complex_query_with_not() {
    let query = r#"not severity == "debug""#;
    let result = parse_and_compile(query);
    assert!(result.is_ok());
    let compiled = result.unwrap();
    assert!(!compiled.sql_where_fragments.is_empty());
}

#[test]
fn test_complex_query_grouped() {
    let query =
        r#"(severity == "error" or severity == "warning") and message contains "connection""#;
    let result = parse_and_compile(query);
    assert!(result.is_ok());
}

#[test]
fn test_query_with_regex_node_pattern() {
    let query = r#"message =~ "rabbit@host[0-9]+""#;
    let result = parse_and_compile(query);
    assert!(result.is_ok());
    let compiled = result.unwrap();
    assert!(!compiled.sql_where_fragments.is_empty());
    let sql = &compiled.sql_where_fragments[0];
    assert!(sql.contains("regexp_matches"));
}

#[test]
fn test_query_with_regex_vhost_pattern() {
    let query = r#"message =~ "vhost.*authenticated""#;
    let result = parse_and_compile(query);
    assert!(result.is_ok());
}

#[test]
fn test_query_with_regex_ip_pattern() {
    let query = r#"message =~ "[0-9]+\\.[0-9]+\\.[0-9]+\\.[0-9]+:[0-9]+""#;
    let result = parse_and_compile(query);
    assert!(result.is_ok());
}

#[test]
fn test_query_erlang_pid_pattern() {
    let query = r#"{erlang_pid="<0.208.0>"}"#;
    let result = parse_and_compile(query);
    assert!(result.is_ok());
}

#[test]
fn test_query_node_pattern() {
    let query = r#"{node="rabbit@host1"}"#;
    let result = parse_and_compile(query);
    assert!(result.is_ok());
}

#[test]
fn test_labels_any_query() {
    let query = r#"labels any ["connections", "disconnects"]"#;
    let result = parse_and_compile(query);
    assert!(result.is_ok());
    let compiled = result.unwrap();
    assert!(!compiled.sql_where_fragments.is_empty());
}

#[test]
fn test_labels_all_query() {
    let query = r#"labels all ["connections", "networking"]"#;
    let result = parse_and_compile(query);
    assert!(result.is_ok());
}

#[test]
fn test_label_has_query() {
    let query = r#"labels ~= "connections""#;
    let result = parse_and_compile(query);
    assert!(result.is_ok());
}

#[test]
fn test_has_doc_url_query() {
    let result = parse_and_compile("has_doc_url");
    assert!(result.is_ok());
}

#[test]
fn test_has_resolution_url_query() {
    let result = parse_and_compile("has_resolution_url");
    assert!(result.is_ok());
}

#[test]
fn test_unlabelled_query() {
    let result = parse_and_compile("unlabelled");
    assert!(result.is_ok());
}

#[test]
fn test_pipeline_limit() {
    let query = "* | limit 100";
    let result = parse_and_compile(query);
    assert!(result.is_ok());
}

#[test]
fn test_pipeline_head() {
    let query = "* | head 50";
    let result = parse_and_compile(query);
    assert!(result.is_ok());
}

#[test]
fn test_pipeline_tail() {
    let query = "* | tail 25";
    let result = parse_and_compile(query);
    assert!(result.is_ok());
}

#[test]
fn test_pipeline_sort_timestamp_desc() {
    let query = "* | sort timestamp desc";
    let result = parse_and_compile(query);
    assert!(result.is_ok());
    let compiled = result.unwrap();
    assert!(compiled.sql_order_by.is_some());
    let order = compiled.sql_order_by.unwrap();
    assert!(order.contains("timestamp"));
    assert!(order.contains("DESC"));
}

#[test]
fn test_pipeline_sort_severity_asc() {
    let query = "* | sort severity asc";
    let result = parse_and_compile(query);
    assert!(result.is_ok());
    let compiled = result.unwrap();
    assert!(compiled.sql_order_by.is_some());
}

#[test]
fn test_pipeline_where() {
    let query = r#"* | where severity == "error""#;
    let result = parse_and_compile(query);
    assert!(result.is_ok());
}

#[test]
fn test_pipeline_count_by() {
    let query = "* | count by severity";
    let result = parse_and_compile(query);
    assert!(result.is_ok());
    let compiled = result.unwrap();
    assert!(compiled.has_aggregation);
}

#[test]
fn test_pipeline_distinct() {
    let query = "* | distinct severity";
    let result = parse_and_compile(query);
    assert!(result.is_ok());
}

#[test]
fn test_pipeline_project() {
    let query = "* | project timestamp, severity, message";
    let result = parse_and_compile(query);
    assert!(result.is_ok());
}

#[test]
fn test_multiple_pipeline_stages() {
    let query = r#"* | where severity == "warning" | sort timestamp desc | limit 100"#;
    let result = parse_and_compile(query);
    assert!(result.is_ok());
    let compiled = result.unwrap();
    assert!(compiled.sql_order_by.is_some());
}

#[test]
fn test_full_realistic_query_connection_analysis() {
    let query = r#"@24h message contains "connection" | where severity == "info" or severity == "warning" | sort timestamp desc | limit 500"#;
    let result = parse_and_compile(query);
    assert!(result.is_ok());
}

#[test]
fn test_full_realistic_query_error_investigation() {
    let query = r#":errors | sort timestamp desc | head 100"#;
    let result = parse_and_compile(query);
    assert!(result.is_ok());
}

#[test]
fn test_full_realistic_query_access_audit() {
    let query = r#"message contains "access denied" or message contains "invalid credentials" | limit 1000"#;
    let result = parse_and_compile(query);
    assert!(result.is_ok());
}

#[test]
fn test_timestamp_comparison_relative() {
    let query = "timestamp >= @1h";
    let result = parse_and_compile(query);
    assert!(result.is_ok());
}

#[test]
fn test_timestamp_comparison_relative_less_than() {
    let query = "timestamp <= @24h";
    let result = parse_and_compile(query);
    assert!(result.is_ok());
}

#[test]
fn test_wildcard_only() {
    let query = "*";
    let result = parse_and_compile(query);
    assert!(result.is_ok());
    let compiled = result.unwrap();
    assert!(compiled.sql_where_fragments.is_empty());
}

#[test]
fn test_invalid_severity_fails() {
    let query = r#"{severity="invalid_level"}"#;
    let parsed = parse(query);
    assert!(parsed.is_ok());
    let compiled = rabbitmq_lqt_ql::compile(&parsed.unwrap());
    assert!(compiled.is_err());
}

#[test]
fn test_invalid_regex_fails() {
    let query = r#"message =~ "[invalid""#;
    let parsed = parse(query);
    assert!(parsed.is_ok());
    let compiled = rabbitmq_lqt_ql::compile(&parsed.unwrap());
    assert!(compiled.is_err());
}

#[test]
fn test_unknown_label_fails() {
    let query = r#"labels any ["nonexistent_label_xyz"]"#;
    let parsed = parse(query);
    assert!(parsed.is_ok());
    let compiled = rabbitmq_lqt_ql::compile(&parsed.unwrap());
    assert!(compiled.is_err());
}

#[test]
fn test_empty_query_fails() {
    let result = parse("");
    assert!(result.is_err());
}

#[test]
fn test_unknown_preset_fails() {
    let result = parse(":nonexistent_preset");
    assert!(result.is_err());
}

#[test]
fn test_invalid_field_fails() {
    let result = parse(r#"{invalid_field="value"}"#);
    assert!(result.is_err());
}

#[test]
fn test_sql_injection_prevention() {
    let query = r#"message contains "'; DROP TABLE logs; --""#;
    let result = parse_and_compile(query);
    assert!(result.is_ok());
    let compiled = result.unwrap();
    let sql = &compiled.sql_where_fragments[0];
    assert!(sql.contains("''; DROP TABLE logs; --"));
}

#[test]
fn test_query_with_apostrophe() {
    let query = r#"message contains "user's connection""#;
    let result = parse_and_compile(query);
    assert!(result.is_ok());
    let compiled = result.unwrap();
    let sql = &compiled.sql_where_fragments[0];
    assert!(sql.contains("user''s connection"));
}

#[test]
fn test_subsystem_selector() {
    let query = r#"{subsystem="connections"}"#;
    let result = parse_and_compile(query);
    assert!(result.is_ok());
}

#[test]
fn test_multiple_selectors() {
    let query = r#"{severity="warning", subsystem="connections"}"#;
    let result = parse_and_compile(query);
    assert!(result.is_ok());
}

#[test]
fn test_complete_query_structure() {
    let query = r#"@7d {severity="warning"} message contains "connection" | where not message contains "closed" | sort timestamp desc | limit 200"#;
    let result = parse_and_compile(query);
    assert!(result.is_ok());
    let compiled = result.unwrap();
    assert!(!compiled.sql_where_fragments.is_empty());
    assert!(compiled.sql_order_by.is_some());
}
