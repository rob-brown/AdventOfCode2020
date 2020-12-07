use super::assert::*;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Point = (i32, i32);

fn trees_hit(trees: &HashSet<Point>, width: i32, height: i32, dx: i32, dy: i32) -> i64 {
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
    let mut trees: HashSet<Point> = HashSet::new();
    let mut width = 0;
    let mut height = 0;

    let file = File::open("input/day3.txt").unwrap();
    for (y, line) in BufReader::new(file).lines().enumerate() {
        height += 1;
        width = 0;

        for (x, char) in line.unwrap().chars().enumerate() {
            width += 1;

            if char == '#' {
                let point = (x as i32, y as i32);
                trees.insert(point);
            }
        }
    }

    let count1 = trees_hit(&trees, width, height, 1, 1);
    let count2 = trees_hit(&trees, width, height, 3, 1);
    let count3 = trees_hit(&trees, width, height, 5, 1);
    let count4 = trees_hit(&trees, width, height, 7, 1);
    let count5 = trees_hit(&trees, width, height, 1, 2);

    assert_eq(Day::new(3, Part::A), 178, count2);
    assert_eq(
        Day::new(3, Part::B),
        3_492_520_200,
        count1 * count2 * count3 * count4 * count5,
    );
}
