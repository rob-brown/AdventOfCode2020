use super::assert::*;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Point3D = (i32, i32, i32);
type Vector3D = (i32, i32, i32);

type Point4D = (i32, i32, i32, i32);
type Vector4D = (i32, i32, i32, i32);

const VECTORS_3D: [Vector3D; 26] = [
    (-1, -1, -1),
    (-1, -1, 0),
    (-1, -1, 1),
    (-1, 0, -1),
    (-1, 0, 0),
    (-1, 0, 1),
    (-1, 1, -1),
    (-1, 1, 0),
    (-1, 1, 1),
    (0, -1, -1),
    (0, -1, 0),
    (0, -1, 1),
    (0, 0, -1),
    (0, 0, 1),
    (0, 1, -1),
    (0, 1, 0),
    (0, 1, 1),
    (1, -1, -1),
    (1, -1, 0),
    (1, -1, 1),
    (1, 0, -1),
    (1, 0, 0),
    (1, 0, 1),
    (1, 1, -1),
    (1, 1, 0),
    (1, 1, 1)
];

const VECTORS_4D: [Vector4D; 80] = [
    (-1, -1, -1, -1),
    (-1, -1, -1, 0),
    (-1, -1, -1, 1),
    (-1, -1, 0, -1),
    (-1, -1, 0, 0),
    (-1, -1, 0, 1),
    (-1, -1, 1, -1),
    (-1, -1, 1, 0),
    (-1, -1, 1, 1),
    (-1, 0, -1, -1),
    (-1, 0, -1, 0),
    (-1, 0, -1, 1),
    (-1, 0, 0, -1),
    (-1, 0, 0, 0),
    (-1, 0, 0, 1),
    (-1, 0, 1, -1),
    (-1, 0, 1, 0),
    (-1, 0, 1, 1),
    (-1, 1, -1, -1),
    (-1, 1, -1, 0),
    (-1, 1, -1, 1),
    (-1, 1, 0, -1),
    (-1, 1, 0, 0),
    (-1, 1, 0, 1),
    (-1, 1, 1, -1),
    (-1, 1, 1, 0),
    (-1, 1, 1, 1),
    (0, -1, -1, -1),
    (0, -1, -1, 0),
    (0, -1, -1, 1),
    (0, -1, 0, -1),
    (0, -1, 0, 0),
    (0, -1, 0, 1),
    (0, -1, 1, -1),
    (0, -1, 1, 0),
    (0, -1, 1, 1),
    (0, 0, -1, -1),
    (0, 0, -1, 0),
    (0, 0, -1, 1),
    (0, 0, 0, -1),
    (0, 0, 0, 1),
    (0, 0, 1, -1),
    (0, 0, 1, 0),
    (0, 0, 1, 1),
    (0, 1, -1, -1),
    (0, 1, -1, 0),
    (0, 1, -1, 1),
    (0, 1, 0, -1),
    (0, 1, 0, 0),
    (0, 1, 0, 1),
    (0, 1, 1, -1),
    (0, 1, 1, 0),
    (0, 1, 1, 1),
    (1, -1, -1, -1),
    (1, -1, -1, 0),
    (1, -1, -1, 1),
    (1, -1, 0, -1),
    (1, -1, 0, 0),
    (1, -1, 0, 1),
    (1, -1, 1, -1),
    (1, -1, 1, 0),
    (1, -1, 1, 1),
    (1, 0, -1, -1),
    (1, 0, -1, 0),
    (1, 0, -1, 1),
    (1, 0, 0, -1),
    (1, 0, 0, 0),
    (1, 0, 0, 1),
    (1, 0, 1, -1),
    (1, 0, 1, 0),
    (1, 0, 1, 1),
    (1, 1, -1, -1),
    (1, 1, -1, 0),
    (1, 1, -1, 1),
    (1, 1, 0, -1),
    (1, 1, 0, 0),
    (1, 1, 0, 1),
    (1, 1, 1, -1),
    (1, 1, 1, 0),
    (1, 1, 1, 1)
];

pub fn part1() {
    let file = File::open("input/day17.txt").unwrap();
    let mut active_nodes: HashSet<Point3D> = HashSet::new();

    for (y, line) in BufReader::new(file).lines().enumerate() {
        for (x, char) in line.unwrap().chars().enumerate() {
            if char == '#' {
                active_nodes.insert((x as i32, y as i32, 0));
            }
        }
    }

    let mut current = active_nodes;
    let mut next = HashSet::new();
    let mut inactive_nodes = HashSet::new();
    let mut cycle_count = 0;

    loop {

        for &point in current.iter() {
            let (x, y, z) = point;
            let mut neighbor_count = 0;

            for &(dx, dy, dz) in VECTORS_3D.iter() {
                let neighbor = (x + dx, y + dy, z + dz);

                if current.contains(&neighbor) {
                    neighbor_count += 1;
                } else {
                    inactive_nodes.insert(neighbor);
                }
            }

            if neighbor_count == 2 || neighbor_count == 3 {
                next.insert(point);
            }
        }

        for &point in inactive_nodes.iter() {
            let (x, y, z) = point;
            let mut neighbor_count = 0;

            for &(dx, dy, dz) in VECTORS_3D.iter() {
                let neighbor = (x + dx, y + dy, z + dz);

                if current.contains(&neighbor) {
                    neighbor_count += 1;
                }
            }

            if neighbor_count == 3 {
                next.insert(point);
            }
        }

        current = next.clone();
        next.clear();

        cycle_count += 1;

        if cycle_count == 6 {
            break;
        }
    }

    assert_eq(Day::new(17, Part::A), 263, current.len());
}

pub fn part2() {
    let file = File::open("input/day17.txt").unwrap();
    let mut active_nodes: HashSet<Point4D> = HashSet::new();

    for (y, line) in BufReader::new(file).lines().enumerate() {
        for (x, char) in line.unwrap().chars().enumerate() {
            if char == '#' {
                active_nodes.insert((x as i32, y as i32, 0, 0));
            }
        }
    }

    let mut current = active_nodes;
    let mut next = HashSet::new();
    let mut inactive_nodes = HashSet::new();
    let mut cycle_count = 0;

    loop {

        for &point in current.iter() {
            let (x, y, z, w) = point;
            let mut neighbor_count = 0;

            for &(dx, dy, dz, dw) in VECTORS_4D.iter() {
                let neighbor = (x + dx, y + dy, z + dz, w + dw);

                if current.contains(&neighbor) {
                    neighbor_count += 1;
                } else {
                    inactive_nodes.insert(neighbor);
                }
            }

            if neighbor_count == 2 || neighbor_count == 3 {
                next.insert(point);
            }
        }

        for &point in inactive_nodes.iter() {
            let (x, y, z, w) = point;
            let mut neighbor_count = 0;

            for &(dx, dy, dz, dw) in VECTORS_4D.iter() {
                let neighbor = (x + dx, y + dy, z + dz, w + dw);

                if current.contains(&neighbor) {
                    neighbor_count += 1;
                }
            }

            if neighbor_count == 3 {
                next.insert(point);
            }
        }

        current = next.clone();
        next.clear();

        cycle_count += 1;

        if cycle_count == 6 {
            break;
        }
    }

    assert_eq(Day::new(17, Part::B), 1680, current.len());
}

pub fn solve() {
    part1();
    part2();
}
