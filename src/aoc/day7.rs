use super::assert::*;
use parse_display::{Display as PDisplay, FromStr as PFromStr};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

type RcString = Rc<String>;

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
    bag: RcString,
}

fn bags_contained(current: &RcString, rules: &HashMap<RcString, Vec<Output>>) -> i32 {
    rules
        .get(current)
        .unwrap_or(&vec![])
        .iter()
        .map(|x| x.quantity * (1 + bags_contained(&x.bag, rules)))
        .sum()
}

fn part1(reverse_rules: &HashMap<RcString, HashSet<RcString>>) {
    let shiny_gold = Rc::new(String::from("shiny gold"));
    let mut all_entries: HashSet<RcString> = reverse_rules.get(&shiny_gold).unwrap().clone();
    let mut current_entries = all_entries.clone();
    let empty = HashSet::new();

    loop {
        let mut new_entries = HashSet::new();

        for entry in current_entries {
            let options = reverse_rules
                .get(&entry)
                .unwrap_or(&empty)
                .iter()
                .cloned();
            new_entries.extend(options);
        }

        if new_entries.is_empty() {
            break;
        } else {
            all_entries.extend(new_entries.iter().cloned());
            current_entries = new_entries;
        }
    }

    assert_eq(Day::new(7, Part::A), 169, all_entries.len());
}

fn part2(rules: &HashMap<RcString, Vec<Output>>) {
    let shiny_gold = Rc::new(String::from("shiny gold"));
    let count = bags_contained(&shiny_gold, rules);

    assert_eq(Day::new(7, Part::B), 82_372, count);
}

pub fn solve() {
    let file = File::open("input/day7.txt").unwrap();
    let mut rules: HashMap<RcString, Vec<Output>> = HashMap::new();
    let mut reverse_rules: HashMap<RcString, HashSet<RcString>> = HashMap::new();

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        let raw_input = line.parse::<RawInput>().unwrap();
        let input = Rc::new(raw_input.bag);
        let mut outputs = Vec::new();

        for text in raw_input.rest.split(", ") {
            let raw_output = text.parse::<RawOutput>().unwrap();

            if raw_output.quantity == "no" {
                continue;
            } else {
                let quantity = raw_output.quantity.parse::<i32>().unwrap();
                let bag = Rc::new(raw_output.bag);
                reverse_rules
                    .entry(bag.clone())
                    .or_insert(HashSet::new())
                    .insert(input.clone());
                let output = Output { quantity, bag };
                outputs.push(output);
            }
        }

        rules.insert(input, outputs);
    }

    part1(&reverse_rules);
    part2(&rules);
}
