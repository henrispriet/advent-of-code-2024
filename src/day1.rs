use aoc_runner_derive::{aoc, aoc_generator};

struct InputData {}

#[aoc_generator(day1)]
fn parse_it(input: &str) -> InputData {
  todo!();
}

#[aoc(day1, part1)]
fn solve_it(input: &InputData) -> u64 {
  todo!();
}

#[test]
fn test_example() {
  let input = EXAMPLE_INPUT;
  let parsed = parse_it(input);
  let result = solve_it(&parsed);

  assert_eq!(result, todo!());
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "";
