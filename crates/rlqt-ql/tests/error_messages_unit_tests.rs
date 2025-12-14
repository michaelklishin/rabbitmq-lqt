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

//! Unit tests for error messages and diagnostics.

use rlqt_ql::errors::CompileError;
use rlqt_ql::{Diagnostic, ParseError, Span, compile, parse};

#[test]
fn test_parse_error_invalid_field_includes_suggestion() {
    let err = ParseError::invalid_field("severit");
    let msg = err.to_string();
    assert!(msg.contains("Invalid field name 'severit'"));
    assert!(msg.contains("Did you mean 'severity'?"));
}

#[test]
fn test_parse_error_unknown_preset_includes_suggestion() {
    let err = ParseError::unknown_preset("erors");
    let msg = err.to_string();
    assert!(msg.contains("Unknown preset ':erors'"));
    assert!(msg.contains("Did you mean 'errors'?"));
}

#[test]
fn test_parse_error_unknown_label_includes_suggestion() {
    let err = ParseError::unknown_label("conections");
    let msg = err.to_string();
    assert!(msg.contains("Unknown label 'conections'"));
    assert!(msg.contains("Did you mean 'connections'?"));
}

#[test]
fn test_parse_error_invalid_severity_includes_suggestion() {
    let err = ParseError::invalid_severity("eror");
    let msg = err.to_string();
    assert!(msg.contains("Invalid severity level 'eror'"));
    assert!(msg.contains("Did you mean 'error'?"));
}

#[test]
fn test_compile_error_invalid_severity_includes_suggestion() {
    let err = CompileError::invalid_severity("warnign");
    let msg = err.to_string();
    assert!(msg.contains("Invalid severity level 'warnign'"));
    assert!(msg.contains("Did you mean 'warning'?"));
}

#[test]
fn test_compile_error_unknown_label_includes_suggestion() {
    let err = CompileError::unknown_label("disconects");
    let msg = err.to_string();
    assert!(msg.contains("Unknown label 'disconects'"));
    assert!(msg.contains("Did you mean 'disconnects'?"));
}

#[test]
fn test_compile_error_invalid_subsystem_includes_suggestion() {
    let err = CompileError::invalid_subsystem("conections");
    let msg = err.to_string();
    assert!(msg.contains("Invalid subsystem 'conections'"));
    assert!(msg.contains("Did you mean 'connections'?"));
}

#[test]
fn test_parse_error_includes_valid_values_list() {
    let err = ParseError::invalid_field("foo");
    let msg = err.to_string();
    assert!(msg.contains("severity"));
    assert!(msg.contains("subsystem"));
    assert!(msg.contains("timestamp"));
}

#[test]
fn test_compile_error_includes_valid_severity_list() {
    let err = CompileError::invalid_severity("foo");
    let msg = err.to_string();
    assert!(msg.contains("debug"));
    assert!(msg.contains("info"));
    assert!(msg.contains("warning"));
    assert!(msg.contains("error"));
}

#[test]
fn test_span_creation() {
    let span = Span::new(10, 5);
    assert_eq!(span.offset, 10);
    assert_eq!(span.length, 5);
    assert_eq!(span.end(), 15);
}

#[test]
fn test_span_at() {
    let span = Span::at(7);
    assert_eq!(span.offset, 7);
    assert_eq!(span.length, 1);
}

#[test]
fn test_diagnostic_builder() {
    let diag = Diagnostic::new("Test error")
        .with_span(Span::new(5, 3))
        .with_source("foo bar baz")
        .with_suggestions(vec!["bar".to_string()])
        .with_help("Try using 'bar' instead");

    assert_eq!(diag.message, "Test error");
    assert!(diag.span.is_some());
    assert!(diag.source.is_some());
    assert!(!diag.suggestions.is_empty());
    assert!(diag.help.is_some());
}

#[test]
fn test_diagnostic_render_basic() {
    let diag = Diagnostic::new("Test error message");
    let output = diag.render();
    assert!(output.contains("error: Test error message"));
}

#[test]
fn test_diagnostic_render_with_span() {
    let diag = Diagnostic::new("Unknown field")
        .with_span(Span::new(4, 3))
        .with_source("foo bar baz");
    let output = diag.render();
    assert!(output.contains("foo bar baz"));
    assert!(output.contains("^^^"));
}

