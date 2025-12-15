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

fn solve_machine(machine: &Machine) -> u64 {
    use z3::{Solver, ast::Int};

    let solver = Solver::new();

    let btn_vars = machine
        .buttons
        .iter()
        .enumerate()
        .map(|(i, _)| Int::new_const(format!("a{i}")))
        .collect::<Vec<_>>();

    for b in &btn_vars {
        solver.assert(b.ge(0));
    }

    for (i, v) in machine.jolts.iter().enumerate() {
        let jolt_vars = machine
            .buttons
            .iter()
            .enumerate()
            .filter_map(|(j, btn)| {
                if btn.indices.contains(&(i as u8)) {
                    Some(&btn_vars[j])
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if !jolt_vars.is_empty() {
            let sum = Int::add(jolt_vars.as_slice());
            solver.assert(sum.eq(Int::from_u64(*v as u64)));
        }
    }

    let mut n = 0;
    while let z3::SatResult::Sat = solver.check() {
        let model = solver.get_model().unwrap();
        n = btn_vars
            .iter()
            .map(|x| model.eval(x, true).unwrap().as_u64().unwrap())
            .sum();

        let sum = Int::add(btn_vars.as_slice());
        solver.assert(sum.lt(Int::from_u64(n)));
    }

    n
}

fn part2(machines: &[Machine]) -> u64 {
    machines.iter()
        .map(solve_machine)
        .sum()
}

fn main() {
    let machines = parse_input(&common::read_stdin());

    let (time, result) = common::timed(|| part1(&machines));
    println!("Part 1: {result} in {time:?}");

    let (time, result) = common::timed(|| part2(&machines));
    println!("Part 2: {result} in {time:?}");
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
        assert_eq!(part2(&input), 33);
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

    #[test]
    fn solve() {
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
            jolts: vec![3, 5, 4, 7],
        };

        assert_eq!(solve_machine(&machine), 10);

        let machine = Machine {
            lights: Lights::from_string("[...#.]"),
            buttons: vec![
                Button::new(&[0, 2, 3, 4]),
                Button::new(&[2, 3]),
                Button::new(&[0, 4]),
                Button::new(&[0, 1, 2]),
                Button::new(&[1, 2, 3, 4]),
            ],
            jolts: vec![7, 5, 12, 7, 2],
        };

        assert_eq!(solve_machine(&machine), 12);
    }
}
