use super::assert::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    let file = File::open("input/day1.txt").unwrap();
    let input: Vec<i32> = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().parse::<i32>().unwrap())
        .collect();

    'outer: for x in input.iter() {
        for y in input.iter() {
            let remainder = 2020 - x - y;

            if remainder < 0 {
                continue;
            }
            else if remainder == 0 {
                assert_eq(Day::new(1, Part::A), 646_779, x * y);
                continue;
            }

            for z in input.iter() {
                if *z == remainder {
                    assert_eq(Day::new(1, Part::B), 246_191_688, x * y * z);
                    break 'outer;
                }
            }
        }
    }
}
