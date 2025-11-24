use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};

fn parse_datetime_flexible(s: &str) -> Result<DateTime<Utc>, String> {
    let len = s.len();
    let bytes = s.as_bytes();

    match len {
        10 => {
            if bytes[4] == b'-'
                && bytes[7] == b'-'
                && let Ok(date) = NaiveDate::parse_from_str(s, "%Y-%m-%d")
            {
                let dt = date
                    .and_hms_opt(0, 0, 0)
                    .ok_or_else(|| "Invalid time".to_string())?;
                return Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc));
            }
        }
        19 => {
            if bytes[4] == b'-'
                && bytes[7] == b'-'
                && bytes[10] == b' '
                && bytes[13] == b':'
                && bytes[16] == b':'
                && let Ok(dt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
            {
                return Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc));
            }
        }
        _ => {}
    }

    if bytes.contains(&b'T') || bytes.contains(&b'+') || bytes.contains(&b'Z') {
        if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
            return Ok(dt.with_timezone(&Utc));
        }
        if let Ok(dt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S") {
            return Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc));
        }
    }

    Err(format!("Could not parse '{}'", s))
}

#[test]
fn test_parse_date_only() {
    let result = parse_datetime_flexible("2025-10-27");
    assert!(result.is_ok());
    let dt = result.unwrap();
    assert_eq!(dt.format("%Y-%m-%d").to_string(), "2025-10-27");
}

#[test]
fn test_parse_datetime_with_space() {
    let result = parse_datetime_flexible("2025-10-27 18:23:00");
    assert!(result.is_ok());
    let dt = result.unwrap();
    assert_eq!(
        dt.format("%Y-%m-%d %H:%M:%S").to_string(),
        "2025-10-27 18:23:00"
    );
}

#[test]
fn test_parse_rfc3339() {
    let result = parse_datetime_flexible("2025-10-27T18:23:00+00:00");
    assert!(result.is_ok());
}

#[test]
fn test_parse_rfc3339_zulu() {
    let result = parse_datetime_flexible("2025-10-27T18:23:00Z");
    assert!(result.is_ok());
}

#[test]
fn test_parse_datetime_with_t_separator() {
    let result = parse_datetime_flexible("2025-10-27T18:23:00");
    assert!(result.is_ok());
}

#[test]
fn test_parse_invalid_format() {
    let result = parse_datetime_flexible("not a date");
    assert!(result.is_err());
}

#[test]
fn test_parse_invalid_date() {
    let result = parse_datetime_flexible("2025-13-45");
    assert!(result.is_err());
}
