use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;
use std::collections::HashMap;

struct InputData {
    rules: Rules,
    updates: Vec<Update>,
}

type PageNum = u32;
type Rules = HashMap<u32, Vec<u32>>;
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
        .fold(Rules::new(), |mut acc: Rules, (before, after)| {
            match acc.get_mut(&before) {
                Some(vec) => vec.push(after),
                None => {
                    acc.insert(before, vec![after]);
                }
            }
            acc
        });

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
        .filter(|Update(upd)| {
            // NOTE: for some godforsaken reason is_sorted_by takes a functions that returns a bool instead of a std::cmp::Ordering
            upd.is_sorted_by(|&a, &b| cmp_pages(a, b, rules).is_le())
        })
        .map(|Update(upd)| upd[upd.len() / 2])
        .sum()
}

fn cmp_pages(first: PageNum, other: PageNum, rules: &Rules) -> Ordering {
    match (rules.get(&first), rules.get(&other)) {
        (Some(cs), Some(co)) if co.contains(&first) && cs.contains(&other) => {
            unreachable!()
        }
        (_, Some(co)) if co.contains(&first) => Ordering::Greater,
        (Some(cs), _) if cs.contains(&other) => Ordering::Less,
        _ => unreachable!(),
    }
}

#[aoc(day5, part2)]
fn solve_part2(InputData { rules, updates }: &InputData) -> PageNum {
    updates
        .iter()
        .filter(|Update(upd)| {
            // NOTE: for some godforsaken reason is_sorted_by takes a functions that returns a bool instead of a std::cmp::Ordering
            !upd.is_sorted_by(|&a, &b| cmp_pages(a, b, rules).is_le())
        })
        .map(|Update(upd)| {
            let mut upd = upd.clone();
            upd.sort_by(|&a, &b| cmp_pages(a, b, rules));
            upd
        })
        .map(|upd| upd[upd.len() / 2])
        .sum()
}

#[test]
fn example_part1() {
    let input = EXAMPLE_INPUT;
    let parsed = parse_it(input);
    let result = solve_part1(&parsed);

    assert_eq!(result, 143);
}

#[test]
fn example_part2() {
    let input = EXAMPLE_INPUT;
    let parsed = parse_it(input);
    let result = solve_part2(&parsed);

    assert_eq!(result, 123);
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
