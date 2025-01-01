use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

struct InputData {
    ops: Vec<Mul>,
}

struct Mul(u32, u32);

#[aoc_generator(day3)]
fn parse_it(input: &str) -> InputData {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let ops = re
        .captures_iter(input)
        .map(|c| {
            let (_, [x, y]) = c.extract();
            let x = str::parse(x).expect("x should be an integer");
            let y = str::parse(y).expect("x should be an integer");
            Mul(x, y)
        })
        .collect();
    InputData { ops }
}

#[aoc(day3, part1)]
fn solve_part1(InputData { ops }: &InputData) -> u32 {
    ops.iter().map(|&Mul(a, b)| a * b).sum()
}

#[aoc(day3, part2)]
fn solve_part2(input: &InputData) -> u32 {
    todo!();
}

#[test]
fn example_part1() {
    let input = EXAMPLE_INPUT;
    let parsed = parse_it(input);
    let result = solve_part1(&parsed);

    assert_eq!(result, 161);
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
const EXAMPLE_INPUT: &str =
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
