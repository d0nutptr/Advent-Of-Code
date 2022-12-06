#![allow(incomplete_features)]
#![feature(iter_array_chunks)]
#![feature(generic_const_exprs)]

mod solutions;

use crate::solutions::*;

macro_rules! run_aoc_days {
    ($day:literal) => {
        Day::<$day>::run();
    };
    ($day:literal, $($days:literal),+) => {
        Day::<$day>::run();
        run_aoc_days!($($days),+);
    }
}

fn main() {
    run_aoc_days![
        1,
        2,
        3,
        4,
        5,
        6
    ];
}