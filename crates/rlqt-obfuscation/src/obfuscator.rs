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

use regex::Regex;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

/// Result of an obfuscation attempt, containing the value and whether it was freshly obfuscated.
#[derive(Debug, Clone)]
pub struct ObfuscatedString {
    pub value: String,
    pub was_obfuscated: bool,
}

impl ObfuscatedString {
    fn original(value: String) -> Self {
        Self {
            value,
            was_obfuscated: true,
        }
    }

    fn already(value: String) -> Self {
        Self {
            value,
            was_obfuscated: false,
        }
    }
}

/// Checks if a hostname looks like it was already obfuscated (e.g., "host1", "host42")
fn is_obfuscated_hostname(hostname: &str) -> bool {
    hostname.starts_with("host")
        && hostname.len() > 4
        && hostname[4..].chars().all(|c| c.is_ascii_digit())
}

/// Checks if a queue name looks like it was already obfuscated (e.g., "queue1", "queue42")
fn is_obfuscated_queue(queue: &str) -> bool {
    queue.starts_with("queue") && queue.len() > 5 && queue[5..].chars().all(|c| c.is_ascii_digit())
}

/// Checks if a username looks like it was already obfuscated (e.g., "user1", "user42")
fn is_obfuscated_username(username: &str) -> bool {
    username.starts_with("user")
        && username.len() > 4
        && username[4..].chars().all(|c| c.is_ascii_digit())
}

/// Checks if a vhost looks like it was already obfuscated (e.g., "vhost1", "vhost42")
fn is_obfuscated_vhost(vhost: &str) -> bool {
    vhost.starts_with("vhost") && vhost.len() > 5 && vhost[5..].chars().all(|c| c.is_ascii_digit())
}

/// Checks if an exchange looks like it was already obfuscated (e.g., "exchange1", "exchange42")
fn is_obfuscated_exchange(exchange: &str) -> bool {
    exchange.starts_with("exchange")
        && exchange.len() > 8
        && exchange[8..].chars().all(|c| c.is_ascii_digit())
}

/// Checks if a stream looks like it was already obfuscated (e.g., "stream1", "stream42")
fn is_obfuscated_stream(stream: &str) -> bool {
    stream.starts_with("stream")
        && stream.len() > 6
        && stream[6..].chars().all(|c| c.is_ascii_digit())
}

/// Checks if a policy looks like it was already obfuscated (e.g., "policy1", "policy42")
fn is_obfuscated_policy(policy: &str) -> bool {
    policy.starts_with("policy")
        && policy.len() > 6
        && policy[6..].chars().all(|c| c.is_ascii_digit())
}

/// Checks if a directory looks like it was already obfuscated (e.g., "/data/path1", "/data/path42")
fn is_obfuscated_directory(dir: &str) -> bool {
    dir.starts_with("/data/path") && dir.len() > 10 && dir[10..].chars().all(|c| c.is_ascii_digit())
}

/// Checks if an IPv4 address looks like it was already obfuscated (e.g., "10.0.0.1", "10.0.1.255")
fn is_obfuscated_ipv4(ip: &str) -> bool {
    ip.starts_with("10.0.")
}

/// Checks if an IPv6 address looks like it was already obfuscated (e.g., "fd00::1", "fd00::42")
fn is_obfuscated_ipv6(ip: &str) -> bool {
    ip.starts_with("fd00::")
}

#[derive(Debug, Default)]
pub struct ObfuscationStats {
    pub hostnames_obfuscated: usize,
    pub directories_obfuscated: usize,
    pub usernames_obfuscated: usize,
    pub vhosts_obfuscated: usize,
    pub queues_obfuscated: usize,
    pub exchanges_obfuscated: usize,
    pub streams_obfuscated: usize,
    pub policies_obfuscated: usize,
    pub ipv4_addresses_obfuscated: usize,
    pub ipv6_addresses_obfuscated: usize,
}

pub struct LogObfuscator {
    hostname_map: HashMap<String, String>,
    directory_map: HashMap<String, String>,
    username_map: HashMap<String, String>,
    vhost_map: HashMap<String, String>,
    queue_map: HashMap<String, String>,
    exchange_map: HashMap<String, String>,
    stream_map: HashMap<String, String>,
    policy_map: HashMap<String, String>,
    ipv4_map: HashMap<String, String>,
    ipv6_map: HashMap<String, String>,

