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

#[derive(Debug, Default)]
pub struct ObfuscationStats {
    pub hostnames_obfuscated: usize,
    pub directories_obfuscated: usize,
    pub usernames_obfuscated: usize,
    pub vhosts_obfuscated: usize,
    pub ipv4_addresses_obfuscated: usize,
    pub ipv6_addresses_obfuscated: usize,
}

pub struct LogObfuscator {
    hostname_map: HashMap<String, String>,
    directory_map: HashMap<String, String>,
    username_map: HashMap<String, String>,
    vhost_map: HashMap<String, String>,
    ipv4_map: HashMap<String, String>,
    ipv6_map: HashMap<String, String>,

    hostname_counter: usize,
    directory_counter: usize,
    username_counter: usize,
    vhost_counter: usize,
    ipv4_counter: usize,
    ipv6_counter: usize,

    node_name_re: Regex,
    ipv4_re: Regex,
    ipv6_re: Regex,
    unix_path_re: Regex,
    user_pattern_re: Regex,
    vhost_pattern_re: Regex,

    stats: ObfuscationStats,
}

impl LogObfuscator {
    pub fn new() -> Self {
        Self {
            hostname_map: HashMap::new(),
            directory_map: HashMap::new(),
            username_map: HashMap::new(),
            vhost_map: HashMap::new(),
            ipv4_map: HashMap::new(),
            ipv6_map: HashMap::new(),

            hostname_counter: 0,
            directory_counter: 0,
            username_counter: 0,
            vhost_counter: 0,
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

            stats: ObfuscationStats::default(),
        }
    }

    pub fn stats(&self) -> &ObfuscationStats {
        &self.stats
    }

    fn get_or_create_hostname(&mut self, hostname: &str) -> String {
        match self.hostname_map.entry(hostname.to_string()) {
            Entry::Occupied(e) => e.get().clone(),
            Entry::Vacant(e) => {
                self.hostname_counter += 1;
                self.stats.hostnames_obfuscated += 1;
                e.insert(format!("host{}", self.hostname_counter)).clone()
            }
        }
    }

    fn get_or_create_directory(&mut self, dir: &str) -> String {
        match self.directory_map.entry(dir.to_string()) {
            Entry::Occupied(e) => e.get().clone(),
            Entry::Vacant(e) => {
                self.directory_counter += 1;
                self.stats.directories_obfuscated += 1;
                e.insert(format!("/data/path{}", self.directory_counter))
                    .clone()
            }
        }
    }

    fn get_or_create_username(&mut self, username: &str) -> String {
        match self.username_map.entry(username.to_string()) {
            Entry::Occupied(e) => e.get().clone(),
            Entry::Vacant(e) => {
                self.username_counter += 1;
                self.stats.usernames_obfuscated += 1;
                e.insert(format!("user{}", self.username_counter)).clone()
            }
        }
    }

    fn get_or_create_vhost(&mut self, vhost: &str) -> String {
        match self.vhost_map.entry(vhost.to_string()) {
            Entry::Occupied(e) => e.get().clone(),
            Entry::Vacant(e) => {
                self.vhost_counter += 1;
                self.stats.vhosts_obfuscated += 1;
                e.insert(format!("vhost{}", self.vhost_counter)).clone()
            }
        }
    }

    fn get_or_create_ipv4(&mut self, ip: &str) -> String {
        if ip == "0.0.0.0" {
            return "0.0.0.0".to_string();
        }

        match self.ipv4_map.entry(ip.to_string()) {
            Entry::Occupied(e) => e.get().clone(),
            Entry::Vacant(e) => {
                self.ipv4_counter += 1;
                self.stats.ipv4_addresses_obfuscated += 1;
                let octet4 = ((self.ipv4_counter - 1) % 255) + 1;
                let octet3 = (self.ipv4_counter - 1) / 255;
                e.insert(format!("10.0.{}.{}", octet3, octet4)).clone()
            }
        }
    }

    fn get_or_create_ipv6(&mut self, ip: &str) -> String {
        match self.ipv6_map.entry(ip.to_string()) {
            Entry::Occupied(e) => e.get().clone(),
            Entry::Vacant(e) => {
                self.ipv6_counter += 1;
                self.stats.ipv6_addresses_obfuscated += 1;
                e.insert(format!("fd00::{}", self.ipv6_counter)).clone()
            }
        }
    }

    pub fn obfuscate_line(&mut self, line: &str) -> String {
        let mut result = line.to_string();

        result = self.obfuscate_node_names(&result);
        result = self.obfuscate_usernames(&result);
        result = self.obfuscate_vhosts(&result);
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

            let obfuscated_hostname = self.get_or_create_hostname(hostname);
            let replacement = format!("{}@{}", node_prefix, obfuscated_hostname);

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

            let full_match = cap.get(0).unwrap();
            let full_str = full_match.as_str();

            let replacement = if full_str.starts_with('"') && full_str.ends_with('"') {
                format!("\"{}\"", obfuscated)
            } else if full_str.starts_with('\'') && full_str.ends_with('\'') {
                format!("'{}'", obfuscated)
            } else {
                obfuscated
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

            let full_match = cap.get(0).unwrap();
            let full_str = full_match.as_str();

            let is_uppercase = full_str.starts_with('U');
            let user_word = if is_uppercase { "User" } else { "user" };

            let replacement = if full_str.contains(": '") {
                format!("{}: '{}'", user_word, obfuscated)
            } else if full_str.contains(": \"") {
                format!("{}: \"{}\"", user_word, obfuscated)
            } else if full_str.contains(" '") {
                format!("{} '{}'", user_word, obfuscated)
            } else {
                format!("{} \"{}\"", user_word, obfuscated)
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

            let full_match = cap.get(0).unwrap();
            let full_str = full_match.as_str();

            let is_virtual_host = full_str.to_lowercase().starts_with("virtual");
            let prefix = if is_virtual_host {
                "virtual host"
            } else {
                "vhost"
            };

            let replacement = if full_str.contains(": '") {
                format!("{}: '{}'", prefix, obfuscated)
            } else if full_str.contains(": \"") {
                format!("{}: \"{}\"", prefix, obfuscated)
            } else if full_str.contains(" '") {
                format!("{} '{}'", prefix, obfuscated)
            } else {
                format!("{} \"{}\"", prefix, obfuscated)
            };

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
            let full_match = cap.get(0).unwrap();
            result.replace_range(full_match.range(), &obfuscated);
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

            let full_match = cap.get(0).unwrap();
            let replacement = format!("[{}]", obfuscated);

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
