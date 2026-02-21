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

use crate::test_helpers::create_test_entry;
use rabbitmq_lqt_lib::Severity;
use rabbitmq_lqt_lib::entry_metadata::subsystem_annotators::annotate_subsystems;
use rabbitmq_lqt_lib::entry_metadata::subsystems::Subsystem;

#[test]
fn test_management_subsystem_plugin() {
    let mut entry = create_test_entry("Management plugin: starting HTTP listener", Severity::Info);
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Management.to_id()));
}

#[test]
fn test_management_subsystem_http_api() {
    let mut entry = create_test_entry("HTTP API: restarting listener", Severity::Info);
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Management.to_id()));
}

#[test]
fn test_management_subsystem_statistics_db() {
    let mut entry = create_test_entry("Statistics database started", Severity::Info);
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Management.to_id()));
}
