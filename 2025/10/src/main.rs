#[derive(Debug, PartialEq, Eq, Clone)]
struct Lights {
    count: usize,
    state: usize,
}
impl std::fmt::Display for Lights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;

        f.write_char('[')?;

        for i in 0..self.count {
            match self.get(i) {
                true => f.write_char('#')?,
                false => f.write_char('.')?,
            }
        }

        f.write_char(']')
    }
}
impl Lights {
    fn empty(count: usize) -> Self {
        Self { count, state: 0 }
    }

    fn all_on(&self) -> bool {
        self.state.count_ones() >= self.count as u32
    }

    fn from_string(s: &str) -> Self {
        let inner = s.trim().trim_matches(['[', ']']);

        let lights = inner.as_bytes().iter().map(|x| match x {
            b'.' => 0,
            b'#' => 1,
            x => panic!("Unexpected light char: {x}"),
        });

        let state = lights.enumerate().fold(0, |state, (i, x)| state | (x << i));

        Self {
            count: inner.len(),
            state,
        }
    }

    fn from_indices(indices: &[usize]) -> Self {
        let count = indices.len().try_into().unwrap();

        let state = indices
            .iter()
            .map(|i| 1 << i)
            .fold(0, |state, bit| state | bit);

        Self { count, state }
    }

    fn get(&self, index: usize) -> bool {
        (self.state >> index) & 1 == 1
    }

    fn combine(&self, rhs: &Self) -> Self {
        Self {
            count: self.count.max(rhs.count),
            state: self.state ^ rhs.state,
        }
    }

    fn toggle(&self, indices: &[usize]) -> Self {
        let rhs = Self::from_indices(indices);

        assert!(indices.len() <= self.count);

        Self {
            count: self.count,
            state: self.state ^ rhs.state,
        }
    }

    fn press_button(&self, btn: &Button) -> Self {
        let new_state = self.state ^ btn.value;
        Self {
            count: self.count,
            state: new_state,
        }
    }
}
impl std::ops::BitXor for Lights {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        self.combine(&rhs)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Button {
    indices: Vec<u8>,
    value: usize,
}
impl Button {
    fn new(items: &[u8]) -> Self {
        let value = items
            .iter()
            .map(|i| 1 << i)
            .fold(0, |state, bit| state | bit);

        Self {
            indices: items.to_vec(),
            value,
        }
    }
}
impl std::fmt::Display for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

#[derive(Debug)]
struct Machine {
    lights: Lights,
    buttons: Vec<Button>,
    jolts: Vec<u8>,
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .lines()
        .filter_map(|line| {
            let (lights, rest) = line.trim().split_once(' ')?;

            let rest = rest.trim();

            if rest.is_empty() {
                return None;
            }

            let lights = Lights::from_string(lights);

            let parts = rest.split(' ');
            let mut buttons = vec![];
            let mut jolts = vec![];

            for part in parts {
                match part.as_bytes() {
                    [b'(', content @ .., b')'] => {
                        let button = content
                            .split(|x| *x == b',')
                            .map(|x| {
                                unsafe { String::from_utf8_unchecked(x.to_vec()) }
                                    .parse()
                                    .unwrap()
                            })
                            .collect::<Vec<u8>>();

                        buttons.push(Button::new(&button))
                    }
                    [b'{', content @ .., b'}'] => {
                        jolts = content
                            .split(|x| *x == b',')
                            .filter_map(|x| {
                                unsafe { String::from_utf8_unchecked(x.to_vec()) }
                                    .parse()
                                    .ok()
                            })
                            .collect::<Vec<u8>>();
                    }
                    _ => continue,
                }
            }

            Some(Machine {
                lights,
                buttons,
                jolts,
            })
        })
        .collect()
}

fn machine_button_presses(machine: &Machine) -> usize {
    use itertools::Itertools;

    (0..)
        .find_map(|i| {
            let mut combos = machine.buttons.iter().combinations(i);
            let combo = combos.find(|btns| {
                let empty = Lights::empty(machine.lights.count);

                let light = btns
                    .iter()
                    .fold(empty, |lights, btn| lights.press_button(btn));

                light == machine.lights
            });

            combo.map(|x| x.len())
        })
        .unwrap_or_else(|| panic!("{machine:?}"))
}

fn part1(machines: &[Machine]) -> usize {
    machines.iter().map(machine_button_presses).sum()
}

fn main() {
    let machines = parse_input(&common::read_stdin());

    let (time, result) = common::timed(|| part1(&machines));
    println!("Part 1: {result} in {time:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lights() {
        let l = Lights::from_string("[.##.]");
        assert_eq!(l.to_string(), "[.##.]");

        let l = Lights::from_string("[#.....]");
        let l = l.press_button(&Button::new(&[0, 3, 4]));
        assert_eq!(l.to_string(), "[...##.]");
    }

    #[test]
    fn example() {
        let input = r#"
            [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
            [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
            [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
        "#;
        let input = parse_input(input);
        assert_eq!(part1(&input), 7);
    }

    #[test]
    fn button_presses() {
        let machine = Machine {
            lights: Lights::from_string("[.##.]"),
            buttons: vec![
                Button::new(&[3]),
                Button::new(&[1, 3]),
                Button::new(&[2]),
                Button::new(&[2, 3]),
                Button::new(&[0, 2]),
                Button::new(&[0, 1]),
            ],
            jolts: vec![],
        };

        assert_eq!(machine_button_presses(&machine), 2);

        let machine = Machine {
            lights: Lights::from_string("[...#.]"),
            buttons: vec![
                Button::new(&[0, 2, 3, 4]),
                Button::new(&[2, 3]),
                Button::new(&[0, 4]),
                Button::new(&[0, 1, 2]),
                Button::new(&[1, 2, 3, 4]),
            ],
            jolts: vec![],
        };

        assert_eq!(machine_button_presses(&machine), 3);
    }
}
