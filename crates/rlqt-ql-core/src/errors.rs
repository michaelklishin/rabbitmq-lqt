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

//! Error types for the RLQT Query Language parser.

use std::fmt::{self, Display, Write};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Span {
    pub offset: usize,
    pub length: usize,
}

impl Span {
    pub fn new(offset: usize, length: usize) -> Self {
        Self { offset, length }
    }

    pub fn at(offset: usize) -> Self {
        Self { offset, length: 1 }
    }

    pub fn end(&self) -> usize {
        self.offset + self.length
    }
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParseError {
    #[error("Unexpected end of input")]
    UnexpectedEof,

    #[error("{}", format_invalid_field(.name, .suggestions))]
    InvalidField {
        name: String,
        suggestions: Vec<String>,
    },

    #[error("Invalid operator '{op}'")]
    InvalidOperator { op: String },

    #[error("Invalid value '{value}': {reason}")]
    InvalidValue { value: String, reason: String },

    #[error("Invalid duration '{input}'. Expected format: <number><unit> (e.g., 1h, 30m, 7d)")]
    InvalidDuration { input: String },

    #[error("Invalid timestamp '{input}': {reason}")]
    InvalidTimestamp { input: String, reason: String },

    #[error("Invalid regex pattern '{pattern}': {reason}")]
    InvalidRegex { pattern: String, reason: String },

    #[error("{}", format_unknown_preset(.name, .suggestions))]
    UnknownPreset {
        name: String,
        suggestions: Vec<String>,
    },

    #[error("{}", format_unknown_label(.name, .suggestions))]
    UnknownLabel {
        name: String,
        suggestions: Vec<String>,
    },

    #[error("Syntax error: expected {expected}, found '{found}'")]
    SyntaxError { expected: String, found: String },

    #[error("Unclosed string literal starting at position {position}")]
    UnclosedString { position: usize },

    #[error("Unclosed regex pattern starting at position {position}")]
    UnclosedRegex { position: usize },

    #[error("Empty query")]
    EmptyQuery,

    #[error("Unexpected character '{ch}' at position {position}")]
    UnexpectedChar { ch: char, position: usize },

    #[error("Parse failed at position {position}: {message}")]
    ParseFailed { message: String, position: usize },

    #[error("{}", format_invalid_severity(.level, .suggestions))]
    InvalidSeverity {
        level: String,
        suggestions: Vec<String>,
    },

    #[error("{}", format_invalid_subsystem(.name, .suggestions))]
    InvalidSubsystem {
        name: String,
        suggestions: Vec<String>,
    },
}

impl ParseError {
    pub fn invalid_field(name: impl Into<String>) -> Self {
        let name = name.into();
        let suggestions = crate::autocomplete::suggest_field(&name);
        Self::InvalidField { name, suggestions }
    }

    pub fn unknown_preset(name: impl Into<String>) -> Self {
        let name = name.into();
        let suggestions = crate::autocomplete::suggest_preset(&name);
        Self::UnknownPreset { name, suggestions }
    }

    pub fn unknown_label(name: impl Into<String>) -> Self {
        let name = name.into();
        let suggestions = crate::autocomplete::suggest_label(&name);
        Self::UnknownLabel { name, suggestions }
    }

    pub fn invalid_severity(level: impl Into<String>) -> Self {
        let level = level.into();
        let suggestions = crate::autocomplete::suggest_severity(&level);
        Self::InvalidSeverity { level, suggestions }
    }

    pub fn invalid_subsystem(name: impl Into<String>) -> Self {
        let name = name.into();
        let suggestions = crate::autocomplete::suggest_subsystem(&name);
        Self::InvalidSubsystem { name, suggestions }
    }

    pub fn position(&self) -> Option<usize> {
        match self {
            Self::ParseFailed { position, .. } => Some(*position),
            Self::UnclosedString { position } => Some(*position),
            Self::UnclosedRegex { position } => Some(*position),
            Self::UnexpectedChar { position, .. } => Some(*position),
            _ => None,
        }
    }

