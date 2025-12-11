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

use rlqt_obfuscation::LogObfuscator;

#[test]
fn test_obfuscate_node_name() {
    let mut obfuscator = LogObfuscator::new();

    let input = "starting Ra system: coordination in directory: rabbit@sunnyside";
    let output = obfuscator.obfuscate_line(input);

    assert!(output.contains("rabbit@host1"));
    assert!(!output.contains("sunnyside"));
}

#[test]
fn test_obfuscate_multiple_node_names_consistently() {
    let mut obfuscator = LogObfuscator::new();

    let line1 = "Node rabbit@sunnyside started";
    let line2 = "Connecting to hare@sunnyside";
    let line3 = "Node rabbit@sunnyside is healthy";

    let output1 = obfuscator.obfuscate_line(line1);
    let output2 = obfuscator.obfuscate_line(line2);
    let output3 = obfuscator.obfuscate_line(line3);

    // sunnyside should map to host1 consistently
    assert!(output1.contains("rabbit@host1"));
    assert!(output2.contains("hare@host1"));
    assert!(output3.contains("rabbit@host1"));

    assert!(!output1.contains("sunnyside"));
    assert!(!output2.contains("sunnyside"));
    assert!(!output3.contains("sunnyside"));
}

#[test]
fn test_obfuscate_different_hostnames() {
    let mut obfuscator = LogObfuscator::new();

    let line1 = "Node rabbit@host-a started";
    let line2 = "Node rabbit@host-b started";

    let output1 = obfuscator.obfuscate_line(line1);
    let output2 = obfuscator.obfuscate_line(line2);

    // Different hostnames should map to different obfuscated values
    assert!(output1.contains("rabbit@host1"));
    assert!(output2.contains("rabbit@host2"));
}

#[test]
fn test_obfuscate_ipv4_address() {
    let mut obfuscator = LogObfuscator::new();

    let input = "connection 127.0.0.1:57942 -> 192.168.1.100:5672";
    let output = obfuscator.obfuscate_line(input);

    assert!(!output.contains("127.0.0.1"));
    assert!(!output.contains("192.168.1.100"));
    assert!(output.contains("10.0.0."));
}

#[test]
fn test_obfuscate_ipv4_consistently() {
    let mut obfuscator = LogObfuscator::new();

    let line1 = "accepting AMQP connection 127.0.0.1:57942 -> 127.0.0.1:5672";
    let line2 = "closing AMQP connection 127.0.0.1:57942 -> 127.0.0.1:5672";

    let output1 = obfuscator.obfuscate_line(line1);
    let output2 = obfuscator.obfuscate_line(line2);

    // Same IP should map to same obfuscated value
    assert_eq!(
        output1.matches("10.0.0.1").count(),
        output2.matches("10.0.0.1").count()
    );
}

#[test]
fn test_obfuscate_ipv4_preserves_0000() {
    let mut obfuscator = LogObfuscator::new();

    let input = "listening for HTTP connections on 0.0.0.0:15674";
    let output = obfuscator.obfuscate_line(input);

    assert!(output.contains("0.0.0.0"));
}

#[test]
fn test_obfuscate_ipv6_address() {
    let mut obfuscator = LogObfuscator::new();

    let input = "accepting AMQP connection [::1]:57941 -> [::1]:5672";
    let output = obfuscator.obfuscate_line(input);

    assert!(!output.contains("[::1]"));
    assert!(output.contains("fd00::"));
}

#[test]
fn test_obfuscate_username() {
    let mut obfuscator = LogObfuscator::new();

    let input = "user 'guest' authenticated and granted access";
    let output = obfuscator.obfuscate_line(input);

    assert!(!output.contains("guest"));
    assert!(output.contains("user 'user1'"));
}

#[test]
fn test_obfuscate_username_consistently() {
    let mut obfuscator = LogObfuscator::new();

    let line1 = "user 'admin' logged in";
    let line2 = "user 'guest' logged in";
    let line3 = "user 'admin' logged out";

    let output1 = obfuscator.obfuscate_line(line1);
    let output2 = obfuscator.obfuscate_line(line2);
    let output3 = obfuscator.obfuscate_line(line3);

    // admin should map to user1, guest to user2
    assert!(output1.contains("user 'user1'"));
    assert!(output2.contains("user 'user2'"));
    assert!(output3.contains("user 'user1'"));
}

