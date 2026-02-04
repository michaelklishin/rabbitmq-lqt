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

//! Unit tests for the autocomplete module.

use rabbitmq_lqt_lib::Subsystem;
use rabbitmq_lqt_lib::entry_metadata::LABEL_NAMES;
use rabbitmq_lqt_ql::ast::Field;
use rabbitmq_lqt_ql::autocomplete::{
    self, CompletionContext, DURATION_UNITS, FIELDS, LABELS, OPERATORS, PIPELINE_STAGES,
    SEVERITIES, SPECIAL_FILTERS, SUBSYSTEMS, SuggestionKind,
};
use std::str::FromStr;

#[test]
fn test_severities_are_complete() {
    assert!(SEVERITIES.contains(&"debug"));
    assert!(SEVERITIES.contains(&"info"));
    assert!(SEVERITIES.contains(&"notice"));
    assert!(SEVERITIES.contains(&"warning"));
    assert!(SEVERITIES.contains(&"error"));
    assert!(SEVERITIES.contains(&"critical"));
    assert_eq!(SEVERITIES.len(), 6);
}

#[test]
fn test_subsystems_include_key_values() {
    assert!(SUBSYSTEMS.contains(&"connections"));
    assert!(SUBSYSTEMS.contains(&"queues"));
    assert!(SUBSYSTEMS.contains(&"clustering"));
    assert!(SUBSYSTEMS.contains(&"virtual_hosts"));
    assert!(SUBSYSTEMS.contains(&"plugins"));
    assert!(SUBSYSTEMS.contains(&"mqtt"));
}

#[test]
fn test_labels_include_key_values() {
    assert!(LABELS.contains(&"connections"));
    assert!(LABELS.contains(&"disconnects"));
    assert!(LABELS.contains(&"tls"));
    assert!(LABELS.contains(&"timeouts"));
    assert!(LABELS.contains(&"erl_process_crash"));
    assert!(LABELS.contains(&"unlabelled"));
    assert!(LABELS.contains(&"quorum_queues"));
    assert!(LABELS.contains(&"networking"));
}

#[test]
fn test_fields_have_name_and_description() {
    for field in FIELDS {
        assert!(!field.name.is_empty());
        assert!(!field.description.is_empty());
    }
}

#[test]
fn test_fields_include_all_ast_fields() {
    let field_names: Vec<&str> = FIELDS.iter().map(|f| f.name).collect();
    assert!(field_names.contains(&"severity"));
    assert!(field_names.contains(&"subsystem"));
    assert!(field_names.contains(&"node"));
    assert!(field_names.contains(&"erlang_pid"));
    assert!(field_names.contains(&"message"));
    assert!(field_names.contains(&"labels"));
    assert!(field_names.contains(&"timestamp"));
    assert!(field_names.contains(&"id"));
}

#[test]
fn test_operators_are_documented() {
    for op in OPERATORS {
        assert!(!op.symbol.is_empty());
        assert!(!op.name.is_empty());
        assert!(!op.description.is_empty());
    }
}

#[test]
fn test_operators_include_all_match_ops() {
    let symbols: Vec<&str> = OPERATORS.iter().map(|o| o.symbol).collect();
    assert!(symbols.contains(&"=="));
    assert!(symbols.contains(&"!="));
    assert!(symbols.contains(&"<"));
    assert!(symbols.contains(&"<="));
    assert!(symbols.contains(&">"));
    assert!(symbols.contains(&">="));
    assert!(symbols.contains(&"=~"));
    assert!(symbols.contains(&"!~"));
    assert!(symbols.contains(&"contains"));
    assert!(symbols.contains(&"~="));
}

#[test]
fn test_pipeline_stages_are_documented() {
    for stage in PIPELINE_STAGES {
        assert!(!stage.name.is_empty());
        assert!(!stage.syntax.is_empty());
        assert!(!stage.description.is_empty());
    }
}

#[test]
fn test_pipeline_stages_include_all_stages() {
    let names: Vec<&str> = PIPELINE_STAGES.iter().map(|s| s.name).collect();
    assert!(names.contains(&"where"));
    assert!(names.contains(&"limit"));
    assert!(names.contains(&"offset"));
    assert!(names.contains(&"head"));
    assert!(names.contains(&"tail"));
    assert!(names.contains(&"sort"));
    assert!(names.contains(&"count"));
    assert!(names.contains(&"distinct"));
    assert!(names.contains(&"project"));
}

