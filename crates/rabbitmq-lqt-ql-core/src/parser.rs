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

//! Parser implementation for the RLQT Query Language.

use crate::ast::{
    Duration, DurationUnit, Field, FilterExpr, LabelMatcher, MatchOp, PipelineStage, Query,
    Selector, SortDirection, SortSpec, Value,
};
use crate::errors::ParseError;
use crate::presets::PresetName;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{escaped_transform, tag, tag_no_case, take_until, take_while1},
    character::complete::{char, digit1, multispace0, multispace1, none_of},
    combinator::{all_consuming, map, map_res, opt, value},
    error::{Error as NomError, ErrorKind},
    multi::{many0, separated_list0, separated_list1},
    sequence::{delimited, pair, preceded},
};
use std::str::FromStr;

pub fn parse(input: &str) -> Result<Query, ParseError> {
    let input = input.trim();
    if input.is_empty() {
        return Err(ParseError::EmptyQuery);
    }

    match all_consuming(parse_query).parse(input) {
        Ok((_, query)) => Ok(query),
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
            let position = input.len().saturating_sub(e.input.len());
            let remaining = e.input;

            if let Some(preset_err) = detect_preset_error(input, remaining) {
                return Err(preset_err);
            }

            Err(ParseError::ParseFailed {
                message: format!("{:?}", e),
                position,
            })
        }
        Err(nom::Err::Incomplete(_)) => Err(ParseError::UnexpectedEof),
    }
}

fn detect_preset_error(original: &str, remaining: &str) -> Option<ParseError> {
    let pos = original.len().saturating_sub(remaining.len());
    if pos > 0 {
        let before = &original[..pos];
        if let Some(colon_pos) = before.rfind(':') {
            let preset_start = colon_pos + 1;
            let preset_name: String = original[preset_start..]
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .collect();
            if !preset_name.is_empty() && PresetName::from_str(&preset_name).is_err() {
                return Some(ParseError::unknown_preset(&preset_name));
            }
        }
    }
    None
}

fn parse_query(input: &str) -> IResult<&str, Query> {
    let (input, _) = multispace0.parse(input)?;
    let (input, time_range) = opt(parse_time_range).parse(input)?;
    let (input, _) = multispace0.parse(input)?;

    let (input, has_wildcard) = opt(char('*')).parse(input)?;
    let (input, _) = multispace0.parse(input)?;

    let (input, selector, filter) = if has_wildcard.is_some() {
        (input, None, None)
    } else {
        let (input, selector) = opt(parse_selector).parse(input)?;
        let (input, _) = multispace0.parse(input)?;
        let (input, filter) = opt(parse_filter_expr).parse(input)?;
        (input, selector, filter)
    };

    let (input, _) = multispace0.parse(input)?;
    let (input, pipeline) = many0(preceded(
        (multispace0, char('|'), multispace0),
        parse_pipeline_stage,
    ))
    .parse(input)?;
    let (input, _) = multispace0.parse(input)?;

    Ok((
        input,
        Query {
            time_range,
            selector,
            filter,
            pipeline,
        },
    ))
}

fn parse_time_range(input: &str) -> IResult<&str, Duration> {
    preceded(char('@'), parse_duration).parse(input)
}

fn parse_duration(input: &str) -> IResult<&str, Duration> {
    let (input, value) = map_res(digit1, |s: &str| s.parse::<i64>()).parse(input)?;
    let (input, unit) = parse_duration_unit(input)?;
    Ok((input, Duration { value, unit }))
}

fn parse_duration_unit(input: &str) -> IResult<&str, DurationUnit> {
    alt((
        value(DurationUnit::Seconds, tag("s")),
        value(DurationUnit::Minutes, tag("m")),
        value(DurationUnit::Hours, tag("h")),
        value(DurationUnit::Days, tag("d")),
        value(DurationUnit::Weeks, tag("w")),
    ))
    .parse(input)
}

fn parse_selector(input: &str) -> IResult<&str, Selector> {
    let (input, matchers) = delimited(
        pair(char('{'), multispace0),
        separated_list1((multispace0, char(','), multispace0), parse_label_matcher),
        pair(multispace0, char('}')),
    )
    .parse(input)?;

    Ok((input, Selector { matchers }))
}

