use aoc_runner_derive::aoc;
use std::ops::{Add, Index, IndexMut};

// NOTE: I kinda hate this solution, I really need to find a better way of working with grids
// I even implemented TWO grid types here smh my head

struct InputData<'a> {
    grid: Grid<'a>,
    start_pos: Pos,
}

#[derive(Debug)]
struct Grid<'a>(Vec<&'a [u8]>);

impl Grid<'_> {
    fn in_bounds(&self, Pos(row, col): Pos) -> bool {
        self.0.get(row).and_then(|r| r.get(col)).is_some()
    }
}

impl Index<Pos> for Grid<'_> {
    type Output = u8;

    fn index(&self, Pos(row, col): Pos) -> &Self::Output {
        &self.0[row][col]
    }
}

#[derive(Clone, Copy, Debug)]
struct Pos(usize, usize);

impl Add<Direction> for Pos {
    type Output = Pos;

    fn add(mut self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::North => self.0 = self.0.overflowing_sub(1).0, // i hate grids
            Direction::East => self.1 += 1,
            Direction::South => self.0 += 1,
            Direction::West => self.1 = self.1.overflowing_sub(1).0, // i hate grids
        }
        self
    }
}

#[derive(Default, Clone, Copy, Debug)]
enum Direction {
    #[default]
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(self) -> Self {
        // NOTE: i would like to just do (self + 1) % 4, but idk how to make the types work
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

const OBSTACLE: u8 = b'#';
const GUARD: u8 = b'^';

// https://github.com/gobanos/cargo-aoc/issues/20
// #[aoc_generator(day6)]
fn parse_it(input: &str) -> InputData {
    let grid: Vec<_> = input.lines().map(str::as_bytes).collect();
    let start_offset = input
        .bytes()
        .position(|b| b == GUARD)
        .expect("there should be a guard");
    let row_len = grid[0].len() + 1; // +1 for newlines
    let start_pos = Pos(start_offset / row_len, start_offset % row_len);
    InputData {
        grid: Grid(grid),
        start_pos,
    }
}

#[derive(Debug)]
struct GridWalker<'a, 'b> {
    pos: Pos,
    facing: Direction,
    grid: &'b Grid<'a>,
}

#[derive(Debug)]
struct SeenGrid {
    grid: Vec<bool>,
    row_len: usize,
}

impl SeenGrid {
    fn new(grid: &Grid<'_>) -> Self {
        let row_len = grid.0[0].len();
        let total_grid_size = grid.0.len() * row_len;
        let seen = vec![false; total_grid_size];
        Self {
            grid: seen,
            row_len,
        }
    }

    fn count(self) -> usize {
        self.grid.into_iter().filter(Clone::clone).count()
    }
}

impl Index<Pos> for SeenGrid {
    type Output = bool;

    fn index(&self, Pos(row, col): Pos) -> &Self::Output {
        &self.grid[self.row_len * row + col]
    }
}

impl IndexMut<Pos> for SeenGrid {
    fn index_mut(&mut self, Pos(row, col): Pos) -> &mut Self::Output {
        &mut self.grid[self.row_len * row + col]
    }
}

impl<'a, 'b> GridWalker<'a, 'b>
where
    'b: 'a,
{
    fn new(grid: &'b Grid<'a>, start_pos: Pos) -> Self {
        Self {
            pos: start_pos,
            grid,
            facing: Direction::default(),
        }
    }

    fn walk(&mut self) -> Option<Pos> {
        let mut next_pos;
        loop {
            next_pos = self.pos + self.facing;
            if !self.grid.in_bounds(next_pos) {
                return None;
            } else if self.grid[next_pos] == OBSTACLE {
                self.facing = self.facing.turn_right();
                continue;
            } else {
                break;
            }
        }
        self.pos = next_pos;
        Some(self.pos)
    }
}

#[aoc(day6, part1)]
fn solve_part1(input: &str) -> usize {
    let InputData { grid, start_pos } = parse_it(input);
    let mut seen = SeenGrid::new(&grid);
    seen[start_pos] = true;
    let mut walker = GridWalker::new(&grid, start_pos);
    for pos in std::iter::from_fn(|| walker.walk()) {
        seen[pos] = true;
    }
    seen.count()
}

#[aoc(day6, part2)]
fn solve_part2(input: &str) -> usize {
    let InputData { grid, start_pos } = parse_it(input);
    todo!();
}

#[test]
fn example_part1() {
    let input = EXAMPLE_INPUT;
    let result = solve_part1(input);

    assert_eq!(result, 41);
}

#[test]
fn example_part2() {
    let input = EXAMPLE_INPUT;
    let result = solve_part2(input);

    assert_eq!(result, 6);
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
