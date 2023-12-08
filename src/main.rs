mod day_08;
use std::{sync::OnceLock, sync::ONCE_INIT};

use day_08::{task1, task2};
use regex::Regex;

fn main() {
    // f32::INFINITY;
    // f32::NAN;
    run();
}

fn run() {
    // let r1 = task1();
    // println!("task1: {r1}");
    let r2 = task2();
    println!("task2: {r2}");
}

macro_rules! regex {
    ($raw:expr) => {{
        static REGEX: OnceLock<regex::Regex> = OnceLock::new();
        REGEX.get_or_init(|| {
            println!("compile regex once");
            Regex::new($raw).expect("Invalid regex expression")
        })
    }};
}
