use std::fmt;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::vec::Vec;
use structopt::StructOpt;

mod day1;
mod day2;
mod day3;

enum Part {
    One,
    Two,
}

impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Part::One => write!(f, "1"),
            Part::Two => write!(f, "2"),
        }
    }
}

pub trait Part1Solution {
    fn part1(&self, reader: &mut dyn BufRead) -> String;
}

pub trait Part2Solution {
    fn part2(&self, reader: &mut dyn BufRead) -> String;
}

#[derive(Debug, StructOpt)]
#[structopt(name = "advent-of-code-2021", about = "my solutions to AoC 2021")]
struct Opt {
    /// Use the file named "test" rather than the file named "input".
    #[structopt(long)]
    test: bool,
    /// Which day's solution to execute.
    day: Option<i32>,
    /// Which part of the day's solution to execute.
    part: Option<i32>,
}

fn main() {
    let p1s: Vec<&dyn Part1Solution> =
        vec![&day1::Solution {}, &day2::Solution {}, &day3::Solution {}];
    let p2s: Vec<&dyn Part2Solution> =
        vec![&day1::Solution {}, &day2::Solution {}, &day3::Solution {}];
    let opt = Opt::from_args();

    if let Some(day) = opt.day {
        if day < 1 {
            println!("Day must be at least 1 (not {}).", day);
            return;
        }
        if day > p1s.len() as i32 {
            println!("Day must be at most {} (not {}).", p1s.len(), day);
            return;
        }
    }
    if let Some(part) = opt.part {
        if part != 1 && part != 2 {
            println!("Part can only be 1 or 2 (not {}).", part);
            return;
        }
    }

    let day_idx: usize = match opt.day {
        Some(x) => x as usize - 1,
        None => p1s.len() - 1,
    };
    let part = match opt.part {
        Some(x) => match x {
            1 => Part::One,
            2 => Part::Two,
            _ => panic!("shouldn't be possible"),
        },
        None => {
            if day_idx >= p2s.len() {
                Part::One
            } else {
                Part::Two
            }
        }
    };

    let path_str = format!(
        "inputs/{}-{}",
        day_idx + 1,
        if opt.test { "test" } else { "input" }
    );
    let path = Path::new(&path_str);
    let display = path.display();
    let input_file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let mut input_file_reader = std::io::BufReader::new(input_file);

    println!("Running day {} part {}", day_idx + 1, part);
    println!(
        "--> {}",
        match part {
            Part::One => p1s[day_idx].part1(&mut input_file_reader),
            Part::Two => p2s[day_idx].part2(&mut input_file_reader),
        }
    );
}
