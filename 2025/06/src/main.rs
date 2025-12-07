#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Debug)]
struct Column {
    numbers: Vec<i64>,
    operation: Operation,
}
impl Column {
    fn calculate(&self) -> i64 {
        let mut sum = self.numbers.first().cloned().unwrap_or(0);

        for num in self.numbers.iter().skip(1) {
            match self.operation {
                Operation::Add => {
                    sum += num;
                }
                Operation::Multiply => {
                    sum *= num;
                }
            }
        }

        sum
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct CharArray(Vec<Vec<u8>>);
impl CharArray {
    fn new(input: &str) -> Self {
        let x = input
            .lines()
            .map(|x| x.as_bytes().to_vec())
            .collect::<Vec<_>>();

        CharArray(x)
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn find_column_windows(&self) -> Vec<(usize, usize)> {
        let height = self.height();
        let width = self.width();

        fn find_leftmost(row: &[u8], x: usize) -> usize {
            // Starting on the right
            (0..=x).rev().find(|i| row[*i] == b' ').unwrap_or(0)
        }

        let mut windows = vec![];

        let mut x = width - 1;

        while x > 0 {
            let left = (0..height).fold(x, |x, y| find_leftmost(&self.0[y], x).min(x));

            if left == 0 {
                windows.push((left, x));
                break;
            } else {
                windows.push((left + 1, x));
                x = left - 1;
            };
        }

        windows
    }

    fn get_column(&self, window: (usize, usize)) -> Column {
        let numbers = (window.0..=window.1).rev().map(|x| {
            let digits = (0..self.height() - 1).filter_map(move |y| {
                let c = self.0[y][x];

                match c {
                    b' ' => None,
                    x => Some(x),
                }
            });

            digits.rev().enumerate().fold(0, |acc, (i, x)| {
                let digit = (x - b'0') as i64;
                acc + digit * 10_i64.pow(i as u32)
            })
        });

        let op_row = self.0.last().unwrap().as_slice();
        let op = op_row[window.0];

        let op = match op {
            b'+' => Operation::Add,
            b'*' => Operation::Multiply,
            x => panic!("Unexpected op: {x}"),
        };

        Column {
            numbers: numbers.collect(),
            operation: op,
        }
    }
}

fn parse_input(input: &str) -> Vec<Column> {
    let mut columns: Vec<Column> = vec![];

    input
        .lines()
        .take_while(|x| !x.is_empty())
        .for_each(|line| {
            let parts = line.trim().split_ascii_whitespace().enumerate();

            for (i, p) in parts {
                match p {
                    "+" => {
                        columns[i].operation = Operation::Add;
                    }
                    "*" => {
                        columns[i].operation = Operation::Multiply;
                    }

                    x => {
                        let num = x.parse().unwrap();

                        if let Some(col) = columns.get_mut(i) {
                            col.numbers.push(num);
                        } else {
                            columns.push(Column {
                                numbers: vec![num],
                                operation: Operation::Add,
                            });
                        }
                    }
                }
            }
        });

    columns
}

fn part1(columns: &[Column]) -> i64 {
    columns.iter().map(|c| c.calculate()).sum()
}

fn part2(input: &str) -> i64 {
    let char_array = CharArray::new(input);
    let windows = char_array.find_column_windows();

    windows
        .iter()
        .map(|w| char_array.get_column(*w))
        .map(|col| col.calculate())
        .sum()
}

fn main() {
    let input = common::read_stdin();
    let columns = parse_input(&input);

    let (time, result) = common::timed(|| part1(&columns));
    println!("Part 1: {result} in {time:?}");

    let (time, result) = common::timed(|| part2(&input));
    println!("Part 2: {result} in {time:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
        let cols = parse_input(input);

        assert_eq!(part1(&cols), 4277556);
        assert_eq!(part2(input), 3263827);
    }
}
