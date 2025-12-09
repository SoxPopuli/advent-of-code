use common::Pos;

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

fn main() {
    let input = common::read_stdin();
    let tiles = parse_input(&input);

    let (time, result) = common::timed(|| part1(&tiles));
    println!("Part 1: {result} in {time:?}");
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
    }
}
