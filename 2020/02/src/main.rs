use common::timed;
use regex::Regex;

thread_local! {
    pub static RE: Regex = Regex::new(r#"^(\d+)\-(\d+) ([a-z]): (\w+)$"#).unwrap();
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Password {
    first: usize,
    second: usize,
    character: char,
    password: String,
}
impl Password {
    fn from_line(line: &str) -> Self {
        let (_, [min, max, c, word]) = RE.with(|x| x.captures(line).unwrap().extract());

        Self {
            first: min.parse().unwrap(),
            second: max.parse().unwrap(),

            character: c.chars().next().unwrap(),
            password: word.to_string(),
        }
    }

    fn is_valid(&self) -> bool {
        let count = self
            .password
            .chars()
            .filter(|c| *c == self.character)
            .count();

        count >= self.first && count <= self.second
    }
}

fn part1(input: &[Password]) -> usize {
    input.iter().filter(|x| x.is_valid()).count()
}

fn part2(input: &[Password]) -> usize {
    input
        .iter()
        .filter(|pwd| {
            let chars = || pwd.password.chars();

            let a = chars().nth(pwd.first - 1).unwrap();
            let b = chars().nth(pwd.second - 1).unwrap();

            (a == pwd.character) ^ (b == pwd.character)
        })
        .count()
}

fn main() {
    let input = common::stream_stdin()
        .lines()
        .map(|x| Password::from_line(&x))
        .collect::<Vec<_>>();

    let (time, result) = timed(|| part1(&input));
    println!("Part 1: {result} in {time:?}");

    let (time, result) = timed(|| part2(&input));
    println!("Part 2: {result} in {time:?}");
}

// Part 1: 622 in 26.75µs
// Part 2: 263 in 197.667µs

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> [Password; 3] {
        [
            Password {
                first: 1,
                second: 3,
                character: 'a',
                password: "abcde".to_string(),
            },
            Password {
                first: 1,
                second: 3,
                character: 'b',
                password: "cdefg".to_string(),
            },
            Password {
                first: 2,
                second: 9,
                character: 'c',
                password: "ccccccccc".to_string(),
            },
        ]
    }

    #[test]
    fn part1() {
        let input = input();
        assert_eq!(super::part1(&input), 2);
    }

    #[test]
    fn part2() {
        let input = input();
        assert_eq!(super::part2(&input), 1);
    }
}
