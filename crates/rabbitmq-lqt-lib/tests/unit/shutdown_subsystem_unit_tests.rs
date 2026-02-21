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
fn test_rabbitmq_is_asked_to_stop() {
    let mut entry = create_test_entry("RabbitMQ is asked to stop...", Severity::Info);
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Shutdown.to_id()));
}

#[test]
fn test_successfully_stopped_rabbitmq() {
    let mut entry = create_test_entry(
        "Successfully stopped RabbitMQ and its dependencies",
        Severity::Info,
    );
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Shutdown.to_id()));
}

#[test]
fn test_stopping_ranch_listeners_for_protocol() {
    let mut entry = create_test_entry(
        "Stopping Ranch listeners for protocol amqp",
        Severity::Debug,
    );
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Shutdown.to_id()));
}

#[test]
fn test_stopping_ra_systems() {
    let mut entry = create_test_entry("Stopping Ra systems", Severity::Info);
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Shutdown.to_id()));
}

#[test]
fn test_no_match_unrelated() {
    let mut entry = create_test_entry("Connection established", Severity::Info);
    annotate_subsystems(&mut entry);
    assert_eq!(entry.subsystem_id, None);
}
