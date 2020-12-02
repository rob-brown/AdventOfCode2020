use super::assert::*;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

pub fn solve() {
    let regex = Regex::new(r"(\d{1,2})-(\d{1,2}) (\w): (\w+)\n").unwrap();
    let mut file = File::open("input/day2.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut first_rule_count = 0;
    let mut second_rule_count = 0;

    for capture in regex.captures_iter(&contents) {
        let min = capture[1].parse::<usize>().unwrap();
        let max = capture[2].parse::<usize>().unwrap();
        let char = capture[3].parse::<String>().unwrap();
        let password = capture[4].parse::<String>().unwrap();

        // Check first password rule.
        let matches = password.matches(&char).count();

        if matches >= min && matches <= max {
            first_rule_count += 1;
        }

        // Check second password rule.
        let first: String = password.chars().skip(min - 1).take(1).collect();
        let second: String = password.chars().skip(max - 1).take(1).collect();

        if first == char && second != char {
            second_rule_count += 1;
        } else if first != char && second == char {
            second_rule_count += 1;
        }
    }

    assert_eq(Day::new(2, Part::A), 569, first_rule_count);
    assert_eq(Day::new(2, Part::B), 346, second_rule_count);
}
