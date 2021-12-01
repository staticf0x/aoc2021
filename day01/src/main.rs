use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_increases(numbers: &Vec<u32>) -> u32 {
    let mut prev_depth = 0;
    let mut increases = 0;

    for &depth in numbers {
        if prev_depth == 0 {
            prev_depth = depth;
            continue;
        }

        if depth > prev_depth {
            increases += 1;
        }

        prev_depth = depth;
    }

    increases
}

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let numbers: Vec<u32> = reader
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    println!("{}", get_increases(&numbers));
    assert_eq!(get_increases(&numbers), 1665);
}
