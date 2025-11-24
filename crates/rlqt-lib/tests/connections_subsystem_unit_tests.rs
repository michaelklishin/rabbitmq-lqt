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
use rlqt_lib::entry_metadata::Annotator;
use rlqt_lib::entry_metadata::subsystem_annotators::{ConnectionsAnnotator, SubsystemAnnotator};
use rlqt_lib::entry_metadata::subsystems::Subsystem;
use test_helpers::create_test_entry;

#[test]
fn test_connection_pattern_ipv4_1() {
    let entry = create_test_entry(
        "connection 127.0.0.1:63890 -> 127.0.0.1:5672",
        Severity::Info,
    );
    let annotator = ConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_connection_pattern_ipv4_2() {
    let entry = create_test_entry(
        "connection 127.0.0.1:63886 -> 127.0.0.1:5672",
        Severity::Info,
    );
    let annotator = ConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_connection_pattern_ipv4_3() {
    let entry = create_test_entry(
        "connection 127.0.0.1:63885 -> 127.0.0.1:5672",
        Severity::Info,
    );
    let annotator = ConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_connection_pattern_case_insensitive() {
    let entry = create_test_entry(
        "Connection 192.168.1.100:52345 -> 10.0.0.1:5672",
        Severity::Info,
    );
    let annotator = ConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_connection_pattern_with_extra_spaces() {
    let entry = create_test_entry(
        "connection  10.0.0.1:51080  ->  10.0.0.2:5672",
        Severity::Info,
    );
    let annotator = ConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_client_unexpectedly_closed_tcp_connection() {
    let entry = create_test_entry("client unexpectedly closed TCP connection", Severity::Info);
    let annotator = ConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_client_unexpectedly_closed_tcp_connection_case_insensitive() {
    let entry = create_test_entry("Client Unexpectedly Closed TCP Connection", Severity::Info);
    let annotator = ConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_accepting_amqp_connection() {
    let entry = create_test_entry("accepting AMQP connection", Severity::Info);
    let annotator = ConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_accepting_amqp_connection_case_insensitive() {
    let entry = create_test_entry("Accepting AMQP Connection from 127.0.0.1", Severity::Info);
    let annotator = ConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_accepting_stomp_connection() {
    let entry = create_test_entry("accepting STOMP connection", Severity::Info);
    let annotator = ConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_mqtt_accepting() {
    let entry = create_test_entry("MQTT accepting new connection", Severity::Info);
    let annotator = ConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_mqtt_accepting_case_insensitive() {
    let entry = create_test_entry("Mqtt Accepting connection", Severity::Info);
    let annotator = ConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_accepting_web_mqtt_connection_uppercase() {
    let entry = create_test_entry("Accepting Web MQTT connection", Severity::Info);
    let annotator = ConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_accepting_web_mqtt_connection_lowercase() {
    let entry = create_test_entry("accepting Web MQTT connection", Severity::Info);
    let annotator = ConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_accepting_web_mqtt_connection_mixed_case() {
    let entry = create_test_entry("Accepting web mqtt CONNECTION", Severity::Info);
    let annotator = ConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_closing_amqp_connection() {
    let entry = create_test_entry("closing AMQP connection", Severity::Info);
    let annotator = ConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_closing_connection_uppercase() {
    let entry = create_test_entry("Closing connection", Severity::Info);
    let annotator = ConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_closing_connection_lowercase() {
    let entry = create_test_entry("closing connection", Severity::Info);
    let annotator = ConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_closing_all_connections_uppercase() {
    let entry = create_test_entry("Closing all connections", Severity::Info);
    let annotator = ConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_closing_all_connections_lowercase() {
    let entry = create_test_entry("closing all connections", Severity::Info);
    let annotator = ConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_connection_pattern_in_longer_message() {
    let entry = create_test_entry(
        "Started processing connection 10.20.30.40:54321 -> 192.168.1.1:5672 successfully",
        Severity::Info,
    );
    let annotator = ConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_no_match_missing_arrow() {
    let entry = create_test_entry("connection 127.0.0.1:63890 127.0.0.1:5672", Severity::Info);
    let annotator = ConnectionsAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_match_invalid_ip_format() {
    let entry = create_test_entry(
        "connection 999.999.999.999:52345 -> 127.0.0.1:5672",
        Severity::Info,
    );
    let annotator = ConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_no_match_missing_port() {
    let entry = create_test_entry("connection 127.0.0.1 -> 127.0.0.1:5672", Severity::Info);
    let annotator = ConnectionsAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_no_match_unrelated_message() {
    let entry = create_test_entry("Some unrelated log message", Severity::Info);
    let annotator = ConnectionsAnnotator;
    assert!(!annotator.does_match(&entry));
}

#[test]
fn test_annotate_sets_subsystem() {
    let mut entry = create_test_entry(
        "connection 127.0.0.1:63890 -> 127.0.0.1:5672",
        Severity::Info,
    );
    let annotator = ConnectionsAnnotator;
    annotator.annotate(&mut entry);
    assert_eq!(entry.subsystem_id, Some(Subsystem::Connections.to_id()));
}

#[test]
fn test_prefix_with_additional_text() {
    let entry = create_test_entry(
        "accepting AMQP connection <0.123.0> (127.0.0.1:52345 -> 127.0.0.1:5672)",
        Severity::Info,
    );
    let annotator = ConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_connection_pattern_ipv6_full() {
    let entry = create_test_entry(
        "connection [2001:0db8:85a3:0000:0000:8a2e:0370:7334]:54321 -> [::1]:5672",
        Severity::Info,
    );
    let annotator = ConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_connection_pattern_ipv6_short() {
    let entry = create_test_entry("connection [::1]:63890 -> [::1]:5672", Severity::Info);
    let annotator = ConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_connection_pattern_ipv6_localhost() {
    let entry = create_test_entry(
        "connection [::ffff:127.0.0.1]:52345 -> [::1]:5672",
        Severity::Info,
    );
    let annotator = ConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_connection_pattern_ipv6_to_ipv4() {
    let entry = create_test_entry(
        "connection [2001:db8::1]:49152 -> 192.168.1.1:5672",
        Severity::Info,
    );
    let annotator = ConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_connection_pattern_ipv4_to_ipv6() {
    let entry = create_test_entry(
        "connection 10.0.0.1:54321 -> [fe80::1]:5672",
        Severity::Info,
    );
    let annotator = ConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_connection_pattern_ipv6_case_insensitive() {
    let entry = create_test_entry(
        "Connection [2001:DB8::1]:50123 -> [::1]:5672",
        Severity::Info,
    );
    let annotator = ConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_connection_pattern_ipv6_with_zones() {
    let entry = create_test_entry(
        "connection [fe80::1]:52000 -> [fe80::2]:5672",
        Severity::Info,
    );
    let annotator = ConnectionsAnnotator;
    assert!(annotator.does_match(&entry));
}

#[test]
fn test_no_match_ipv6_missing_brackets() {
    let entry = create_test_entry("connection 2001:db8::1:5672 -> ::1:5672", Severity::Info);
    let annotator = ConnectionsAnnotator;
    assert!(!annotator.does_match(&entry));
}