#[test]
fn test_diagnostic_render_with_suggestion() {
    let diag = Diagnostic::new("Unknown label").with_suggestions(vec!["connections".to_string()]);
    let output = diag.render();
    assert!(output.contains("did you mean 'connections'?"));
}

#[test]
fn test_diagnostic_render_with_help() {
    let diag = Diagnostic::new("Invalid value").with_help("Check the documentation");
    let output = diag.render();
    assert!(output.contains("help: Check the documentation"));
}

#[test]
fn test_diagnostic_from_parse_error_invalid_field() {
    let err = ParseError::invalid_field("severit");
    let diag = Diagnostic::from(&err);
    assert!(diag.message.contains("Invalid field name"));
    assert!(!diag.suggestions.is_empty());
    assert!(diag.help.is_some());
}

#[test]
fn test_diagnostic_from_parse_error_unknown_preset() {
    let err = ParseError::unknown_preset("erors");
    let diag = Diagnostic::from(&err);
    assert!(diag.message.contains("Unknown preset"));
    assert!(!diag.suggestions.is_empty());
}

#[test]
fn test_diagnostic_from_compile_error_invalid_severity() {
    let err = CompileError::invalid_severity("eror");
    let diag = Diagnostic::from(&err);
    assert!(diag.message.contains("Invalid severity"));
    assert!(!diag.suggestions.is_empty());
}

#[test]
fn test_diagnostic_display_trait() {
    let diag = Diagnostic::new("Test error");
    let display_output = format!("{}", diag);
    let render_output = diag.render();
    assert_eq!(display_output, render_output);
}

#[test]
fn test_parse_invalid_severity_produces_good_error() {
    let result = parse(r#"{severity="invalid_level"}"#);
    assert!(result.is_ok());

    let query = result.unwrap();
    let result = compile(&query);
    assert!(result.is_err());

    let err = result.unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("Invalid severity level"));
    assert!(msg.contains("invalid_level"));
}

#[test]
fn test_parse_unknown_label_produces_good_error() {
    let result = parse(r#"labels any ["nonexistent_label"]"#);
    assert!(result.is_ok());

    let query = result.unwrap();
    let result = compile(&query);
    assert!(result.is_err());

    let err = result.unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("Unknown label"));
    assert!(msg.contains("nonexistent_label"));
}

#[test]
fn test_parse_unknown_preset_produces_good_error() {
    let result = parse(":nonexistent_preset");
    assert!(result.is_err());

    let err = result.unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("Unknown preset"),
        "Expected 'Unknown preset' but got: {}",
        msg
    );
}

#[test]
fn test_parse_invalid_field_produces_good_error() {
    let result = parse(r#"{invalid_field="value"}"#);
    assert!(result.is_err());

    let err = result.unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("Parse failed"));
}

#[test]
fn test_typo_correction_for_severity_erro() {
    let err = CompileError::invalid_severity("erro");
    let msg = err.to_string();
    assert!(msg.contains("Did you mean 'error'?"));
}

#[test]
fn test_typo_correction_for_severity_warn() {
    let err = CompileError::invalid_severity("warn");
    let msg = err.to_string();
    assert!(msg.contains("Did you mean 'warning'?"));
}

#[test]
fn test_typo_correction_for_label_connection() {
    let err = CompileError::unknown_label("connection");
    let msg = err.to_string();
    assert!(msg.contains("Did you mean 'connections'?"));
}

#[test]
fn test_typo_correction_for_label_timeout() {
    let err = CompileError::unknown_label("timeout");
    let msg = err.to_string();
    assert!(msg.contains("Did you mean 'timeouts'?"));
}

#[test]
fn test_no_suggestion_for_very_different_input() {
    let err = CompileError::unknown_label("xyzabc123");
    let msg = err.to_string();
    assert!(!msg.contains("Did you mean"));
}

#[test]
fn test_parse_error_empty_query() {
    let result = parse("");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(err, ParseError::EmptyQuery));
    let msg = err.to_string();
    assert!(msg.contains("Empty query"));
}

#[test]
fn test_parse_error_preserves_position() {
    let result = parse(r#"{severity="error"} | invalid_stage"#);
    assert!(result.is_err());
    let err = result.unwrap_err();
    match err {
        ParseError::ParseFailed { position, .. } => {
            assert!(position > 0);
        }
        _ => panic!("Expected ParseFailed error"),
    }
}
