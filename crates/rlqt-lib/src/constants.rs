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
pub const METADATA_STORE_DOC_URL_ID: i16 = 1;
pub const ALARMS_DOC_URL_ID: i16 = 2;

pub const ISSUE_14181: i16 = 1;
pub const ISSUE_14213: i16 = 2;
pub const DISCUSSION_14094: i16 = 3;
pub const PULL_REQUEST_14409: i16 = 4;

pub fn doc_url_from_id(id: i16) -> Option<&'static str> {
    match id {
        METADATA_STORE_DOC_URL_ID => Some("https://www.rabbitmq.com/docs/metadata-store"),
        ALARMS_DOC_URL_ID => Some("https://www.rabbitmq.com/docs/alarms"),
        _ => None,
    }
}

pub fn resolution_or_discussion_url_from_id(id: i16) -> Option<&'static str> {
    match id {
        ISSUE_14181 => Some("https://github.com/rabbitmq/rabbitmq-server/issues/14181"),
        ISSUE_14213 => Some("https://github.com/rabbitmq/rabbitmq-server/issues/14213"),
        DISCUSSION_14094 => Some("https://github.com/rabbitmq/rabbitmq-server/discussions/14094"),
        PULL_REQUEST_14409 => Some("https://github.com/rabbitmq/rabbitmq-server/pull/14409"),
        _ => None,
    }
}
