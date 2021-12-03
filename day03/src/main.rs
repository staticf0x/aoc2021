use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_bin(s: &str) -> u32 {
    u32::from_str_radix(s, 2).unwrap()
}

fn get_bit(n: u32, pos: usize) -> u8 {
    (n & (1 << pos) > 0) as u8
}

fn get_most_common(numbers: &Vec<u32>, pos: usize) -> u8 {
    let column_bits: Vec<u8> = numbers
        .clone()
        .into_iter()
        .map(|n| get_bit(n, pos))
        .collect();
    let ones: Vec<u8> = column_bits.iter().filter(|n| **n == 1).cloned().collect();
    let zeroes: Vec<u8> = column_bits.iter().filter(|n| **n == 0).cloned().collect();

    if ones.len() > zeroes.len() {
        1
    } else {
        0
    }
}

fn get_gamma(numbers: &Vec<u32>, size: usize) -> u32 {
    let mut bits: Vec<u8> = Vec::new();

    for i in 0..size {
        let most_common = get_most_common(&numbers, i);
        bits.insert(0, most_common);
    }

    let mut n: u32 = 0;

    for i in 0..size {
        n += (bits[(size - i - 1) as usize] as u32) << i;
    }

    n
}

fn get_epsilon(numbers: &Vec<u32>, size: usize) -> u32 {
    let mut bits: Vec<u8> = Vec::new();

    for i in 0..size {
        let least_common = if get_most_common(&numbers, i) == 1 {
            0
        } else {
            1
        };
        bits.insert(0, least_common);
    }

    let mut n: u32 = 0;

    for i in 0..size {
        n += (bits[(size - i - 1) as usize] as u32) << i;
    }

    n
}

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut numbers: Vec<u32> = Vec::new();

    for line in reader.lines() {
        let num = parse_bin(line.unwrap().as_str());
        numbers.push(num);
    }

    let gamma = get_gamma(&numbers, 12);
    let epsilon = get_epsilon(&numbers, 12);

    println!("{}", gamma * epsilon);
    assert_eq!(gamma * epsilon, 4138664);
}

#[test]
fn test_parse_bin() {
    assert_eq!(parse_bin("1000"), 8);
    assert_eq!(parse_bin("1101"), 13);
    assert_eq!(parse_bin("0"), 0);
    assert_eq!(parse_bin("1"), 1);
    assert_eq!(parse_bin("101001011000"), 2648);
}

#[test]
fn test_get_bit() {
    assert_eq!(get_bit(8, 0), 0);
    assert_eq!(get_bit(8, 3), 1);
    assert_eq!(get_bit(13, 0), 1);
    assert_eq!(get_bit(13, 1), 0);
    assert_eq!(get_bit(13, 2), 1);
    assert_eq!(get_bit(13, 3), 1);
}

#[test]
fn test_get_most_common() {
    // 8:  1000
    // 13: 1101
    // 0:  0000
    assert_eq!(get_most_common(&vec![8, 13, 0], 0), 0);
    assert_eq!(get_most_common(&vec![8, 13, 0], 1), 0);
    assert_eq!(get_most_common(&vec![8, 13, 0], 2), 0);
    assert_eq!(get_most_common(&vec![8, 13, 0], 3), 1);
}

#[test]
fn test_get_gamma() {
    let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    let mut numbers: Vec<u32> = Vec::new();

    for line in input.lines() {
        let num = parse_bin(line);
        numbers.push(num);
    }

    assert_eq!(get_gamma(&numbers, 5), 22);
}
#[test]
fn test_get_epsilon() {
    let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    let mut numbers: Vec<u32> = Vec::new();

    for line in input.lines() {
        let num = parse_bin(line);
        numbers.push(num);
    }

    assert_eq!(get_epsilon(&numbers, 5), 9);
}