#[test]
fn test_obfuscate_vhost() {
    let mut obfuscator = LogObfuscator::new();

    let input = "granted access to vhost 'rabbitmqadmin.shovels.test22'";
    let output = obfuscator.obfuscate_line(input);

    assert!(!output.contains("rabbitmqadmin.shovels.test22"));
    assert!(output.contains("vhost 'vhost1'"));
}

#[test]
fn test_obfuscate_default_vhost() {
    let mut obfuscator = LogObfuscator::new();

    let input = "granted access to vhost '/'";
    let output = obfuscator.obfuscate_line(input);

    assert!(!output.contains("vhost '/'"));
    assert!(output.contains("vhost 'vhost1'"));
}

#[test]
fn test_obfuscate_vhost_consistently() {
    let mut obfuscator = LogObfuscator::new();

    let line1 = "vhost 'production' created";
    let line2 = "vhost 'staging' created";
    let line3 = "vhost 'production' deleted";

    let output1 = obfuscator.obfuscate_line(line1);
    let output2 = obfuscator.obfuscate_line(line2);
    let output3 = obfuscator.obfuscate_line(line3);

    assert!(output1.contains("vhost 'vhost1'"));
    assert!(output2.contains("vhost 'vhost2'"));
    assert!(output3.contains("vhost 'vhost1'"));
}

#[test]
fn test_obfuscate_directory_path() {
    let mut obfuscator = LogObfuscator::new();

    let input = "data_dir => \"/Users/username/Tools/rabbitmq/mnesia/rabbit@eng.orgname.com\"";
    let output = obfuscator.obfuscate_line(input);

    assert!(!output.contains("/Users/username"));
    assert!(output.contains("/data/path"));
}

#[test]
fn test_obfuscate_directory_consistently() {
    let mut obfuscator = LogObfuscator::new();

    let line1 = "Reading from /home/user/rabbitmq/data";
    let line2 = "Writing to /var/log/rabbitmq/rabbit.log";
    let line3 = "Reading from /home/user/rabbitmq/data again";

    let output1 = obfuscator.obfuscate_line(line1);
    let output2 = obfuscator.obfuscate_line(line2);
    let output3 = obfuscator.obfuscate_line(line3);

    assert!(output1.contains("/data/path1"));
    assert!(output2.contains("/data/path2"));
    assert!(output3.contains("/data/path1"));
}

#[test]
fn test_obfuscate_complex_log_line() {
    let mut obfuscator = LogObfuscator::new();

    let input = "2025-11-28 00:38:15.858785-08:00 [info] <0.3228.0> connection 127.0.0.1:64169 -> 127.0.0.1:5672 - Shovel test: user 'guest' authenticated and granted access to vhost 'rabbitmqadmin.shovels.test22'";
    let output = obfuscator.obfuscate_line(input);

    assert!(output.contains("[info]"));
    assert!(output.contains("<0.3228.0>"));

    assert!(!output.contains("127.0.0.1"));
    assert!(!output.contains("guest"));
    assert!(!output.contains("rabbitmqadmin.shovels.test22"));
}

#[test]
fn test_obfuscate_virtual_host_variant() {
    let mut obfuscator = LogObfuscator::new();

    let input = "Adding virtual host 'production' (description: 'Production', tags: [])";
    let output = obfuscator.obfuscate_line(input);

    assert!(!output.contains("production"));
    assert!(output.contains("virtual host 'vhost1'"));
}