fn parse_label_matcher(input: &str) -> IResult<&str, LabelMatcher> {
    let (input, field) = parse_field(input)?;
    let (input, _) = multispace0.parse(input)?;
    let (input, op) = parse_selector_op(input)?;
    let (input, _) = multispace0.parse(input)?;
    let (input, value) = parse_value(input)?;

    Ok((input, LabelMatcher { field, op, value }))
}

fn parse_field(input: &str) -> IResult<&str, Field> {
    let (input, field_name) =
        take_while1(|c: char| c.is_alphanumeric() || c == '_').parse(input)?;

    let field = match field_name.to_lowercase().as_str() {
        "severity" | "level" => Field::Severity,
        "subsystem" => Field::Subsystem,
        "node" => Field::Node,
        "erlang_pid" | "pid" => Field::ErlangPid,
        "message" | "msg" => Field::Message,
        "labels" | "label" => Field::Labels,
        "timestamp" | "time" | "ts" => Field::Timestamp,
        "id" => Field::Id,
        _ => {
            return Err(nom::Err::Error(NomError::new(input, ErrorKind::Tag)));
        }
    };

    Ok((input, field))
}

fn parse_selector_op(input: &str) -> IResult<&str, MatchOp> {
    alt((
        value(MatchOp::NotRegex, tag("!~")),
        value(MatchOp::NotEq, tag("!=")),
        value(MatchOp::NotEq, tag("<>")),
        value(MatchOp::Regex, tag("=~")),
        value(MatchOp::Eq, tag("==")),
        value(MatchOp::Eq, tag("=")),
        value(MatchOp::LtEq, tag("<=")),
        value(MatchOp::GtEq, tag(">=")),
        value(MatchOp::Lt, tag("<")),
        value(MatchOp::Gt, tag(">")),
        value(MatchOp::HasLabel, tag("~=")),
    ))
    .parse(input)
}

fn parse_match_op(input: &str) -> IResult<&str, MatchOp> {
    alt((
        parse_selector_op,
        value(MatchOp::Contains, tag_no_case("contains")),
        value(MatchOp::IContains, tag_no_case("icontains")),
    ))
    .parse(input)
}

fn parse_value(input: &str) -> IResult<&str, Value> {
    alt((
        parse_null_value,
        parse_boolean_value,
        parse_label_list,
        parse_relative_time_value,
        parse_regex_value,
        parse_string_value,
        parse_integer_value,
    ))
    .parse(input)
}

fn parse_relative_time_value(input: &str) -> IResult<&str, Value> {
    let (input, _) = char('@').parse(input)?;
    let (input, duration) = parse_duration(input)?;
    Ok((input, Value::RelativeTime(duration)))
}

fn parse_null_value(input: &str) -> IResult<&str, Value> {
    value(Value::Null, alt((tag("null"), tag("none")))).parse(input)
}

fn parse_boolean_value(input: &str) -> IResult<&str, Value> {
    alt((
        value(Value::Boolean(true), tag("true")),
        value(Value::Boolean(false), tag("false")),
    ))
    .parse(input)
}

fn parse_string_value(input: &str) -> IResult<&str, Value> {
    let (input, s) = parse_quoted_string(input)?;
    Ok((input, Value::String(s)))
}

fn parse_quoted_string(input: &str) -> IResult<&str, String> {
    alt((parse_double_quoted_string, parse_single_quoted_string)).parse(input)
}

fn parse_double_quoted_string(input: &str) -> IResult<&str, String> {
    delimited(
        char('"'),
        map(
            opt(escaped_transform(
                none_of("\\\""),
                '\\',
                alt((
                    value('\\', char('\\')),
                    value('"', char('"')),
                    value('\n', char('n')),
                    value('\r', char('r')),
                    value('\t', char('t')),
                )),
            )),
            |opt_s| opt_s.unwrap_or_default(),
        ),
        char('"'),
    )
    .parse(input)
}

