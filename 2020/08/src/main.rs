#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Instruction {
    Acc,
    Jmp,
    Nop,
}

type Instructions = Vec<(Instruction, isize)>;

fn calc_jmp(ip: usize, value: isize) -> usize {
    if value < 0 {
        ip - (value.abs() as usize)
    } else {
        ip + value as usize
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum TerminationKind {
    Loop,
    Continue,
    End,
}
impl TerminationKind {
    fn should_continue(&self) -> bool {
        matches!(self, TerminationKind::Continue)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct State {
    instruction_pointer: usize,
    accumulator: isize,
    instructions: Vec<(Instruction, isize)>,
    visited: Vec<bool>,
}
impl State {
    fn new(instructions: Instructions) -> State {
        let visited = Vec::from_iter(std::iter::repeat_n(false, instructions.len()));

        Self {
            instruction_pointer: 0,
            accumulator: 0,
            instructions,
            visited,
        }
    }

    fn reset(&mut self) {
        self.instruction_pointer = 0;
        self.accumulator = 0;

        for v in self.visited.iter_mut() {
            *v = false;
        }
    }

    fn loop_to_end(&mut self) -> TerminationKind {
        loop {
            match self.step() {
                TerminationKind::Continue => continue,
                x => return x,
            }
        }
    }

    fn step(&mut self) -> TerminationKind {
        match self.instructions.get(self.instruction_pointer) {
            None => TerminationKind::End,
            Some((op, value)) => {
                if self.visited[self.instruction_pointer] {
                    return TerminationKind::Loop;
                } else {
                    self.visited[self.instruction_pointer] = true;
                }

                match op {
                    Instruction::Acc => self.accumulator += *value,
                    Instruction::Jmp => {
                        self.instruction_pointer = calc_jmp(self.instruction_pointer, *value);
                        return TerminationKind::Continue;
                    }
                    Instruction::Nop => {}
                }

                self.instruction_pointer += 1;

                TerminationKind::Continue
            }
        }
    }
}

fn read_instructions(input: &str) -> Instructions {
    input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (op, value) = line.split_once(' ').unwrap();

            let op = match op {
                "nop" => Instruction::Nop,
                "acc" => Instruction::Acc,
                "jmp" => Instruction::Jmp,
                x => panic!("Unexpected instruction: {x}"),
            };

            let value = value.parse().unwrap();

            (op, value)
        })
        .collect::<Vec<_>>()
}

fn part_1(input: &str) -> isize {
    let mut state = State::new(read_instructions(input));

    while state.step().should_continue() {}

    state.accumulator
}

fn part_2(input: &str) -> isize {
    let state = State::new(read_instructions(input));

    let nop_indices = state
        .instructions
        .iter()
        .enumerate()
        .filter_map(|(i, (instruction, _))| {
            if *instruction == Instruction::Nop {
                Some(i)
            } else {
                None
            }
        });

    let end_state = nop_indices
        .map(|i| {
            let mut state = state.clone();
            state.instructions[i].0 = Instruction::Jmp;

            let term_kind = state.loop_to_end();

            (state, term_kind)
        })
        .find(|(_, term)| *term == TerminationKind::End);

    if let Some((state, _)) = end_state {
        return state.accumulator;
    }

    let jump_indices = state
        .instructions
        .iter()
        .enumerate()
        .filter_map(|(i, (instruction, _))| {
            if *instruction == Instruction::Jmp {
                Some(i)
            } else {
                None
            }
        });

    let end_state = jump_indices
        .map(|i| {
            let mut state = state.clone();
            state.instructions[i].0 = Instruction::Nop;

            let term_kind = state.loop_to_end();

            (state, term_kind)
        })
        .find(|(_, term)| *term == TerminationKind::End);

    end_state.unwrap().0.accumulator
}

fn main() {
    let input = common::read_stdin();

    let (time, result) = common::timed(|| part_1(&input));
    println!("Part 1: {result} in {time:?}");

    let (time, result) = common::timed(|| part_2(&input));
    println!("Part 2: {result} in {time:?}");
}
