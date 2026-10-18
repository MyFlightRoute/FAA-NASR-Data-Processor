use chrono::{NaiveDate, TimeZone, Utc};

pub fn discord_timestamp(effective_date: &str) -> Result<String, chrono::ParseError> {
    let date = NaiveDate::parse_from_str(effective_date, "%Y-%m-%d")?;
    let datetime = date.and_hms_opt(9, 0, 0).expect("valid time");
    let utc_datetime = Utc.from_utc_datetime(&datetime);

    Ok(format!("<t:{}:R>", utc_datetime.timestamp()))
}