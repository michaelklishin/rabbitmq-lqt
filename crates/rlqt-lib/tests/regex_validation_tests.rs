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

use rlqt_lib::entry_metadata::shared::{
    SHOVEL_CONNECT_ERROR_PATTERN, SHOVEL_CONNECTED_PATTERN, SHOVEL_IN_VHOST_PATTERN,
    SHOVEL_RECEIVED_PATTERN, SHOVEL_TOPOLOGY_PATTERN, VIRTUAL_HOSTS_PATTERN,
};

#[test]
fn test_all_static_regexes_compile() {
    let _ = &*VIRTUAL_HOSTS_PATTERN;
    let _ = &*SHOVEL_IN_VHOST_PATTERN;
    let _ = &*SHOVEL_CONNECTED_PATTERN;
    let _ = &*SHOVEL_TOPOLOGY_PATTERN;
    let _ = &*SHOVEL_RECEIVED_PATTERN;
    let _ = &*SHOVEL_CONNECT_ERROR_PATTERN;
}

#[test]
fn test_shared_patterns_are_valid() {
    assert!(VIRTUAL_HOSTS_PATTERN.is_match("recovering 42 queues of type rabbit_classic_queue"));
    assert!(SHOVEL_IN_VHOST_PATTERN.is_match("shovel 'my-shovel' in virtual host '/' started"));
    assert!(SHOVEL_CONNECTED_PATTERN.is_match("shovel 'my-shovel' connected to destination"));
    assert!(SHOVEL_TOPOLOGY_PATTERN.is_match("shovel 'test' has finished setting up its topology"));
    assert!(
        SHOVEL_RECEIVED_PATTERN.is_match("shovel 'test' received a 'basic.ack' from the server")
    );
    assert!(SHOVEL_CONNECT_ERROR_PATTERN.is_match("shovel 'test' could not connect to source"));
}
