use aoc_runner_derive::{aoc, aoc_generator};

struct InputData {}

#[aoc_generator(dayX)]
fn parse_it(input: &str) -> InputData {
    todo!();
}

#[aoc(dayX, part1)]
fn solve_part1(input: &InputData) -> u32 {
    todo!();
}

#[aoc(dayX, part2)]
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
const EXAMPLE_INPUT: &str = "";
