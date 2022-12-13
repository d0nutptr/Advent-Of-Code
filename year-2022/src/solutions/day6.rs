use std::ops::{BitAndAssign, BitOrAssign, Shl};
use std::simd::{u32x16, u8x64, simd_swizzle, Swizzle2, Which, SimdPartialEq, ToBitMask};
use crate::solutions::{AdventSolution, Day};
use seq_macro::seq;

impl AdventSolution for Day<6> {
    type OutputOne = usize;
    type OutputTwo = usize;

    fn problem_one(input: &str) -> Self::OutputOne {
        // println!("\tsimdeez: {}", simdeez(input, 4));

        println!("\tgg: {}", crazy_simd_solution::<14>(input));

        solve_problem_without_bitset(input, 4)
    }

    fn problem_two(input: &str) -> Self::OutputTwo {
        solve_problem_without_bitset(input, 14)
    }
}

// Original solution to the problem
fn _solve_problem(input: &str, window: usize) -> usize {
    use bit_set::BitSet;

    input
        .as_bytes()
        .windows(window)
        .map(|elements| {
            elements
                .into_iter()
                .map(|elem| *elem as usize)
                .collect::<BitSet>()
        })
        .position(|set| set.len() == window)
        .unwrap() + window
}

pub fn solve_problem_without_bitset(input: &str, window: usize) -> usize {
    input
        .as_bytes()
        .windows(window)
        .filter_map(|element| {
            element
                .iter()
                .map(|elem| 1 << (*elem & 31))
                .reduce(|left, right| left | right)
        })
        .position(|num: u64| num.count_ones() == window as _)
        .unwrap() + window
}

pub fn solve_insane(input: &str, window: usize) -> usize {
    let data = input.as_bytes();

    let mut index = 0;
    let mut current_value = 0;
    let mut size = 0;

    'outer: while size < window {
        // note - store this?
        let value = 1 << (data[index] - b'a');
        index += 1;
        size += 1;

        if current_value & value == 0 {
            current_value |= value;
        } else {
            size = 1;
            current_value = value;
            let mut backward_index = index - 2;

            loop {
                let value = 1 << (data[backward_index] - b'a');

                if current_value & value == 0 {
                    current_value |= value;
                    backward_index -= 1;
                    size += 1;
                } else {
                    continue 'outer;
                }
            }
        }
    }

    index
}


pub fn fastest(input: &str, window: usize) -> usize {
    let data = input.as_bytes();

    let mut filter = 0u32;

    for i in 0 .. window - 1 {
        filter ^= 1 << (data[i] & 31);
    }

    for i in window - 1 .. data.len() {
        filter ^= 1 << (data[i] & 31);

        if filter.count_ones() == window as u32 {
            return i + 1;
        }

        filter ^= 1 << (data[i + 1 - window] & 31);
    }

    unreachable!()
}

pub fn crazy_simd_solution<const WINDOW: usize>(input: &str) -> usize
where
    Assert<{WINDOW < 16}>: IsTrue,
    Assert<{WINDOW > 8}>: IsTrue
{
    let data = input.as_bytes();
    let (chunks, _) = data.as_chunks::<64>();

    let vectors = chunks
        .iter()
        .map(|arr| u8x64::from_array(*arr));

    for (idx, vector) in vectors.enumerate() {
        if let Some(position) = get_next_unique_pos(vector) {
            let start = idx * 64 + position * 8 - WINDOW; // idx offset + block position - WINDOW to catch leading edge of window
            let until = (idx + 1) * 64;
            if let Some(answer) = run_iterative(&data[start .. until], WINDOW) {
                return answer + start;
            }
        }
    }

    unimplemented!()
}


fn run_iterative(data: &[u8], window: usize) -> Option<usize> {
    let mut filter = 0u32;

    data.iter()
        .take(window - 1)
        .for_each(|c| filter ^= 1 << (c % 32));

    // take 3 blocks around
    data.windows(window)
        .position(|w| {
            let first = w[0];
            let last = w[w.len() - 1];
            filter ^= 1 << (last % 32);
            let res = filter.count_ones() == window as _;
            filter ^= 1 << (first % 32);
            res
        })
        .map(|x| x + window)
}

fn get_next_unique_pos(block: u8x64) -> Option<usize> {
    let mut mask = block.clone().simd_eq(block.clone().rotate_lanes_left::<1>());
    mask.bitor_assign(block.clone().simd_eq(block.clone().rotate_lanes_left::<2>()));
    mask.bitor_assign(block.clone().simd_eq(block.clone().rotate_lanes_left::<3>()));
    mask.bitor_assign(block.clone().simd_eq(block.clone().rotate_lanes_left::<4>()));
    mask.bitor_assign(block.clone().simd_eq(block.clone().rotate_lanes_left::<5>()));
    mask.bitor_assign(block.clone().simd_eq(block.clone().rotate_lanes_left::<6>()));

    mask.to_bitmask()
        .to_be_bytes()
        .iter()
        .position(|num| *num == 0)
}

