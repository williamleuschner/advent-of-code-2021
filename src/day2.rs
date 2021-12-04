use crate::{Part1Solution, Part2Solution};
use std::io::BufRead;

pub struct Solution {}

enum Motion {
    Up(i32),
    Down(i32),
    Forward(i32),
}

struct Submarine {
    hpos: i32,
    vpos: i32,
    aim: i32,
}

impl Submarine {
    fn navigate_part2(&mut self, motions: Vec<Motion>) {
        for motion in motions {
            match motion {
                Motion::Up(distance) => self.aim -= distance,
                Motion::Down(distance) => self.aim += distance,
                Motion::Forward(distance) => {
                    self.hpos += distance;
                    self.vpos += self.aim * distance;
                }
            }
        }
    }

    fn navigate_part1(&mut self, motions: Vec<Motion>) {
        for motion in motions {
            match motion {
                Motion::Up(distance) => self.vpos -= distance,
                Motion::Down(distance) => self.vpos += distance,
                Motion::Forward(distance) => self.hpos += distance,
            }
        }
    }
}

fn make_motion(direction: &str, distance: i32) -> Option<Motion> {
    match direction {
        "forward" => Some(Motion::Forward(distance)),
        "up" => Some(Motion::Up(distance)),
        "down" => Some(Motion::Down(distance)),
        _ => None,
    }
}

fn compute_motions_from_reader(reader: &mut dyn BufRead) -> Vec<Motion> {
    let lines = reader.lines();
    let mut motions = Vec::with_capacity(100);
    for maybe_line in lines {
        let line = maybe_line.expect("oops, I/O error");
        let line_split = line.split(' ').collect::<Vec<_>>();
        if line_split.len() != 2 {
            println!("error consuming line: did not contain 2 fields");
            continue;
        }
        let direction = line_split[0];
        let distance = line_split[1].parse::<i32>().expect("oops, not an i32");
        let motion = make_motion(direction, distance).expect("oops, invalid motion keyword");
        motions.push(motion);
    }
    motions
}

impl Part1Solution for Solution {
    fn part1(&self, reader: &mut dyn BufRead) -> String {
        let mut sub = Submarine {
            hpos: 0,
            vpos: 0,
            aim: 0,
        };
        let motions = compute_motions_from_reader(reader);
        sub.navigate_part1(motions);
        format!(
            "The product of the sub's positions is {}",
            sub.hpos * sub.vpos
        )
    }
}

impl Part2Solution for Solution {
    fn part2(&self, reader: &mut dyn BufRead) -> String {
        let mut sub = Submarine {
            hpos: 0,
            vpos: 0,
            aim: 0,
        };
        let motions = compute_motions_from_reader(reader);
        sub.navigate_part2(motions);
        format!(
            "The product of the sub's positions is {}",
            sub.hpos * sub.vpos
        )
    }
}
