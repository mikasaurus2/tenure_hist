#![deny(warnings)]
#![warn(rust_2018_idioms)]

use chrono::{NaiveDate, Utc};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::result::Result;

fn convert_times(lines: Vec<String>) -> Vec<i64> {
    let vec_transform = |date: &String| -> i64 {
        //println!("{}", date);
        let values: Vec<String> = date.split('/').map(|s| s.to_string()).collect();
        let date_time = NaiveDate::from_ymd(
            values[2].parse::<i32>().unwrap() + 2000,
            values[0].parse::<u32>().unwrap(),
            values[1].parse::<u32>().unwrap(),
        )
        .and_hms(0, 0, 0);
        //println!("{}", date_time.timestamp());
        date_time.timestamp()
    };

    lines.iter().map(vec_transform).collect()
}

fn generate_histogram(start_times: Vec<i64>, now: i64) -> Vec<i32> {
    let mut histogram = Vec::new();

    for start_time in start_times.iter() {
        let mut elapsed = now - start_time;
        elapsed = elapsed / 365 / 24 / 60 / 60;
        //println!("years elapsed: {}", elapsed);
        // consider try_as and unwrap()
        if elapsed as usize >= histogram.len() {
            histogram.resize(elapsed as usize + 1, 0);
        }
        histogram[elapsed as usize] += 1;
    }
    histogram
}

fn print_histogram(histogram: Vec<i32>) {
    let mut total_count = 0;
    let mut total_years = 0;
    for (year_index, count) in histogram.iter().enumerate() {
        print!("{:2} Years: ", year_index);
        for _i in 0..*count {
            print!("=");
            total_years += year_index;
            total_count += 1;
        }
        println!();
    }

    println!();
    println!("{} total current employees", total_count);
    println!("{} years average current tenure", total_years / total_count);
}

fn make_histogram() -> Result<(), Box<dyn Error>> {
    let f = File::open("./start_dates.txt")?;
    let reader = BufReader::new(f);
    let lines: Vec<String> = reader
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
    //println!("{:?}", lines);

    let start_times = convert_times(lines);
    let now = Utc::now().timestamp();
    //println!("{:?}", start_times);
    //println!("now is: {}", now);

    let histogram = generate_histogram(start_times, now);
    //println!("{:?}", histogram);

    print_histogram(histogram);

    Ok(())
}

fn main() {
    if let Err(e) = make_histogram() {
        println!("Application error: {}", e);
    }
}
