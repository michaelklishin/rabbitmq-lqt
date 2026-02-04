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

pub static VIRTUAL_HOST_STOPPING_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^virtual host '[^']+' is stopping$")
        .expect("VIRTUAL_HOST_STOPPING_PATTERN is a valid regex")
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

#[inline]
pub fn matches_cq_storage(msg_lower: &str) -> bool {
    msg_lower.contains("message refcount")
        || msg_lower.contains("finished rebuilding index")
        || msg_lower.contains("rebuilding indices from scratch")
        || msg_lower.contains("rebuilding message location index")
}

#[inline]
pub fn matches_virtual_hosts(msg_lower: &str) -> bool {
    msg_lower.contains("adding vhost")
        || msg_lower.contains("deleting vhost")
        || msg_lower.contains("message store for directory")
        || msg_lower.contains("default queue type for vhost")
        || msg_lower.contains("started message store of type")
        || msg_lower.contains("starting message stores for vhost")
        || msg_lower.contains("setting segment_entry_count for vhost")
        || msg_lower.contains("recovering data for virtual host")
        || msg_lower.contains("parsed virtual host tags")
        || msg_lower.contains("default queue type of virtual host")
        || msg_lower.contains("virtual host processes reconciliation")
        || msg_lower.contains("will reconcile virtual host")
        || msg_lower.contains("will reschedule virtual host process reconciliation")
        || msg_lower.contains("will make sure that processes of")
        || msg_lower.contains("deletion protection")
        || msg_lower.contains("deleting message store directory for vhost")
        || msg_lower.contains("making sure data directory") && msg_lower.contains("for vhost")
        || VIRTUAL_HOSTS_PATTERN.is_match(msg_lower)
        || VIRTUAL_HOST_STOPPING_PATTERN.is_match(msg_lower)
}

#[inline]
pub fn matches_shovels(msg_lower: &str) -> bool {
    msg_lower.contains("rabbit_shovel_dyn_worker_sup_sup")
        || msg_lower.contains("rabbit_shovel_worker")
        || msg_lower.contains("asked to start a dynamic shovel named")
        || msg_lower.contains("for component 'shovel'")
        || msg_lower.contains("shovel: operating mode")
        || msg_lower.contains("asked to stop a dynamic shovel")
        || msg_lower.starts_with("shovel '")
        || SHOVEL_PATTERN.is_match(msg_lower)
}

#[inline]
pub fn matches_federation(msg_lower: &str) -> bool {
    msg_lower.contains("federation queue")
        || msg_lower.contains("disconnecting from queue")
        || msg_lower.contains("for component 'federation-upstream'")
        || msg_lower.contains("pg scope rabbitmq_queue_federation")
        || msg_lower.contains("pg scope rabbitmq_exchange_federation")
}

#[inline]
pub fn matches_policies(msg_lower: &str) -> bool {
    msg_lower.contains("policy ")
}

#[inline]
pub fn matches_consumer_delivery_timeout(msg_lower: &str) -> bool {
    msg_lower.contains("timed out")
        && msg_lower.contains("waiting for a consumer acknowledgement of a delivery")
}

#[inline]
pub fn matches_plugins(msg_lower: &str) -> bool {
    msg_lower.contains("loading the following plugins")
        || msg_lower.contains("setting plugins up")
        || msg_lower.contains("plugins (prelaunch phase)")
        || msg_lower.contains("setting plugins")
        || msg_lower.contains("the following plugins")
        || msg_lower.contains("management plugin:")
        || msg_lower.contains("prometheus metrics:")
        || msg_lower.contains(" exited with reason")
        || msg_lower.starts_with("stopping application")
        || msg_lower.starts_with("plugins changed;")
}

#[inline]
pub fn matches_oauth2(msg_lower: &str) -> bool {
    msg_lower.contains("oauth 2")
        || msg_lower.contains("oauth2")
        || msg_lower.contains("oauth_provider_id")
        || msg_lower.contains("resource_server")
        || msg_lower.contains("jwks")
        || msg_lower.contains("jwk")
        || msg_lower.contains("jwt")
}

#[inline]
pub fn matches_management(msg_lower: &str) -> bool {
    msg_lower.starts_with("management plugin:")
        || msg_lower.starts_with("http api:")
        || msg_lower.contains("statistics database")
}

#[inline]
pub fn matches_raft(msg_lower: &str) -> bool {
    msg_lower.starts_with("wal:")
        || msg_lower.starts_with("ra system")
        || msg_lower.starts_with("ra:")
        || msg_lower.starts_with("ra node")
        || msg_lower.starts_with("ra_log")
        || msg_lower.starts_with("ra_snapshot")
        || msg_lower.starts_with("ra_monitors")
        || msg_lower.starts_with("starting ra system")
        || msg_lower.starts_with("trigger election in")
        || msg_lower.starts_with("ra_system_recover")
        || msg_lower.starts_with("segment_writer")
}

#[inline]
pub fn matches_streams(msg_lower: &str) -> bool {
    msg_lower.contains("osiris")
        || msg_lower.starts_with("stream:")
        || msg_lower.contains("rabbit_stream")
        || msg_lower.contains("stream ")
}

#[inline]
pub fn matches_shutdown(msg_lower: &str) -> bool {
    msg_lower.starts_with("stopping ranch") || msg_lower.starts_with("stopping ra systems")
}
