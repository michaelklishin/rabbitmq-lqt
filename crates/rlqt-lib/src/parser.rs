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
use crate::entry_metadata::labels::LogEntryLabels;
use crate::{Result, Severity};
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use nom::{
    Err as NomErr, IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, space1},
    combinator::{map_res, recognize},
    error::{Error as NomError, ErrorKind},
    sequence::delimited,
};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Initial capacity for the entries vector during parsing.
/// This value is not very scientific but helps a lot of reallocations
/// compared to the original version with a much smaller initial capacity,
/// according to benchmarks and profiling.
const INITIAL_ENTRIES_CAPACITY: usize = 16384;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParsedLogEntry {
    pub sequence_id: usize,
    pub explicit_id: Option<i64>,
    pub timestamp: DateTime<Utc>,
    pub severity: Severity,
    pub process_id: String,
    pub message: String,
    pub message_lowercased: String,
    pub subsystem_id: Option<i16>,
    pub labels: LogEntryLabels,
    pub resolution_or_discussion_url_id: Option<i16>,
    pub doc_url_id: Option<i16>,
}

impl ParsedLogEntry {
    /// Check if this log entry spans multiple lines
    #[inline]
    pub fn is_multiline(&self) -> bool {
        self.message.contains('\n')
    }

    #[inline]
    fn is_continuation_of(&self, other: &ParsedLogEntry) -> bool {
        self.timestamp == other.timestamp
            && self.severity == other.severity
            && self.process_id == other.process_id
    }

    #[inline]
    fn append_continuation(&mut self, content: &str) {
        self.message.reserve(1 + content.len());
        self.message.push('\n');
        self.message.push_str(content);

        self.message_lowercased.reserve(1 + content.len());
        self.message_lowercased.push('\n');
        self.message_lowercased.push_str(&content.to_lowercase());
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseResult {
    pub entries: Vec<ParsedLogEntry>,
    pub total_lines: usize,
}

fn process_new_entry(
    entries: &mut Vec<ParsedLogEntry>,
    current_entry: &mut Option<ParsedLogEntry>,
    new_entry: ParsedLogEntry,
) {
    let is_continuation = current_entry
        .as_ref()
        .is_some_and(|prev| new_entry.is_continuation_of(prev));

    match (is_continuation, current_entry.as_mut()) {
        (true, Some(prev_entry)) => {
            prev_entry.append_continuation(&new_entry.message);
        }
        (false, _) => {
            if let Some(prev_entry) = current_entry.take() {
                entries.push(prev_entry);
            }
            *current_entry = Some(new_entry);
        }
        (true, None) => {}
    }
}

fn process_continuation_line(
    current_entry: &mut Option<ParsedLogEntry>,
    line: &str,
    line_number: usize,
) {
    if let Some(entry) = current_entry {
        entry.append_continuation(line.trim_end());
    } else {
        log::warn!("Orphaned continuation line {}: {}", line_number + 1, line);
    }
}

pub fn parse_log_file<R: BufRead>(reader: R) -> Result<ParseResult> {
    let mut entries = Vec::with_capacity(INITIAL_ENTRIES_CAPACITY);
    let mut current_entry: Option<ParsedLogEntry> = None;
    let mut total_lines = 0;

    for (line_number, line_result) in reader.lines().enumerate() {
        total_lines = line_number + 1;
        let line = line_result.map_err(|e| crate::Error::ReadLine {
            line: total_lines,
            source: e,
        })?;

        match parse_log_entry(&line) {
            Ok((_, entry)) => process_new_entry(&mut entries, &mut current_entry, entry),
            Err(_) => process_continuation_line(&mut current_entry, &line, line_number),
        }
    }

    if let Some(entry) = current_entry {
        entries.push(entry);
    }

    for (i, entry) in entries.iter_mut().enumerate() {
        entry.sequence_id = i;
    }

    Ok(ParseResult {
        entries,
        total_lines,
    })
}

pub fn count_log_lines(path: &Path) -> Result<usize> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines().count())
}

/// Parse a single log entry line
/// Format: "2025-10-27 11:23:27.566558-07:00 [notice] <0.208.0> Message"
fn parse_log_entry(input: &str) -> IResult<&str, ParsedLogEntry> {
    let (input, timestamp) = parse_timestamp(input)?;
    let (input, _) = space1.parse(input)?;
    let (input, severity) = parse_severity(input)?;
    let (input, _) = space1.parse(input)?;
    let (input, process_id) = parse_process_id(input)?;
    let (input, _) = char(' ').parse(input)?;
    let trimmed_message = input.trim_end();
    let message = trimmed_message.to_string();
    let message_lowercased = trimmed_message.to_lowercase();

    Ok((
        "",
        ParsedLogEntry {
            sequence_id: 0,
            explicit_id: None,
            timestamp,
            severity,
            process_id,
            message,
            message_lowercased,
            subsystem_id: None,
            labels: LogEntryLabels::default(),
            resolution_or_discussion_url_id: None,
            doc_url_id: None,
        },
    ))
}

