#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Id {
    first: u64,
    last: u64,
}
impl Id {
    pub fn find_repeated(&self) -> impl Iterator<Item = u64> {
        (self.first..=self.last).filter(|x| is_repeated(*x))
    }

    pub fn find_repeated_sequence(&self) -> impl Iterator<Item = u64> {
        (self.first..=self.last).filter(|x| is_repeated_sequence(*x))
    }
}

fn is_repeated(id: u64) -> bool {
    let id_str = id.to_string();
    let len = id_str.len();

    if !len.is_multiple_of(2) {
        // Not even means value can't repeat
        return false;
    }

    let (first, last) = id_str.split_at(len / 2);

    first == last
}

fn is_repeated_sequence(id: u64) -> bool {
    fn repeats(pattern: &str, s: &str) -> bool {
        if s.is_empty() {
            return true;
        }

        let sub_str = &s[0..pattern.len()];

        if pattern != sub_str {
            false
        } else {
            repeats(pattern, &s[pattern.len()..])
        }
    }

    let id_str = id.to_string();
    let id_len = id_str.len();

    // No point in checking the whole value
    for seq_len in 1..id_len {
        if !id_len.is_multiple_of(seq_len) {
            continue;
        }

        let window = &id_str[0..seq_len];

        if repeats(window, &id_str[seq_len..]) {
            return true;
        }
    }

    false
}

fn parse_input(input: &str) -> Vec<Id> {
    let ids = input.trim().split(',');

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
    input.iter().flat_map(|id| id.find_repeated()).sum()
}

fn part2(input: &[Id]) -> u64 {
    input
        .iter()
        .flat_map(|id| id.find_repeated_sequence())
        .sum()
}

fn main() {
    let input = { parse_input(&common::read_stdin()) };
    let (time, result) = common::timed(|| part1(&input));
    println!("Part 1: {result} in {time:?}");

    let (time, result) = common::timed(|| part2(&input));
    println!("Part 2: {result} in {time:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validity() {
        assert!(is_repeated(11));
        assert!(is_repeated(22));
        assert!(is_repeated(1010));
        assert!(is_repeated(38593859));
        assert!(is_repeated(1188511885));

        assert!(!is_repeated(10));
        assert!(!is_repeated(12));
        assert!(!is_repeated(2222220));

        // ─────────────── pt2 ───────────────
        assert!(is_repeated_sequence(11));
        assert!(is_repeated_sequence(22));
        assert!(is_repeated_sequence(111));
        assert!(is_repeated_sequence(999));
        assert!(is_repeated_sequence(1010));
        assert!(is_repeated_sequence(565656));
        assert!(is_repeated_sequence(38593859));
        assert!(is_repeated_sequence(824824824));
        assert!(is_repeated_sequence(1188511885));
        assert!(is_repeated_sequence(2121212121));

        assert!(!is_repeated_sequence(100));
        assert!(!is_repeated_sequence(123));
        assert!(!is_repeated_sequence(2222220));
    }

    #[test]
    fn example() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
        1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
        824824821-824824827,2121212118-2121212124";

        let input = parse_input(input);

        assert_eq!(part1(&input), 1227775554);
        assert_eq!(part2(&input), 4174379265);
    }
}
