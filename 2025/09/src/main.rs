use common::Pos;
use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<Pos> {
    input
        .lines()
        .filter_map(|line| {
            let mut parts = line.trim().split(',');

            Some(Pos::new(
                parts.next()?.parse().ok()?,
                parts.next()?.parse().ok()?,
            ))
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Rect {
    a: Pos,
    b: Pos,
}
impl Rect {
    fn size(&self) -> usize {
        let x_diff = self.a.x.abs_diff(self.b.x) + 1;
        let y_diff = self.a.y.abs_diff(self.b.y) + 1;

        x_diff * y_diff
    }

    fn top_left(&self) -> Pos {
        Pos {
            x: self.a.x.min(self.b.x),
            y: self.a.y.min(self.b.y),
        }
    }

    fn bottom_right(&self) -> Pos {
        Pos {
            x: self.a.x.max(self.b.x),
            y: self.a.y.max(self.b.y),
        }
    }

    fn contains(&self, point: Pos) -> bool {
        let top_left = self.top_left();
        let bottom_right = self.bottom_right();

        point.x > top_left.x
            && point.x < bottom_right.x
            && point.y > top_left.y
            && point.y < bottom_right.y
    }

    fn contains_midpoint(&self, start: Pos, end: Pos) -> bool {
        let midpoint = Pos {
            x: (start.x + end.x) / 2,
            y: (start.y + end.y) / 2,
        };
        self.contains(midpoint)
    }
}

fn part1(tiles: &[Pos]) -> usize {
    let mut rects = vec![];

    for i in 0..tiles.len() {
        for j in i + 1..tiles.len() {
            rects.push(Rect {
                a: tiles[i],
                b: tiles[j],
            });
        }
    }

    rects.into_iter().map(|x| x.size()).max().unwrap()
}

fn part2(points: &[Pos]) -> usize {
    let mut areas = HashMap::new();

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let rect = Rect {
                a: points[i],
                b: points[j],
            };

            areas.insert((i, j), rect.size());
        }
    }

    let sorted = {
        let mut sorted = areas.into_iter().collect::<Vec<_>>();
        sorted.sort_by_key(|(_, area)| *area);
        sorted
    };

    sorted
        .into_iter()
        .rev()
        .find_map(|((x, y), area)| {
            let rect = Rect {
                a: points[x],
                b: points[y],
            };

            let contains_point = || points.iter().any(|p| rect.contains(*p));

            let contains_midpoint = || {
                (0..points.len() - 1)
                    .map(|i| (points[i], points[i + 1]))
                    .any(|(a, b)| rect.contains_midpoint(a, b))
            };

            if !contains_point()
                && !contains_midpoint()
                && (!rect.contains_midpoint(points[0], points[points.len() - 1]))
            {
                Some(area)
            } else {
                None
            }
        })
        .unwrap()
}

fn main() {
    let input = common::read_stdin();
    let tiles = parse_input(&input);

    let (time, result) = common::timed(|| part1(&tiles));
    println!("Part 1: {result} in {time:?}");

    let (time, result) = common::timed(|| part2(&tiles));
    println!("Part 2: {result} in {time:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"
            7,1
            11,1
            11,7
            9,7
            9,5
            2,5
            2,3
            7,3
        "#;

        let tiles = parse_input(input);
        assert_eq!(part1(&tiles), 50);

        assert_eq!(part2(&tiles), 24);
    }
}
