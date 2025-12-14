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

//! Error types for the RLQT Query Language compiler.

use rlqt_ql_core::{Diagnostic, ParseError, autocomplete};
use std::fmt::Write;
use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("Parse error: {0}")]
    Parse(#[from] ParseError),

    #[error("Compile error: {0}")]
    Compile(#[from] CompileError),
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CompileError {
    #[error("Regex compilation failed for pattern '{pattern}': {reason}")]
    RegexCompilation { pattern: String, reason: String },

    #[error("{}", format_compile_invalid_severity(.level, .suggestions))]
    InvalidSeverity {
        level: String,
        suggestions: Vec<String>,
    },

    #[error("{}", format_compile_invalid_subsystem(.subsystem, .suggestions))]
    InvalidSubsystem {
        subsystem: String,
        suggestions: Vec<String>,
    },

    #[error("{}", format_compile_unknown_label(.label, .suggestions))]
    UnknownLabel {
        label: String,
        suggestions: Vec<String>,
    },

    #[error("Unsupported operation: {operation}")]
    UnsupportedOperation { operation: String },

    #[error("Invalid timestamp: {reason}")]
    InvalidTimestamp { reason: String },
}

impl CompileError {
    pub fn invalid_severity(level: impl Into<String>) -> Self {
        let level = level.into();
        let suggestions = autocomplete::suggest_severity(&level);
        Self::InvalidSeverity { level, suggestions }
    }

    pub fn invalid_subsystem(subsystem: impl Into<String>) -> Self {
        let subsystem = subsystem.into();
        let suggestions = autocomplete::suggest_subsystem(&subsystem);
        Self::InvalidSubsystem {
            subsystem,
            suggestions,
        }
    }

    pub fn unknown_label(label: impl Into<String>) -> Self {
        let label = label.into();
        let suggestions = autocomplete::suggest_label(&label);
        Self::UnknownLabel { label, suggestions }
    }
}

fn format_compile_invalid_severity(level: &str, suggestions: &[String]) -> String {
    let valid = "debug, info, notice, warning, error, critical";
    let mut msg = format!("Invalid severity level '{level}'. Valid levels: {valid}");
    append_suggestions(&mut msg, suggestions);
    msg
}

fn format_compile_invalid_subsystem(subsystem: &str, suggestions: &[String]) -> String {
    let mut msg = format!("Invalid subsystem '{subsystem}'");
    append_suggestions(&mut msg, suggestions);
    msg
}

fn format_compile_unknown_label(label: &str, suggestions: &[String]) -> String {
    let mut msg = format!("Unknown label '{label}'");
    append_suggestions(&mut msg, suggestions);
    msg
}

fn append_suggestions(msg: &mut String, suggestions: &[String]) {
    if !suggestions.is_empty() {
        let _ = write!(msg, ". Did you mean '{}'?", suggestions[0]);
    }
}

impl From<&CompileError> for Diagnostic {
    fn from(err: &CompileError) -> Self {
        match err {
            CompileError::InvalidSeverity { level, suggestions } => {
                Diagnostic::new(format!("Invalid severity level '{}'", level))
                    .with_suggestions(suggestions.clone())
                    .with_help("Valid levels: debug, info, notice, warning, error, critical")
            }

            CompileError::InvalidSubsystem {
                subsystem,
                suggestions,
            } => Diagnostic::new(format!("Invalid subsystem '{}'", subsystem))
                .with_suggestions(suggestions.clone()),

            CompileError::UnknownLabel { label, suggestions } => {
                Diagnostic::new(format!("Unknown label '{}'", label))
                    .with_suggestions(suggestions.clone())
            }

            _ => Diagnostic::new(err.to_string()),
        }
    }
}
