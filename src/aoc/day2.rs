use super::assert::*;
use parse_display::{Display as PDisplay, FromStr as PFromStr};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PDisplay, PFromStr, Debug)]
#[display("{min}-{max} {char}: {password}")]
struct PasswordRule {
    min: usize,
    max: usize,
    char: char,
    password: String,
}

pub fn solve() {
    let file = File::open("input/day2.txt").unwrap();

    let mut first_rule_count = 0;
    let mut second_rule_count = 0;

    for line in BufReader::new(file).lines() {
        let rule = line.unwrap().parse::<PasswordRule>().unwrap();

        // Check first password rule.
        let matches = rule.password.chars().filter(|x| *x == rule.char).count();

        if matches >= rule.min && matches <= rule.max {
            first_rule_count += 1;
        }

        // Check second password rule.
        let first = rule.password.chars().nth(rule.min - 1);
        let second = rule.password.chars().nth(rule.max - 1);

        if (first == Some(rule.char)) ^ (second == Some(rule.char)) {
            second_rule_count += 1;
        }
    }

    assert_eq(Day::new(2, Part::A), 569, first_rule_count);
    assert_eq(Day::new(2, Part::B), 346, second_rule_count);
}
