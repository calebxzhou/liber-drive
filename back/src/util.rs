use std::{
    fs::File,
    io::{self, BufRead, Write},
    path::PathBuf,
};

use axum::http::HeaderValue;
use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};
use env_logger::Builder;
use log::{info, LevelFilter};

pub type AnyError = Box<dyn std::error::Error>;
pub type ResultAnyErr<T> = Result<T, AnyError>;
macro_rules! str_err {
    ($e:expr) => {
        Box::<dyn std::error::Error>::from($e.to_string()) as Box<dyn std::error::Error>
    };
} 

pub fn convert_http_date_to_u64(header_value: &HeaderValue) -> Result<u64, AnyError> {
    // Convert the HeaderValue to a str
    let str_value = header_value.to_str()?;

    // Parse the str to a DateTime
    let datetime = DateTime::parse_from_rfc2822(str_value)?;

    // Convert the DateTime to a u64 timestamp
    let timestamp = datetime.timestamp() as u64;

    Ok(timestamp)
}
pub fn convert_u64_to_http_date(timestamp: u64) -> Result<String, AnyError> {
    // Convert the u64 timestamp to an i64
    let timestamp_i64: i64 = timestamp.try_into()?;

    // Create a DateTime from the timestamp using `timestamp_opt`
    let datetime = Utc
        .timestamp_opt(timestamp_i64, 0)
        .single() // Extracts the single DateTime<Utc> if it's valid
        .ok_or("Invalid timestamp")?; // Converts to an error if the result is None or Ambiguous

    // Format the DateTime in the HTTP-date format
    let http_date = datetime.format("%a, %d %b %Y %H:%M:%S GMT").to_string();

    Ok(http_date)
}
//日期字符串转时间 2024-01-01 12:12:12
pub fn date_str_to_timestamp(date_time_str: &str) -> Result<u64, AnyError> {
    Ok(
        NaiveDateTime::parse_from_str(date_time_str, "%Y-%m-%d %H:%M:%S")?
            .timestamp()
            .try_into()?,
    )
}
//文件名转时间 IMG_20200916_205836.jpg
pub fn filename_to_timestamp(filename: &str) -> Result<u64, AnyError> {
    // Extract the date and time portion from the filename
    let filename_parts = filename.split("IMG_").collect::<Vec<&str>>();
    if filename_parts.len() > 1 {
        //点前面的
        let filename = filename_parts[1].split(".").collect::<Vec<&str>>()[0];
        Ok(NaiveDateTime::parse_from_str(filename, "%Y%m%d%H%M%S")?
            .timestamp()
            .try_into()?)
    } else {
        Err(str_err!("the file name not contains IMG_"))
    }

    // Parse the date and time string
}
//载入工作目录
pub fn load_drive_dirs() -> ResultAnyErr<Vec<PathBuf>> {
    let path = std::path::Path::new("./drive_dir.txt");
    let file = if path.exists() {
        File::open(path)?
    } else {
        let mut file = File::create(path)?;
        file.write_all(b".")?;
        file
    };
    let reader = io::BufReader::new(file);
    let mut dirs = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let line = line.trim().to_owned();
        info!("工作目录：{}", line);
        let dir = PathBuf::from(line).canonicalize()?;
        info!("工作目录：{}", dir.display().to_string());
        dirs.push(dir);
    }
    Ok(dirs)
}
//载入日志
pub fn logger_init() {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();
}
