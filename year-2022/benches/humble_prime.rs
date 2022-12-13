use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use aoc::solutions::{Day, AdventSolution};
use aoc::solutions::day6::{solve_problem_without_bitset as d0nuts_solution_day6, solve_insane, fastest, simdeez, gg};


fn primes_solution_day6(input: &str, window: usize) -> usize {
    input
        .as_bytes()
        .windows(window)
        .position(move |set| {
            let mut data: u32 = 0;
            for &c in set {
                let prev = data;
                data |= 1 << (c - b'a');
                if prev == data {
                    return false;
                }
            }
            return true
        })
        .unwrap() + window
}

pub fn benny_solution(i: &str, num: usize) -> usize {
    let i = i.as_bytes();

    let mut filter = 0u32;
    i.iter()
        .take(num - 1)
        .for_each(|c| filter ^= 1 << (c % 32));
    i.windows(num)
        .position(|w| {
            let first = w[0];
            let last = w[w.len() - 1];
            filter ^= 1 << (last % 32);
            let res = filter.count_ones() == num as u32;
            filter ^= 1 << (first % 32);
            res
        })
        .map(|x| x + num)
        .unwrap()
}

pub fn resistcorp(input: &str, num: usize) -> usize {
    let input = input.as_bytes();

    let mut i = num;

    loop {
        let next = &input[i-num..i];
        let (different, offset) = window_different_2(next);
        i+=offset+1;
        if different {
            break i -1 ;
        }
    }
}

fn window_different_2(sub : &[u8]) -> (bool, usize) {
    let len = sub.len();
    for i in 0..len{
        for j in 0..i {
            if sub[i..=i] == sub[j..=j] {

                return (false, j);
            }
        }
    }

    (true, 0)
}

pub fn dperez(input: &[u8], window: usize) -> Option<usize> {
    let mut idx = 0;
    'outer: while idx + window - 1 < input.len() {
        let mut state = 0;
        for (next_idx, byte) in input[idx..idx+window].iter().enumerate().rev() {
            let bit_idx = byte % 32;
            if state & (1 << bit_idx) != 0 {
                idx += next_idx + 1;
                continue 'outer;
            }
            state |= 1 << bit_idx;
        }
        return Some(idx);
    }
    return None
}


fn day_6_bench(c: &mut Criterion) {
    const SHORT_WINDOW: usize = 4;
    const LARGE_WINDOW: usize = 14;

    let input = include_str!("../inputs/day6.txt");
    let mut group = c.benchmark_group("Day 6 Solutions");
    group.confidence_level(0.99);

    group.bench_with_input(BenchmarkId::new("d0nut (simdeez)", 4), &4, |b, window| {
        b.iter(|| {
            simdeez::<4>(&input)
        })
    });

    group.bench_with_input(BenchmarkId::new("d0nut (gg2)", 14), &14, |b, window| {
        b.iter(|| {
            gg::<14>(&input)
        })
    });

    group.bench_with_input(BenchmarkId::new("dperez", 14), &14, |b, window| {
        b.iter(|| {
            dperez(&input.as_bytes(), *window)
        })
    });

    group.bench_with_input(BenchmarkId::new("resistcorp", 4), &4, |b, window| {
        b.iter(|| {
            resistcorp(&input, *window)
        })
    });

    for window in &[SHORT_WINDOW, LARGE_WINDOW] {
        group.bench_with_input(BenchmarkId::new("prime", window), window, |b, window| {
            b.iter(|| {
                primes_solution_day6(&input, *window)
            })
        });

        group.bench_with_input(BenchmarkId::new("benny", window), window, |b, window| {
            b.iter(|| {
                benny_solution(&input, *window)
            })
        });

        group.bench_with_input(BenchmarkId::new("d0nut", window), window, |b, window| {
            b.iter(|| {
                d0nuts_solution_day6(&input, *window)
            })
        });

        group.bench_with_input(BenchmarkId::new("d0nut (insane)", window), window, |b, window| {
            b.iter(|| {
                solve_insane(&input, *window)
            })
        });
    }
}


criterion_group!(benches, day_6_bench);
criterion_main!(benches);