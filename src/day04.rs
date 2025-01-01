use aoc_runner_derive::aoc;
// TODO: could use ndarray here

struct InputData<'a> {
    grid: Vec<&'a [u8]>,
}

const WORD: &[u8] = b"XMAS";
const WORD_LEN: usize = const { WORD.len() };

// https://github.com/gobanos/cargo-aoc/issues/20
// #[aoc_generator(day4)]
fn parse_it(input: &str) -> InputData<'_> {
    let grid = input.lines().map(str::as_bytes).collect();
    InputData { grid }
}

// couldn't figure out the trait bounds, so here's a macro instead
macro_rules! cartesian {
    ($left: expr, $right: expr) => {
        ($left)
            .into_iter()
            .flat_map(|l| std::iter::repeat(l).zip($right))
    };
}

#[aoc(day4, part1)]
fn solve_part1(input: &str) -> usize {
    let InputData { grid } = parse_it(input);
    cartesian!(0..grid.len(), 0..grid[0].len())
        .map(|(row, column)| count_words(row, column, &grid))
        .sum()
}

fn count_words(row: usize, column: usize, grid: &[&[u8]]) -> usize {
    cartesian!(-1..=1isize, -1..=1isize)
        .filter(|&(i, j)| {
            (0..WORD_LEN).all(|k| {
                grid.get(offset(row, i, k))
                    .and_then(|row| row.get(offset(column, j, k)))
                    == Some(&WORD[k])
            })
        })
        .count()
}

fn offset(base: usize, offset: isize, multiplier: usize) -> usize {
    let idx = base as isize + offset * multiplier as isize;
    // as usize on negative isize does wonky things :/
    if idx >= 0 {
        idx as usize
    } else {
        usize::MAX
    }
}

#[aoc(day4, part2)]
fn solve_part2(input: &str) -> usize {
    let InputData { grid } = parse_it(input);
    cartesian!(0..grid.len() - 2, 0..grid[0].len() - 2)
        .filter(|&(i, j)| {
            x_mas(&[
                &grid[i][j..j + 3],
                &grid[i + 1][j..j + 3],
                &grid[i + 2][j..j + 3],
            ])
        })
        .count()
}

fn x_mas(kernel: &[&[u8]]) -> bool {
    kernel[1][1] == b'A'
        && ((kernel[0][0] == b'M' && kernel[2][2] == b'S')
            || (kernel[0][0] == b'S' && kernel[2][2] == b'M'))
        && ((kernel[2][0] == b'M' && kernel[0][2] == b'S')
            || (kernel[2][0] == b'S' && kernel[0][2] == b'M'))
}

#[test]
fn example_part1() {
    let input = EXAMPLE_INPUT;
    let result = solve_part1(input);

    assert_eq!(result, 18);
}

#[test]
fn example_part2() {
    let input = EXAMPLE_INPUT;
    let result = solve_part2(input);

    assert_eq!(result, 9);
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
