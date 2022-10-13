use std::time;

use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike};

fn convert_u64_to_i64(num: u64) -> i64 {
    if num > i64::MAX as u64 {
        i64::MAX
    } else {
        num as i64
    }
}

fn datetime_to_tuple(datetime: &str) -> (i32, u32, u32, u32, u32, u32) {
    let year: i32 = datetime[0..4].parse().unwrap();
    let month: u32 = datetime[5..7].parse().unwrap();
    let date: u32 = datetime[8..10].parse().unwrap();
    let hour: u32 = datetime[11..13].parse().unwrap();
    let minute: u32 = datetime[14..16].parse().unwrap();
    let second: u32 = datetime[17..19].parse().unwrap();

    (year, month, date, hour, minute, second)
}

/// Returns UNIX Time in seconds
pub fn now() -> u64 {
    let duration = time::SystemTime::now()
        .duration_since(time::SystemTime::UNIX_EPOCH)
        .unwrap();
    duration.as_secs()
}

pub fn unixtime_to_datetime(unixtime: u64) -> String {
    let unixtime = convert_u64_to_i64(unixtime);
    let naive = NaiveDateTime::from_timestamp(unixtime, 0);
    format!(
        "{}-{:02}-{:02} {:02}:{:02}:{:02}",
        naive.year(),
        naive.month(),
        naive.day(),
        naive.hour(),
        naive.minute(),
        naive.second()
    )
}

pub fn datetime_to_unixtime(datetime: &str) -> u64 {
    let tup = datetime_to_tuple(datetime);
    let naive = NaiveDate::from_ymd(tup.0, tup.1, tup.2).and_hms(tup.3, tup.4, tup.5);
    naive.timestamp().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(unixtime_to_datetime(0), "1970-01-01 00:00:00");
        assert_eq!(unixtime_to_datetime(1665463180), "2022-10-11 04:39:40");
        assert_eq!(datetime_to_unixtime("1970-01-01 00:00:00"), 0);
        assert_eq!(datetime_to_unixtime("2022-10-11 04:39:40"), 1665463180);
    }
}