#[test]
fn test_stats_tracking() {
    let mut obfuscator = LogObfuscator::new();

    obfuscator.obfuscate_line("rabbit@server1 connected");
    obfuscator.obfuscate_line("rabbit@server2 connected");
    obfuscator.obfuscate_line("user 'admin' logged in");
    obfuscator.obfuscate_line("vhost 'test' created");
    obfuscator.obfuscate_line("queue 'orders' created");
    obfuscator.obfuscate_line("exchange 'events' created");
    obfuscator.obfuscate_line("stream 'logs' created");
    obfuscator.obfuscate_line("policy 'ha-all' applied");
    obfuscator.obfuscate_line("connection 192.168.1.1:1234");
    obfuscator.obfuscate_line("connection [::1]:1234");
    obfuscator.obfuscate_line("/home/user/data");

    let stats = obfuscator.stats();
    assert_eq!(stats.hostnames_obfuscated, 2);
    assert_eq!(stats.usernames_obfuscated, 1);
    assert_eq!(stats.vhosts_obfuscated, 1);
    assert_eq!(stats.queues_obfuscated, 1);
    assert_eq!(stats.exchanges_obfuscated, 1);
    assert_eq!(stats.streams_obfuscated, 1);
    assert_eq!(stats.policies_obfuscated, 1);
    assert_eq!(stats.ipv4_addresses_obfuscated, 1);
    assert_eq!(stats.ipv6_addresses_obfuscated, 1);
    assert_eq!(stats.directories_obfuscated, 1);
}

#[test]
fn test_no_obfuscation_needed() {
    let mut obfuscator = LogObfuscator::new();

    let input = "Ra system starting up";
    let output = obfuscator.obfuscate_line(input);

    assert_eq!(input, output);
}

#[test]
fn test_obfuscate_username_case_insensitive() {
    let mut obfuscator = LogObfuscator::new();

    let line1 = "user 'admin' logged in";
    let line2 = "User 'admin' authenticated successfully";

    let output1 = obfuscator.obfuscate_line(line1);
    let output2 = obfuscator.obfuscate_line(line2);

    assert!(output1.contains("user 'user1'"));
    assert!(output2.contains("User 'user1'"));
    assert!(!output1.contains("admin"));
    assert!(!output2.contains("admin"));
}

#[test]
fn test_obfuscate_ipv6_consistently() {
    let mut obfuscator = LogObfuscator::new();

    let line1 = "accepting AMQP connection [::1]:57941 -> [::1]:5672";
    let line2 = "closing AMQP connection [::1]:57941 -> [::1]:5672";

    let output1 = obfuscator.obfuscate_line(line1);
    let output2 = obfuscator.obfuscate_line(line2);

    assert_eq!(
        output1.matches("[fd00::1]").count(),
        output2.matches("[fd00::1]").count()
    );
}

#[test]
fn test_obfuscate_empty_line() {
    let mut obfuscator = LogObfuscator::new();

    let output = obfuscator.obfuscate_line("");

    assert_eq!(output, "");
}

#[test]
fn test_obfuscate_multiple_ips_same_line() {
    let mut obfuscator = LogObfuscator::new();

    let input = "connection 192.168.1.1:5672 -> 192.168.1.2:5672 via 172.16.0.1";
    let output = obfuscator.obfuscate_line(input);

    assert!(!output.contains("192.168.1.1"));
    assert!(!output.contains("192.168.1.2"));
    assert!(!output.contains("172.16.0.1"));
    assert!(output.contains("10.0.0.1"));
    assert!(output.contains("10.0.0.2"));
    assert!(output.contains("10.0.0.3"));
}

#[test]
fn test_obfuscate_multiple_users_same_line() {
    let mut obfuscator = LogObfuscator::new();

    let input = "user 'admin' granted permissions to user 'guest'";
    let output = obfuscator.obfuscate_line(input);

    assert!(!output.contains("admin"));
    assert!(!output.contains("guest"));
    assert!(output.contains("user 'user1'"));
    assert!(output.contains("user 'user2'"));
}

#[test]
fn test_ipv4_counter_rollover() {
    let mut obfuscator = LogObfuscator::new();

    for i in 1..=256 {
        let ip = format!("192.168.{}.1", i);
        let _ = obfuscator.obfuscate_line(&format!("connection {}", ip));
    }

    let stats = obfuscator.stats();
    assert_eq!(stats.ipv4_addresses_obfuscated, 256);

    let output = obfuscator.obfuscate_line("connection 192.168.1.1");
    assert!(output.contains("10.0.0.1"));

    let output = obfuscator.obfuscate_line("connection 192.168.256.1");
    assert!(output.contains("10.0.1.1"));
}

#[test]
fn test_double_obfuscation_prevented_username() {
    let mut obfuscator1 = LogObfuscator::new();
    let input = "user 'admin' logged in";
    let first_pass = obfuscator1.obfuscate_line(input);
    assert!(first_pass.contains("user 'user1'"));
    assert_eq!(obfuscator1.stats().usernames_obfuscated, 1);

    let mut obfuscator2 = LogObfuscator::new();
    let second_pass = obfuscator2.obfuscate_line(&first_pass);
    assert_eq!(first_pass, second_pass);
    assert_eq!(obfuscator2.stats().usernames_obfuscated, 0);
}

