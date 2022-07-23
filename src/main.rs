extern crate chrono;
use chrono::prelude::*;
use chrono::Date;
use chrono::{Local, TimeZone};
use std::str::FromStr;

pub fn main() {
    let future: &str = "07-11-2023";
    let today = Local::today();

    let splitfuture: Vec<&str> = future.split("-").collect();
    let year: i32 = FromStr::from_str(splitfuture[2]).unwrap();
    let month: u32 = FromStr::from_str(splitfuture[0]).unwrap();
    let day: u32 = FromStr::from_str(splitfuture[1]).unwrap();
    let futuredate = Local.ymd(year, month, day);

    let (years, months, days) = diff(futuredate, today);

    
    println!("{} years {} months {} days hours:seconds ", years, months, days);

}


fn diff(date1: Date<Local>, date2: Date<Local>) -> (i32, i32, i32) {
    let y1 = date1.year();
    let m1 = date1.month();
    let d1 = date1.day();

    let y2 = date2.year();
    let m2 = date2.month();
    let d2 = date2.day();

    let mut year = y1 as i32 - y2 as i32;
    let mut month = m1 as i32 - m2 as i32;
    let mut day = d1 as i32 - d2 as i32;

 
    let days_in_m1 = get_days_from_month(y1, m1);
    if day < 0 {
        // days in month:
        let t = Local.ymd(y1, m1, days_in_m1 as u32);
        day += days_in_m1 - t.day() as i32;
        month -= 1;
    }
    if month < 0 {
        month += 12;
        year -= 1;
    }

    return (year, month, day.abs());
}

fn get_days_from_month(year: i32, month: u32) -> i32 {
    Local
        .ymd(
            match month {
                12 => year + 1,
                _ => year,
            },
            match month {
                12 => 1,
                _ => month + 1,
            },
            1,
        )
        .signed_duration_since(Local.ymd(year, month, 1))
        .num_days() as i32
}