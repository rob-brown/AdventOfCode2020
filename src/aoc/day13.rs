use super::assert::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

// From https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

pub fn solve() {
    let file = File::open("input/day13.txt").unwrap();
    let lines = BufReader::new(file).lines().collect::<Vec<_>>();

    let arrival = lines[0].as_ref().unwrap().parse::<i64>().unwrap();
    let busses: Vec<(i64, i64)> = lines[1]
        .as_ref()
        .unwrap()
        .split(",")
        .enumerate()
        .filter(|&(_, x)| x != "x")
        .map(|(n, x)| (n as i64, x.parse::<i64>().unwrap()))
        .collect();

    let (bus, wait) = busses
        .iter()
        .map(|&(_, b)| (b, b - arrival % b))
        .min_by_key(|&(_, w)| w)
        .unwrap();

    assert_eq(Day::new(13, Part::A), 102, bus * wait);

    let modulii = busses.iter().map(|&(_, b)| b).collect::<Vec<_>>();
    let residues = busses.iter().map(|&(i, b)| b - i).collect::<Vec<_>>();
    let t = chinese_remainder(&residues.to_owned(), &modulii).unwrap();

    assert_eq(Day::new(13, Part::B), 327_300_950_120_029, t);
}
