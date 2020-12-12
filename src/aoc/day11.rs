use super::assert::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Point = (i32, i32);
type Vector = (i32, i32);
type Index = usize;

const VECTORS: [Vector; 8] = [
    (-1, -1),
    (-1, 0),
    (0, -1),
    (1, -1),
    (-1, 1),
    (1, 0),
    (0, 1),
    (1, 1),
];

const WIDTH: usize = 91;
const HEIGHT: usize = 90;
const SIZE: usize = WIDTH * HEIGHT;

struct Seat {
    point: Point,
    index: Index,
    neighbors: Vec<Index>,
    extended_neighbors: Vec<Index>,
}

impl Seat {
    fn new(point: Point) -> Self {
        let index = Seat::index(point.0 as usize, point.1 as usize);
        Seat {
            point,
            index,
            neighbors: Vec::new(),
            extended_neighbors: Vec::new(),
        }
    }

    fn index(x: usize, y: usize) -> Index {
        y * WIDTH + x
    }

    fn is_point_valid(x: i32, y: i32) -> bool {
        x >= 0 && x < WIDTH as i32 && y >= 0 && y < HEIGHT as i32
    }

    fn add_neighbors(&mut self, spaces: &[Occupancy]) {
        let mut neighbors = Vec::new();

        for &(dx, dy) in VECTORS.iter() {
            let x = self.point.0 + dx;
            let y = self.point.1 + dy;

            if !Seat::is_point_valid(x, y) {
                continue;
            }

            let index = Seat::index(x as usize, y as usize);

            if spaces[index] == Occupancy::Filled {
                neighbors.push(index);
            }
        }

        // If only a few neighbors, the seat will never empty.
        if neighbors.len() >= 4 {
            self.neighbors = neighbors;
        }
    }

    fn add_extended_neighbors(&mut self, spaces: &[Occupancy]) {
        let mut neighbors = Vec::new();

        for &(dx, dy) in VECTORS.iter() {
            let (mut x, mut y) = self.point;

            loop {
                x += dx;
                y += dy;

                if !Seat::is_point_valid(x, y) {
                    break;
                }

                let index = Seat::index(x as usize, y as usize);

                if spaces[index] == Occupancy::Filled {
                    neighbors.push(index);
                    break;
                }
            }
        }

        // If only a few neighbors, the seat will never empty.
        if neighbors.len() >= 5 {
            self.extended_neighbors = neighbors;
        }
    }

    fn neighbor_count(&self, spaces: &[Occupancy]) -> usize {
        self.neighbors
            .iter()
            .filter(|i| spaces[**i] == Occupancy::Filled)
            .count()
    }

    fn extended_neighbor_count(&self, spaces: &[Occupancy]) -> usize {
        self.extended_neighbors
            .iter()
            .filter(|i| spaces[**i] == Occupancy::Filled)
            .count()
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Occupancy {
    Floor,
    Empty,
    Filled,
}

#[allow(dead_code)]
fn print_grid(grid: &[Occupancy]) {
    for (n, x) in grid.iter().enumerate() {
        match x {
            Occupancy::Floor => print!("."),
            Occupancy::Empty => print!("L"),
            Occupancy::Filled => print!("#"),
        }

        if (n + 1) % WIDTH == 0 {
            println!();
        }
    }
    println!("=======");
}

fn simulate<F>(seats: &[Seat], spaces: &[Occupancy; SIZE], limit: usize, fun: F) -> usize
where
    F: Fn(&Seat, &[Occupancy]) -> usize,
{
    let mut current = *spaces;
    let mut next = *spaces;

    loop {
        // print_grid(&current);
        let mut toggled = 0;

        for seat in seats.iter() {
            match current[seat.index] {
                Occupancy::Empty => {
                    if fun(&seat, &current) == 0 {
                        toggled += 1;
                        next[seat.index] = Occupancy::Filled;
                    } else {
                        next[seat.index] = Occupancy::Empty;
                    }
                }
                Occupancy::Filled => {
                    if fun(&seat, &current) >= limit {
                        toggled += 1;
                        next[seat.index] = Occupancy::Empty;
                    } else {
                        next[seat.index] = Occupancy::Filled;
                    }
                }
                Occupancy::Floor => (),
            }
        }

        if toggled == 0 {
            break;
        }

        std::mem::swap(&mut current, &mut next);
    }

    seats
        .iter()
        .filter(|s| current[s.index] == Occupancy::Filled)
        .count()
}

pub fn solve() {
    let file = File::open("input/day11.txt").unwrap();
    let mut spaces: [Occupancy; SIZE] = [Occupancy::Floor; SIZE];
    let mut seats = Vec::new();

    for (y, line) in BufReader::new(file).lines().enumerate() {
        for (x, char) in line.unwrap().chars().enumerate() {
            if char == 'L' {
                let point = (x as i32, y as i32);
                let index = Seat::index(x, y);
                // Mark as filled. Saves one iteration and some neighbor checks.
                spaces[index] = Occupancy::Filled;
                seats.push(Seat::new(point));
            }
        }
    }

    for seat in seats.iter_mut() {
        seat.add_neighbors(&spaces);
        seat.add_extended_neighbors(&spaces);
    }

    let count = simulate(&seats, &spaces, 4, Seat::neighbor_count);
    assert_eq(Day::new(11, Part::A), 2113, count);

    let count = simulate(&seats, &spaces, 5, Seat::extended_neighbor_count);
    assert_eq(Day::new(11, Part::B), 1865, count);
}