fn parse_single_quoted_string(input: &str) -> IResult<&str, String> {
    delimited(
        char('\''),
        map(
            opt(escaped_transform(
                none_of("\\'"),
                '\\',
                alt((
                    value('\\', char('\\')),
                    value('\'', char('\'')),
                    value('\n', char('n')),
                    value('\r', char('r')),
                    value('\t', char('t')),
                )),
            )),
            |opt_s| opt_s.unwrap_or_default(),
        ),
        char('\''),
    )
    .parse(input)
}

fn parse_regex_value(input: &str) -> IResult<&str, Value> {
    let (input, _) = char('/').parse(input)?;
    let (input, pattern) = take_until("/").parse(input)?;
    let (input, _) = char('/').parse(input)?;
    Ok((input, Value::Regex(pattern.to_string())))
}

fn parse_integer_value(input: &str) -> IResult<&str, Value> {
    let (input, neg) = opt(char('-')).parse(input)?;
    let (input, digits) = digit1.parse(input)?;

    let value: i64 = digits
        .parse()
        .map_err(|_| nom::Err::Error(NomError::new(input, ErrorKind::Digit)))?;

    let value = if neg.is_some() { -value } else { value };
    Ok((input, Value::Integer(value)))
}

fn parse_label_list(input: &str) -> IResult<&str, Value> {
    let (input, labels) = delimited(
        pair(char('['), multispace0),
        separated_list0((multispace0, char(','), multispace0), parse_quoted_string),
        pair(multispace0, char(']')),
    )
    .parse(input)?;

    Ok((input, Value::LabelList(labels)))
}

pub(crate) fn parse_filter_expr(input: &str) -> IResult<&str, FilterExpr> {
    parse_or_expr(input)
}

fn parse_or_expr(input: &str) -> IResult<&str, FilterExpr> {
    let (input, first) = parse_and_expr(input)?;
    let (input, rest) = many0(preceded(
        (
            multispace0,
            alt((tag_no_case("or"), tag("||"))),
            multispace0,
        ),
        parse_and_expr,
    ))
    .parse(input)?;

    let expr = rest
        .into_iter()
        .fold(first, |acc, e| FilterExpr::Or(Box::new(acc), Box::new(e)));

    Ok((input, expr))
}

fn parse_and_expr(input: &str) -> IResult<&str, FilterExpr> {
    let (input, first) = parse_unary_expr(input)?;
    let (input, rest) = many0(preceded(
        (
            multispace0,
            alt((tag_no_case("and"), tag("&&"))),
            multispace0,
        ),
        parse_unary_expr,
    ))
    .parse(input)?;

    let expr = rest
        .into_iter()
        .fold(first, |acc, e| FilterExpr::And(Box::new(acc), Box::new(e)));

    Ok((input, expr))
}

fn parse_unary_expr(input: &str) -> IResult<&str, FilterExpr> {
    alt((
        map(
            preceded(
                pair(alt((tag_no_case("not"), tag("!"))), multispace0),
                parse_unary_expr,
            ),
            |e| FilterExpr::Not(Box::new(e)),
        ),
        parse_primary_expr,
    ))
    .parse(input)
}

fn parse_primary_expr(input: &str) -> IResult<&str, FilterExpr> {
    alt((
        parse_grouped_expr,
        parse_preset_expr,
        parse_special_filter,
        parse_hashtag_label,
        parse_label_filter,
        parse_subsystem_filter,
        parse_comparison_expr,
    ))
    .parse(input)
}

fn parse_hashtag_label(input: &str) -> IResult<&str, FilterExpr> {
    let (input, negated) = opt(char('-')).parse(input)?;
    let (input, _) = take_while1(|c: char| c == '#').parse(input)?;
    let (input, label) =
        take_while1(|c: char| c.is_alphanumeric() || c == '_' || c == ':').parse(input)?;

    let expr = FilterExpr::LabelAny(vec![label.to_string()]);
    if negated.is_some() {
        Ok((input, FilterExpr::Not(Box::new(expr))))
    } else {
        Ok((input, expr))
    }
}

fn parse_grouped_expr(input: &str) -> IResult<&str, FilterExpr> {
    let (input, expr) = delimited(
        pair(char('('), multispace0),
        parse_filter_expr,
        pair(multispace0, char(')')),
    )
    .parse(input)?;

    Ok((input, FilterExpr::Grouped(Box::new(expr))))
}

