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

//! WebAssembly bindings for RLQT Query Language parser and autocomplete.

use rlqt_ql_core::{autocomplete, parse, presets::PresetName};
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[derive(Serialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub error_message: Option<String>,
    pub error_position: Option<usize>,
    pub suggestions: Vec<String>,
}

#[wasm_bindgen]
pub fn validate_query(input: &str) -> JsValue {
    let result = match parse(input) {
        Ok(_) => ValidationResult {
            valid: true,
            error_message: None,
            error_position: None,
            suggestions: vec![],
        },
        Err(e) => ValidationResult {
            valid: false,
            error_message: Some(e.to_string()),
            error_position: e.position(),
            suggestions: e.suggestions(),
        },
    };
    serde_wasm_bindgen::to_value(&result).unwrap()
}

#[derive(Serialize)]
pub struct FieldInfo {
    pub name: &'static str,
    pub aliases: Vec<&'static str>,
    pub description: &'static str,
    pub example_values: Vec<&'static str>,
}

#[derive(Serialize)]
pub struct OperatorInfo {
    pub symbol: &'static str,
    pub aliases: Vec<&'static str>,
    pub name: &'static str,
    pub description: &'static str,
}

#[derive(Serialize)]
pub struct PipelineStageInfo {
    pub name: &'static str,
    pub aliases: Vec<&'static str>,
    pub syntax: &'static str,
    pub description: &'static str,
}

#[derive(Serialize)]
pub struct DurationUnitInfo {
    pub suffix: &'static str,
    pub name: &'static str,
    pub example: &'static str,
}

#[derive(Serialize)]
pub struct PresetInfo {
    pub name: &'static str,
    pub description: &'static str,
    pub query_string: &'static str,
}

#[derive(Serialize)]
pub struct AutocompleteData {
    pub severities: Vec<&'static str>,
    pub subsystems: Vec<&'static str>,
    pub labels: Vec<&'static str>,
    pub fields: Vec<FieldInfo>,
    pub operators: Vec<OperatorInfo>,
    pub pipeline_stages: Vec<PipelineStageInfo>,
    pub duration_units: Vec<DurationUnitInfo>,
    pub presets: Vec<PresetInfo>,
    pub special_filters: Vec<&'static str>,
}

#[wasm_bindgen]
pub fn get_autocomplete_data() -> JsValue {
    let data = AutocompleteData {
        severities: autocomplete::SEVERITIES.to_vec(),
        subsystems: autocomplete::SUBSYSTEMS.to_vec(),
        labels: autocomplete::LABELS.to_vec(),
        fields: autocomplete::FIELDS
            .iter()
            .map(|f| FieldInfo {
                name: f.name,
                aliases: f.aliases.to_vec(),
                description: f.description,
                example_values: f.example_values.to_vec(),
            })
            .collect(),
        operators: autocomplete::OPERATORS
            .iter()
            .map(|o| OperatorInfo {
                symbol: o.symbol,
                aliases: o.aliases.to_vec(),
                name: o.name,
                description: o.description,
            })
            .collect(),
        pipeline_stages: autocomplete::PIPELINE_STAGES
            .iter()
            .map(|s| PipelineStageInfo {
                name: s.name,
                aliases: s.aliases.to_vec(),
                syntax: s.syntax,
                description: s.description,
            })
            .collect(),
        duration_units: autocomplete::DURATION_UNITS
            .iter()
            .map(|d| DurationUnitInfo {
                suffix: d.suffix,
                name: d.name,
                example: d.example,
            })
            .collect(),
        presets: PresetName::all()
            .iter()
            .map(|p| PresetInfo {
                name: p.as_str(),
                description: p.description(),
                query_string: p.query_string(),
            })
            .collect(),
        special_filters: autocomplete::SPECIAL_FILTERS.to_vec(),
    };
    serde_wasm_bindgen::to_value(&data).unwrap()
}

#[wasm_bindgen]
pub fn complete_field(prefix: &str) -> JsValue {
    let completions: Vec<&str> = autocomplete::complete_field(prefix);
    serde_wasm_bindgen::to_value(&completions).unwrap()
}

#[wasm_bindgen]
pub fn complete_severity(prefix: &str) -> JsValue {
    let completions: Vec<&str> = autocomplete::complete_severity(prefix);
    serde_wasm_bindgen::to_value(&completions).unwrap()
}

#[wasm_bindgen]
pub fn complete_subsystem(prefix: &str) -> JsValue {
    let completions: Vec<&str> = autocomplete::complete_subsystem(prefix);
    serde_wasm_bindgen::to_value(&completions).unwrap()
}

#[wasm_bindgen]
pub fn complete_label(prefix: &str) -> JsValue {
    let completions: Vec<&str> = autocomplete::complete_label(prefix);
    serde_wasm_bindgen::to_value(&completions).unwrap()
}

#[wasm_bindgen]
pub fn complete_preset(prefix: &str) -> JsValue {
    let completions: Vec<&str> = autocomplete::complete_preset(prefix);
    serde_wasm_bindgen::to_value(&completions).unwrap()
}

#[wasm_bindgen]
pub fn suggest_field(input: &str) -> JsValue {
    let suggestions = autocomplete::suggest_field(input);
    serde_wasm_bindgen::to_value(&suggestions).unwrap()
}

#[wasm_bindgen]
pub fn suggest_severity(input: &str) -> JsValue {
    let suggestions = autocomplete::suggest_severity(input);
    serde_wasm_bindgen::to_value(&suggestions).unwrap()
}

#[wasm_bindgen]
pub fn suggest_subsystem(input: &str) -> JsValue {
    let suggestions = autocomplete::suggest_subsystem(input);
    serde_wasm_bindgen::to_value(&suggestions).unwrap()
}

#[wasm_bindgen]
pub fn suggest_label(input: &str) -> JsValue {
    let suggestions = autocomplete::suggest_label(input);
    serde_wasm_bindgen::to_value(&suggestions).unwrap()
}

#[wasm_bindgen]
pub fn suggest_preset(input: &str) -> JsValue {
    let suggestions = autocomplete::suggest_preset(input);
    serde_wasm_bindgen::to_value(&suggestions).unwrap()
}
