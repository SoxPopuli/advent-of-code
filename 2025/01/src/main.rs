#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Rotation {
    direction: Direction,
    distance: i32,
}
impl std::fmt::Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dir = match self.direction {
            Direction::Left => "L",
            Direction::Right => "R",
        };

        write!(f, "{dir}{}", self.distance)
    }
}

fn parse_input(input: &str) -> Vec<Rotation> {
    input
        .lines()
        .take_while(|x| !x.is_empty())
        .map(|x| {
            let (dir, dist) = x.split_at(1);

            let dir = match dir {
                "L" | "l" => Direction::Left,
                "R" | "r" => Direction::Right,

                x => panic!("Unexpected direction char: {x}"),
            };

            let dist = dist.parse().unwrap();

            Rotation {
                direction: dir,
                distance: dist,
            }
        })
        .collect()
}

fn wrap_value(value: i32) -> i32 {
    value.rem_euclid(100)
}

fn part1(rotations: &[Rotation]) -> i32 {
    rotations
        .iter()
        .fold((50, 0), |(pos, mut hits), rot| {
            let new_pos = match rot.direction {
                Direction::Left => pos - rot.distance,
                Direction::Right => pos + rot.distance,
            };

            let new_pos = wrap_value(new_pos);

            if new_pos == 0 {
                hits += 1;
            }

            (new_pos, hits)
        })
        .1
}

fn part2(rotations: &[Rotation]) -> i32 {
    rotations
        .iter()
        .fold((50, 0), |(pos, mut hits), rot| {
            let new_pos = match rot.direction {
                Direction::Left => pos - rot.distance,
                Direction::Right => pos + rot.distance,
            };
            let new_pos = wrap_value(new_pos);

            let rotations = rot.distance / 100;
            hits += rotations;

            if pos != 0 {
                let is_zero = new_pos == 0;
                let wrap_pos = rot.direction == Direction::Right && new_pos < pos;
                let wrap_neg = rot.direction == Direction::Left && new_pos > pos;

                if is_zero || wrap_pos || wrap_neg {
                    hits += 1;
                }
            }

            (new_pos, hits)
        })
        .1
}

fn main() {
    let input = common::read_stdin();
    let rotations = parse_input(&input);

    let (time, result) = common::timed(|| part1(&rotations));
    println!("Part 1: {result} in {time:?}");

    let (time, result) = common::timed(|| part2(&rotations));
    println!("Part 2: {result} in {time:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_test() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
        let input = parse_input(input);

        assert_eq!(part1(&input), 3, "part 1");
        assert_eq!(part2(&input), 6, "part 2");
    }

    #[test]
    fn over_rotate() {
        let input = vec![Rotation {
            distance: 1000,
            direction: Direction::Right,
        }];

        assert_eq!(part2(&input), 10);
    }
}
