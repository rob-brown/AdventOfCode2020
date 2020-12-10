use super::assert::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    let file = File::open("input/day10.txt").unwrap();
    let mut all_numbers = Vec::new();

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        let n = line.parse::<i32>().unwrap();
        all_numbers.push(n);
    }

    all_numbers.sort();

    let mut previous = 0;
    let mut ones = 0;
    let mut threes = 1;
    let mut run = 0;
    let mut combos: usize = 1;

    for &n in all_numbers.iter() {
        let difference = n - previous;

        match difference {
            1 => {
                ones += 1;
                run += 1;
            }
            3 => {
                threes += 1;

                match run {
                    2 => combos *= 2,
                    3 => combos *= 4,
                    4 => combos *= 7,
                    _ => (),
                }

                run = 0;
            }
            _ => (),
        }

        previous = n;
    }

    // Catch the last run.
    match run {
        2 => combos *= 2,
        3 => combos *= 4,
        4 => combos *= 7,
        _ => (),
    }

    assert_eq(Day::new(10, Part::A), 2574, ones * threes);
    assert_eq(Day::new(10, Part::A), 2_644_613_988_352, combos);
}