    pub fn suggestions(&self) -> Vec<String> {
        match self {
            Self::InvalidField { suggestions, .. } => suggestions.clone(),
            Self::UnknownPreset { suggestions, .. } => suggestions.clone(),
            Self::UnknownLabel { suggestions, .. } => suggestions.clone(),
            Self::InvalidSeverity { suggestions, .. } => suggestions.clone(),
            Self::InvalidSubsystem { suggestions, .. } => suggestions.clone(),
            _ => vec![],
        }
    }
}

fn format_invalid_field(name: &str, suggestions: &[String]) -> String {
    let valid = "severity, subsystem, node, erlang_pid, message, labels, timestamp, id";
    let mut msg = format!("Invalid field name '{name}'. Valid fields: {valid}");
    append_suggestions(&mut msg, suggestions);
    msg
}

fn format_unknown_preset(name: &str, suggestions: &[String]) -> String {
    let valid =
        "errors, crashes, errors_or_crashes, disconnects, tls_issues, access_denied, timeouts";
    let mut msg = format!("Unknown preset ':{name}'. Valid presets: {valid}");
    append_suggestions(&mut msg, suggestions);
    msg
}

fn format_unknown_label(name: &str, suggestions: &[String]) -> String {
    let mut msg = format!("Unknown label '{name}'");
    append_suggestions(&mut msg, suggestions);
    msg
}

fn format_invalid_severity(level: &str, suggestions: &[String]) -> String {
    let valid = "debug, info, notice, warning, error, critical";
    let mut msg = format!("Invalid severity level '{level}'. Valid levels: {valid}");
    append_suggestions(&mut msg, suggestions);
    msg
}

fn format_invalid_subsystem(name: &str, suggestions: &[String]) -> String {
    let mut msg = format!("Invalid subsystem '{name}'");
    append_suggestions(&mut msg, suggestions);
    msg
}

fn append_suggestions(msg: &mut String, suggestions: &[String]) {
    if !suggestions.is_empty() {
        let _ = write!(msg, ". Did you mean '{}'?", suggestions[0]);
    }
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub message: String,
    pub span: Option<Span>,
    pub source: Option<String>,
    pub suggestions: Vec<String>,
    pub help: Option<String>,
}

impl Diagnostic {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            span: None,
            source: None,
            suggestions: Vec::new(),
            help: None,
        }
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }

    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    pub fn with_suggestions(mut self, suggestions: Vec<String>) -> Self {
        self.suggestions = suggestions;
        self
    }

    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    pub fn render(&self) -> String {
        let mut output = String::new();
        let _ = writeln!(output, "error: {}", self.message);

        if let (Some(source), Some(span)) = (&self.source, &self.span) {
            let _ = writeln!(output, "  |");
            let _ = writeln!(output, "  | {}", source);

            let padding = " ".repeat(span.offset);
            let underline = "^".repeat(span.length.max(1));
            let _ = writeln!(output, "  | {}{}", padding, underline);
        }

        if !self.suggestions.is_empty() {
            let _ = writeln!(output, "  |");
            let suggestion = &self.suggestions[0];
            let _ = writeln!(output, "  = help: did you mean '{}'?", suggestion);
        }

        if let Some(help) = &self.help {
            if self.suggestions.is_empty() {
                let _ = writeln!(output, "  |");
            }
            let _ = writeln!(output, "  = help: {}", help);
        }

        output
    }
}

impl Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

impl From<&ParseError> for Diagnostic {
    fn from(err: &ParseError) -> Self {
        match err {
            ParseError::InvalidField { name, suggestions } => Diagnostic::new(format!(
                "Invalid field name '{}'",
                name
            ))
            .with_suggestions(suggestions.clone())
            .with_help("Valid fields: severity, subsystem, node, erlang_pid, message, labels, timestamp, id"),

            ParseError::UnknownPreset { name, suggestions } => Diagnostic::new(format!(
                "Unknown preset ':{}'. Valid presets: errors, crashes, errors_or_crashes, disconnects, tls_issues, access_denied, timeouts",
                name
            ))
            .with_suggestions(suggestions.clone()),

            ParseError::UnknownLabel { name, suggestions } => Diagnostic::new(format!(
                "Unknown label '{}'",
                name
            ))
            .with_suggestions(suggestions.clone()),

            ParseError::InvalidSeverity { level, suggestions } => Diagnostic::new(format!(
                "Invalid severity level '{}'",
                level
            ))
            .with_suggestions(suggestions.clone())
            .with_help("Valid levels: debug, info, notice, warning, error, critical"),

            ParseError::InvalidSubsystem { name, suggestions } => Diagnostic::new(format!(
                "Invalid subsystem '{}'",
                name
            ))
            .with_suggestions(suggestions.clone()),

            _ => Diagnostic::new(err.to_string()),
        }
    }
}
