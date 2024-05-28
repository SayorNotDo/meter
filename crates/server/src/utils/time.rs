use chrono::{NaiveDateTime, NaiveTime, NaiveDate};
use time::PrimitiveDateTime;

pub fn time_convert(ori_time: PrimitiveDateTime) -> Result<NaiveDateTime, &'static str> {
    let chrono_naive_date = NaiveDate::from_ymd_opt(
        ori_time.year(),
        ori_time.month() as u32,
        ori_time.day() as u32,
    ).ok_or("Invalid Date")?;
    let chrono_naive_time = NaiveTime::from_hms_nano_opt(
        ori_time.hour() as u32,
        ori_time.minute() as u32,
        ori_time.second() as u32,
        ori_time.nanosecond(),
    ).ok_or("Invalid time")?;
    Ok(NaiveDateTime::new(chrono_naive_date, chrono_naive_time))
}