use std::collections::HashMap;
use std::fmt;
use std::io::prelude::*;
use std::io::{self};

extern crate aoc16;

enum Cardinal {
    North,
    East,
    South,
    West,
}

impl fmt::Display for Cardinal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = match *self {
            Cardinal::North => "North",
            Cardinal::East  => "East",
            Cardinal::South => "South",
            Cardinal::West  => "West",
        };
        write!(f, "{}", out)
    }
}

impl Cardinal {

    /* can't figure out how to cast an enum to int increment it and cast back
       maybe not possible in rust? */

    fn turn_left(&self) -> Cardinal {
        return match *self {
            Cardinal::West  => Cardinal::South,
            Cardinal::South => Cardinal::East,
            Cardinal::East  => Cardinal::North,
            Cardinal::North => Cardinal::West,
        }
    }

    fn turn_right(&self) -> Cardinal {
        return match *self {
            Cardinal::North => Cardinal::East,
            Cardinal::East  => Cardinal::South,
            Cardinal::South => Cardinal::West,
            Cardinal::West  => Cardinal::North,
        }
    }

}

fn main() {
    let mut input = String::new();

    let _ = io::stdin().read_to_string(&mut input);

    let start_pos = aoc16::Point {x: 0, y: 0};
    let mut facing = Cardinal::North;
    let mut pos = aoc16::Point {x: 0, y: 0};
    let mut double_visit: Option<aoc16::Point> = None;
    let mut visited = HashMap::new();

    for instruction in input.split(",") {
        facing = match instruction.trim().chars().nth(0).unwrap() {
            'R' => facing.turn_right(),
            'L' => facing.turn_left(),
            _ => panic!("Invalid instruction encountered."),
        };
        let steps = instruction.trim()[1..].parse::<i32>().unwrap();
        for _ in 0..steps {
            match facing {
                Cardinal::North => pos.y += 1,
                Cardinal::East  => pos.x += 1,
                Cardinal::South => pos.y -= 1,
                Cardinal::West  => pos.x -= 1,
            }
            if visited.contains_key(&pos) && double_visit == None {
                double_visit = Some(pos);
            }
            visited.insert(pos, 1);
        }
    }

    println!("Distance to last pos: {}", pos.manhattan_distance(start_pos));
    if double_visit.is_some() {
        println!("Distance to first pos visited twice: {}", double_visit.unwrap().manhattan_distance(start_pos));
    }

}
