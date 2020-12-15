use super::assert::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    let file = File::open("input/day15.txt").unwrap();

    let lines: Vec<String> = BufReader::new(file).lines().map(|x| x.unwrap()).collect();
    let mut turn = 1;
    let mut previous = None;
    let mut history = HashMap::new();

    for c in lines[0].split(",") {
        if let Some(n) = previous {
            history.insert(n, turn - 1);
        }

        let n = c.parse::<i32>().unwrap();

        previous = Some(n);
        turn += 1;
    }

    let mut previous = previous.unwrap();

    loop {
        let next;

        if let Some(n) = history.get(&previous) {
            next = turn - n - 1;
        } else {
            next = 0;
        }

        history.insert(previous, turn - 1);
        previous = next;

        if turn == 2020 {
            assert_eq(Day::new(15, Part::A), 289, previous);
        } else if turn == 30_000_000 {
            assert_eq(Day::new(15, Part::B), 1_505_722, previous);
            break;
        }

        turn += 1;
    }
}
