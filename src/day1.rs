use std::io;
use std::io::BufRead;

fn analyze(window_size: usize) {
    println!("problem 1 part 1 or 2");
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
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
    println!("there were {} increases", increase_count);
}

pub fn main() {
    analyze(3);
}
