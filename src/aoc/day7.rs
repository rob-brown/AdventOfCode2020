use super::assert::*;
use parse_display::{Display as PDisplay, FromStr as PFromStr};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PDisplay, PFromStr, Debug)]
#[display("{bag} bags contain {rest}.")]
struct RawInput {
    bag: String,
    rest: String,
}

#[derive(PFromStr, PartialEq, Debug)]
#[from_str(regex = "(?P<quantity>\\d+|no) (?P<bag>.*) bags?")]
struct RawOutput {
    quantity: String,
    bag: String,
}

#[derive(Clone, Debug)]
struct Output {
    quantity: i32,
    bag: String,
}

#[derive(Clone, Debug)]
struct Rule {
    input: String,
    outputs: Vec<Output>,
}

pub fn solve() {
    let file = File::open("input/day7.txt").unwrap();
    let mut rules = Vec::new();
    let mut map: HashMap<String, HashSet<String>> = HashMap::new();

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        let input = line.parse::<RawInput>().unwrap();
        let mut outputs = Vec::new();

        for o in input.rest.split(", ") {
            let raw = o.parse::<RawOutput>().unwrap();

            if raw.quantity == "no" {
                continue;
            } else {
                let quantity = raw.quantity.parse::<i32>().unwrap();
                let bag = raw.bag;
                let entry = map.entry(bag.clone()).or_insert(HashSet::new());
                entry.insert(input.bag.clone());
                let output = Output { quantity, bag };
                outputs.push(output);
            }
        }

        let rule = Rule {
            input: input.bag,
            outputs,
        };
        rules.push(rule);
    }

    let mut all_entries = map.get("shiny gold").unwrap().clone();
    let mut current_entries = all_entries.clone();

    loop {
        let mut new_entries = HashSet::new();

        for e in current_entries {
            if let Some(options) = map.get(&e) {
                for o in options {
                    if !all_entries.contains(o) {
                        new_entries.insert(o.clone());
                        all_entries.insert(o.clone());
                    }
                }
            }
        }

        if new_entries.is_empty() {
            break;
        } else {
            current_entries = new_entries;
        }
    }

    assert_eq(Day::new(7, Part::A), 169, all_entries.len());
}
