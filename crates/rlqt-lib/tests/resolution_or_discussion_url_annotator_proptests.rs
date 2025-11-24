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

use proptest::prelude::*;
use rlqt_lib::Severity;
use rlqt_lib::entry_metadata::Annotator;
use rlqt_lib::entry_metadata::resolution_url_annotators::{
    Discussion14094Annotator, Issue14181Annotator, Issue14213Annotator, PullRequest14409Annotator,
};
use test_helpers::create_test_entry;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_issue_14181_does_not_match_arbitrary_text(s in "[a-zA-Z0-9 ,.:;!@#$%^&*()\\[\\]{}]{1,200}") {
        let entry = create_test_entry(&s, Severity::Info);
        let annotator = Issue14181Annotator;

        if !s.contains("rabbit_msg_store") || !s.contains("reader_pread_parse") || !s.contains("eof") {
            prop_assert!(!annotator.does_match(&entry));
        }
    }

    #[test]
    fn prop_issue_14181_requires_all_keywords(
        prefix in "[a-zA-Z0-9 ]{0,50}",
        suffix in "[a-zA-Z0-9 ]{0,50}"
    ) {
        let annotator = Issue14181Annotator;

        let entry1 = create_test_entry(&format!("{} rabbit_msg_store {}", prefix, suffix), Severity::Info);
        prop_assert!(!annotator.does_match(&entry1));

        let entry2 = create_test_entry(&format!("{} reader_pread_parse {}", prefix, suffix), Severity::Info);
        prop_assert!(!annotator.does_match(&entry2));

        let entry3 = create_test_entry(&format!("{} eof {}", prefix, suffix), Severity::Info);
        prop_assert!(!annotator.does_match(&entry3));
    }

    #[test]
    fn prop_issue_14213_does_not_match_arbitrary_text(s in "[a-zA-Z0-9 ,.:;!@#$%^&*()\\[\\]{}]{1,200}") {
        let entry = create_test_entry(&s, Severity::Info);
        let annotator = Issue14213Annotator;

        if !s.contains("mc_amqpl") || !s.contains("to_091") || !s.contains("as_is") {
            prop_assert!(!annotator.does_match(&entry));
        }
    }

    #[test]
    fn prop_issue_14213_requires_all_keywords(
        prefix in "[a-zA-Z0-9 ]{0,50}",
        suffix in "[a-zA-Z0-9 ]{0,50}"
    ) {
        let annotator = Issue14213Annotator;

        let entry1 = create_test_entry(&format!("{} mc_amqpl {}", prefix, suffix), Severity::Info);
        prop_assert!(!annotator.does_match(&entry1));

        let entry2 = create_test_entry(&format!("{} to_091 {}", prefix, suffix), Severity::Info);
        prop_assert!(!annotator.does_match(&entry2));

        let entry3 = create_test_entry(&format!("{} as_is {}", prefix, suffix), Severity::Info);
        prop_assert!(!annotator.does_match(&entry3));
    }


    #[test]
    fn prop_discussion_14094_does_not_match_arbitrary_text(s in "[a-zA-Z0-9 ,.:;!@#$%^&*()\\[\\]{}]{1,200}") {
        let entry = create_test_entry(&s, Severity::Info);
        let annotator = Discussion14094Annotator;

        let has_required = s.contains("badmatch") && s.contains("error") && s.contains("eacces");
        let has_module = s.contains("rabbit_classic_queue_index_v2,new_segment_file,3")
            || s.contains("rabbit_classic_queue_store_v2,'-flush_buffer/2-fun-0-',4");

        if !has_required || !has_module {
            prop_assert!(!annotator.does_match(&entry));
        }
    }

    #[test]
    fn prop_discussion_14094_requires_specific_module_pattern(
        prefix in "[a-zA-Z0-9 ]{0,50}",
        suffix in "[a-zA-Z0-9 ]{0,50}"
    ) {
        let annotator = Discussion14094Annotator;

        let entry1 = create_test_entry(&format!("{} badmatch error eacces rabbit_other_module {}", prefix, suffix), Severity::Info);
        prop_assert!(!annotator.does_match(&entry1));

        let entry2 = create_test_entry(&format!("{} badmatch error eacces rabbit_classic_queue_store_v2 {}", prefix, suffix), Severity::Info);
        prop_assert!(!annotator.does_match(&entry2));

        let entry3 = create_test_entry(&format!("{} badmatch error eacces new_segment_file {}", prefix, suffix), Severity::Info);
        prop_assert!(!annotator.does_match(&entry3));
    }

    #[test]
    fn prop_no_annotator_matches_random_alphanumeric(s in "[a-zA-Z0-9]{1,100}") {
        let entry = create_test_entry(&s, Severity::Info);

        let annotator1 = Issue14181Annotator;
        let annotator2 = Issue14213Annotator;
        let annotator3 = Discussion14094Annotator;
        let annotator4 = PullRequest14409Annotator;

        prop_assert!(!annotator1.does_match(&entry));
        prop_assert!(!annotator2.does_match(&entry));
        prop_assert!(!annotator3.does_match(&entry));
        prop_assert!(!annotator4.does_match(&entry));
    }

    #[test]
    fn prop_no_annotator_matches_random_erlang_like_text(
        module in "[a-z_]{5,20}",
        function in "[a-z_]{5,20}",
        _arity in 0..10u8
    ) {
        let message = format!("{{error, {{{},{},[]}}}}", module, function);
        let entry = create_test_entry(&message, Severity::Info);

        let annotator1 = Issue14181Annotator;
        let annotator2 = Issue14213Annotator;
        let annotator3 = Discussion14094Annotator;

        if module != "rabbit_msg_store" && function != "reader_pread_parse" {
            prop_assert!(!annotator1.does_match(&entry));
        }
        if module != "mc_amqpl" && function != "to_091" {
            prop_assert!(!annotator2.does_match(&entry));
        }
        if module != "rabbit_classic_queue_store_v2" && module != "rabbit_classic_queue_index_v2" {
            prop_assert!(!annotator3.does_match(&entry));
        }
    }
    #[test]
    fn prop_pull_request_14409_does_not_match_arbitrary_text(s in "[a-zA-Z0-9 ,.:;!@#$%^&*()\\[\\]{}]{1,200}") {
        let entry = create_test_entry(&s, Severity::Info);
        let annotator = PullRequest14409Annotator;

        if !s.contains("case_clause") || !s.contains("khepri") || !s.contains("mismatching_node") || !s.contains("if_node_exists") {
            prop_assert!(!annotator.does_match(&entry));
        }
    }

    #[test]
    fn prop_pull_request_14409_requires_all_keywords(
        prefix in "[a-zA-Z0-9 ]{0,50}",
        suffix in "[a-zA-Z0-9 ]{0,50}"
    ) {
        let annotator = PullRequest14409Annotator;

        let entry1 = create_test_entry(&format!("{} case_clause khepri mismatching_node {}", prefix, suffix), Severity::Info);
        prop_assert!(!annotator.does_match(&entry1));

        let entry2 = create_test_entry(&format!("{} case_clause khepri if_node_exists {}", prefix, suffix), Severity::Info);
        prop_assert!(!annotator.does_match(&entry2));

        let entry3 = create_test_entry(&format!("{} case_clause mismatching_node if_node_exists {}", prefix, suffix), Severity::Info);
        prop_assert!(!annotator.does_match(&entry3));

        let entry4 = create_test_entry(&format!("{} khepri mismatching_node if_node_exists {}", prefix, suffix), Severity::Info);
        prop_assert!(!annotator.does_match(&entry4));
    }
}
