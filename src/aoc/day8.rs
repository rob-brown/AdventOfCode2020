use super::assert::*;
use parse_display::FromStr as PFromStr;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
enum OpCode {
    Nop,
    Acc,
    Jmp,
}

impl OpCode {
    fn parse(string: &str) -> Option<OpCode> {
        match string {
            "nop" => Some(OpCode::Nop),
            "acc" => Some(OpCode::Acc),
            "jmp" => Some(OpCode::Jmp),
            _ => None,
        }
    }
}

#[derive(PFromStr, PartialEq, Debug)]
#[from_str(regex = "(?P<instruction>.*) (?P<sign>\\+|-)(?P<value>\\d+)")]
struct RawOperation {
    instruction: String,
    sign: String,
    value: i32,
}

#[derive(Debug, Clone)]
struct Operation {
    instruction: OpCode,
    value: i32,
}

struct Machine {
    // Program counter
    pc: usize,
    // Accumulator
    acc: i32,
    ops: Vec<Operation>,
}

impl Machine {
    fn new(ops: Vec<Operation>) -> Self {
        Machine { pc: 0, acc: 0, ops }
    }

    fn step(&mut self) {
        let op = &self.ops[self.pc];
        self.pc += 1;

        match op.instruction {
            OpCode::Acc => {
                self.acc += op.value;
            }
            OpCode::Jmp => {
                self.pc += op.value as usize - 1;
            }
            OpCode::Nop => (),
        }
    }

    fn is_terminated(&self) -> bool {
        self.pc >= self.ops.len()
    }
}

fn part1(ops: &[Operation]) {
    // Run until any instruction repeated.
    let mut instructions_run = HashSet::new();
    let mut machine = Machine::new(ops.to_owned());

    loop {
        if instructions_run.contains(&machine.pc) {
            assert_eq(Day::new(8, Part::A), 1928, machine.acc);
            break;
        }

        instructions_run.insert(machine.pc);
        machine.step();
    }
}

fn part2(ops: &[Operation]) {
    // Modify a single instruction until the corrupted one is found.
    'outer: for n in 0..ops.len() {
        let mut modified_ops = ops.to_owned();
        let op = modified_ops.get_mut(n).unwrap();

        match op.instruction {
            OpCode::Acc => continue,
            OpCode::Jmp => op.instruction = OpCode::Nop,
            OpCode::Nop => op.instruction = OpCode::Jmp,
        }

        let mut machine = Machine::new(modified_ops);
        let mut instructions_run = HashSet::new();

        loop {
            if instructions_run.contains(&machine.pc) {
                // Failure! A loop was detected.
                continue 'outer;
            }

            if machine.is_terminated() {
                // Success!
                break;
            }

            instructions_run.insert(machine.pc);
            machine.step();
        }

        assert_eq(Day::new(8, Part::B), 1319, machine.acc);
        break;
    }
}

pub fn solve() {
    let file = File::open("input/day8.txt").unwrap();
    let mut ops = Vec::new();

    for line in BufReader::new(file).lines() {
        let raw = line.unwrap().parse::<RawOperation>().unwrap();

        let mut op = Operation {
            instruction: OpCode::parse(&raw.instruction).unwrap(),
            value: raw.value,
        };

        if raw.sign == "-" {
            op.value *= -1;
        }
        ops.push(op);
    }

    part1(&ops);
    part2(&ops);
}
