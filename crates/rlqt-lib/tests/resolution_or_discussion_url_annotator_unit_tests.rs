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

mod test_helpers;

use rlqt_lib::Severity;
use rlqt_lib::constants::{DISCUSSION_14094, ISSUE_14181, ISSUE_14213, PULL_REQUEST_14409};
use rlqt_lib::entry_metadata::Annotator;
use rlqt_lib::entry_metadata::resolution_url_annotators::{
    Discussion14094Annotator, Issue14181Annotator, Issue14213Annotator, PullRequest14409Annotator,
    ResolutionOrDiscussionUrlAnnotator, annotate_resolution_or_discussion_urls,
};
use test_helpers::create_test_entry;

#[test]
fn test_issue_14181_annotator_matches() {
    let message = r#"{function_clause,
  [{rabbit_msg_store,reader_pread_parse,
    [[eof,eof,eof,eof,eof,eof]],
    [{file,"src/rabbit_msg_store.erl"},{line,687}]},"#;
    let entry = create_test_entry(message, Severity::Info);
    let annotator = Issue14181Annotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_issue_14181_annotator_matches_variant() {
    let message = r#"{rabbit_msg_store,reader_pread_parse,
    [[eof]], [{file,"src/rabbit_msg_store.erl"},{line,686}]}"#;
    let entry = create_test_entry(message, Severity::Info);
    let annotator = Issue14181Annotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_issue_14181_annotator_no_match() {
    let entry = create_test_entry("Unrelated error message", Severity::Info);
    let annotator = Issue14181Annotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_issue_14181_annotator_annotates() {
    let annotator = Issue14181Annotator;
    let mut entry = create_test_entry("rabbit_msg_store reader_pread_parse eof", Severity::Info);
    annotator.annotate(&mut entry);
    assert_eq!(entry.resolution_or_discussion_url_id, Some(ISSUE_14181));
}

#[test]
fn test_issue_14213_annotator_matches() {
    let message = r#"exception exit: {function_clause,
    [{mc_amqpl,to_091,
         [<<"decimal-32">>,{as_is,116,<<124,0,0,0>>}],
         [{file,"mc_amqpl.erl"},{line,747}]},"#;
    let entry = create_test_entry(message, Severity::Info);
    let annotator = Issue14213Annotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_issue_14213_annotator_matches_variant() {
    let message = r#"{mc_amqpl,to_091,[<<"float">>,{as_is,114,<<0,0,200,127>>}]}"#;
    let entry = create_test_entry(message, Severity::Info);
    let annotator = Issue14213Annotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_issue_14213_annotator_no_match_without_as_is() {
    let message = r#"{mc_amqpl,to_091,[<<"decimal-32">>,123]}"#;
    let entry = create_test_entry(message, Severity::Info);
    let annotator = Issue14213Annotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_issue_14213_annotator_no_match() {
    let entry = create_test_entry("Unrelated error message", Severity::Info);
    let annotator = Issue14213Annotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_issue_14213_annotator_annotates() {
    let annotator = Issue14213Annotator;
    let mut entry = create_test_entry("mc_amqpl to_091 as_is error", Severity::Info);
    annotator.annotate(&mut entry);
    assert_eq!(entry.resolution_or_discussion_url_id, Some(ISSUE_14213));
}

#[test]
fn test_discussion_14094_annotator_matches_store_v2() {
    let message = r#"{{badmatch,{error,eacces}},
[{rabbit_classic_queue_store_v2,'-flush_buffer/2-fun-0-',4,
[{file,"rabbit_classic_queue_store_v2.erl"},{line,207}]},"#;
    let entry = create_test_entry(message, Severity::Info);
    let annotator = Discussion14094Annotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_discussion_14094_annotator_matches_index_v2() {
    let message = r#"{{badmatch,{error,eacces}},
[{rabbit_classic_queue_index_v2,new_segment_file,3,
[{file,"rabbit_classic_queue_index_v2.erl"},{line,456}]},"#;
    let entry = create_test_entry(message, Severity::Info);
    let annotator = Discussion14094Annotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_discussion_14094_annotator_no_match_without_badmatch() {
    let message = r#"{error,eacces} rabbit_classic_queue_store_v2"#;
    let entry = create_test_entry(message, Severity::Info);
    let annotator = Discussion14094Annotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_discussion_14094_annotator_no_match_wrong_module() {
    let message = r#"{{badmatch,{error,eacces}},
[{rabbit_other_module,some_function,3},"#;
    let entry = create_test_entry(message, Severity::Info);
    let annotator = Discussion14094Annotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_discussion_14094_annotator_no_match() {
    let entry = create_test_entry("Unrelated error message", Severity::Info);
    let annotator = Discussion14094Annotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_discussion_14094_annotator_annotates() {
    let annotator = Discussion14094Annotator;
    let mut entry = create_test_entry(
        "badmatch error eacces rabbit_classic_queue_store_v2,'-flush_buffer/2-fun-0-',4",
        Severity::Info,
    );
    annotator.annotate(&mut entry);
    assert_eq!(
        entry.resolution_or_discussion_url_id,
        Some(DISCUSSION_14094)
    );
}

#[test]
fn test_pull_request_14409_annotator_matches() {
    let message = r#"{case_clause,{error,{khepri,mismatching_node,
                            #{node_name => <<"exchange_name">>,
                              node_props => #{payload_version => 1},
                              node_path =>
                                  [rabbitmq,vhosts,<<"/">>,exchanges,
                                   <<"exchange_name">>],
                              condition => {if_node_exists,false},
                              node_is_target => true}}}}"#;
    let entry = create_test_entry(message, Severity::Info);
    let annotator = PullRequest14409Annotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_pull_request_14409_annotator_matches_variant() {
    let message = "case_clause error khepri mismatching_node if_node_exists false";
    let entry = create_test_entry(message, Severity::Info);
    let annotator = PullRequest14409Annotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_pull_request_14409_annotator_no_match_without_if_node_exists() {
    let message = "case_clause error khepri mismatching_node";
    let entry = create_test_entry(message, Severity::Info);
    let annotator = PullRequest14409Annotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_pull_request_14409_annotator_no_match() {
    let entry = create_test_entry("Unrelated error message", Severity::Info);
    let annotator = PullRequest14409Annotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_pull_request_14409_annotator_annotates() {
    let annotator = PullRequest14409Annotator;
    let mut entry = create_test_entry(
        "case_clause khepri mismatching_node if_node_exists condition error",
        Severity::Info,
    );
    annotator.annotate(&mut entry);
    assert_eq!(
        entry.resolution_or_discussion_url_id,
        Some(PULL_REQUEST_14409)
    );
}

#[test]
fn test_annotators_handle_empty_messages() {
    let entry = create_test_entry("", Severity::Info);
    assert!(!Issue14181Annotator.does_match(&entry));
    assert!(!Issue14213Annotator.does_match(&entry));
    assert!(!Discussion14094Annotator.does_match(&entry));
    assert!(!PullRequest14409Annotator.does_match(&entry));
}

#[test]
fn test_annotators_handle_whitespace_only() {
    let entry = create_test_entry("   \t\n   ", Severity::Info);
    assert!(!Issue14181Annotator.does_match(&entry));
    assert!(!Issue14213Annotator.does_match(&entry));
    assert!(!Discussion14094Annotator.does_match(&entry));
    assert!(!PullRequest14409Annotator.does_match(&entry));
}

#[test]
fn test_annotate_function_stops_at_first_match() {
    let mut entry = create_test_entry("rabbit_msg_store reader_pread_parse eof", Severity::Info);
    entry.resolution_or_discussion_url_id = None;

    annotate_resolution_or_discussion_urls(&mut entry);

    assert_eq!(entry.resolution_or_discussion_url_id, Some(ISSUE_14181));
}

#[test]
fn test_annotate_function_preserves_existing_annotation() {
    let mut entry = create_test_entry("rabbit_msg_store reader_pread_parse eof", Severity::Info);
    entry.resolution_or_discussion_url_id = Some(999);

    annotate_resolution_or_discussion_urls(&mut entry);

    assert_eq!(entry.resolution_or_discussion_url_id, Some(999));
}

#[test]
fn test_annotate_function_no_match() {
    let mut entry = create_test_entry("completely unrelated log message", Severity::Info);
    entry.resolution_or_discussion_url_id = None;

    annotate_resolution_or_discussion_urls(&mut entry);

    assert_eq!(entry.resolution_or_discussion_url_id, None);
}
