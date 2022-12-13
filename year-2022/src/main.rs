#![allow(incomplete_features)]
#![feature(iter_array_chunks)]
#![feature(generic_const_exprs)]
#![feature(portable_simd)]
#![feature(slice_flatten)]
#![feature(slice_as_chunks)]
#![feature(array_chunks)]
#![feature(binary_heap_into_iter_sorted)]

pub mod solutions;

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
        6,
        7
    ];
}