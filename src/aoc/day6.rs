use super::assert::*;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

pub fn solve() {
    let mut file = File::open("input/day6.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut any_sum = 0;
    let mut all_sum = 0;

    for group in contents.split("\n\n") {
        let mut map = HashMap::new();
        let mut group_size = 0;

        for entry in group.split_whitespace() {

            for c in entry.chars() {
                let count = map.entry(c).or_insert(0);
                *count += 1;
            }

            group_size += 1;
        }

        let all_count = map
            .iter()
            .filter(|(_, c)| **c == group_size)
            .count();
        all_sum += all_count;
        any_sum += map.len();
    }

    assert_eq(Day::new(6, Part::A), 6335, any_sum);
    assert_eq(Day::new(6, Part::B), 3392, all_sum);
}