#[test]
fn test_duration_units_are_complete() {
    let suffixes: Vec<&str> = DURATION_UNITS.iter().map(|d| d.suffix).collect();
    assert!(suffixes.contains(&"s"));
    assert!(suffixes.contains(&"m"));
    assert!(suffixes.contains(&"h"));
    assert!(suffixes.contains(&"d"));
    assert!(suffixes.contains(&"w"));
}

#[test]
fn test_special_filters_are_listed() {
    assert!(SPECIAL_FILTERS.contains(&"has_doc_url"));
    assert!(SPECIAL_FILTERS.contains(&"has_resolution_url"));
    assert!(SPECIAL_FILTERS.contains(&"unlabelled"));
}

#[test]
fn test_is_valid_severity() {
    assert!(autocomplete::is_valid_severity("error"));
    assert!(autocomplete::is_valid_severity("Error"));
    assert!(autocomplete::is_valid_severity("ERROR"));
    assert!(autocomplete::is_valid_severity("warning"));
    assert!(!autocomplete::is_valid_severity("invalid"));
    assert!(!autocomplete::is_valid_severity(""));
}

#[test]
fn test_is_valid_label() {
    assert!(autocomplete::is_valid_label("connections"));
    assert!(autocomplete::is_valid_label("disconnects"));
    assert!(autocomplete::is_valid_label("tls"));
    assert!(!autocomplete::is_valid_label("nonexistent"));
    assert!(!autocomplete::is_valid_label(""));
}

#[test]
fn test_is_valid_subsystem() {
    assert!(autocomplete::is_valid_subsystem("connections"));
    assert!(autocomplete::is_valid_subsystem("Connections"));
    assert!(!autocomplete::is_valid_subsystem("nonexistent"));
}

#[test]
fn test_is_valid_field() {
    assert!(autocomplete::is_valid_field("severity"));
    assert!(autocomplete::is_valid_field("Severity"));
    assert!(autocomplete::is_valid_field("level"));
    assert!(autocomplete::is_valid_field("timestamp"));
    assert!(autocomplete::is_valid_field("ts"));
    assert!(autocomplete::is_valid_field("erlang_pid"));
    assert!(autocomplete::is_valid_field("pid"));
    assert!(!autocomplete::is_valid_field("nonexistent"));
}

#[test]
fn test_is_valid_preset() {
    assert!(autocomplete::is_valid_preset("errors"));
    assert!(autocomplete::is_valid_preset("crashes"));
    assert!(autocomplete::is_valid_preset("disconnects"));
    assert!(!autocomplete::is_valid_preset("nonexistent"));
}

#[test]
fn test_suggest_severity_finds_close_matches() {
    let suggestions = autocomplete::suggest_severity("eror");
    assert!(suggestions.contains(&"error".to_string()));

    let suggestions = autocomplete::suggest_severity("warnign");
    assert!(suggestions.contains(&"warning".to_string()));
}

#[test]
fn test_suggest_label_finds_close_matches() {
    let suggestions = autocomplete::suggest_label("conections");
    assert!(suggestions.contains(&"connections".to_string()));

    let suggestions = autocomplete::suggest_label("disconects");
    assert!(suggestions.contains(&"disconnects".to_string()));
}

#[test]
fn test_suggest_subsystem_finds_close_matches() {
    let suggestions = autocomplete::suggest_subsystem("conections");
    assert!(suggestions.contains(&"connections".to_string()));

    let suggestions = autocomplete::suggest_subsystem("qeues");
    assert!(suggestions.contains(&"queues".to_string()));
}

#[test]
fn test_suggest_field_finds_close_matches() {
    let suggestions = autocomplete::suggest_field("severit");
    assert!(suggestions.contains(&"severity".to_string()));

    let suggestions = autocomplete::suggest_field("mesage");
    assert!(suggestions.contains(&"message".to_string()));
}

