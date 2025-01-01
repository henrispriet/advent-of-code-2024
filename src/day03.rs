use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

struct InputData {
    ops: Vec<Op>,
}

#[derive(Clone, Copy, Debug)]
enum Op {
    Mul(Mul),
    Enable(bool),
}
#[derive(Clone, Copy, Debug)]
struct Mul(u32, u32);

#[aoc_generator(day3)]
fn parse_it(input: &str) -> InputData {
    let regex = Regex::new(r"mul\((?<x>\d+),(?<y>\d+)\)|do(n't)?\(\)").unwrap();
    let ops = regex
        .captures_iter(input)
        .map(|c| match &c[0] {
            "do()" => Op::Enable(true),
            "don't()" => Op::Enable(false),
            _ => {
                let x = c["x"].parse().expect("x should be an integer");
                let y = c["y"].parse().expect("y should be an integer");
                Op::Mul(Mul(x, y))
            }
        })
        .collect();

    InputData { ops }
}

#[aoc(day3, part1)]
fn solve_part1(InputData { ops }: &InputData) -> u32 {
    ops.iter()
        .filter_map(|&op| match op {
            Op::Mul(Mul(x, y)) => Some(x * y),
            _ => None,
        })
        .sum()
}

#[aoc(day3, part2)]
fn solve_part2(InputData { ops }: &InputData) -> u32 {
    let mut enabled = true;
    ops.iter()
        .filter_map(|&op| match (enabled, op) {
            (true, Op::Mul(Mul(x, y))) => Some(x * y),
            (_, Op::Enable(new)) => {
                enabled = new;
                None
            }
            _ => None,
        })
        .sum()
}

#[test]
fn example_part1() {
    let input = EXAMPLE_INPUT1;
    let parsed = parse_it(input);
    let result = solve_part1(&parsed);

    assert_eq!(result, 161);
}

#[test]
fn example_part2() {
    let input = EXAMPLE_INPUT2;
    let parsed = parse_it(input);
    let result = solve_part2(&parsed);

    assert_eq!(result, 48);
}

#[cfg(test)]
const EXAMPLE_INPUT1: &str =
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
#[cfg(test)]
const EXAMPLE_INPUT2: &str =
    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
