use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq, Eq, Clone)]
struct InputData {
    keys: Vec<Key>,
    locks: Vec<Lock>,
}

const FULL_CHAR: u8 = b'#';
const PINS: usize = 5;
const HEIGHT: u8 = 6;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Key([u8; PINS]);
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Lock([u8; PINS]);

#[aoc_generator(day25)]
fn parse_it(input: &str) -> InputData {
    let (locks, keys) = input.split("\n\n").partition::<Vec<_>, _>(|x| {
        x.lines()
            .next()
            .expect("key/lock should have at least one line")
            .bytes()
            .all(|c| c == FULL_CHAR)
    });

    let keys = keys
        .into_iter()
        .map(|k| count_pin_lens(k.lines().rev()))
        .map(Key)
        .collect();
    let locks = locks
        .into_iter()
        .map(|l| count_pin_lens(l.lines()))
        .map(Lock)
        .collect();
    InputData { keys, locks }
}

fn count_pin_lens<'a>(mut lines: impl Iterator<Item = &'a str>) -> [u8; PINS] {
    // discard first line, since it is all #s
    _ = lines.next();

    lines.fold([0; PINS], |mut acc, x| {
        for (i, y) in x[..PINS].bytes().enumerate() {
            if y == FULL_CHAR {
                acc[i] += 1
            }
        }
        acc
    })
}

#[aoc(day25, part1)]
fn solve_it(InputData { keys, locks }: &InputData) -> u64 {
    // naive: cartesian product
    let mut fits = 0;

    for Key(k) in keys {
        for Lock(l) in locks {
            if std::iter::zip(k, l).all(|(k, l)| k + l < HEIGHT) {
                fits += 1;
            }
        }
    }

    fits
}
