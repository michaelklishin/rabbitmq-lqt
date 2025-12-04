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
use rlqt_lib::entry_metadata::label_annotators::annotate_labels;
use rlqt_lib::entry_metadata::labels::LogEntryLabels;
use test_helpers::create_test_entry;

#[test]
fn test_consumer_delivery_timeout() {
    let entry = create_test_entry(
        "Consumer 'consumer-tag-998754663370' on channel 1 and queue 'qq.1' in vhost '/' has timed out \
         waiting for a consumer acknowledgement of a delivery with delivery tag = 10. Timeout used: 180000 ms. \
         This timeout value can be configured, see consumers doc guide to learn more",
        Severity::Warning,
    );
    let labels = annotate_labels(&entry);
    assert!(labels.contains(LogEntryLabels::CONSUMERS));
    assert!(labels.contains(LogEntryLabels::TIMEOUTS));
}
