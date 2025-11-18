use common::timed;
use utils::collect_vec::CollectVec;

fn part1(input: &[i64]) -> i64 {
    let len = input.len();

    for i in 0..len {
        for j in 0..len {
            if (input[i] + input[j]) == 2020 {
                return input[i] * input[j];
            }
        }
    }

    -1
}

fn part2(input: &[i64]) -> i64 {
    let len = input.len();

    for i in 0..len {
        for j in 0..len {
            for k in 0..len {
                if (input[i] + input[j] + input[k]) == 2020 {
                    return input[i] * input[j] * input[k];
                }
            }
        }
    }

    -1
}

fn main() {
    let mut input = common::stream_stdin()
        .lines()
        .map(|line| line.parse::<i64>().expect("Failed to parse input line"))
        .collect_vec();
    input.sort();

    let (time, result) = timed(|| part1(&input));
    println!("Part 1: {result} in {:?}", time);

    let (time, result) = timed(|| part2(&input));
    println!("Part 2: {result} in {:?}", time);
}

// Part 1: 55776 in 333ns
// Part 2: 223162626 in 42.375µs

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let input = [1721, 979, 366, 299, 675, 1456];
        assert_eq!(super::part1(&input), 514579);
    }

    #[test]
    fn part2() {
        let input = [1721, 979, 366, 299, 675, 1456];
        assert_eq!(super::part2(&input), 241861950);
    }
}
