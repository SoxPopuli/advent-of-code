use common::{timed, Tap};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Gate {
    And,
    Or,
    Xor,
}
impl Gate {
    fn apply(&self, a: u8, b: u8) -> u8 {
        match self {
            Self::And => a & b,
            Self::Or => a | b,
            Self::Xor => a ^ b,
        }
    }
}

#[derive(Debug, Clone)]
struct Command {
    a: String,
    b: String,
    gate: Gate,
    output: String,
}

#[derive(Debug, Clone)]
struct Device {
    inputs: HashMap<String, u8>,
    commands: Vec<Command>,
}
impl Device {
    fn new(input: &str) -> Self {
        let (inputs, commands) = input.split_once("\n\n").unwrap();

        let inputs = inputs
            .lines()
            .filter_map(|line| {
                let (variable, value) = line.trim_ascii().split_once(": ")?;
                Some((variable.into(), value.parse().unwrap()))
            })
            .collect();

        let commands = commands
            .lines()
            .filter_map(|line| {
                let mut line = line.trim_ascii().split_ascii_whitespace();
                let a = line.next()?;
                let gate = line.next()?;
                let b = line.next()?;
                let _ = line.next()?;
                let output = line.next()?;

                Some(Command {
                    a: a.into(),
                    b: b.into(),
                    output: output.into(),
                    gate: match gate {
                        "AND" => Gate::And,
                        "OR" => Gate::Or,
                        "XOR" => Gate::Xor,
                        x => panic!("Unexpected gate: {x}"),
                    },
                })
            })
            .collect::<Vec<_>>();

        Device { inputs, commands }
    }

    fn run(mut self) -> Vec<(String, u8)> {
        let mut visited = HashSet::new();

        while visited.len() < self.commands.len() {
            for (i, c) in self.commands.iter().enumerate() {
                if visited.contains(&i) {
                    continue;
                } else if self.inputs.contains_key(&c.a) && self.inputs.contains_key(&c.b) {
                    visited.insert(i);

                    let a = self.inputs[&c.a];
                    let b = self.inputs[&c.b];
                    let val = c.gate.apply(a, b);

                    self.inputs
                        .entry(c.output.clone())
                        .and_modify(|e| *e = val)
                        .or_insert(val);
                }
            }
        }

        self.inputs.into_iter().collect()
    }

    fn get_swaps(&self) -> Vec<String> {
        let max_z = self
            .commands
            .iter()
            .filter(|x| x.output.starts_with('z'))
            .map(|x| &x.output)
            .max()
            .unwrap();
        let mut incorrect = HashSet::new();

        for cmd in &self.commands {
            #[allow(clippy::if_same_then_else)]
            if cmd.output.starts_with('z') && cmd.gate != Gate::Xor && cmd.output != *max_z {
                incorrect.insert(cmd.output.clone());
            } else if cmd.gate == Gate::Xor
                && !cmd.a.starts_with(['x', 'y', 'z'])
                && !cmd.b.starts_with(['x', 'y', 'z'])
                && !cmd.output.starts_with(['x', 'y', 'z'])
            {
                incorrect.insert(cmd.output.clone());
            } else if cmd.gate == Gate::And && (cmd.a != "x00" && cmd.b != "x00") {
                for c2 in &self.commands {
                    if (cmd.output == c2.a || cmd.output == c2.b) && c2.gate != Gate::Or {
                        incorrect.insert(cmd.output.clone());
                    }
                }
            } else if cmd.gate == Gate::Xor {
                for c2 in &self.commands {
                    if (cmd.output == c2.a || cmd.output == c2.b) && c2.gate == Gate::Or {
                        incorrect.insert(cmd.output.clone());
                    }
                }
            }
        }

        let mut incorrect = Vec::from_iter(incorrect);
        incorrect.sort();
        incorrect
    }
}

fn combine(data: &[(String, u8)], prefix: char) -> u64 {
    let data = data
        .iter()
        .filter(|x| x.0.starts_with(prefix))
        .collect::<Vec<_>>()
        .tap_mut(|x| x.sort_by_key(|x| &x.0));

    data.into_iter().fold(0, |acc, (name, val)| {
        let shift: u64 = name[1..].parse().unwrap();
        let val = (*val as u64) << shift;

        acc | val
    })
}

fn main() {
    let device = Device::new(&common::read_stdin());
    let (time, result) = timed(|| combine(&device.clone().run(), 'z'));
    println!("Part 1: {result} in {}μs", time.as_micros());

    let (time, swaps) = timed(|| device.get_swaps().join(","));
    println!("Part 2: {swaps} in {}μs", time.as_micros());
}

// Part 1: 47666458872582 in 481μs
// Part 2: dnt,gdf,gwc,jst,mcm,z05,z15,z30 in 297μs