#[test]
fn test_double_obfuscation_prevented_vhost() {
    let mut obfuscator1 = LogObfuscator::new();
    let input = "vhost 'production' created";
    let first_pass = obfuscator1.obfuscate_line(input);
    assert!(first_pass.contains("vhost 'vhost1'"));
    assert_eq!(obfuscator1.stats().vhosts_obfuscated, 1);

    let mut obfuscator2 = LogObfuscator::new();
    let second_pass = obfuscator2.obfuscate_line(&first_pass);
    assert_eq!(first_pass, second_pass);
    assert_eq!(obfuscator2.stats().vhosts_obfuscated, 0);
}

#[test]
fn test_double_obfuscation_prevented_hostname() {
    let mut obfuscator1 = LogObfuscator::new();
    let input = "rabbit@sunnyside connected";
    let first_pass = obfuscator1.obfuscate_line(input);
    assert!(first_pass.contains("rabbit@host1"));
    assert_eq!(obfuscator1.stats().hostnames_obfuscated, 1);

    let mut obfuscator2 = LogObfuscator::new();
    let second_pass = obfuscator2.obfuscate_line(&first_pass);
    assert_eq!(first_pass, second_pass);
    assert_eq!(obfuscator2.stats().hostnames_obfuscated, 0);
}

#[test]
fn test_double_obfuscation_prevented_queue() {
    let mut obfuscator1 = LogObfuscator::new();
    let input = "queue 'events.incoming' created";
    let first_pass = obfuscator1.obfuscate_line(input);
    assert!(first_pass.contains("queue 'queue1'"));
    assert_eq!(obfuscator1.stats().queues_obfuscated, 1);

    let mut obfuscator2 = LogObfuscator::new();
    let second_pass = obfuscator2.obfuscate_line(&first_pass);
    assert_eq!(first_pass, second_pass);
    assert_eq!(obfuscator2.stats().queues_obfuscated, 0);
}

#[test]
fn test_double_obfuscation_prevented_exchange() {
    let mut obfuscator1 = LogObfuscator::new();
    let input = "exchange 'amq.topic' created";
    let first_pass = obfuscator1.obfuscate_line(input);
    assert!(first_pass.contains("exchange 'exchange1'"));
    assert_eq!(obfuscator1.stats().exchanges_obfuscated, 1);

    let mut obfuscator2 = LogObfuscator::new();
    let second_pass = obfuscator2.obfuscate_line(&first_pass);
    assert_eq!(first_pass, second_pass);
    assert_eq!(obfuscator2.stats().exchanges_obfuscated, 0);
}

#[test]
fn test_double_obfuscation_prevented_stream() {
    let mut obfuscator1 = LogObfuscator::new();
    let input = "stream 'logs.events' created";
    let first_pass = obfuscator1.obfuscate_line(input);
    assert!(first_pass.contains("stream 'stream1'"));
    assert_eq!(obfuscator1.stats().streams_obfuscated, 1);

    let mut obfuscator2 = LogObfuscator::new();
    let second_pass = obfuscator2.obfuscate_line(&first_pass);
    assert_eq!(first_pass, second_pass);
    assert_eq!(obfuscator2.stats().streams_obfuscated, 0);
}

#[test]
fn test_double_obfuscation_prevented_policy() {
    let mut obfuscator1 = LogObfuscator::new();
    let input = "policy 'ha-all' applied";
    let first_pass = obfuscator1.obfuscate_line(input);
    assert!(first_pass.contains("policy 'policy1'"));
    assert_eq!(obfuscator1.stats().policies_obfuscated, 1);

    let mut obfuscator2 = LogObfuscator::new();
    let second_pass = obfuscator2.obfuscate_line(&first_pass);
    assert_eq!(first_pass, second_pass);
    assert_eq!(obfuscator2.stats().policies_obfuscated, 0);
}

