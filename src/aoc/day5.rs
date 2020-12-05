use super::assert::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn midpoint(lo: usize, hi: usize) -> usize {
	(hi - lo) / 2 + lo
}

fn find_seat(line: &str) -> usize {
    let mut row_lo = 0;
    let mut row_hi = 127;
    let mut col_lo = 0;
    let mut col_hi = 7;

    for char in line.chars() {
        match char {
            'F' => row_hi = midpoint(row_lo, row_hi),
            'B' => row_lo = midpoint(row_lo, row_hi),
            'L' => col_hi = midpoint(col_lo, col_hi),
            'R' => col_lo = midpoint(col_lo, col_hi),
            _ => (),
        }
    }

    row_hi * 8 + col_hi
}

pub fn solve() {
    let mut seats: [u8; 1024] = [0; 1024];
    let mut max_id = 0;
    let file = File::open("input/day5.txt").unwrap();

    for line in BufReader::new(file).lines() {
        let id = find_seat(&line.unwrap().clone());
        seats[id] = 1;
        max_id = max_id.max(id);
    }

    let mut seat_id = max_id - 1;

    loop {
        if seats[seat_id] == 0 {
            break;
        }

        seat_id -= 1;
    }

    assert_eq(Day::new(5, Part::A), 885, max_id);
    assert_eq(Day::new(5, Part::B), 623, seat_id);
}
