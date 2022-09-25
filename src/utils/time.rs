use chrono::{DateTime, NaiveDateTime, Utc};

use crate::Result;

const DATE_FORMATTER: &str = "%Y/%m/%d";
const TIME_FORMATTER: &str = "%H:%M:%S";
const DATETIME_FORMATTER: &str = "%Y/%m/%d %H:%M:%S";

pub fn now() -> i64 {
    Utc::now().timestamp()
}

pub fn from_timestamp(ts: i64) -> DateTime<Utc> {
    DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(ts, 0), Utc)
}

pub fn format(ts: i64, fmt: &str) -> String {
    from_timestamp(ts).format(fmt).to_string()
}

pub fn date_str(ts: i64) -> String {
    format(ts, DATE_FORMATTER)
}
pub fn time_str(ts: i64) -> String {
    format(ts, TIME_FORMATTER)
}
pub fn datetime_str(ts: i64) -> String {
    format(ts, DATETIME_FORMATTER)
}
