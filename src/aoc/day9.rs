use super::assert::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::VecDeque;
use std::cmp::Ordering;

fn can_sum(numbers: &VecDeque<usize>, target: usize) -> bool {
    for x in numbers.iter() {
        for y in numbers.iter() {
            if x == y {
                continue;
            } else if x + y == target {
                return true
            }
        }
    }

    false
}

fn find_weakness(numbers: &[usize], target: usize) -> usize {
    'outer: for i in 0..numbers.len() {
        let mut sum = numbers[i];
        let mut min = sum;
        let mut max = sum;

        for &n in numbers.iter().skip(i + 1) {
            sum += n;
            min = min.min(n);
            max = max.max(n);

            match sum.cmp(&target) {
                Ordering::Equal => return min + max,
                Ordering::Greater => continue 'outer,
                Ordering::Less => (),
            }
        }
    }

    panic!("No weakness found!");
}

pub fn solve() {
    let file = File::open("input/day9.txt").unwrap();
    let mut numbers = VecDeque::new();
    let mut all_numbers = Vec::new();

    for line in BufReader::new(file).lines() {
        let number = line.unwrap().parse::<usize>().unwrap();
        all_numbers.push(number);

        if numbers.len() == 25 {
            if !can_sum(&numbers, number) {
                assert_eq(Day::new(9, Part::A), 542529149, number);
                let weakness = find_weakness(&all_numbers, number);
                assert_eq(Day::new(9, Part::B), 75678618, weakness);
                break;
            }
            numbers.pop_front();
        }

        numbers.push_back(number);
    }
}