fn parse_date(input: &str) -> IResult<&str, (i32, u32, u32)> {
    let (input, year) = map_res(digit1, |s: &str| s.parse::<i32>()).parse(input)?;
    let (input, _) = char('-').parse(input)?;
    let (input, month) = map_res(digit1, |s: &str| s.parse::<u32>()).parse(input)?;
    let (input, _) = char('-').parse(input)?;
    let (input, day) = map_res(digit1, |s: &str| s.parse::<u32>()).parse(input)?;
    Ok((input, (year, month, day)))
}

fn parse_time(input: &str) -> IResult<&str, (u32, u32, u32, u32)> {
    let (input, hour) = map_res(digit1, |s: &str| s.parse::<u32>()).parse(input)?;
    let (input, _) = char(':').parse(input)?;
    let (input, minute) = map_res(digit1, |s: &str| s.parse::<u32>()).parse(input)?;
    let (input, _) = char(':').parse(input)?;
    let (input, second) = map_res(digit1, |s: &str| s.parse::<u32>()).parse(input)?;
    let (input, _) = char('.').parse(input)?;
    let (input, microseconds) = map_res(digit1, |s: &str| s.parse::<u32>()).parse(input)?;
    Ok((input, (hour, minute, second, microseconds)))
}

fn parse_timezone(input: &str) -> IResult<&str, i32> {
    let (input, tz_sign) = alt((char('+'), char('-'))).parse(input)?;
    let (input, tz_hour) = map_res(digit1, |s: &str| s.parse::<i32>()).parse(input)?;
    let (input, _) = char(':').parse(input)?;
    let (input, tz_minute) = map_res(digit1, |s: &str| s.parse::<i32>()).parse(input)?;
    let tz_offset_seconds = (tz_hour * 3600 + tz_minute * 60) * if tz_sign == '-' { -1 } else { 1 };
    Ok((input, tz_offset_seconds))
}

fn nom_verify_error() -> NomErr<NomError<&'static str>> {
    NomErr::Error(NomError::new("", ErrorKind::Verify))
}

fn build_datetime(
    date: (i32, u32, u32),
    time: (u32, u32, u32, u32),
    tz_offset_seconds: i32,
) -> Result<DateTime<Utc>, NomErr<NomError<&'static str>>> {
    let (year, month, day) = date;
    let (hour, minute, second, microseconds) = time;

    let naive_date = NaiveDate::from_ymd_opt(year, month, day).ok_or_else(nom_verify_error)?;

    let naive_time = NaiveTime::from_hms_micro_opt(hour, minute, second, microseconds)
        .ok_or_else(nom_verify_error)?;

    let naive_datetime = NaiveDateTime::new(naive_date, naive_time);

    let offset = FixedOffset::east_opt(tz_offset_seconds).ok_or_else(nom_verify_error)?;

    let dt = offset
        .from_local_datetime(&naive_datetime)
        .single()
        .ok_or_else(nom_verify_error)?;

    Ok(dt.to_utc())
}

fn parse_timestamp(input: &str) -> IResult<&str, DateTime<Utc>> {
    let (input, date) = parse_date(input)?;
    let (input, _) = space1.parse(input)?;
    let (input, time) = parse_time(input)?;
    let (input, tz_offset) = parse_timezone(input)?;
    let datetime = build_datetime(date, time, tz_offset)
        .map_err(|_| NomErr::Error(NomError::new(input, ErrorKind::Verify)))?;
    Ok((input, datetime))
}

/// Parses a severity level.
/// Example: "[notice]", "[debug]", "[info]", "[warning]", "[error]", "[critical]"
fn parse_severity(input: &str) -> IResult<&str, Severity> {
    let (input, severity_str) = delimited(
        char('['),
        alt((
            tag("debug"),
            tag("info"),
            tag("notice"),
            tag("warning"),
            tag("error"),
            tag("critical"),
        )),
        char(']'),
    )
    .parse(input)?;

    let severity = severity_str
        .parse::<Severity>()
        .map_err(|_| NomErr::Error(NomError::new(input, ErrorKind::Verify)))?;

    Ok((input, severity))
}

/// Parses an Erlang process ID.
/// Example: "<0.208.0>"
fn parse_process_id(input: &str) -> IResult<&str, String> {
    let (input, pid) = recognize(delimited(
        char('<'),
        (digit1, char('.'), digit1, char('.'), digit1),
        char('>'),
    ))
    .parse(input)?;

    Ok((input, pid.to_string()))
}
