use super::assert::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

fn trees_hit(trees: &HashSet<(i32, i32)>, dx: i32, dy: i32) -> i64 {
    let width = 31;
    let height = 323;

    let mut x = 0;
    let mut y = 0;
    let mut tree_count = 0;

    while y <= height {
        if trees.contains(&(x, y)) {
            tree_count += 1;
        }

        x = (x + dx) % width;
        y = y + dy;
    }

    tree_count
}

pub fn solve() {

    let mut trees: HashSet<(i32, i32)> = HashSet::new();

    let file = File::open("input/day3.txt").unwrap();
    for (y, line) in BufReader::new(file).lines().enumerate() {
        for (x, char) in String::from(line.unwrap()).chars().enumerate() {
            if char == '#' {
                let point = (x as i32, y as i32);
                trees.insert(point);
            }
        }
    }

    let count1 = trees_hit(&trees, 1, 1);
    let count2 = trees_hit(&trees, 3, 1);
    let count3 = trees_hit(&trees, 5, 1);
    let count4 = trees_hit(&trees, 7, 1);
    let count5 = trees_hit(&trees, 1, 2);

    assert_eq(Day::new(3, Part::A), 178, count2);
    assert_eq(Day::new(3, Part::B), 3_492_520_200, count1 * count2 * count3 * count4 * count5);
}