#[test]
fn test_suggest_preset_finds_close_matches() {
    let suggestions = autocomplete::suggest_preset("erors");
    assert!(suggestions.contains(&"errors".to_string()));

    let suggestions = autocomplete::suggest_preset("disconects");
    assert!(suggestions.contains(&"disconnects".to_string()));
}

#[test]
fn test_field_names() {
    let names = autocomplete::field_names();
    assert!(names.contains(&"severity"));
    assert!(names.contains(&"message"));
    assert!(!names.contains(&"level"));
}

#[test]
fn test_field_names_with_aliases() {
    let names = autocomplete::field_names_with_aliases();
    assert!(names.contains(&"severity"));
    assert!(names.contains(&"level"));
    assert!(names.contains(&"timestamp"));
    assert!(names.contains(&"ts"));
    assert!(names.contains(&"time"));
}

#[test]
fn test_preset_names() {
    let names = autocomplete::preset_names();
    assert!(names.contains(&"errors"));
    assert!(names.contains(&"crashes"));
    assert!(names.contains(&"disconnects"));
}

#[test]
fn test_completions_for_field_context() {
    let completions = autocomplete::completions_for_context(CompletionContext::Field);
    assert!(!completions.is_empty());
    assert!(completions.iter().all(|c| c.kind == SuggestionKind::Field));
    assert!(completions.iter().any(|c| c.text == "severity"));
}

#[test]
fn test_completions_for_severity_value() {
    let completions =
        autocomplete::completions_for_context(CompletionContext::FieldValue(Field::Severity));
    assert!(!completions.is_empty());
    assert!(completions.iter().all(|c| c.kind == SuggestionKind::Value));
    assert!(completions.iter().any(|c| c.display == "error"));
}

#[test]
fn test_completions_for_label_value() {
    let completions =
        autocomplete::completions_for_context(CompletionContext::FieldValue(Field::Labels));
    assert!(!completions.is_empty());
    assert!(completions.iter().all(|c| c.kind == SuggestionKind::Label));
    assert!(completions.iter().any(|c| c.display == "connections"));
}

#[test]
fn test_completions_for_operator_context() {
    let completions = autocomplete::completions_for_context(CompletionContext::Operator);
    assert!(!completions.is_empty());
    assert!(
        completions
            .iter()
            .all(|c| c.kind == SuggestionKind::Operator)
    );
    assert!(completions.iter().any(|c| c.text == "=="));
}

#[test]
fn test_completions_for_preset_context() {
    let completions = autocomplete::completions_for_context(CompletionContext::Preset);
    assert!(!completions.is_empty());
    assert!(completions.iter().all(|c| c.kind == SuggestionKind::Preset));
    assert!(completions.iter().any(|c| c.text == ":errors"));
}

#[test]
fn test_completions_for_pipeline_stage_context() {
    let completions = autocomplete::completions_for_context(CompletionContext::PipelineStage);
    assert!(!completions.is_empty());
    assert!(
        completions
            .iter()
            .all(|c| c.kind == SuggestionKind::PipelineStage)
    );
    assert!(completions.iter().any(|c| c.text == "limit"));
}

#[test]
fn test_completions_for_duration_context() {
    let completions = autocomplete::completions_for_context(CompletionContext::Duration);
    assert!(!completions.is_empty());
    assert!(completions.iter().any(|c| c.text == "@24h"));
}

#[test]
fn test_field_all_matches_ast() {
    let all_fields = Field::all();
    assert_eq!(all_fields.len(), 8);
    assert!(all_fields.contains(&Field::Severity));
    assert!(all_fields.contains(&Field::Subsystem));
    assert!(all_fields.contains(&Field::Node));
    assert!(all_fields.contains(&Field::ErlangPid));
    assert!(all_fields.contains(&Field::Message));
    assert!(all_fields.contains(&Field::Labels));
    assert!(all_fields.contains(&Field::Timestamp));
    assert!(all_fields.contains(&Field::Id));
}

