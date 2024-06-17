use chrono::{DateTime, Utc};
use time::PrimitiveDateTime;

pub fn to_utc(original_time: PrimitiveDateTime) -> DateTime<Utc> {
    let timestamp = original_time.assume_utc().unix_timestamp_nanos();
    DateTime::from_timestamp_nanos(timestamp as i64)
}

pub fn to_utc_or_default(opt_time: Option<PrimitiveDateTime>) -> Option<DateTime<Utc>> {
    opt_time.map(to_utc)
}