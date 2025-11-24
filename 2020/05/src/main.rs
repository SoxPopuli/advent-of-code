#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum RowHalf {
    Front,
    Back,
}
impl From<RowHalf> for Half {
    fn from(value: RowHalf) -> Self {
        match value {
            RowHalf::Front => Half::Lower,
            RowHalf::Back => Half::Upper,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum ColumnHalf {
    Left,
    Right,
}
impl From<ColumnHalf> for Half {
    fn from(value: ColumnHalf) -> Self {
        match value {
            ColumnHalf::Left => Half::Lower,
            ColumnHalf::Right => Half::Upper,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Half {
    Upper,
    Lower,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Region {
    min: u8,
    max: u8,
}
impl Region {
    const WHOLE_ROW: Region = Region { min: 0, max: 127 };
    const WHOLE_COLUMN: Region = Region { min: 0, max: 7 };

    fn partition(&self, half: impl Into<Half>) -> Self {
        let half_size = self.size() / 2;

        match half.into() {
            Half::Lower => Self {
                min: self.min,
                max: self.max - half_size,
            },
            Half::Upper => Self {
                min: self.min + half_size,
                max: self.max,
            },
        }
    }

    fn size(&self) -> u8 {
        self.max - self.min + 1
    }
}

fn get_halves(line: &str) -> ([RowHalf; 7], [ColumnHalf; 3]) {
    let mut row_halves = [RowHalf::Front; _];
    let mut col_halves = [ColumnHalf::Left; _];

    (0..7).for_each(|i| match line.chars().nth(i) {
        Some('F') => row_halves[i] = RowHalf::Front,
        Some('B') => row_halves[i] = RowHalf::Back,

        x => panic!("Unexpected value: {x:?}"),
    });

    (7..10)
        .enumerate()
        .for_each(|(i, c)| match line.chars().nth(c) {
            Some('R') => col_halves[i] = ColumnHalf::Right,
            Some('L') => col_halves[i] = ColumnHalf::Left,

            x => panic!("Unexpected value: {x:?}"),
        });

    (row_halves, col_halves)
}

fn read_pass(line: &str) -> u32 {
    let (row_halves, col_halves) = get_halves(line);

    let row = {
        let mut row = Region::WHOLE_ROW;
        for r in row_halves {
            row = row.partition(r);
        }
        row.min
    };

    let col = {
        let mut col = Region::WHOLE_COLUMN;
        for c in col_halves {
            col = col.partition(c);
        }
        col.min
    };

    seat_id(row, col)
}

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(read_pass)
        .max()
        .unwrap()
}

fn part_2(input: &str) -> u32 {
    use std::collections::BinaryHeap;

    let ids = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(read_pass)
        .collect::<BinaryHeap<_>>();

    let mut it = ids.into_iter().skip(1).peekable();

    while let Some(current) = it.next()
        && let Some(next) = it.peek()
    {
        if next - current != 1 {
            return next - 1;
        }
    }

    panic!("no value found")
}

fn main() {
    let input = common::read_stdin();

    let (time, result) = common::timed(|| part_1(&input));
    println!("Part 1: {result} in {time:?}");

    let (time, result) = common::timed(|| part_2(&input));
    println!("Part 2: {result} in {time:?}");
}

fn seat_id(row: u8, column: u8) -> u32 {
    (row as u32 * 8) + column as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(read_pass("BFFFBBFRRR"), 567);
        assert_eq!(read_pass("FFFBBBFRRR"), 119);
        assert_eq!(read_pass("BBFFBBFRLL"), 820);
    }

    #[test]
    fn partition_test() {
        assert_eq!(
            Region::WHOLE_ROW.partition(RowHalf::Front),
            Region { min: 0, max: 63 }
        );

        assert_eq!(
            Region { min: 0, max: 63 }.partition(RowHalf::Back),
            Region { min: 32, max: 63 }
        );

        assert_eq!(
            Region { min: 32, max: 63 }.partition(RowHalf::Front),
            Region { min: 32, max: 47 }
        );

        assert_eq!(
            Region { min: 32, max: 47 }.partition(RowHalf::Back),
            Region { min: 40, max: 47 }
        );

        assert_eq!(
            Region { min: 40, max: 47 }.partition(RowHalf::Back),
            Region { min: 44, max: 47 }
        );

        assert_eq!(
            Region { min: 44, max: 47 }.partition(RowHalf::Front),
            Region { min: 44, max: 45 }
        );

        assert_eq!(
            Region { min: 44, max: 45 }.partition(RowHalf::Front),
            Region { min: 44, max: 44 }
        );

        assert_eq!(
            Region::WHOLE_COLUMN.partition(ColumnHalf::Right),
            Region { min: 4, max: 7 }
        );
        assert_eq!(
            Region { min: 4, max: 7 }.partition(ColumnHalf::Left),
            Region { min: 4, max: 5 }
        );
        assert_eq!(
            Region { min: 4, max: 5 }.partition(ColumnHalf::Right),
            Region { min: 5, max: 5 }
        );
    }
}
