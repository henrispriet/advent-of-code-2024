use aoc_runner_derive::{aoc, aoc_generator};

struct InputData {
    left: Vec<u32>,
    right: Vec<u32>,
}

#[aoc_generator(day1)]
fn parse_it(input: &str) -> InputData {
    let (left, right) = input
        .lines()
        .map(|line| {
            let mut it = line
                .split_whitespace()
                .map(|w| w.parse().expect("word should be an integer"));
            let left: u32 = it.next().expect("line should contain two numbers");
            let right = it.next().expect("line should contain two numbers");
            (left, right)
        })
        .collect();
    InputData { left, right }
}

#[aoc(day1, part1)]
fn solve_part1(InputData { left, right }: &InputData) -> u32 {
    // this is sad :(
    let (mut left, mut right) = (left.clone(), right.clone());
    left.sort_unstable();
    right.sort_unstable();
    std::iter::zip(left, right)
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

#[aoc(day1, part2)]
fn solve_part2(InputData { left, right }: &InputData) -> u32 {
    let right_count = right
        .iter()
        .fold(std::collections::HashMap::new(), |mut acc, x| {
            match acc.get_mut(x) {
                Some(count) => *count += 1,
                None => {
                    acc.insert(x, 1);
                }
            }
            acc
        });

    left.iter()
        .filter_map(|l| right_count.get(l).map(|r| l * r))
        .sum()
}

#[test]
fn example_part1() {
    let input = EXAMPLE_INPUT;
    let parsed = parse_it(input);
    let result = solve_part1(&parsed);

    assert_eq!(result, 11);
}

#[test]
fn example_part2() {
    let input = EXAMPLE_INPUT;
    let parsed = parse_it(input);
    let result = solve_part2(&parsed);

    assert_eq!(result, 31);
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3
";
