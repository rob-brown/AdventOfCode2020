use super::assert::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Point = (i32, i32);

#[derive(PartialEq, Eq, Debug, Clone)]
enum Occupancy {
    Floor,
    Empty,
    Filled,
}

fn occupancy(positions: &HashMap<Point, Occupancy>) -> usize {
    positions
        .iter()
        .filter(|(_, v)| **v == Occupancy::Filled)
        .count()
}

fn too_many_neighbors(positions: &HashMap<Point, Occupancy>, point: Point, limit: usize) -> bool {
    let mut count = 0;
    let (x, y) = point;

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dy == 0 && dx == 0 {
                continue;
            }

            let p = (x + dx, y + dy);

            if let Some(state) = positions.get(&p) {
                if *state == Occupancy::Filled {
                    count += 1;

                    if count >= limit {
                        return true;
                    }
                }
            }
        }
    }

    false
}

fn too_many_extended_neighbors(
    positions: &HashMap<Point, Occupancy>,
    point: Point,
    limit: usize,
) -> bool {
    let mut count = 0;

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dy == 0 && dx == 0 {
                continue;
            }

            let mut p = point;

            loop {
                p = (p.0 + dx, p.1 + dy);
                match positions.get(&p) {
                    Some(Occupancy::Floor) => continue,
                    Some(Occupancy::Filled) => {
                        count += 1;

                        if count >= limit {
                            return true;
                        }
                    }
                    Some(Occupancy::Empty) => (),
                    None => (),
                }

                break;
            }
        }
    }

    false
}

pub fn solve() {
    let file = File::open("input/day11.txt").unwrap();
    let mut positions: HashMap<Point, Occupancy> = HashMap::new();

    for (y, line) in BufReader::new(file).lines().enumerate() {
        for (x, char) in line.unwrap().chars().enumerate() {
            let point = (x as i32, y as i32);
            match char {
                '.' => {
                    positions.insert(point, Occupancy::Floor);
                }
                'L' => {
                    positions.insert(point, Occupancy::Empty);
                }
                _ => (),
            }
        }
    }

    let positions = positions;
    let mut previous_positions = positions.clone();

    loop {
        let mut new_positions = previous_positions.clone();

        let mut toggle_count = 0;
        for (&point, state) in previous_positions.iter() {
            match state {
                Occupancy::Floor => (),
                Occupancy::Empty => {
                    if !too_many_neighbors(&previous_positions, point, 1) {
                        toggle_count += 1;
                        new_positions.insert(point, Occupancy::Filled);
                    }
                }
                Occupancy::Filled => {
                    if too_many_neighbors(&previous_positions, point, 4) {
                        toggle_count += 1;
                        new_positions.insert(point, Occupancy::Empty);
                    }
                }
            }
        }

        if toggle_count == 0 {
            break;
        }

        previous_positions = new_positions;
    }

    assert_eq(Day::new(11, Part::A), 2113, occupancy(&previous_positions));

    let mut previous_positions = positions.clone();

    loop {
        let mut new_positions = previous_positions.clone();

        let mut toggle_count = 0;
        for (&point, state) in previous_positions.iter() {
            match state {
                Occupancy::Floor => (),
                Occupancy::Empty => {
                    if !too_many_extended_neighbors(&previous_positions, point, 1) {
                        toggle_count += 1;
                        new_positions.insert(point, Occupancy::Filled);
                    }
                }
                Occupancy::Filled => {
                    if too_many_extended_neighbors(&previous_positions, point, 5) {
                        toggle_count += 1;
                        new_positions.insert(point, Occupancy::Empty);
                    }
                }
            }
        }

        if toggle_count == 0 {
            break;
        }

        previous_positions = new_positions;
    }

    assert_eq(Day::new(11, Part::B), 1865, occupancy(&previous_positions));
}
