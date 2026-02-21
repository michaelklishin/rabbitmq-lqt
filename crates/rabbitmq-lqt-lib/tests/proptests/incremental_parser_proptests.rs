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

use proptest::prelude::*;
use rabbitmq_lqt_lib::parser::IncrementalParser;

proptest! {
    #[test]
    fn feed_line_never_panics_on_arbitrary_strings(line in ".*") {
        let mut parser = IncrementalParser::new(0);
        let _ = parser.feed_line(&line);
        let _ = parser.flush();
    }

    #[test]
    fn single_valid_entry_always_produces_one_entry_after_flush(
        msg in "[a-zA-Z ]{1,50}"
    ) {
        let line = format!(
            "2025-10-27 11:23:27.566558-07:00 [notice] <0.208.0> {}",
            msg
        );
        let mut parser = IncrementalParser::new(0);
        let immediate = parser.feed_line(&line);
        let flushed = parser.flush();

        let total = usize::from(immediate.is_some()) + usize::from(flushed.is_some());
        prop_assert_eq!(total, 1);
    }
}