    hostname_counter: usize,
    directory_counter: usize,
    username_counter: usize,
    vhost_counter: usize,
    queue_counter: usize,
    exchange_counter: usize,
    stream_counter: usize,
    policy_counter: usize,
    ipv4_counter: usize,
    ipv6_counter: usize,

    node_name_re: Regex,
    ipv4_re: Regex,
    ipv6_re: Regex,
    unix_path_re: Regex,
    user_pattern_re: Regex,
    vhost_pattern_re: Regex,
    queue_pattern_re: Regex,
    exchange_pattern_re: Regex,
    stream_pattern_re: Regex,
    policy_pattern_re: Regex,
    erlang_queue_tuple_re: Regex,

    stats: ObfuscationStats,
}

impl LogObfuscator {
    pub fn new() -> Self {
        Self {
            hostname_map: HashMap::new(),
            directory_map: HashMap::new(),
            username_map: HashMap::new(),
            vhost_map: HashMap::new(),
            queue_map: HashMap::new(),
            exchange_map: HashMap::new(),
            stream_map: HashMap::new(),
            policy_map: HashMap::new(),
            ipv4_map: HashMap::new(),
            ipv6_map: HashMap::new(),

            hostname_counter: 0,
            directory_counter: 0,
            username_counter: 0,
            vhost_counter: 0,
            queue_counter: 0,
            exchange_counter: 0,
            stream_counter: 0,
            policy_counter: 0,
            ipv4_counter: 0,
            ipv6_counter: 0,

            // Matches Erlang node names like rabbit@hostname, hare@sunnyside
            node_name_re: Regex::new(r"([a-zA-Z_][a-zA-Z0-9_]*)@([a-zA-Z][a-zA-Z0-9._-]*)")
                .unwrap(),

            // Matches IPv4 addresses
            ipv4_re: Regex::new(r"\b(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})\b").unwrap(),

            // Matches IPv6 addresses in bracketed form like [::1] or [fe80::1]
            ipv6_re: Regex::new(r"\[([0-9a-fA-F:]+)\]").unwrap(),

            // Matches Unix-style paths starting with /Users, /home, /var, /tmp, /opt, /etc
            unix_path_re: Regex::new(
                r#"(?:"|')?(/(?:Users|home|var|tmp|opt|etc|data)/[a-zA-Z0-9._/-]+)(?:"|')?"#,
            )
            .unwrap(),

            // Matches user patterns like: user 'username', User 'username', user: 'username'
            user_pattern_re: Regex::new(r#"(?i)user[:\s]+['"]([^'"]+)['"]"#).unwrap(),

            // Matches vhost patterns like: vhost '/', vhost: 'name', vhost "name", virtual host '/'
            vhost_pattern_re: Regex::new(r#"(?:vhost|virtual\s+host)[:\s]+['"]([^'"]+)['"]"#)
                .unwrap(),

            // Matches queue patterns like: queue 'name', Queue 'name'
            queue_pattern_re: Regex::new(r#"(?i)queue[:\s]+['"]([^'"]+)['"]"#).unwrap(),

            // Matches exchange patterns like: exchange 'name', Exchange 'name'
            exchange_pattern_re: Regex::new(r#"(?i)exchange[:\s]+['"]([^'"]+)['"]"#).unwrap(),

            // Matches stream patterns like: stream 'name', Stream 'name', Stream: 'name'
            stream_pattern_re: Regex::new(r#"(?i)stream[:\s]+['"]([^'"]+)['"]"#).unwrap(),

            // Matches policy patterns like: policy 'name', Policy 'name'
            policy_pattern_re: Regex::new(r#"(?i)policy[:\s]+['"]([^'"]+)['"]"#).unwrap(),

            // Matches Erlang tuples containing queue names and node names like:
            // {'QueueName','rabbit@host'} in quorum queue Raft log messages
            erlang_queue_tuple_re: Regex::new(
                r#"\{'([^']+)','([a-zA-Z_][a-zA-Z0-9_]*)@([a-zA-Z][a-zA-Z0-9._-]*)'\}"#,
            )
            .unwrap(),

            stats: ObfuscationStats::default(),
        }
    }

    pub fn stats(&self) -> &ObfuscationStats {
        &self.stats
    }

    fn get_or_create_hostname(&mut self, hostname: &str) -> ObfuscatedString {
        if is_obfuscated_hostname(hostname) {
            return ObfuscatedString::already(hostname.to_string());
        }

        match self.hostname_map.entry(hostname.to_string()) {
            Entry::Occupied(e) => ObfuscatedString::original(e.get().clone()),
            Entry::Vacant(e) => {
                self.hostname_counter += 1;
                self.stats.hostnames_obfuscated += 1;
                ObfuscatedString::original(
                    e.insert(format!("host{}", self.hostname_counter)).clone(),
                )
            }
        }
    }

    fn get_or_create_directory(&mut self, dir: &str) -> ObfuscatedString {
        if is_obfuscated_directory(dir) {
            return ObfuscatedString::already(dir.to_string());
        }

        match self.directory_map.entry(dir.to_string()) {
            Entry::Occupied(e) => ObfuscatedString::original(e.get().clone()),
            Entry::Vacant(e) => {
                self.directory_counter += 1;
                self.stats.directories_obfuscated += 1;
                ObfuscatedString::original(
                    e.insert(format!("/data/path{}", self.directory_counter))
                        .clone(),
                )
            }
        }
    }

    fn get_or_create_username(&mut self, username: &str) -> ObfuscatedString {
        if is_obfuscated_username(username) {
            return ObfuscatedString::already(username.to_string());
        }

        match self.username_map.entry(username.to_string()) {
            Entry::Occupied(e) => ObfuscatedString::original(e.get().clone()),
            Entry::Vacant(e) => {
                self.username_counter += 1;
                self.stats.usernames_obfuscated += 1;
                ObfuscatedString::original(
                    e.insert(format!("user{}", self.username_counter)).clone(),
                )
            }
        }
    }

    fn get_or_create_vhost(&mut self, vhost: &str) -> ObfuscatedString {
        if is_obfuscated_vhost(vhost) {
            return ObfuscatedString::already(vhost.to_string());
        }

        match self.vhost_map.entry(vhost.to_string()) {
            Entry::Occupied(e) => ObfuscatedString::original(e.get().clone()),
            Entry::Vacant(e) => {
                self.vhost_counter += 1;
                self.stats.vhosts_obfuscated += 1;
                ObfuscatedString::original(e.insert(format!("vhost{}", self.vhost_counter)).clone())
            }
        }
    }

    fn get_or_create_queue(&mut self, queue: &str) -> ObfuscatedString {
        if is_obfuscated_queue(queue) {
            return ObfuscatedString::already(queue.to_string());
        }

        match self.queue_map.entry(queue.to_string()) {
            Entry::Occupied(e) => ObfuscatedString::original(e.get().clone()),
            Entry::Vacant(e) => {
                self.queue_counter += 1;
                self.stats.queues_obfuscated += 1;
                ObfuscatedString::original(e.insert(format!("queue{}", self.queue_counter)).clone())
            }
        }
    }

    fn get_or_create_exchange(&mut self, exchange: &str) -> ObfuscatedString {
        if is_obfuscated_exchange(exchange) {
            return ObfuscatedString::already(exchange.to_string());
        }

        match self.exchange_map.entry(exchange.to_string()) {
            Entry::Occupied(e) => ObfuscatedString::original(e.get().clone()),
            Entry::Vacant(e) => {
                self.exchange_counter += 1;
                self.stats.exchanges_obfuscated += 1;
                ObfuscatedString::original(
                    e.insert(format!("exchange{}", self.exchange_counter))
                        .clone(),
                )
            }
        }
    }

    fn get_or_create_stream(&mut self, stream: &str) -> ObfuscatedString {
        if is_obfuscated_stream(stream) {
            return ObfuscatedString::already(stream.to_string());
        }

        match self.stream_map.entry(stream.to_string()) {
            Entry::Occupied(e) => ObfuscatedString::original(e.get().clone()),
            Entry::Vacant(e) => {
                self.stream_counter += 1;
                self.stats.streams_obfuscated += 1;
                ObfuscatedString::original(
                    e.insert(format!("stream{}", self.stream_counter)).clone(),
                )
            }
        }
    }

    fn get_or_create_policy(&mut self, policy: &str) -> ObfuscatedString {
        if is_obfuscated_policy(policy) {
            return ObfuscatedString::already(policy.to_string());
        }

        match self.policy_map.entry(policy.to_string()) {
            Entry::Occupied(e) => ObfuscatedString::original(e.get().clone()),
            Entry::Vacant(e) => {
                self.policy_counter += 1;
                self.stats.policies_obfuscated += 1;
                ObfuscatedString::original(
                    e.insert(format!("policy{}", self.policy_counter)).clone(),
                )
            }
        }
    }

    fn get_or_create_ipv4(&mut self, ip: &str) -> ObfuscatedString {
        if ip == "0.0.0.0" {
            return ObfuscatedString::already("0.0.0.0".to_string());
        }

        if is_obfuscated_ipv4(ip) {
            return ObfuscatedString::already(ip.to_string());
        }

        match self.ipv4_map.entry(ip.to_string()) {
            Entry::Occupied(e) => ObfuscatedString::original(e.get().clone()),
            Entry::Vacant(e) => {
                self.ipv4_counter += 1;
                self.stats.ipv4_addresses_obfuscated += 1;
                let octet4 = ((self.ipv4_counter - 1) % 255) + 1;
                let octet3 = (self.ipv4_counter - 1) / 255;
                ObfuscatedString::original(e.insert(format!("10.0.{}.{}", octet3, octet4)).clone())
            }
        }
    }

    fn get_or_create_ipv6(&mut self, ip: &str) -> ObfuscatedString {
        if is_obfuscated_ipv6(ip) {
            return ObfuscatedString::already(ip.to_string());
        }

        match self.ipv6_map.entry(ip.to_string()) {
            Entry::Occupied(e) => ObfuscatedString::original(e.get().clone()),
            Entry::Vacant(e) => {
                self.ipv6_counter += 1;
                self.stats.ipv6_addresses_obfuscated += 1;
                ObfuscatedString::original(e.insert(format!("fd00::{}", self.ipv6_counter)).clone())
            }
        }
    }

    pub fn obfuscate_line(&mut self, line: &str) -> String {
        let mut result = line.to_string();

        // Erlang tuples must be processed first to extract queue/hostname before
        // general node name obfuscation transforms them
        result = self.obfuscate_erlang_queue_tuples(&result);
        result = self.obfuscate_node_names(&result);
        result = self.obfuscate_usernames(&result);
        result = self.obfuscate_vhosts(&result);
        result = self.obfuscate_queues(&result);
        result = self.obfuscate_exchanges(&result);
        result = self.obfuscate_streams(&result);
        result = self.obfuscate_policies(&result);
        result = self.obfuscate_ipv4(&result);
        result = self.obfuscate_ipv6(&result);
        result = self.obfuscate_directories(&result);

        result
    }

    fn obfuscate_node_names(&mut self, input: &str) -> String {
        let captures: Vec<_> = self.node_name_re.captures_iter(input).collect();
        if captures.is_empty() {
            return input.to_string();
        }

        let mut result = input.to_string();
        for cap in captures.iter().rev() {
            let full_match = cap.get(0).unwrap();
            let node_prefix = cap.get(1).unwrap().as_str();
            let hostname = cap.get(2).unwrap().as_str();

            let obfuscated = self.get_or_create_hostname(hostname);
            if !obfuscated.was_obfuscated {
                continue;
            }

            let replacement = format!("{}@{}", node_prefix, obfuscated.value);
            result.replace_range(full_match.range(), &replacement);
        }
        result
    }

    fn obfuscate_directories(&mut self, input: &str) -> String {
        let captures: Vec<_> = self.unix_path_re.captures_iter(input).collect();
        if captures.is_empty() {
            return input.to_string();
        }

        let mut result = input.to_string();
        for cap in captures.iter().rev() {
            let path = cap.get(1).unwrap().as_str();
            let obfuscated = self.get_or_create_directory(path);
            if !obfuscated.was_obfuscated {
                continue;
            }

            let full_match = cap.get(0).unwrap();
            let full_str = full_match.as_str();

            let replacement = if full_str.starts_with('"') && full_str.ends_with('"') {
                format!("\"{}\"", obfuscated.value)
            } else if full_str.starts_with('\'') && full_str.ends_with('\'') {
                format!("'{}'", obfuscated.value)
            } else {
                obfuscated.value
            };

            result.replace_range(full_match.range(), &replacement);
        }
        result
    }

    fn obfuscate_usernames(&mut self, input: &str) -> String {
        let captures: Vec<_> = self.user_pattern_re.captures_iter(input).collect();
        if captures.is_empty() {
            return input.to_string();
        }

        let mut result = input.to_string();
        for cap in captures.iter().rev() {
            let username = cap.get(1).unwrap().as_str();
            let obfuscated = self.get_or_create_username(username);
            if !obfuscated.was_obfuscated {
                continue;
            }

            let full_match = cap.get(0).unwrap();
            let full_str = full_match.as_str();

            let is_uppercase = full_str.starts_with('U');
            let user_word = if is_uppercase { "User" } else { "user" };

            let replacement = if full_str.contains(": '") {
                format!("{}: '{}'", user_word, obfuscated.value)
            } else if full_str.contains(": \"") {
                format!("{}: \"{}\"", user_word, obfuscated.value)
            } else if full_str.contains(" '") {
                format!("{} '{}'", user_word, obfuscated.value)
            } else {
                format!("{} \"{}\"", user_word, obfuscated.value)
            };

            result.replace_range(full_match.range(), &replacement);
        }
        result
    }

    fn obfuscate_vhosts(&mut self, input: &str) -> String {
        let captures: Vec<_> = self.vhost_pattern_re.captures_iter(input).collect();
        if captures.is_empty() {
            return input.to_string();
        }

        let mut result = input.to_string();
        for cap in captures.iter().rev() {
            let vhost = cap.get(1).unwrap().as_str();
            let obfuscated = self.get_or_create_vhost(vhost);
            if !obfuscated.was_obfuscated {
                continue;
            }

            let full_match = cap.get(0).unwrap();
            let full_str = full_match.as_str();

            let is_virtual_host = full_str.to_lowercase().starts_with("virtual");
            let prefix = if is_virtual_host {
                "virtual host"
            } else {
                "vhost"
            };

            let replacement = if full_str.contains(": '") {
                format!("{}: '{}'", prefix, obfuscated.value)
            } else if full_str.contains(": \"") {
                format!("{}: \"{}\"", prefix, obfuscated.value)
            } else if full_str.contains(" '") {
                format!("{} '{}'", prefix, obfuscated.value)
            } else {
                format!("{} \"{}\"", prefix, obfuscated.value)
            };

            result.replace_range(full_match.range(), &replacement);
        }
        result
    }

    fn obfuscate_queues(&mut self, input: &str) -> String {
        let captures: Vec<_> = self.queue_pattern_re.captures_iter(input).collect();
        if captures.is_empty() {
            return input.to_string();
        }

        let mut result = input.to_string();
        for cap in captures.iter().rev() {
            let queue = cap.get(1).unwrap().as_str();
            let obfuscated = self.get_or_create_queue(queue);
            if !obfuscated.was_obfuscated {
                continue;
            }

            let full_match = cap.get(0).unwrap();
            let full_str = full_match.as_str();

            let is_uppercase = full_str.starts_with('Q');
            let keyword = if is_uppercase { "Queue" } else { "queue" };

            let replacement = if full_str.contains(": '") {
                format!("{}: '{}'", keyword, obfuscated.value)
            } else if full_str.contains(": \"") {
                format!("{}: \"{}\"", keyword, obfuscated.value)
            } else if full_str.contains(" '") {
                format!("{} '{}'", keyword, obfuscated.value)
            } else {
                format!("{} \"{}\"", keyword, obfuscated.value)
            };

            result.replace_range(full_match.range(), &replacement);
        }
        result
    }

    fn obfuscate_exchanges(&mut self, input: &str) -> String {
        let captures: Vec<_> = self.exchange_pattern_re.captures_iter(input).collect();
        if captures.is_empty() {
            return input.to_string();
        }

        let mut result = input.to_string();
        for cap in captures.iter().rev() {
            let exchange = cap.get(1).unwrap().as_str();
            let obfuscated = self.get_or_create_exchange(exchange);
            if !obfuscated.was_obfuscated {
                continue;
            }

            let full_match = cap.get(0).unwrap();
            let full_str = full_match.as_str();

            let is_uppercase = full_str.starts_with('E');
            let keyword = if is_uppercase { "Exchange" } else { "exchange" };

            let replacement = if full_str.contains(": '") {
                format!("{}: '{}'", keyword, obfuscated.value)
            } else if full_str.contains(": \"") {
                format!("{}: \"{}\"", keyword, obfuscated.value)
            } else if full_str.contains(" '") {
                format!("{} '{}'", keyword, obfuscated.value)
            } else {
                format!("{} \"{}\"", keyword, obfuscated.value)
            };

            result.replace_range(full_match.range(), &replacement);
        }
        result
    }

    fn obfuscate_streams(&mut self, input: &str) -> String {
        let captures: Vec<_> = self.stream_pattern_re.captures_iter(input).collect();
        if captures.is_empty() {
            return input.to_string();
        }

        let mut result = input.to_string();
        for cap in captures.iter().rev() {
            let stream = cap.get(1).unwrap().as_str();
            let obfuscated = self.get_or_create_stream(stream);
            if !obfuscated.was_obfuscated {
                continue;
            }

            let full_match = cap.get(0).unwrap();
            let full_str = full_match.as_str();

            let is_uppercase = full_str.starts_with('S');
            let keyword = if is_uppercase { "Stream" } else { "stream" };

            let replacement = if full_str.contains(": '") {
                format!("{}: '{}'", keyword, obfuscated.value)
            } else if full_str.contains(": \"") {
                format!("{}: \"{}\"", keyword, obfuscated.value)
            } else if full_str.contains(" '") {
                format!("{} '{}'", keyword, obfuscated.value)
            } else {
                format!("{} \"{}\"", keyword, obfuscated.value)
            };

            result.replace_range(full_match.range(), &replacement);
        }
        result
    }

    fn obfuscate_policies(&mut self, input: &str) -> String {
        let captures: Vec<_> = self.policy_pattern_re.captures_iter(input).collect();
        if captures.is_empty() {
            return input.to_string();
        }

        let mut result = input.to_string();
        for cap in captures.iter().rev() {
            let policy = cap.get(1).unwrap().as_str();
            let obfuscated = self.get_or_create_policy(policy);
            if !obfuscated.was_obfuscated {
                continue;
            }

            let full_match = cap.get(0).unwrap();
            let full_str = full_match.as_str();

            let is_uppercase = full_str.starts_with('P');
            let keyword = if is_uppercase { "Policy" } else { "policy" };

            let replacement = if full_str.contains(": '") {
                format!("{}: '{}'", keyword, obfuscated.value)
            } else if full_str.contains(": \"") {
                format!("{}: \"{}\"", keyword, obfuscated.value)
            } else if full_str.contains(" '") {
                format!("{} '{}'", keyword, obfuscated.value)
            } else {
                format!("{} \"{}\"", keyword, obfuscated.value)
            };

            result.replace_range(full_match.range(), &replacement);
        }
        result
    }

    fn obfuscate_erlang_queue_tuples(&mut self, input: &str) -> String {
        let captures: Vec<_> = self.erlang_queue_tuple_re.captures_iter(input).collect();
        if captures.is_empty() {
            return input.to_string();
        }

        let mut result = input.to_string();
        for cap in captures.iter().rev() {
            let queue_name = cap.get(1).unwrap().as_str();
            let node_prefix = cap.get(2).unwrap().as_str();
            let hostname = cap.get(3).unwrap().as_str();

            let obfuscated_queue = self.get_or_create_queue(queue_name);
            let obfuscated_hostname = self.get_or_create_hostname(hostname);

            // Skip if both are already obfuscated
            if !obfuscated_queue.was_obfuscated && !obfuscated_hostname.was_obfuscated {
                continue;
            }

            let full_match = cap.get(0).unwrap();
            let replacement = format!(
                "{{'{}','{}@{}'}}",
                obfuscated_queue.value, node_prefix, obfuscated_hostname.value
            );

            result.replace_range(full_match.range(), &replacement);
        }
        result
    }

    fn obfuscate_ipv4(&mut self, input: &str) -> String {
        let captures: Vec<_> = self.ipv4_re.captures_iter(input).collect();
        if captures.is_empty() {
            return input.to_string();
        }

        let mut result = input.to_string();
        for cap in captures.iter().rev() {
            let ip = cap.get(1).unwrap().as_str();
            let obfuscated = self.get_or_create_ipv4(ip);
            if !obfuscated.was_obfuscated {
                continue;
            }
            let full_match = cap.get(0).unwrap();
            result.replace_range(full_match.range(), &obfuscated.value);
        }
        result
    }

    fn obfuscate_ipv6(&mut self, input: &str) -> String {
        let captures: Vec<_> = self.ipv6_re.captures_iter(input).collect();
        if captures.is_empty() {
            return input.to_string();
        }

        let mut result = input.to_string();
        for cap in captures.iter().rev() {
            let ip = cap.get(1).unwrap().as_str();
            let obfuscated = self.get_or_create_ipv6(ip);
            if !obfuscated.was_obfuscated {
                continue;
            }

            let full_match = cap.get(0).unwrap();
            let replacement = format!("[{}]", obfuscated.value);

            result.replace_range(full_match.range(), &replacement);
        }
        result
    }
}

impl Default for LogObfuscator {
    fn default() -> Self {
        Self::new()
    }
}
