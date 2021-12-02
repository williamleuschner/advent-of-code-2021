use std::io;
use std::io::BufRead;
use std::vec;

fn part1() {
	println!("problem 1 part 1");
	let stdin = io::stdin();
	let lines = stdin.lock().lines();
	let mut increase_count = 0;
	let mut last_val = 9999999;
	for line in lines {
		let line_int = line.expect("oops, I/O error").parse::<i32>().expect("oops, not an int");
		if line_int > last_val {
			increase_count += 1;
		}
		last_val = line_int;
	}
	println!("there were {} increases", increase_count);
}

fn part2() {
	println!("problem 1 part 2");
	let sum_window = 3;
	let stdin = io::stdin();
	let lines = stdin.lock().lines();
	let mut increase_count = 0;
	let mut last_val = 999999999;
	let int_lines = lines
		.map(|line| {
			line.expect("oops, I/O error").parse::<i32>().expect("oops, not an i32")
		})
		.collect::<Vec<_>>();
	let windows = int_lines.windows(sum_window);
	for window in windows {
		let window_sum = window.iter().sum();
		if window_sum > last_val {
			increase_count += 1;
		}
		last_val = window_sum;
	}
	println!("there were {} increases", increase_count);
}

pub fn main() {
	part2();
}
