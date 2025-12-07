use std::collections::{HashMap, HashSet};

use common::Pos;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Splitter;
impl std::fmt::Display for Splitter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        f.write_char('^')
    }
}

type Grid = common::grid2::Grid<Splitter>;

#[derive(Debug)]
struct Input {
    start_pos: Pos,
    grid: Grid,
}
impl Input {
    fn new(input: &str) -> Self {
        let mut start_pos = Pos::default();
        let grid = Grid::from_grid_string(input, |pos, cell| match cell {
            b'S' => {
                start_pos = pos;
                None
            }
            b'^' => Some(Splitter),
            _ => None,
        });

        Self { start_pos, grid }
    }

    fn count_splits(&self) -> u32 {
        use common::vectors::*;

        fn beam(pos: Pos, grid: &Grid, visited: &mut HashSet<Pos>) -> u32 {
            let next_pos = pos + DOWN;

            if !grid.is_inside(&next_pos) {
                0
            } else if grid.get(&next_pos).is_some() {
                let split = if visited.contains(&next_pos) {
                    return 0;
                } else {
                    visited.insert(next_pos);
                    1
                };

                split + beam(next_pos + LEFT, grid, visited) + beam(next_pos + RIGHT, grid, visited)
            } else {
                beam(next_pos, grid, visited)
            }
        }

        beam(self.start_pos, &self.grid, &mut HashSet::new())
    }

    fn count_splits_permutations(&self) -> u64 {
        use common::vectors::*;

        let mut splits: HashMap<Pos, u64> = HashMap::new();

        fn beam(pos: Pos, grid: &Grid, visited: &mut HashMap<Pos, u64>) -> u64 {
            let next_pos = pos + DOWN;

            if !grid.is_inside(&next_pos) {
                0
            } else if grid.get(&next_pos).is_some() {
                if visited.contains_key(&next_pos) {
                    *visited.get(&next_pos).unwrap()
                } else {
                    let splits =
                        1 + beam(pos + LEFT, grid, visited) + beam(pos + RIGHT, grid, visited);
                    visited.insert(next_pos, splits);

                    splits
                }
            } else {
                beam(pos + DOWN, grid, visited)
            }
        }

        beam(self.start_pos, &self.grid, &mut splits) + 1
    }
}

fn main() {
    let input = common::read_stdin();
    let input = Input::new(&input);

    let (time, result) = common::timed(|| input.count_splits());
    println!("Part 1: {result} in {time:?}");

    let (time, result) = common::timed(|| input.count_splits_permutations());
    println!("Part 2: {result} in {time:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = ".......S.......\n\
                     ...............\n\
                     .......^.......\n\
                     ...............\n\
                     ......^.^......\n\
                     ...............\n\
                     .....^.^.^.....\n\
                     ...............\n\
                     ....^.^...^....\n\
                     ...............\n\
                     ...^.^...^.^...\n\
                     ...............\n\
                     ..^...^.....^..\n\
                     ...............\n\
                     .^.^.^.^.^...^.\n\
                     ...............\n";

        let grid = Input::new(input);

        assert_eq!(grid.count_splits(), 21);
        assert_eq!(grid.count_splits_permutations(), 40);
    }
}
