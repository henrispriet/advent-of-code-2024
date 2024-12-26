#[derive(Debug, PartialEq, Eq, Clone)]
struct InputData {
    keys: Vec<Key>,
    locks: Vec<Lock>,
}

const PINS: usize = 5;
const HEIGHT: u8 = 6;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Key([u8; PINS]);
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Lock([u8; PINS]);

fn parse_it(input: &str) -> InputData {
    let (locks, keys) = input.split("\n\n").partition::<Vec<_>, _>(|x| {
        x.lines()
            .next()
            .expect("key/lock should have at least one line")
            .bytes()
            .all(|c| c == b'#')
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
            if y == b'#' {
                acc[i] += 1
            }
        }
        acc
    })
}

fn solve_it(InputData { keys, locks }: InputData) -> u64 {
    // naive: cartesian product
    let mut fits = 0;

    for Key(k) in &keys {
        for Lock(l) in &locks {
            if std::iter::zip(k, l).all(|(k, l)| k + l < HEIGHT) {
                fits += 1;
            }
        }
    }

    fits
}

fn do_it(input: &str) -> u64 {
    solve_it(parse_it(input))
}

fn main() {
    let input = include_str!("./puzzle_input.txt");
    let result = do_it(input);
    println!("{result}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_small() {
        let result = parse_it(INPUT_SMALL);
        assert_eq!(
            result,
            InputData {
                keys: vec![
                    Key([5, 0, 2, 1, 3]),
                    Key([4, 3, 4, 0, 2]),
                    Key([3, 0, 2, 0, 1])
                ],
                locks: vec![Lock([0, 5, 3, 4, 3]), Lock([1, 2, 0, 5, 3])]
            }
        );
    }

    #[test]
    fn small() {
        let result = do_it(INPUT_SMALL);
        assert_eq!(result, 3);
    }

    #[ignore = "small not done"]
    #[test]
    fn part1() {
        let input = include_str!("puzzle_input.txt");
        let result = do_it(input);
        assert_eq!(result, todo!());
    }

    const INPUT_SMALL: &'static str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
}
