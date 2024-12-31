use std::ops::ControlFlow;

use aoc_runner_derive::{aoc, aoc_generator};

struct InputData {
    sequences: Vec<Vec<i32>>,
}

#[aoc_generator(day2)]
fn parse_it(input: &str) -> InputData {
    let sequences = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(str::parse)
                .collect::<Result<_, _>>()
                .expect("line should contain a sequence of numbers")
        })
        .collect();
    InputData { sequences }
}

#[aoc(day2, part1)]
fn solve_part1(InputData { sequences }: &InputData) -> usize {
    sequences
        .iter()
        .filter(|seq| {
            let sign = (seq[1] - seq[0]).signum();
            seq.windows(2).all(|x| {
                let diff = x[1] - x[0];
                diff.signum() == sign && (1..=3).contains(&diff.abs())
            })
        })
        .count()
}

#[aoc(day2, part2)]
fn solve_part2(input: &InputData) -> u32 {
    todo!();
}

#[test]
fn example_part1() {
    let input = EXAMPLE_INPUT;
    let parsed = parse_it(input);
    let result = solve_part1(&parsed);

    assert_eq!(result, 2);
}

#[ignore = "todo"]
#[test]
fn example_part2() {
    let input = EXAMPLE_INPUT;
    let parsed = parse_it(input);
    let result = solve_part2(&parsed);

    assert_eq!(result, todo!());
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
