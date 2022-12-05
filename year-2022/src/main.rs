#![feature(iter_array_chunks)]
#![feature(generic_const_exprs)]

mod solutions;

use crate::solutions::*;

fn main() {
    Day::<1>::run();
    Day::<2>::run();
    Day::<3>::run();
    Day::<4>::run();
    Day::<5>::run();
}

// const fn run_aoc<const DayNum: usize>()
//     where [(); { DayNum - 1 }]:
// {
//     if DayNum > 0 {
//         run_aoc::<{ DayNum - 1 }>() ;
//
//         Day::<{ DayNum as u64 }>::run();
//     }
// }