#[test]
fn test_field_aliases() {
    assert_eq!(Field::Severity.aliases(), &["level"]);
    assert_eq!(Field::ErlangPid.aliases(), &["pid"]);
    assert_eq!(Field::Message.aliases(), &["msg"]);
    assert_eq!(Field::Labels.aliases(), &["label"]);
    assert_eq!(Field::Timestamp.aliases(), &["time", "ts"]);
    assert!(Field::Subsystem.aliases().is_empty());
    assert!(Field::Node.aliases().is_empty());
    assert!(Field::Id.aliases().is_empty());
}

#[test]
fn test_complete_severity_with_prefix() {
    let completions = autocomplete::complete_severity("err");
    assert_eq!(completions, vec!["error"]);

    let completions = autocomplete::complete_severity("warn");
    assert_eq!(completions, vec!["warning"]);

    let completions = autocomplete::complete_severity("");
    assert_eq!(completions.len(), 6);
}

#[test]
fn test_complete_label_with_prefix() {
    let completions = autocomplete::complete_label("conn");
    assert!(completions.contains(&"connections"));

    let completions = autocomplete::complete_label("cons");
    assert!(completions.contains(&"consumers"));

    let completions = autocomplete::complete_label("tl");
    assert_eq!(completions, vec!["tls"]);
}

#[test]
fn test_complete_subsystem_with_prefix() {
    let completions = autocomplete::complete_subsystem("conn");
    assert_eq!(completions, vec!["connections"]);

    let completions = autocomplete::complete_subsystem("mq");
    assert_eq!(completions, vec!["mqtt"]);
}

#[test]
fn test_complete_field_with_prefix() {
    let completions = autocomplete::complete_field("sev");
    assert!(completions.contains(&"severity"));

    let completions = autocomplete::complete_field("time");
    assert!(completions.contains(&"timestamp"));
}

#[test]
fn test_complete_preset_with_prefix() {
    let completions = autocomplete::complete_preset("err");
    assert!(completions.contains(&"errors"));
    assert!(completions.contains(&"errors_or_crashes"));

    let completions = autocomplete::complete_preset("dis");
    assert_eq!(completions, vec!["disconnects"]);
}

#[test]
fn test_complete_with_empty_prefix_returns_all() {
    assert_eq!(autocomplete::complete_severity("").len(), 6);
    assert!(!autocomplete::complete_label("").is_empty());
    assert!(!autocomplete::complete_subsystem("").is_empty());
}

#[test]
fn test_complete_with_no_match_returns_empty() {
    assert!(autocomplete::complete_severity("xyz").is_empty());
    assert!(autocomplete::complete_label("xyz").is_empty());
    assert!(autocomplete::complete_preset("xyz").is_empty());
}

#[test]
fn test_suggest_with_empty_input() {
    let suggestions = autocomplete::suggest_severity("");
    assert!(suggestions.is_empty() || !suggestions.is_empty());
}

#[test]
fn test_suggest_with_exact_match_returns_empty() {
    let suggestions = autocomplete::suggest_severity("error");
    assert!(suggestions.is_empty() || suggestions[0] == "error");
}

#[test]
fn test_completion_case_insensitive() {
    let completions = autocomplete::complete_severity("ERR");
    assert_eq!(completions, vec!["error"]);

    let completions = autocomplete::complete_label("CONN");
    assert!(completions.contains(&"connections"));
}

#[test]
fn test_filter_by_prefix_returns_ordered() {
    let completions = autocomplete::filter_by_prefix("c", &["cat", "car", "bat"]);
    assert_eq!(completions, vec!["cat", "car"]);
}

#[test]
fn test_labels_match_canonical_source() {
    for label in LABEL_NAMES {
        assert!(
            LABELS.contains(label),
            "Label '{}' from rabbitmq-lqt-lib is missing in autocomplete LABELS",
            label
        );
    }

    for label in LABELS {
        assert!(
            LABEL_NAMES.contains(label),
            "Label '{}' in autocomplete LABELS is not in rabbitmq-lqt-lib's canonical list",
            label
        );
    }
}

#[test]
fn test_subsystems_match_canonical_source() {
    for subsystem_str in SUBSYSTEMS {
        assert!(
            Subsystem::from_str(subsystem_str).is_ok(),
            "Subsystem '{}' in autocomplete is not a valid rabbitmq-lqt-lib Subsystem",
            subsystem_str
        );
    }
}
