use super::assert::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::{Reverse, Ordering};

pub fn solve() {
    let file = File::open("input/day1.txt").unwrap();
    let mut input: Vec<i32> = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().parse::<i32>().unwrap())
        .collect();
    input.sort_by_key(|&b| Reverse(b));

    'outer: for i in 0..input.len() {
        let x = input[i];

        for j in (i + 1)..input.len() {
            let y = input[j];
            let remainder = 2020 - x - y;

            match remainder.cmp(&0) {
                Ordering::Equal => {
                    assert_eq(Day::new(1, Part::A), 646_779, x * y);
                    continue;
                }
                Ordering::Less => continue,
                Ordering::Greater => (),
            }

            for z in input.iter().skip(j + 1) {
                if *z == remainder {
                    assert_eq(Day::new(1, Part::B), 246_191_688, x * y * z);
                    break 'outer;
                }
            }
        }
    }
}
