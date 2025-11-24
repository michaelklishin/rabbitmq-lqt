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

use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};

pub fn parse_datetime_flexible(s: &str) -> Result<DateTime<Utc>, String> {
    let len = s.len();

    match len {
        10 => {
            if let Ok(date) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
                let dt = date.and_time(NaiveTime::MIN);
                return Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc));
            }
        }
        19 => {
            if let Ok(dt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
                return Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc));
            }
        }
        _ => {}
    }

    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Ok(dt.with_timezone(&Utc));
    }

    chrono_english::parse_date_string(s, Utc::now(), chrono_english::Dialect::Us).map_err(|e| {
        format!(
            "Could not parse '{}' as a date/time. Supported formats:\n  - Date: YYYY-MM-DD (e.g., 2025-10-27)\n  - DateTime: YYYY-MM-DD HH:MM:SS (e.g., 2025-10-27 18:23:00)\n  - RFC 3339 (e.g., 2025-10-27T18:23:00Z)\n  - Natural language: 'yesterday', '2 days ago', 'last Monday', '1 week ago'\nError: {}",
            s, e
        )
    })
}
