#[derive(Debug)]
struct BatteryBank(Vec<u8>);
impl BatteryBank {
    fn largest_value(&self) -> u32 {
        let mut max = u32::MIN;

        for i in 0..self.0.len() {
            for j in i + 1..self.0.len() {
                let jolts = (self.0[i] as u32 * 10) + self.0[j] as u32;
                max = max.max(jolts);
            }
        }

        max
    }

    fn lowest_indices(&self) -> [usize; 3] {
        let mut lowest = [0; 3];

        let sorted = {
            let mut x = self.0.iter().enumerate().collect::<Vec<_>>();

            x.sort_by(|a, b| a.1.cmp(b.1));

            x
        };

        let lowest_3 = &sorted[..3];

        for (i, (idx, _)) in lowest_3.iter().enumerate() {
            lowest[i] = *idx;
        }

        lowest
    }

    fn largest_12(&self) -> u64 {
        let lowest_3 = self.lowest_indices();
        let mut jolts: u64 = 0;

        for (i, x) in self.0.iter().enumerate() {
            if !lowest_3.contains(&i) {
                jolts *= 10;
                jolts += *x as u64;
            }
        }

        dbg!(jolts)
    }
}

fn parse_input(input: &str) -> Vec<BatteryBank> {
    input
        .lines()
        .take_while(|x| !x.is_empty())
        .map(|line| {
            let batteries = line
                .trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect();

            BatteryBank(batteries)
        })
        .collect()
}

fn part1(input: &[BatteryBank]) -> u32 {
    input.iter().map(|x| x.largest_value()).sum()
}

fn part2(input: &[BatteryBank]) -> u64 {
    input.iter()
        .map(|x| x.largest_12())
        .sum()
}

fn main() {
    let input = parse_input(&common::read_stdin());

    let (time, result) = common::timed(|| part1(&input));
    println!("Part 1: {result} in {time:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "987654321111111\n\
            811111111111119\n\
            234234234234278\n\
            818181911112111";
        let input = parse_input(input);

        assert_eq!(part1(&input), 357);
        assert_eq!(part2(&input), 3121910778619);
    }
}
