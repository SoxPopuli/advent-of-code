#[derive(Debug)]
pub enum Validity<T> {
    Valid(T),
    Invalid(T),
}

#[derive(Debug, Clone)]
pub struct Values {
    items: Vec<u64>,
    start: usize,
    end: usize,
}
impl Values {
    fn next(&mut self) -> Option<Validity<u64>> {
        let next = *self.items.get(self.end)?;

        for i in self.start..self.end {
            for j in self.start..self.end {
                let sum = self.items[i] + self.items[j];
                if next == sum {
                    self.start += 1;
                    self.end += 1;
                    return Some(Validity::Valid(sum));
                }
            }
        }

        Some(Validity::Invalid(next))
    }
}

fn part_1(mut values: Values) -> u64 {
    loop {
        match values.next() {
            Some(Validity::Invalid(x)) => return x,
            Some(Validity::Valid(_)) => continue,
            None => panic!("End of values"),
        }
    }
}

#[derive(Debug)]
pub struct Range {
    start: usize,
    end: usize,
}

fn try_find_contiguous_sum(target: u64, values: &[u64], width: usize) -> Option<Range> {
    let end = values.len() - width;

    for i in 0..end {
        let slice = &values[i..i + width];
        let sum: u64 = slice.iter().sum();

        if sum == target {
            return Some(Range {
                start: i,
                end: i + width - 1,
            });
        }
    }

    None
}

fn part_2(values: Values, target: u64) -> u64 {
    for width in 2..100 {
        if let Some(range) = try_find_contiguous_sum(target, &values.items, width) {
            let min = values.items[range.start..=range.end]
                .iter()
                .copied()
                .min()
                .unwrap();
            let max = values.items[range.start..=range.end]
                .iter()
                .copied()
                .max()
                .unwrap();

            return min + max;
        }
    }

    panic!("Range not found")
}

fn main() {
    let preamble = 25;

    let input = common::read_stdin();
    let values = input
        .lines()
        .take_while(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();
    let values = Values {
        items: values,
        start: 0,
        end: preamble,
    };

    let (time, result) = common::timed(|| part_1(values.clone()));
    println!("Part 1: {result} in {time:?}");

    let (time, result) = common::timed(|| part_2(values, result));
    println!("Part 2: {result} in {time:?}");
}

// Part 1: 22406676 in 20.635µs
// Part 2: 2942387 in 26.89µs
