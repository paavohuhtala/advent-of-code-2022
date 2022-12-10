const INPUT: &str = include_str!("./day10.input");

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Noop,
    Addx(i64),
}

impl Instruction {
    fn cycles(self) -> u8 {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        }
    }
}

fn parse_instruction(s: &str) -> Instruction {
    let mut parts = s.split_whitespace();
    match parts.next() {
        Some("noop") => Instruction::Noop,
        Some("addx") => Instruction::Addx(parts.next().unwrap().parse().unwrap()),
        _ => panic!("Unknown instruction"),
    }
}

fn parse_input() -> Vec<Instruction> {
    INPUT.lines().map(parse_instruction).collect()
}

struct Vm {
    x: i64,
    cycles: i64,
    halted: bool,
    instructions: Vec<Instruction>,
    instruction_pointer: usize,
    cycles_to_next_instruction: u8,
}

impl Vm {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            x: 1,
            cycles: 1,
            halted: false,
            cycles_to_next_instruction: instructions[0].cycles(),
            instructions,
            instruction_pointer: 0,
        }
    }

    fn get_current_instruction(&self) -> Option<Instruction> {
        self.instructions.get(self.instruction_pointer).copied()
    }

    fn execute_cycle(&mut self) {
        if self.halted {
            return;
        }

        if self.cycles_to_next_instruction > 0 {
            self.cycles += 1;
            self.cycles_to_next_instruction -= 1;
        }

        if self.cycles_to_next_instruction == 0 {
            let current_instruction = self.get_current_instruction();
            self.execute_instruction(current_instruction.unwrap());
            self.instruction_pointer += 1;

            if let Some(instruction) = self.get_current_instruction() {
                self.cycles_to_next_instruction = instruction.cycles();
            } else {
                self.halted = true;
            }
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Noop => {}
            Instruction::Addx(x) => {
                self.x += x;
            }
        }
    }
}

pub fn day10a() {
    let instructions = parse_input();
    let mut vm = Vm::new(instructions);

    let mut signal_sum = 0;

    while !vm.halted {
        vm.execute_cycle();

        if (vm.cycles + 20) % 40 == 0 {
            let signal_strength = vm.cycles * vm.x;
            signal_sum += signal_strength;
        }
    }

    println!("Day 10a: {}", signal_sum);
}

pub fn day10b() {
    let instructions = parse_input();
    let mut vm = Vm::new(instructions);

    let mut screen_buffer: [char; 40 * 6] = ['.'; 40 * 6];

    for i in 0..screen_buffer.len() {
        let x_pos = i % 40;
        if (vm.x - x_pos as i64).abs() <= 1 {
            screen_buffer[i] = '#';
        } else {
            screen_buffer[i] = '.';
        }

        vm.execute_cycle();
    }

    for y in 0..6 {
        for x in 0..40 {
            print!("{}", screen_buffer[y * 40 + x]);
        }
        println!();
    }
}
