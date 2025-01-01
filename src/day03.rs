use aoc_runner_derive::{aoc, aoc_generator};

struct InputData {}

#[aoc_generator(day3)]
fn parse_it(input: &str) -> InputData {
    todo!();
}

#[aoc(day3, part1)]
fn solve_part1(input: &InputData) -> u32 {
    todo!();
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

    assert_eq!(result, todo!());
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
