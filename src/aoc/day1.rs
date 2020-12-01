use super::assert::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    let mut input = Vec::new();

    let file = File::open("input/day1.txt").unwrap();
    for line in BufReader::new(file).lines() {
        let number = String::from(line.unwrap()).parse::<i32>().unwrap();
        input.push(number)
    }

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
