use std::collections::HashMap;

use common::{Pos, grid2::Grid};

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
struct Item;
impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        f.write_char('#')
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Shape(Grid<Item>);
impl std::fmt::Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug)]
struct Region {
    width: u32,
    height: u32,
    indices: Vec<u32>,
}
impl Region {
    fn can_fit(&self, shapes: &[Shape]) -> bool {
        let area = self.width as usize * self.height as usize;

        let mut shape_points = 0;

        for (i, &count) in self.indices.iter().enumerate() {
            let s = &shapes[i];
            shape_points += s.0.items.len() * count as usize;
        }

        shape_points <= area
    }
}

#[derive(Debug)]
struct Input {
    shapes: Vec<Shape>,
    regions: Vec<Region>,
}

fn parse_input(input: &str) -> Input {
    let mut lines = input.lines().map(|x| x.trim()).filter(|x| !x.is_empty());

    let mut shapes = vec![];
    let mut regions = vec![];

    while let Some(line) = lines.next() {
        if line.contains("x") {
            let (size, indices) = line.split_once(": ").unwrap();
            let (width, height) = size.split_once('x').unwrap();

            let width = width.parse().unwrap();
            let height = height.parse().unwrap();
            let indices = indices
                .split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect();

            regions.push(Region {
                width,
                height,
                indices,
            });
        } else {
            let shape_parts = (0..3).filter_map(|_| lines.next());

            let mut grid = Grid {
                width: 3,
                height: 3,
                items: HashMap::new(),
            };

            for (y, line) in shape_parts.into_iter().enumerate() {
                for (x, &c) in line.as_bytes().iter().enumerate() {
                    if c == b'#' {
                        grid.items.insert(Pos::new(x as isize, y as isize), Item);
                    }
                }
            }

            shapes.push(Shape(grid));
        }
    }

    Input { shapes, regions }
}

fn part1(input: &Input) -> usize {
    input
        .regions
        .iter()
        .filter(|r| r.can_fit(&input.shapes))
        .count()
}

fn main() {
    let input = parse_input(&common::read_stdin());

    let (time, result) = common::timed(|| part1(&input));
    println!("Part 1: {result} in {time:?}");
}