#[test]
fn test_double_obfuscation_prevented_ipv4() {
    let mut obfuscator1 = LogObfuscator::new();
    let input = "connection 192.168.1.100:5672";
    let first_pass = obfuscator1.obfuscate_line(input);
    assert!(first_pass.contains("10.0.0.1"));
    assert_eq!(obfuscator1.stats().ipv4_addresses_obfuscated, 1);

    let mut obfuscator2 = LogObfuscator::new();
    let second_pass = obfuscator2.obfuscate_line(&first_pass);
    assert_eq!(first_pass, second_pass);
    assert_eq!(obfuscator2.stats().ipv4_addresses_obfuscated, 0);
}

#[test]
fn test_double_obfuscation_prevented_ipv6() {
    let mut obfuscator1 = LogObfuscator::new();
    let input = "connection [::1]:5672";
    let first_pass = obfuscator1.obfuscate_line(input);
    assert!(first_pass.contains("[fd00::1]"));
    assert_eq!(obfuscator1.stats().ipv6_addresses_obfuscated, 1);

    let mut obfuscator2 = LogObfuscator::new();
    let second_pass = obfuscator2.obfuscate_line(&first_pass);
    assert_eq!(first_pass, second_pass);
    assert_eq!(obfuscator2.stats().ipv6_addresses_obfuscated, 0);
}

#[test]
fn test_double_obfuscation_prevented_directory() {
    let mut obfuscator1 = LogObfuscator::new();
    let input = "reading from /home/user/rabbitmq/data";
    let first_pass = obfuscator1.obfuscate_line(input);
    assert!(first_pass.contains("/data/path1"));
    assert_eq!(obfuscator1.stats().directories_obfuscated, 1);

    let mut obfuscator2 = LogObfuscator::new();
    let second_pass = obfuscator2.obfuscate_line(&first_pass);
    assert_eq!(first_pass, second_pass);
    assert_eq!(obfuscator2.stats().directories_obfuscated, 0);
}

#[test]
fn test_double_obfuscation_prevented_erlang_tuple() {
    let mut obfuscator1 = LogObfuscator::new();
    let input = "detected a new leader {'UnifiedPortal_up.request','rabbit@sunnyside'}";
    let first_pass = obfuscator1.obfuscate_line(input);
    assert!(first_pass.contains("{'queue1','rabbit@host1'}"));
    assert_eq!(obfuscator1.stats().queues_obfuscated, 1);
    assert_eq!(obfuscator1.stats().hostnames_obfuscated, 1);

    let mut obfuscator2 = LogObfuscator::new();
    let second_pass = obfuscator2.obfuscate_line(&first_pass);
    assert_eq!(first_pass, second_pass);
    assert_eq!(obfuscator2.stats().queues_obfuscated, 0);
    assert_eq!(obfuscator2.stats().hostnames_obfuscated, 0);
}

#[test]
fn test_partially_obfuscated_erlang_tuple_queue_only() {
    let mut obfuscator1 = LogObfuscator::new();
    // First obfuscator processes only the queue name pattern
    obfuscator1.obfuscate_line("queue 'MyQueue' created");
    assert_eq!(obfuscator1.stats().queues_obfuscated, 1);

    // Now feed an Erlang tuple where the queue is already-obfuscated but the hostname is not
    let mut obfuscator2 = LogObfuscator::new();
    let input = "detected a new leader {'queue1','rabbit@sunnyside'}";
    let output = obfuscator2.obfuscate_line(input);

    // queue1 should be preserved, hostname should be obfuscated
    assert!(output.contains("{'queue1','rabbit@host1'}"));
    assert_eq!(obfuscator2.stats().queues_obfuscated, 0);
    assert_eq!(obfuscator2.stats().hostnames_obfuscated, 1);
}

#[test]
fn test_partially_obfuscated_erlang_tuple_hostname_only() {
    let mut obfuscator = LogObfuscator::new();
    // Erlang tuple where the hostname is already-obfuscated but the queue is not
    let input = "detected a new leader {'MyRealQueue','rabbit@host1'}";
    let output = obfuscator.obfuscate_line(input);

    // Queue should be obfuscated, host1 should be preserved
    assert!(output.contains("{'queue1','rabbit@host1'}"));
    assert_eq!(obfuscator.stats().queues_obfuscated, 1);
    assert_eq!(obfuscator.stats().hostnames_obfuscated, 0);
}
