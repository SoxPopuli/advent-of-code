#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Id {
    first: u64,
    last: u64,
}
impl Id {
    pub fn find_invalid(&self) -> impl Iterator<Item = u64> {
        (self.first..=self.last).filter(|x| is_invalid(*x))
    }
}

fn is_invalid(id: u64) -> bool {
    let id_str = id.to_string();
    let len = id_str.len();

    if !len.is_multiple_of(2) {
        // Not even means value can't repeat
        return false;
    }

    let (first, last) = id_str.split_at(len / 2);

    first == last
}

fn parse_input(input: &str) -> Vec<Id> {
    let ids = input.split(',');

    ids.map(|id| {
        let (first, last) = id.split_once('-').unwrap();

        Id {
            first: first.parse().unwrap(),
            last: last.parse().unwrap(),
        }
    })
    .collect()
}

fn part1(input: &[Id]) -> u64 {
    input.iter().flat_map(|id| id.find_invalid()).sum()
}

fn main() {
    let input = { parse_input(&common::read_stdin()) };
    let (time, result) = common::timed(|| part1(&input));
    println!("Part 1: {result} in {time:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validity() {
        assert!(is_invalid(11));
        assert!(is_invalid(22));
        assert!(is_invalid(1010));
        assert!(is_invalid(1188511885));
        assert!(is_invalid(38593859));

        assert!(!is_invalid(10));
        assert!(!is_invalid(12));
        assert!(!is_invalid(2222220));
    }

    #[test]
    fn example() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
        1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
        824824821-824824827,2121212118-2121212124";

        let input = parse_input(input);

        assert_eq!(part1(&input), 1227775554);
    }
}
