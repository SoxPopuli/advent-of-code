use std::collections::HashSet;

use common::{Pos, timed};

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,

    trees: HashSet<Pos>,
}
impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;

        for y in 0..self.height {
            for x in 0..self.width {
                let c = if self.trees.contains(&Pos::new(x as isize, y as isize)) {
                    '#'
                } else {
                    '.'
                };
                f.write_char(c)?;
            }

            if y < self.height - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}
impl Grid {
    pub fn new(data: &str) -> Self {
        let lines = data.lines();

        let mut width = 0;
        let mut height = 0;

        let mut trees = HashSet::new();

        for line in lines {
            width = width.max(line.len());

            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    trees.insert(Pos {
                        x: x.try_into().unwrap(),
                        y: height.try_into().unwrap(),
                    });
                }
            }

            height += 1;
        }

        Self {
            width,
            height,
            trees,
        }
    }

    pub fn has_tree(&self, pos: Pos) -> bool {
        self.trees.contains(&pos)
    }
}

fn calc_slope(grid: &Grid, right: isize, down: isize) -> i64 {
    let width = grid.width as isize;
    let height = grid.height as isize;

    let mut x = 0;
    let mut y = 0;

    let mut trees = 0;

    while y < height {
        x = (x + right) % width;
        y += down;

        if grid.has_tree(Pos { x, y }) {
            trees += 1;
        }
    }

    trees
}

fn part1(grid: &Grid) -> i64 {
    calc_slope(grid, 3, 1)
}

fn part2(grid: &Grid) -> i64 {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .map(|(right, down)| calc_slope(grid, right, down))
        .product()
}

fn main() {
    let input = common::read_stdin();
    let grid = Grid::new(&input);

    let (time, result) = timed(|| part1(&grid));
    println!("Part 1: {result} in {time:?}");

    let (time, result) = timed(|| part2(&grid));
    println!("Part 2: {result} in {time:?}");
}

// Part 1: 216 in 16.542µs
// Part 2: 6708199680 in 149.083µs
