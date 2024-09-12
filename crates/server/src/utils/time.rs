use chrono::{DateTime, NaiveDate, Utc};
use time::{Date, OffsetDateTime, PrimitiveDateTime};

pub fn to_utc(original_time: PrimitiveDateTime) -> DateTime<Utc> {
    let timestamp = original_time.assume_utc().unix_timestamp_nanos();
    DateTime::from_timestamp_nanos(timestamp as i64)
}

pub fn to_utc_or_default(opt_time: Option<PrimitiveDateTime>) -> Option<DateTime<Utc>> {
    opt_time.map(to_utc)
}

pub fn to_date(datetime: DateTime<Utc>) -> Date {
    let timestamp = datetime.timestamp();
    let odt = OffsetDateTime::from_unix_timestamp(timestamp).unwrap();
    odt.date()
}

pub fn to_date_or_default(opt_datetime: Option<DateTime<Utc>>) -> Option<Date> {
    opt_datetime.map(to_date)
}

pub fn to_naive_date(opt_date: Option<Date>) -> Option<NaiveDate> {
    match opt_date {
        Some(date) => NaiveDate::from_ymd_opt(date.year(), date.month() as u32, date.day() as u32),
        None => NaiveDate::from_ymd_opt(0, 0, 0),
    }
}
