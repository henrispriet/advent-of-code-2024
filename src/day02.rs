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
        // why is `.filter(is_valid)` not allowed here?
        .filter(|seq| is_valid(seq))
        .count()
}

fn is_valid(seq: &[i32]) -> bool {
    let sign = (seq[1] - seq[0]).signum();
    seq.windows(2).all(|x| {
        let diff = x[1] - x[0];
        diff.signum() == sign && (1..=3).contains(&diff.abs())
    })
}

#[aoc(day2, part2)]
fn solve_part2(InputData { sequences }: &InputData) -> usize {
    sequences
        .iter()
        .filter(|seq| {
            is_valid(seq)
                || (0..seq.len()).any(|i| {
                    // why is `seq.clone()` not allowed here?
                    let mut new_seq = Vec::clone(seq);
                    new_seq.remove(i);
                    is_valid(&new_seq)
                })
        })
        .count()
}

#[test]
fn example_part1() {
    let input = EXAMPLE_INPUT;
    let parsed = parse_it(input);
    let result = solve_part1(&parsed);

    assert_eq!(result, 2);
}

#[test]
fn example_part2() {
    let input = EXAMPLE_INPUT;
    let parsed = parse_it(input);
    let result = solve_part2(&parsed);

    assert_eq!(result, 4);
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
