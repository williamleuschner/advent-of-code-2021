use crate::{Part1Solution, Part2Solution};
use std::io::BufRead;
use std::vec;

pub struct Solution {}

impl Part1Solution for Solution {
    fn part1(&self, reader: &mut dyn BufRead) -> String {
        let mut lines = reader.lines().peekable();
        let bit_count = lines
            .peek()
            .expect("oops, I/O error")
            .as_ref()
            .expect("oops, other I/O error")
            .len();
        let mut one_count_per_bit = vec![0; bit_count];
        let mut line_count = 0;
        for maybe_line in lines {
            let line = maybe_line.expect("oops, I/O error");
            line_count += 1;
            for (index, digit) in line.char_indices() {
                if digit == '1' {
                    one_count_per_bit[index] += 1;
                }
            }
        }
        println!(
            "line_count = {}; one_count_per_bit = {:?}",
            line_count, one_count_per_bit
        );
        let frequency_threshold = line_count / 2;
        let one_is_most_common: Vec<bool> = one_count_per_bit
            .iter()
            .map(|count| count > &frequency_threshold)
            .collect();
        println!("one_is_most_common = {:?}", one_is_most_common);
        let gamma_rate = bool_vec_to_int(&one_is_most_common);
        let epsilon_rate = bool_vec_to_int(&one_is_most_common.iter().map(|x| !x).collect());
        println!("gamma_rate = {}", gamma_rate);
        println!("epsilon_rate = {}", epsilon_rate);
        format!("Power consumption is {}", gamma_rate * epsilon_rate)
    }
}

#[derive(Copy, Clone)]
enum Count {
    Ones,
    Zeroes,
}

fn bool_vec_to_int(v: &Vec<bool>) -> i32 {
    assert!(v.len() <= 32);
    v.iter()
        .fold(0, |acc, el| (acc * 2) + (if *el { 1 } else { 0 }))
}

fn build_bit_frequency_table(for_values: &Vec<i32>, of_size: usize) -> Vec<f32> {
    let total_bits = for_values.len();
    let mut table = vec![0; of_size];
    for bit_idx in 0..of_size {
        for value in for_values {
            if value & (1 << (of_size - bit_idx - 1)) != 0 {
                table[bit_idx] += 1;
            }
        }
    }

    table
        .iter()
        .map(|v| (*v as f32) / (total_bits as f32))
        .collect()
}

fn recursive_helper(report_values: &Vec<i32>, max_bits: usize, count: Count, bit_idx: i32) -> i32 {
    let frequencies = build_bit_frequency_table(report_values, max_bits);
    // println!(
    //     "helper called with report_values = {:?}; frequencies = {:?}",
    //     report_values, frequencies
    // );
    // println!(
    //     "\tfrequencies[{}] = {}",
    //     bit_idx, frequencies[bit_idx as usize]
    // );
    let filter_value = match count {
        Count::Ones => {
            if frequencies[bit_idx as usize] >= 0.5 {
                1
            } else {
                0
            }
        }
        Count::Zeroes => {
            if frequencies[bit_idx as usize] < 0.5 {
                1
            } else {
                0
            }
        }
    };
    let bit_pos = frequencies.len() as i32 - 1 - bit_idx;
    // println!(
    //     "\tfiltering using value & {:#016b} == {}",
    //     1 << bit_pos,
    //     filter_value
    // );
    let filtered_values: Vec<i32> = report_values
        .iter()
        .filter(|value| *value & (1 << bit_pos) == (filter_value << bit_pos))
        .map(|value| *value)
        .collect();
    if filtered_values.len() <= 1 {
        filtered_values
            .into_iter()
            .next()
            .expect("this shouldn't happen")
    } else {
        recursive_helper(&filtered_values, max_bits, count, bit_idx + 1)
    }
}

impl Part2Solution for Solution {
    fn part2(&self, reader: &mut dyn BufRead) -> String {
        let mut lines = reader.lines().peekable();
        let bit_count = lines
            .peek()
            .expect("oops, I/O error")
            .as_ref()
            .expect("oops, other I/O error")
            .len();
        let collected_lines = lines
            .map(|maybe_line| {
                i32::from_str_radix(&maybe_line.expect("oops, I/O error").to_owned(), 2)
                    .expect("oops, not an i32 in binary")
            })
            .collect();

        let o2_gen_rating = recursive_helper(&collected_lines, bit_count, Count::Ones, 0);
        let co2_scrub_rating = recursive_helper(&collected_lines, bit_count, Count::Zeroes, 0);
        format!("Life support rating: {}", o2_gen_rating * co2_scrub_rating)
    }
}
