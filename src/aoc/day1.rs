use super::assert::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    let file = File::open("input/day1.txt").unwrap();
    let mut input: Vec<i32> = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().parse::<i32>().unwrap())
        .collect();
    input.sort_by(|a, b| b.cmp(a));

    'outer: for i in 0..input.len() {
        let x = input[i];

        for j in (i + 1)..input.len() {
            let y = input[j];
            let remainder = 2020 - x - y;

            if remainder < 0 {
                continue;
            }
            else if remainder == 0 {
                assert_eq(Day::new(1, Part::A), 646_779, x * y);
                continue;
            }

            for k in (j + 1)..input.len() {
                let z = input[k];
                if z == remainder {
                    assert_eq(Day::new(1, Part::B), 246_191_688, x * y * z);
                    break 'outer;
                }
            }
        }
    }
}