fn parse_preset_expr(input: &str) -> IResult<&str, FilterExpr> {
    let (input, _) = char(':').parse(input)?;
    let (input, name) = take_while1(|c: char| c.is_alphanumeric() || c == '_').parse(input)?;

    let preset = PresetName::from_str(name)
        .map_err(|_| nom::Err::Failure(NomError::new(input, ErrorKind::Tag)))?;

    Ok((input, FilterExpr::Preset(preset)))
}

fn parse_special_filter(input: &str) -> IResult<&str, FilterExpr> {
    alt((
        value(FilterExpr::HasDocUrl, tag_no_case("has_doc_url")),
        value(
            FilterExpr::HasResolutionUrl,
            tag_no_case("has_resolution_url"),
        ),
        value(FilterExpr::Unlabelled, tag_no_case("unlabelled")),
        value(FilterExpr::Unlabelled, tag_no_case("unlabeled")),
    ))
    .parse(input)
}

fn parse_label_filter(input: &str) -> IResult<&str, FilterExpr> {
    let (input, _) = tag_no_case("labels").parse(input)?;
    let (input, _) = multispace0.parse(input)?;

    alt((
        map(
            preceded(
                pair(tag_no_case("any"), multispace0),
                parse_label_list_value,
            ),
            FilterExpr::LabelAny,
        ),
        map(
            preceded(
                pair(tag_no_case("all"), multispace0),
                parse_label_list_value,
            ),
            FilterExpr::LabelAll,
        ),
        map(
            preceded(pair(tag("~="), multispace0), parse_quoted_string),
            |label| {
                FilterExpr::Comparison(Box::new(LabelMatcher {
                    field: Field::Labels,
                    op: MatchOp::HasLabel,
                    value: Value::String(label),
                }))
            },
        ),
        map(preceded(pair(tag("=="), multispace0), tag("none")), |_| {
            FilterExpr::Unlabelled
        }),
    ))
    .parse(input)
}

fn parse_label_list_value(input: &str) -> IResult<&str, Vec<String>> {
    delimited(
        pair(char('['), multispace0),
        separated_list1((multispace0, char(','), multispace0), parse_quoted_string),
        pair(multispace0, char(']')),
    )
    .parse(input)
}

fn parse_subsystem_filter(input: &str) -> IResult<&str, FilterExpr> {
    let (input, _) = tag_no_case("subsystem").parse(input)?;
    let (input, _) = multispace0.parse(input)?;

    let (input, _) = tag_no_case("any").parse(input)?;
    let (input, _) = multispace0.parse(input)?;
    let (input, subsystems) = parse_label_list_value(input)?;

    Ok((input, FilterExpr::SubsystemAny(subsystems)))
}

fn parse_comparison_expr(input: &str) -> IResult<&str, FilterExpr> {
    let (input, field) = parse_field(input)?;
    let (input, _) = multispace0.parse(input)?;
    let (input, op) = parse_match_op(input)?;
    let (input, _) = multispace0.parse(input)?;
    let (input, value) = parse_value(input)?;

    Ok((
        input,
        FilterExpr::Comparison(Box::new(LabelMatcher { field, op, value })),
    ))
}

fn parse_pipeline_stage(input: &str) -> IResult<&str, PipelineStage> {
    alt((
        parse_where_stage,
        parse_limit_stage,
        parse_offset_stage,
        parse_head_stage,
        parse_tail_stage,
        parse_sort_stage,
        parse_count_stage,
        parse_distinct_stage,
        parse_project_stage,
    ))
    .parse(input)
}

fn parse_where_stage(input: &str) -> IResult<&str, PipelineStage> {
    let (input, _) = tag_no_case("where").parse(input)?;
    let (input, _) = multispace1.parse(input)?;
    let (input, expr) = parse_filter_expr(input)?;
    Ok((input, PipelineStage::Where(expr)))
}

