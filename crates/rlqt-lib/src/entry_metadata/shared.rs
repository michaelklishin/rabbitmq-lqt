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
use std::sync::LazyLock;

pub static VIRTUAL_HOSTS_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"recovering \d+ queues of type").expect("VIRTUAL_HOSTS_PATTERN is a valid regex")
});

pub static SHOVEL_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"shovel\s+(?:'[^']+'|<<[^>]+>>)\s+(?:",
        r"in\s+virtual\s+host|",
        r"connected|",
        r"has\s+finished\s+setting\s+up\s+its\s+topology|",
        r"received\s+a\s+'[^']+'\s+from\s+the\s+server|",
        r"could not connect to source",
        r")"
    ))
    .expect("SHOVEL_PATTERN is a valid regex")
});

pub fn matches_cq_storage(msg_lower: &str) -> bool {
    msg_lower.contains("message refcount")
        || msg_lower.contains("finished rebuilding index")
        || msg_lower.contains("rebuilding indices from scratch")
        || msg_lower.contains("rebuilding message location index")
}

pub fn matches_virtual_hosts(msg_lower: &str) -> bool {
    msg_lower.contains("adding vhost")
        || msg_lower.contains("deleting vhost")
        || msg_lower.contains("message store for directory")
        || msg_lower.contains("default queue type for vhost")
        || msg_lower.contains("started message store of type")
        || msg_lower.contains("starting message stores for vhost")
        || msg_lower.contains("setting segment_entry_count for vhost")
        || msg_lower.contains("recovering data for virtual host")
        || VIRTUAL_HOSTS_PATTERN.is_match(msg_lower)
}

pub fn matches_shovels(msg_lower: &str) -> bool {
    msg_lower.contains("rabbit_shovel_dyn_worker_sup_sup")
        || msg_lower.contains("rabbit_shovel_worker")
        || msg_lower.contains("asked to start a dynamic shovel named")
        || msg_lower.contains("for component 'shovel'")
        || SHOVEL_PATTERN.is_match(msg_lower)
}

#[inline]
pub fn matches_federation(msg_lower: &str) -> bool {
    msg_lower.contains("federation queue")
        || msg_lower.contains("disconnecting from queue")
        || msg_lower.contains("for component 'federation-upstream'")
}

pub fn matches_plugins(msg_lower: &str) -> bool {
    msg_lower.contains("loading the following plugins")
        || msg_lower.contains("setting plugins up")
        || msg_lower.contains("plugins (prelaunch phase)")
        || msg_lower.contains("setting plugins")
        || msg_lower.contains("the following plugins")
}
