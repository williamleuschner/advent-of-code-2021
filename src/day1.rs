use crate::{Part1Solution, Part2Solution};
use std::io::BufRead;

fn analyze(window_size: usize, reader: &mut dyn BufRead) -> String {
    let lines = reader.lines();
    let mut increase_count = 0;
    let mut last_val = 999999999;
    let int_lines = lines
        .map(|line| {
            line.expect("oops, I/O error")
                .parse::<i32>()
                .expect("oops, not an i32")
        })
        .collect::<Vec<_>>();
    let windows = int_lines.windows(window_size);
    for window in windows {
        let window_sum = window.iter().sum();
        if window_sum > last_val {
            increase_count += 1;
        }
        last_val = window_sum;
    }
    format!("There were {} increases.", increase_count)
}

pub struct Solution {}

impl Part1Solution for Solution {
    fn part1(&self, reader: &mut dyn BufRead) -> String {
        analyze(1, reader)
    }
}

impl Part2Solution for Solution {
    fn part2(&self, reader: &mut dyn BufRead) -> String {
        analyze(3, reader)
    }
}
