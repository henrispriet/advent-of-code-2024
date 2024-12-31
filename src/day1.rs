use aoc_runner_derive::{aoc, aoc_generator};

struct InputData {
  left: Vec<i32>,
  right: Vec<i32>,
}

#[aoc_generator(day1)]
fn parse_it(input: &str) -> InputData {
  let (left, right) = input.lines()
    .map(|line| {
      let mut it = line.split_whitespace()
        .map(|w| w.parse().expect("word should be an integer"));
      let left: i32 = it.next().expect("line should contain two numbers");
      let right = it.next().expect("line should contain two numbers");
      (left, right)
    }).collect();
  InputData { left, right }
}

#[aoc(day1, part1)]
fn solve_it(InputData { left, right }: &InputData) -> u32 {
  // this is sad :(
  let (mut left, mut right) = (left.clone(), right.clone());
  left.sort_unstable();
  right.sort_unstable();
  std::iter::zip(left, right).map(|(l, r)| (l - r).unsigned_abs()).sum()
}

#[test]
fn test_example() {
  let input = EXAMPLE_INPUT;
  let parsed = parse_it(input);
  let result = solve_it(&parsed);

  assert_eq!(result, 11);
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3
";
