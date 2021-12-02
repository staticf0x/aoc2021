use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

struct Movement {
    direction: String,
    step: u32,
}

impl FromStr for Movement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, step) = s.split_once(" ").unwrap();
        Ok(Movement {
            direction: String::from(direction),
            step: step.parse().unwrap(),
        })
    }
}

fn part1(movements: &Vec<Movement>) -> u32 {
    let mut depth: u32 = 0;
    let mut horizontal: u32 = 0;

    for movement in movements {
        match movement.direction.as_str() {
            "down" => {
                depth += movement.step;
            }
            "up" => {
                depth -= movement.step;
            }
            "forward" => {
                horizontal += movement.step;
            }
            _ => {
                panic!("Undefined movement {}", movement.direction);
            }
        }
    }

    depth * horizontal
}

fn part2(movements: &Vec<Movement>) -> u32 {
    let mut depth: u32 = 0;
    let mut horizontal: u32 = 0;
    let mut aim: u32 = 0;

    for movement in movements {
        match movement.direction.as_str() {
            "down" => {
                aim += movement.step;
            }
            "up" => {
                aim -= movement.step;
            }
            "forward" => {
                horizontal += movement.step;
                depth += aim * movement.step;
            }
            _ => {
                panic!("Undefined movement {}", movement.direction);
            }
        }
    }

    depth * horizontal
}

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut movements: Vec<Movement> = Vec::new();

    for line in reader.lines() {
        let movement = Movement::from_str(line.unwrap().as_str()).unwrap();
        movements.push(movement);
    }

    println!("{}", part1(&movements));
    assert_eq!(part1(&movements), 1648020);

    println!("{}", part2(&movements));
    assert_eq!(part2(&movements), 1759818555);
}
