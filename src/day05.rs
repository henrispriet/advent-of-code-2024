use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

struct InputData {
    rules: HashMap<u32, Vec<u32>>,
    updates: Vec<Update>,
}

type PageNum = u32;
#[derive(Debug)]
struct Update(Vec<PageNum>);

#[aoc_generator(day5)]
fn parse_it(input: &str) -> InputData {
    let (rules, updates) = input
        .split_once("\n\n")
        .expect("input should consist of rules and updates");

    let rules = rules
        .lines()
        .map(|line| {
            let (before, after) = line.split_once('|').expect("rules should be split by '|'");
            let before = before.parse().expect("before should be an integer");
            let after = after.parse().expect("after should be an integer");
            (before, after)
        })
        .fold(
            HashMap::new(),
            |mut acc: HashMap<u32, Vec<u32>>, (before, after)| {
                match acc.get_mut(&before) {
                    Some(vec) => vec.push(after),
                    None => {
                        acc.insert(before, vec![after]);
                    }
                }
                acc
            },
        );

    let updates = updates
        .lines()
        .map(|line| {
            line.split(',')
                .map(str::parse)
                .collect::<Result<_, _>>()
                .map(Update)
                .expect("updates should be integers")
        })
        .collect();

    InputData { rules, updates }
}

#[aoc(day5, part1)]
fn solve_part1(InputData { rules, updates }: &InputData) -> PageNum {
    updates
        .iter()
        .filter(|&upd| {
            let mut marked = HashSet::new();
            upd.0.iter().all(|page| {
                marked.insert(*page);
                rules
                    .get(page)
                    .map(|children| !children.iter().any(|child| marked.contains(child)))
                    .unwrap_or(true)
            })
        })
        .map(|Update(upd)| upd[upd.len() / 2])
        .sum()
}

#[aoc(day5, part2)]
fn solve_part2(input: &InputData) -> usize {
    todo!();
}

#[test]
fn example_part1() {
    let input = EXAMPLE_INPUT;
    let parsed = parse_it(input);
    let result = solve_part1(&parsed);

    assert_eq!(result, 143);
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
const EXAMPLE_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
