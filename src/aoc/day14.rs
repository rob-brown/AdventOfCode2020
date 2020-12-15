use super::assert::*;
use parse_display::{Display as PDisplay, FromStr as PFromStr};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PDisplay, PFromStr, PartialEq, Debug)]
enum Instruction {
    #[display("mask = {mask}")]
    Mask { mask: String },
    #[display("mem[{index}] = {value}")]
    Mem { index: usize, value: usize },
}

fn apply_mask_v1(mask: &str, value: usize) -> usize {
    let mut result = value;

    for (n, c) in mask.chars().enumerate() {
        match c {
            '1' => {
                let shift = 35 - n;
                result |= 1 << shift;
            }
            '0' => {
                let shift = 35 - n;
                result &= !(1 << shift);
            }
            _ => (),
        }
    }

    result
}

fn apply_mask_v2(mask: &str, address: usize) -> Vec<usize> {
    let mut address = address;
    let mut variants = Vec::new();

    for (n, c) in mask.chars().enumerate() {
        let shift = 35 - n;

        match c {
            '1' => {
                address |= 1 << shift;
            }
            'X' => {
                variants.push(shift);
            }
            _ => (),
        }
    }

    let mut result = vec![address];

    for shift in variants {
        result = result
            .iter()
            .flat_map(|&x| vec![x & !(1 << shift), x | (1 << shift)])
            .collect();
    }

    result
}

pub fn solve() {
    let file = File::open("input/day14.txt").unwrap();
    let mut memory1: HashMap<usize, usize> = HashMap::new();
    let mut memory2: HashMap<usize, usize> = HashMap::new();
    let mut mask = String::from("");

    for line in BufReader::new(file).lines() {
        match line.unwrap().parse::<Instruction>().unwrap() {
            Instruction::Mask { mask: m } => {
                mask = String::from(m);
            }
            Instruction::Mem { index: i, value: v } => {
                let masked = apply_mask_v1(&mask, v);
                memory1.insert(i, masked);

                for address in apply_mask_v2(&mask, i) {
                    memory2.insert(address, v);
                }
            }
        }
    }

    let sum: usize = memory1.iter().map(|(_, v)| v).sum();
    assert_eq(Day::new(14, Part::A), 13_105_044_880_745, sum);

    let sum: usize = memory2.iter().map(|(_, v)| v).sum();
    assert_eq(Day::new(14, Part::B), 3_505_392_154_485, sum);
}