fn parse_limit_stage(input: &str) -> IResult<&str, PipelineStage> {
    let (input, _) = tag_no_case("limit").parse(input)?;
    let (input, _) = multispace1.parse(input)?;
    let (input, n) = map_res(digit1, |s: &str| s.parse::<u64>()).parse(input)?;
    Ok((input, PipelineStage::Limit(n)))
}

fn parse_offset_stage(input: &str) -> IResult<&str, PipelineStage> {
    let (input, _) = alt((tag_no_case("offset"), tag_no_case("skip"))).parse(input)?;
    let (input, _) = multispace1.parse(input)?;
    let (input, n) = map_res(digit1, |s: &str| s.parse::<u64>()).parse(input)?;
    Ok((input, PipelineStage::Offset(n)))
}

fn parse_head_stage(input: &str) -> IResult<&str, PipelineStage> {
    let (input, _) = alt((tag_no_case("head"), tag_no_case("first"))).parse(input)?;
    let (input, _) = multispace1.parse(input)?;
    let (input, n) = map_res(digit1, |s: &str| s.parse::<u64>()).parse(input)?;
    Ok((input, PipelineStage::Head(n)))
}

fn parse_tail_stage(input: &str) -> IResult<&str, PipelineStage> {
    let (input, _) = alt((tag_no_case("tail"), tag_no_case("last"))).parse(input)?;
    let (input, _) = multispace1.parse(input)?;
    let (input, n) = map_res(digit1, |s: &str| s.parse::<u64>()).parse(input)?;
    Ok((input, PipelineStage::Tail(n)))
}

fn parse_sort_stage(input: &str) -> IResult<&str, PipelineStage> {
    let (input, _) = alt((
        value((), tag_no_case("sort")),
        value((), (tag_no_case("order"), multispace1, tag_no_case("by"))),
    ))
    .parse(input)?;
    let (input, _) = multispace1.parse(input)?;
    let (input, field) = parse_field(input)?;
    let (input, _) = multispace0.parse(input)?;
    let (input, direction) = opt(parse_sort_direction).parse(input)?;

    Ok((
        input,
        PipelineStage::Sort(SortSpec {
            field,
            direction: direction.unwrap_or_default(),
        }),
    ))
}

fn parse_sort_direction(input: &str) -> IResult<&str, SortDirection> {
    alt((
        value(SortDirection::Asc, tag_no_case("asc")),
        value(SortDirection::Desc, tag_no_case("desc")),
    ))
    .parse(input)
}

fn parse_count_stage(input: &str) -> IResult<&str, PipelineStage> {
    let (input, _) = tag_no_case("count").parse(input)?;
    let (input, by_field) = opt(preceded(
        (multispace1, tag_no_case("by"), multispace1),
        parse_field,
    ))
    .parse(input)?;
    Ok((input, PipelineStage::CountBy(by_field)))
}

fn parse_distinct_stage(input: &str) -> IResult<&str, PipelineStage> {
    let (input, _) = tag_no_case("distinct").parse(input)?;
    let (input, _) = multispace1.parse(input)?;
    let (input, fields) =
        separated_list1((multispace0, char(','), multispace0), parse_field).parse(input)?;
    Ok((input, PipelineStage::Distinct(fields)))
}

fn parse_project_stage(input: &str) -> IResult<&str, PipelineStage> {
    let (input, _) = alt((tag_no_case("project"), tag_no_case("select"))).parse(input)?;
    let (input, _) = multispace1.parse(input)?;
    let (input, fields) =
        separated_list1((multispace0, char(','), multispace0), parse_field).parse(input)?;
    Ok((input, PipelineStage::Project(fields)))
}

pub fn parse_filter_only(input: &str) -> Result<FilterExpr, ParseError> {
    let input = input.trim();
    if input.is_empty() {
        return Err(ParseError::EmptyQuery);
    }

    match all_consuming(parse_filter_expr).parse(input) {
        Ok((_, expr)) => Ok(expr),
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
            let position = input.len().saturating_sub(e.input.len());
            Err(ParseError::ParseFailed {
                message: format!("{:?}", e),
                position,
            })
        }
        Err(nom::Err::Incomplete(_)) => Err(ParseError::UnexpectedEof),
    }
}
