use super::assert::*;
use parse_display::FromStr as PFromStr;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Point = (i32, i32);

#[derive(Debug, Clone, Copy)]
enum Heading {
    North,
    South,
    East,
    West,
}

#[derive(PFromStr, PartialEq, Debug)]
#[from_str(regex = "(?P<instruction>\\w)(?P<magnitude>\\d+)")]
struct Instruction {
    instruction: String,
    magnitude: i32,
}

fn new_heading(instruction: &Instruction, heading: Heading) -> (Heading, Option<Heading>) {
    match (&*instruction.instruction, heading, instruction.magnitude) {
        ("N", _, _) => (heading, Some(Heading::North)),
        ("S", _, _) => (heading, Some(Heading::South)),
        ("E", _, _) => (heading, Some(Heading::East)),
        ("W", _, _) => (heading, Some(Heading::West)),
        ("L", Heading::North, 90) => (Heading::West, None),
        ("L", Heading::North, 180) => (Heading::South, None),
        ("L", Heading::North, 270) => (Heading::East, None),
        ("L", Heading::South, 90) => (Heading::East, None),
        ("L", Heading::South, 180) => (Heading::North, None),
        ("L", Heading::South, 270) => (Heading::West, None),
        ("L", Heading::East, 90) => (Heading::North, None),
        ("L", Heading::East, 180) => (Heading::West, None),
        ("L", Heading::East, 270) => (Heading::South, None),
        ("L", Heading::West, 90) => (Heading::South, None),
        ("L", Heading::West, 180) => (Heading::East, None),
        ("L", Heading::West, 270) => (Heading::North, None),
        ("R", Heading::North, 90) => (Heading::East, None),
        ("R", Heading::North, 180) => (Heading::South, None),
        ("R", Heading::North, 270) => (Heading::West, None),
        ("R", Heading::South, 90) => (Heading::West, None),
        ("R", Heading::South, 180) => (Heading::North, None),
        ("R", Heading::South, 270) => (Heading::East, None),
        ("R", Heading::East, 90) => (Heading::South, None),
        ("R", Heading::East, 180) => (Heading::West, None),
        ("R", Heading::East, 270) => (Heading::North, None),
        ("R", Heading::West, 90) => (Heading::North, None),
        ("R", Heading::West, 180) => (Heading::East, None),
        ("R", Heading::West, 270) => (Heading::South, None),
        ("F", heading, _) => (heading, Some(heading)),
        _ => panic!(),
    }
}

fn travel(position: Point, heading: Heading, magnitude: i32) -> Point {
    match heading {
        Heading::North => (position.0, position.1 + magnitude),
        Heading::South => (position.0, position.1 - magnitude),
        Heading::East => (position.0 + magnitude, position.1),
        Heading::West => (position.0 - magnitude, position.1),
    }
}

fn travel_with_waypoint(
    instruction: &Instruction,
    position: Point,
    waypoint: Point,
) -> (Point, Point) {
    match (&*instruction.instruction, instruction.magnitude) {
        ("N", magnitude) => (position, (waypoint.0, waypoint.1 + magnitude)),
        ("S", magnitude) => (position, (waypoint.0, waypoint.1 - magnitude)),
        ("E", magnitude) => (position, (waypoint.0 + magnitude, waypoint.1)),
        ("W", magnitude) => (position, (waypoint.0 - magnitude, waypoint.1)),
        ("L", 90) | ("R", 270) => (position, (-waypoint.1, waypoint.0)),
        (_, 180) => (position, (-waypoint.0, -waypoint.1)),
        ("L", 270) | ("R", 90) => (position, (waypoint.1, -waypoint.0)),
        ("F", magnitude) => {
            let new_position = (
                position.0 + magnitude * waypoint.0,
                position.1 + magnitude * waypoint.1,
            );
            (new_position, waypoint)
        }
        _ => panic!(),
    }
}

fn part1(instructions: &[Instruction]) {
    let mut heading = Heading::East;
    let mut position = (0, 0);

    for i in instructions.iter() {
        let (new_heading, direction) = new_heading(&i, heading);
        heading = new_heading;
        if let Some(direction) = direction {
            position = travel(position, direction, i.magnitude);
        }
    }

    assert_eq(
        Day::new(12, Part::A),
        1601,
        position.0.abs() + position.1.abs(),
    );
}

fn part2(instructions: &[Instruction]) {
    let mut position = (0, 0);
    let mut waypoint = (10, 1);

    for i in instructions.iter() {
        let (new_position, new_waypoint) = travel_with_waypoint(&i, position, waypoint);
        position = new_position;
        waypoint = new_waypoint;
    }

    assert_eq(
        Day::new(12, Part::B),
        13_340,
        position.0.abs() + position.1.abs(),
    );
}

pub fn solve() {
    let file = File::open("input/day12.txt").unwrap();
    let mut instructions = Vec::new();

    for line in BufReader::new(file).lines() {
        let instruction = line.unwrap().parse::<Instruction>().unwrap();
        instructions.push(instruction);
    }

    part1(&instructions);
    part2(&instructions);
}
