 
use std::{error::Error, time::{SystemTime, UNIX_EPOCH}};

use axum::http::HeaderValue;
use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};  

pub fn human_readable_size(bytes: u64) -> String {
    const UNITS: [&str; 9] = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    let mut size = bytes as f64;
    let mut i = 0;
    while size >= 1024.0 && i < UNITS.len() - 1 {
        size /= 1024.0;
        i += 1;
    }
    format!("{:.2} {}", size, UNITS[i])
}


pub fn now_string() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn system_time_to_date_time(t: SystemTime) -> DateTime<Local> {
    let (sec, nsec) = match t.duration_since(UNIX_EPOCH) {
        Ok(dur) => (dur.as_secs() as i64, dur.subsec_nanos()),
        Err(e) => {
            // unlikely but should be handled
            let dur = e.duration();
            let (sec, nsec) = (dur.as_secs() as i64, dur.subsec_nanos());
            if nsec == 0 {
                (-sec, 0)
            } else {
                (-sec - 1, 1_000_000_000 - nsec)
            }
        }
    };
    Local.timestamp_opt(sec, nsec).unwrap()
}   

pub fn convert_http_date_to_u64(header_value: &HeaderValue) -> Result<u64, Box<dyn std::error::Error>> {
    // Convert the HeaderValue to a str
    let str_value = header_value.to_str()?;

    // Parse the str to a DateTime
    let datetime = DateTime::parse_from_rfc2822(str_value)?;

    // Convert the DateTime to a u64 timestamp
    let timestamp = datetime.timestamp() as u64;

    Ok(timestamp)
}
pub fn convert_u64_to_http_date(timestamp: u64) -> Result<String, Box<dyn std::error::Error>> {
    // Convert the u64 timestamp to an i64
    let timestamp_i64: i64 = timestamp.try_into()?;

    // Create a DateTime from the timestamp
    let datetime = Utc.timestamp(timestamp_i64, 0);

    // Format the DateTime in the HTTP-date format
    let http_date = datetime.format("%a, %d %b %Y %H:%M:%S GMT").to_string();

    // Convert the string to a HeaderValue

    Ok(http_date)
}
//日期字符串转时间 2024-01-01 12:12:12
pub fn date_str_to_timestamp(date_time_str:&String)->Result<u64,Box<dyn Error>>{ 
    Ok( NaiveDateTime::parse_from_str(date_time_str, "%Y-%m-%d %H:%M:%S")?.timestamp().try_into()?)

}
//文件名转时间 IMG_20200916_205836.jpg
pub fn filename_to_timestamp(filename: &String) -> Result<u64,Box<dyn Error>> {
    // Extract the date and time portion from the filename
    let datetime_str = filename
    .replace("IMG", "")
    .replace(".jpg", "")
    .replace(".heic", "").replace('_', "");

    // Parse the date and time string
    Ok(NaiveDateTime::parse_from_str(&datetime_str, "%Y%m%d%H%M%S")?.timestamp().try_into()?)
}