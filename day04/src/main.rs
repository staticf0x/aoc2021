use std::fs::File;
use std::io::{BufRead, BufReader, Read};

#[derive(Debug, Clone, Copy)]
struct Pos {
    number: u32,
    marked: bool,
}

#[derive(Debug)]
struct Board {
    numbers: Vec<Vec<Pos>>,
}

impl Board {
    fn new(board_str: &[&String]) -> Self {
        let mut lines = Vec::new();

        for line in board_str {
            let line_numbers = parse_line(line);
            lines.push(line_numbers);
        }

        Board { numbers: lines }
    }

    fn is_win(&self) -> bool {
        for line in &self.numbers {
            if line.iter().all(|p| p.marked) {
                return true;
            }
        }

        for i in 0..5 {
            let col: Vec<Pos> = (&self.numbers)
                .iter()
                .map(|line| line.get(i).unwrap())
                .copied()
                .collect();

            if col.iter().all(|p| p.marked) {
                return true;
            }
        }

        false
    }

    fn mark(&mut self, number: u32) {
        for line in &mut self.numbers {
            for column in line {
                if column.number == number {
                    column.marked = true;
                }
            }
        }
    }

    fn sum_unmarked(&self) -> u32 {
        (&self.numbers)
            .iter()
            .flatten()
            .filter(|p| !p.marked)
            .map(|p| p.number)
            .sum()
    }
}

fn parse_line(line: &&String) -> Vec<Pos> {
    line.split(" ")
        .filter(|p| !p.is_empty())
        .map(|n| Pos {
            number: n.parse().unwrap(),
            marked: false,
        })
        .collect()
}

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Load lines from input
    let lines: Vec<String> = reader.lines().into_iter().map(|l| l.unwrap()).collect();

    // First line of marks
    let numbers: Vec<u32> = lines
        .iter()
        .nth(0)
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    // Load boards into Board structs
    let mut boards = Vec::new();

    for board_str in lines[2..]
        .into_iter()
        .filter(|l| !l.is_empty())
        .collect::<Vec<&String>>()
        .chunks(5)
    {
        let board = Board::new(board_str);
        boards.push(board);
    }

    // Mark numbers and check for win
    'numbers: for number in numbers {
        for board in boards.iter_mut() {
            board.mark(number);

            if board.is_win() {
                println!("{}", board.sum_unmarked() * number);
                break 'numbers;
            }
        }
    }
}
