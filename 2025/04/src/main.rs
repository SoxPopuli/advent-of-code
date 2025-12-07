use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
struct Item;
impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@")
    }
}

type Grid = common::grid2::Grid<Item>;

fn parse_input(input: &str) -> Grid {
    Grid::from_grid_string(input, |_, cell| match cell {
        b'@' => Some(Item),
        _ => None,
    })
}

fn part1(input: &Grid) -> usize {
    input
        .iter()
        .filter(|(pos, _)| {
            let neighbours = input.iter_adjacent(**pos).count();
            neighbours < 4
        })
        .count()
}

fn part2(mut input: Grid) -> usize {
    let mut total = 0;

    loop {
        let items = input
            .iter()
            .filter(|(pos, _)| {
                let neighbours = input.iter_adjacent(**pos).count();
                neighbours < 4
            })
            .map(|(pos, _)| *pos)
            .collect::<Vec<_>>();

        if items.is_empty() {
            break;
        }

        total += items.len();

        for pos in items {
            input.items.remove(&pos);
        }
    }

    total
}

fn main() {
    let input = parse_input(&common::read_stdin());

    let (time, result) = common::timed(|| part1(&input));
    println!("Part 1: {result} in {time:?}");

    let (time, result) = common::timed(|| part2(input));
    println!("Part 2: {result} in {time:?}");
}
