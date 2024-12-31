use aoc_runner_derive::{aoc, aoc_generator};

struct InputData {
}

#[aoc_generator(day2)]
fn parse_it(input: &str) -> InputData {
  todo!();
}

#[aoc(day2, part1)]
fn solve_part1(input: &InputData) -> u32 {
  todo!();
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

  assert_eq!(result, todo!());
}

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
