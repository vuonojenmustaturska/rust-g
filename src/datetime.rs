extern crate zero85;
const I48_SIZE: usize = 6;

use chrono::prelude::*;
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};
use byteorder::{LittleEndian, WriteBytesExt, ReadBytesExt};
use std::io::Cursor;
use std::mem::size_of;
use std::io::{Error, ErrorKind};
use std::time::Duration;
use zero85::{ToZ85,FromZ85};

byond_fn! { 
    datetime_add_duration(datetime_string, duration_string) {
        let datetime = z85_to_datetime(datetime_string.to_string()).ok()?;
        let duration = chrono::Duration::from_std(z85_to_duration(duration_string.to_string()).ok()?).ok()?;
        let result = datetime + duration;

        datetime_to_z85::<FixedOffset>(result).ok()
    } 
}

byond_fn! { 
    datetime_sub_duration(datetime_string, duration_string) {
        let datetime = z85_to_datetime(datetime_string.to_string()).ok()?;
        let duration = chrono::Duration::from_std(z85_to_duration(duration_string.to_string()).ok()?).ok()?;
        let result = datetime - duration;

        datetime_to_z85::<FixedOffset>(result).ok()
    } 
}

byond_fn! { 
    datetime_from_format(source_string, format) {
        match format {
                "utcnow" => datetime_to_z85::<Utc>(Utc::now()).ok(),
                "localnow" => datetime_to_z85::<Local>(Local::now()).ok(),
                "rfc3339" => datetime_to_z85::<FixedOffset>(DateTime::<FixedOffset>::parse_from_rfc3339(source_string).ok()?).ok(),
                "rfc2822" => datetime_to_z85::<FixedOffset>(DateTime::<FixedOffset>::parse_from_rfc2822(source_string).ok()?).ok(),
                "timestamp" => datetime_to_z85::<Utc>(DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(source_string.parse::<i64>().ok()?, 0), Utc)).ok(),
                _ => datetime_to_z85::<FixedOffset>(DateTime::<FixedOffset>::parse_from_str(source_string, format).ok()?).ok()
        }
    } 
}

byond_fn! { 
    datetime_to_format(datetime_string, format) {
        if let Ok(datetime) = z85_to_datetime(datetime_string.to_string())
        {
            match format {
                "rfc3339" => Some(datetime.to_rfc3339()),
                "rfc2822" => Some(datetime.to_rfc2822()),
                "timestamp" => Some(datetime.timestamp().to_string()),
                _ => Some(datetime.format(format).to_string())
            }
        }
        else
        {
            None
        }

    } 
}

// zero85 spits out two different error types 
fn z85error_to_stderror(z85: impl std::error::Error) -> std::io::Error {
    // the errors print sensibly, though
    std::io::Error::new(ErrorKind::InvalidInput, format!("Z85 error: {}", z85))
}

fn datetime_to_z85<Tz: TimeZone>(dt: DateTime<Tz>) -> Result<String, std::io::Error> {
    datetime_to_bytes(dt)?.to_z85().map_err(z85error_to_stderror)
}

fn z85_to_datetime(z85: String) -> Result<DateTime<FixedOffset>, std::io::Error> {
    bytes_to_datetime(z85.from_z85().map_err(z85error_to_stderror)?)
}

fn duration_to_z85(dr: Duration) -> Result<String, std::io::Error> {
    duration_to_bytes(dr)?.to_z85().map_err(z85error_to_stderror)
}

fn z85_to_duration(z85: String) -> Result<Duration, std::io::Error> {
    bytes_to_duration(z85.from_z85().map_err(z85error_to_stderror)?)
}

fn datetime_to_bytes<Tz: TimeZone>(dt: DateTime<Tz>) -> Result<Vec<u8>, std::io::Error> {
    let mut r = Vec::with_capacity(I48_SIZE + size_of::<u32>() + size_of::<i16>());
    r.write_i48::<LittleEndian>(dt.timestamp())?;
    r.write_u32::<LittleEndian>(dt.timestamp_subsec_nanos())?;
    r.write_i16::<LittleEndian>((dt.offset().fix().local_minus_utc()/60) as i16)?;
    Ok(r)
}

fn bytes_to_datetime(bt: Vec<u8>) -> Result<DateTime<FixedOffset>, std::io::Error> {
    let mut cursor = Cursor::new(bt);
    let millis = cursor.read_i48::<LittleEndian>()?;
    let nanos = cursor.read_u32::<LittleEndian>()?;
    let offset = FixedOffset::east((cursor.read_i16::<LittleEndian>()? as i32)*60);
    Ok(DateTime::<FixedOffset>::from_utc(NaiveDateTime::from_timestamp(millis, nanos), offset))
}

fn duration_to_bytes(dr: Duration) -> Result<Vec<u8>, std::io::Error> {
    let mut r = Vec::with_capacity(size_of::<u64>() + size_of::<u32>());
    r.write_u64::<LittleEndian>(dr.as_secs())?;
    r.write_u32::<LittleEndian>(dr.subsec_nanos())?;
    Ok(r)
}

fn bytes_to_duration(bt: Vec<u8>) -> Result<Duration, std::io::Error> {
    let mut cursor = Cursor::new(bt);
    let secs = cursor.read_u64::<LittleEndian>()?;
    let nanos = cursor.read_u32::<LittleEndian>()?;
    Ok(Duration::new(secs, nanos))
}