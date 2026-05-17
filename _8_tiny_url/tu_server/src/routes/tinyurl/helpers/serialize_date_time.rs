use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;

#[derive(Debug)]
pub enum FDateTime {
    Date(NaiveDate),
    DateTime(DateTime<Utc>),
}

impl<'de> Deserialize<'de> for FDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        if let Ok(dt) = DateTime::parse_from_rfc3339(&s) {
            return Ok(FDateTime::DateTime(dt.with_timezone(&Utc)));
        }

        if let Ok(date) = NaiveDate::parse_from_str(&s, "%Y-%m-%d") {
            return Ok(FDateTime::Date(date));
        }

        Err(serde::de::Error::custom("invalid date/datetime"))
    }
}