const FIRST_INDEXES: [usize; 16] = seq!(N in 0 .. 16 { [ #( N, )* ] });
const SECOND_INDEXES: [usize; 16] = seq!(N in 16 .. 32 { [ #( N, )* ] });
const THIRD_INDEXES: [usize; 16] = seq!(N in 32 .. 48 { [ #( N, )* ] });
const FOURTH_INDEXES: [usize; 16] = seq!(N in 48 .. 64 { [ #( N, )* ] });

pub fn simdeez<const WINDOW: usize>(input: &str) -> usize {
    let vec_of_31 = u8x64::splat(31);

    let data = input.as_bytes();

    let mut idx = 0;

    while idx < data.len() - 64 {
        let mut register = u8x64::from_slice(&data[idx .. ]);
        register.bitand_assign(&vec_of_31);

        let first_array: u32x16 = u32x16::splat(1).shl(simd_swizzle!(register.clone(), FIRST_INDEXES).cast::<u32>());
        let second_array: u32x16 = u32x16::splat(1).shl(simd_swizzle!(register.clone(), SECOND_INDEXES).cast::<u32>());
        let third_array: u32x16 = u32x16::splat(1).shl(simd_swizzle!(register.clone(), THIRD_INDEXES).cast::<u32>());
        let fourth_array: u32x16 = u32x16::splat(1).shl(simd_swizzle!(register.clone(), FOURTH_INDEXES).cast::<u32>());

        let first_bitset = {
            let mut output = first_array.clone();

            output.bitor_assign(Shiftu32x16::<1>::swizzle2(first_array.clone(), second_array.clone()));
            output.bitor_assign(Shiftu32x16::<2>::swizzle2(first_array.clone(), second_array.clone()));
            output.bitor_assign(Shiftu32x16::<3>::swizzle2(first_array.clone(), second_array.clone()));

            output
        };

        let second_bitset = {
            let mut output = second_array.clone();
            output.bitor_assign(Shiftu32x16::<1>::swizzle2(second_array.clone(), third_array.clone()));
            output.bitor_assign(Shiftu32x16::<2>::swizzle2(second_array.clone(), third_array.clone()));
            output.bitor_assign(Shiftu32x16::<3>::swizzle2(second_array, third_array.clone()));

            output
        };

        let third_bitset = {
            let mut output = third_array.clone();
            output.bitor_assign(Shiftu32x16::<1>::swizzle2(third_array.clone(), fourth_array.clone()));
            output.bitor_assign(Shiftu32x16::<2>::swizzle2(third_array.clone(), fourth_array.clone()));
            output.bitor_assign(Shiftu32x16::<3>::swizzle2(third_array, fourth_array));

            output
        };

        let processed_data = [first_bitset.to_array(), second_bitset.to_array(), third_bitset.to_array()];

        let possible_index = processed_data.iter().flatten().position(|elem| elem.count_ones() == WINDOW as u32);

        if let Some(offset) = possible_index {
            return idx + offset + WINDOW;
        }

        idx += 48;
    }

    unimplemented!()
}


struct Shiftu32x16<const SHIFT: usize>;

impl<const SHIFT: usize> Shiftu32x16<SHIFT> {
    pub const fn get_index_for_offset() -> [Which; 16] {
        const fn get_index_entry<const SHIFT: usize, const INDEX: usize>() -> Which {
            match SHIFT + INDEX {
                value if value < 16 => Which::First(value),
                value => Which::Second(value - 16)
            }
        }

        [
            get_index_entry::<SHIFT, 0>(),
            get_index_entry::<SHIFT, 1>(),
            get_index_entry::<SHIFT, 2>(),
            get_index_entry::<SHIFT, 3>(),
            get_index_entry::<SHIFT, 4>(),
            get_index_entry::<SHIFT, 5>(),
            get_index_entry::<SHIFT, 6>(),
            get_index_entry::<SHIFT, 7>(),
            get_index_entry::<SHIFT, 8>(),
            get_index_entry::<SHIFT, 9>(),
            get_index_entry::<SHIFT, 10>(),
            get_index_entry::<SHIFT, 11>(),
            get_index_entry::<SHIFT, 12>(),
            get_index_entry::<SHIFT, 13>(),
            get_index_entry::<SHIFT, 14>(),
            get_index_entry::<SHIFT, 15>()
        ]
    }
}

impl<const SHIFT: usize> Swizzle2<16, 16> for Shiftu32x16<SHIFT>
where
    Assert<{ SHIFT <= 16 }>: IsTrue
{
    const INDEX: [Which; 16] = Self::get_index_for_offset();
}

pub enum Assert<const CHECK: bool> {}

pub trait IsTrue {}

impl IsTrue for Assert<true> {}