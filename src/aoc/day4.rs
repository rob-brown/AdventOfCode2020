use super::assert::*;
use parse_display::{Display as PDisplay, FromStr as PFromStr};
use std::fs::File;
use std::io::prelude::*;

#[derive(PDisplay, PFromStr, Debug)]
#[display("{key}:{value}")]
struct Pair {
    key: String,
    value: String,
}

fn validate_byr(value: &str) -> bool {
    let year = value.parse::<i32>().unwrap();
    year >= 1920 && year <= 2002
}

fn validate_iyr(value: &str) -> bool {
    let year = value.parse::<i32>().unwrap();
    year >= 2010 && year <= 2020
}

fn validate_eyr(value: &str) -> bool {
    let year = value.parse::<i32>().unwrap();
    year >= 2020 && year <= 2030
}

fn validate_hgt(value: &str) -> bool {
    let unit = &value[(value.len() - 2)..];

    match unit {
        "in" => {
            let magnitude = (&value[..(value.len() - 2)]).parse::<i32>().unwrap();
            magnitude >= 59 && magnitude <= 76
        }
        "cm" => {
            let magnitude = (&value[..(value.len() - 2)]).parse::<i32>().unwrap();
            magnitude >= 150 && magnitude <= 193
        }
        _ => false,
    }
}

fn validate_hcl(value: &str) -> bool {
    let prefix = &value[..1];
    let rest = &value[1..];

    prefix == "#" && rest.len() == 6
}

const EYE_COLORS: [&'static str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn validate_ecl(value: &str) -> bool {
    EYE_COLORS.contains(&value)
}

fn validate_pid(value: &str) -> bool {
    value.len() == 9
}

pub fn solve() {
    let mut file = File::open("input/day4.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut keys_valid = 0;
    let mut values_valid = 0;

    for passport in contents.split("\n\n") {
        let mut byr = false;
        let mut iyr = false;
        let mut eyr = false;
        let mut hgt = false;
        let mut hcl = false;
        let mut ecl = false;
        let mut pid = false;
        let mut passport_valid = true;

        for field in passport.split_whitespace() {
            let pair = field.parse::<Pair>().unwrap();

            match &*pair.key {
                "byr" => {
                    byr = true;
                    passport_valid &= validate_byr(&pair.value);
                }
                "iyr" => {
                    iyr = true;
                    passport_valid &= validate_iyr(&pair.value);
                }
                "eyr" => {
                    eyr = true;
                    passport_valid &= validate_eyr(&pair.value);
                }
                "hgt" => {
                    hgt = true;
                    passport_valid &= validate_hgt(&pair.value);
                }
                "hcl" => {
                    hcl = true;
                    passport_valid &= validate_hcl(&pair.value);
                }
                "ecl" => {
                    ecl = true;
                    passport_valid &= validate_ecl(&pair.value);
                }
                "pid" => {
                    pid = true;
                    passport_valid &= validate_pid(&pair.value);
                }
                _ => (),
            }
        }

        if byr && iyr && eyr && hgt && hcl && ecl && pid {
            keys_valid += 1;

            if passport_valid {
                values_valid += 1;
            }
        }
    }

    assert_eq(Day::new(4, Part::A), 235, keys_valid);
    assert_eq(Day::new(4, Part::B), 194, values_valid);
}
