use std::env;
use std::fs::File;
use std::io::{stdin, Read};
use std::time::Instant;

fn main() {
    let mut args = env::args();
    args.next();
    match args.next() {
        None => println!("Usage: cargo run N < input/N"),
        Some(arg) => match arg.as_str() {
            "timed" => time_all(),
            "all" => all(),
            "8tt" => day8::run_ttaylor(stdin()),
            "10int" => day10::run_int(stdin()),
            "10rec" => day10::run_rec(stdin()),
            day => run(day.parse().unwrap(), stdin()),
        },
    };
}

mod common;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
// mod day16;
mod day17;
mod day18;
// mod day19;
mod day2;
// mod day20;
mod day21;
mod day22;
// mod day23;
// mod day24;
// mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn run<R: Read>(day: u8, r: R) {
    match day {
        1 => day1::run(r),
        2 => day2::run(r),
        3 => day3::run(r),
        4 => day4::run(r),
        5 => day5::run(r),
        6 => day6::run(r),
        7 => day7::run(r),
        8 => day8::run(r),
        9 => day9::run(r),
        10 => day10::run(r),
        11 => day11::run(r),
        12 => day12::run(r),
        13 => day13::run(r),
        14 => day14::run(r),
        15 => day15::run(r),
        // 16 => day16::run(r),
        17 => day17::run(r),
        18 => day18::run(r),
        // 19 => day19::run(r),
        // 20 => day20::run(r),
        21 => day21::run(r),
        22 => day22::run(r),
        // 23 => day23::run(r),
        // 24 => day24::run(r),
        // 25 => day25::run(r),
        _ => println!("TODO!"),
    };
}

fn open_input(day: u8) -> Result<File, std::io::Error> {
    let path = format!("input/{}", day);
    File::open(&path)
}

fn all() {
    for day in 1..=25 {
        if let Ok(f) = open_input(day) {
            println!("------");
            println!("day {}", day);
            run(day, f);
        }
    }
}

fn time_all() {
    let start = Instant::now();
    for day in 1..=25 {
        time(day);
        match day {
            8 => time_fn(8, "(ttaylor)", day8::run_ttaylor),
            10 => {
                time_fn(10, "(int accumulator)", day10::run_int);
                time_fn(10, "(recursive)", day10::run_rec);
            }
            _ => (),
        };
    }
    println!(
        "*** total time: {} ms",
        1000.0 * start.elapsed().as_secs_f32()
    );
}

fn time(day: u8) {
    if let Ok(f) = open_input(day) {
        let start = Instant::now();
        run(day, f);
        let ms = elapsed_ms(start);
        println!("** day {}: {} ms", day, ms);
    }
}

fn time_fn<F: Fn(File)>(day: u8, label: &str, run: F) {
    if let Ok(f) = open_input(day) {
        let start = Instant::now();
        run(f);
        let ms = elapsed_ms(start);
        println!("** day {} {}: {} ms", day, label, ms);
    }
}

fn elapsed_ms(start: Instant) -> f32 {
    1000.0 * start.elapsed().as_secs_f32()
}
