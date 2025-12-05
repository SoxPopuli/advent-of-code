#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RangeKind {
    Start,
    End,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Range {
    start: u64,
    end: u64,
}
impl Range {
    #[allow(dead_code)]
    pub fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }

    fn is_inside(&self, value: u64) -> bool {
        value >= self.start && value <= self.end
    }

    fn merge(ranges: &[Range]) -> Vec<Range> {
        if ranges.is_empty() {
            return vec![];
        }

        let mut ranges = ranges.to_vec();
        ranges.sort_by(|a, b| a.start.cmp(&b.start));

        let mut merged = vec![ranges[0].clone()];

        for current in ranges.iter().skip(1) {
            let last = merged.last_mut().unwrap();

            if current.start <= last.end {
                last.end = last.end.max(current.end);
            } else {
                merged.push(current.clone());
            }
        }

        merged
    }
}

fn parse_input(input: &str) -> (Vec<Range>, Vec<u64>) {
    let mut lines = input.lines();

    let mut ranges = vec![];
    let mut values = vec![];

    while let Some(line) = lines.next().map(|x| x.trim())
        && !line.is_empty()
    {
        let range = line.split_once('-').unwrap();

        ranges.push(Range {
            start: range.0.parse().unwrap(),
            end: range.1.parse().unwrap(),
        });
    }

    while let Some(line) = lines.next().map(|x| x.trim())
        && !line.is_empty()
    {
        let value = line.parse().unwrap();
        values.push(value);
    }

    (ranges, values)
}

fn part1(ranges: &[Range], values: &[u64]) -> usize {
    let merged_ranges = Range::merge(ranges);

    values
        .iter()
        .filter(|x| merged_ranges.iter().any(|range| range.is_inside(**x)))
        .count()
}

fn part2(ranges: &[Range]) -> usize {
    let merged_ranges = Range::merge(ranges);

    merged_ranges
        .into_iter()
        .flat_map(|x| x.start..=x.end)
        .count()
}

fn main() {
    let (ranges, values) = parse_input(&common::read_stdin());

    let (time, result) = common::timed(|| part1(&ranges, &values));
    println!("Part 1: {result} in {time:?}");

    let (time, result) = common::timed(|| part2(&ranges));
    println!("Part 2: {result} in {time:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> &'static str {
        "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32"
    }

    #[test]
    fn example() {
        let input = parse_input(example_input());

        assert_eq!(part1(&input.0, &input.1), 3);
        assert_eq!(part2(&input.0), 14);
    }

    #[test]
    fn merged() {
        let input = [
            Range::new(3, 5),
            Range::new(10, 14),
            Range::new(12, 18),
            Range::new(16, 20),
        ];
        assert_eq!(
            Range::merge(&input),
            vec![Range::new(3, 5), Range::new(10, 20),]
        );
    }
}
