use crate::{Part1Solution, Part2Solution};
use regex::Regex;
use std::io::BufRead;

pub struct Solution {}

#[derive(Clone, Copy, Debug)]
struct BingoCell {
    value: i32,
    marked: bool,
}

impl BingoCell {
    fn mark(&mut self) {
        self.marked = true;
    }
}

#[derive(Debug)]
struct BingoBoard {
    board: [[BingoCell; 5]; 5],
    has_won: bool,
}

impl BingoBoard {
    fn call(&mut self, number: i32) {
        for row in 0..5 {
            for col in 0..5 {
                if self.board[row][col].value == number {
                    self.board[row][col].mark();
                }
            }
        }
    }

    fn check(&self) -> bool {
        for row in 0..5 {
            if self.board[row].iter().map(|x| x.marked).all(|x| x) {
                return true;
            }
        }
        for col in 0..5 {
            let mut bingo = true;
            for row in 0..5 {
                if !self.board[row][col].marked {
                    bingo = false;
                }
            }
            if bingo {
                return true;
            }
        }
        return false;
    }

    fn score(&self, last_call: i32) -> i32 {
        let sum: i32 = self
            .board
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|x| !x.marked)
                    .map(|x| x.value)
                    .sum::<i32>()
            })
            .sum();
        sum * last_call
    }
}

#[derive(Debug)]
struct BingoGame {
    boards: Vec<BingoBoard>,
    calls: Vec<i32>,
}

impl BingoGame {
    fn play_part1(&mut self) -> Option<i32> {
        for number in &self.calls {
            for board in &mut self.boards {
                board.call(*number);
                if board.check() {
                    return Some(board.score(*number));
                }
            }
        }
        return None;
    }

    fn play_part2(&mut self) -> Option<i32> {
        let mut most_recent_score: Option<i32> = None;
        for number in &self.calls {
            for board in &mut self.boards {
                board.call(*number);
                if board.check() && !board.has_won {
                    println!("Newest winning board state: {:?}", board);
                    most_recent_score = Some(board.score(*number));
                    board.has_won = true;
                }
            }
        }
        return most_recent_score;
    }

    fn from_bufread(reader: &mut dyn BufRead) -> Self {
        let mut lines = reader.lines();
        let first_line = lines
            .next()
            .expect("oops, not enough input")
            .expect("oops, I/O error");
        let calls_str = first_line.split(',');
        let calls = calls_str
            .map(|c| c.parse::<i32>().expect("oops, not an i32"))
            .collect::<Vec<i32>>();

        lines.next();

        let mut boards: Vec<BingoBoard> = Vec::with_capacity(32);
        let bingo_line_matcher = Regex::new(r"^ *(\d+) +(\d+) +(\d+) +(\d+) +(\d+)$").unwrap();
        loop {
            let mut board_data = [[BingoCell {
                value: 0,
                marked: false,
            }; 5]; 5];
            for index in 0..5 {
                let line = lines.next().expect("oops, EOF").expect("oops, I/O error");
                println!("bingo line is: \"{}\"", line);
                let cap = bingo_line_matcher
                    .captures(&line)
                    .expect("oops, bingo line did not match regex");
                for col in 0..5 {
                    board_data[index][col].value = cap
                        .get(col + 1)
                        .expect("oops, regex didn't match")
                        .as_str()
                        .parse::<i32>()
                        .expect("oops, not an i32");
                }
            }

            boards.push(BingoBoard {
                board: board_data,
                has_won: false,
            });
            if lines.next().is_none() {
                break;
            }
        }
        Self { boards, calls }
    }
}

impl Part1Solution for Solution {
    fn part1(&self, reader: &mut dyn BufRead) -> String {
        let mut game = BingoGame::from_bufread(reader);
        match game.play_part1() {
            Some(score) => format!("The final score is {}", score),
            None => format!("The game never finishes"),
        }
    }
}

impl Part2Solution for Solution {
    fn part2(&self, reader: &mut dyn BufRead) -> String {
        let mut game = BingoGame::from_bufread(reader);
        match game.play_part2() {
            Some(score) => format!("The final score is {}", score),
            None => format!("The game never finishes"),
        }
    }
}
