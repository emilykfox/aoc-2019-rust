pub mod memory;

use memory::Memory;
use std::collections::VecDeque;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ExitReason {
    NormalExit,
    NoInput,
}

#[derive(Clone)]
pub struct Interpreter {
    program: Vec<isize>,
    noun: Option<isize>,
    verb: Option<isize>,
    inputs: VecDeque<isize>,

    memory: Memory,
    relative_base: isize,
    fresh_start: bool,
    instr_ptr: usize,
    outputs: VecDeque<isize>,
}

impl Interpreter {
    pub fn new(program: &str) -> Self {
        let program = program
            .split(',')
            .map(|int| int.parse::<isize>().expect(int))
            .collect::<Vec<isize>>();
        Interpreter {
            program,
            noun: None,
            verb: None,
            inputs: VecDeque::new(),
            memory: Memory::new(),
            relative_base: 0,
            fresh_start: true,
            instr_ptr: 0,
            outputs: VecDeque::new(),
        }
    }

    pub fn set_noun_verb(&mut self, noun: isize, verb: isize) {
        self.noun = Some(noun);
        self.verb = Some(verb);
    }

    pub fn unset_noun_verb(&mut self) {
        self.noun = None;
        self.verb = None;
    }

    pub fn insert_input(&mut self, input: isize) {
        self.inputs.push_back(input);
    }

    pub fn clear_inputs(&mut self) {
        self.inputs.clear();
    }

    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    pub fn get_output(&mut self) -> Option<isize> {
        self.outputs.pop_front()
    }

    pub fn drain_outputs(&mut self) -> Vec<isize> {
        let outputs = std::mem::take(&mut self.outputs);
        outputs.into_iter().collect::<Vec<_>>()
    }

    // Loads the program passed to `new` into memory and executes it.
    pub fn execute(&mut self) -> ExitReason {
        if self.fresh_start {
            self.memory.load(&self.program);

            self.relative_base = 0;

            if let Some(noun) = self.noun {
                self.memory[1] = noun;
            }
            if let Some(verb) = self.verb {
                self.memory[2] = verb;
            }

            self.instr_ptr = 0;
            self.fresh_start = false;
        }
        loop {
            match self.memory[self.instr_ptr] % 100 {
                1 => {
                    let locations = self.parameter_locations(self.instr_ptr, 3);
                    self.memory[locations[2]] =
                        self.memory[locations[0]] + self.memory[locations[1]];

                    self.instr_ptr += 4;
                }
                2 => {
                    let locations = self.parameter_locations(self.instr_ptr, 3);
                    self.memory[locations[2]] =
                        self.memory[locations[0]] * self.memory[locations[1]];

                    self.instr_ptr += 4;
                }
                3 => {
                    let locations = self.parameter_locations(self.instr_ptr, 1);
                    let input = self.inputs.pop_front();
                    if let Some(input) = input {
                        self.memory[locations[0]] = input;

                        self.instr_ptr += 2;
                    } else {
                        return ExitReason::NoInput;
                    }
                }
                4 => {
                    let locations = self.parameter_locations(self.instr_ptr, 1);
                    self.outputs.push_back(self.memory[locations[0]]);

                    self.instr_ptr += 2;
                }
                5 => {
                    let locations = self.parameter_locations(self.instr_ptr, 2);
                    if self.memory[locations[0]] != 0 {
                        self.instr_ptr = self.memory[locations[1]] as usize;
                    } else {
                        self.instr_ptr += 3;
                    }
                }
                6 => {
                    let locations = self.parameter_locations(self.instr_ptr, 2);
                    if self.memory[locations[0]] == 0 {
                        self.instr_ptr = self.memory[locations[1]] as usize;
                    } else {
                        self.instr_ptr += 3;
                    }
                }
                7 => {
                    let locations = self.parameter_locations(self.instr_ptr, 3);
                    self.memory[locations[2]] =
                        (self.memory[locations[0]] < self.memory[locations[1]]) as isize;
                    self.instr_ptr += 4;
                }
                8 => {
                    let locations = self.parameter_locations(self.instr_ptr, 3);
                    self.memory[locations[2]] =
                        (self.memory[locations[0]] == self.memory[locations[1]]) as isize;
                    self.instr_ptr += 4;
                }
                9 => {
                    let locations = self.parameter_locations(self.instr_ptr, 1);
                    self.relative_base += self.memory[locations[0]];
                    self.instr_ptr += 2;
                }
                99 => {
                    self.fresh_start = true;
                    return ExitReason::NormalExit;
                }
                _ => panic!("Bad opcode {}!", self.memory[self.instr_ptr]),
            };
        }
    }

    fn parameter_locations(&self, address: usize, count: usize) -> Vec<usize> {
        let mut locations = Vec::with_capacity(count);

        let mut moded_opcode = self.memory[address] / 10;
        for parameter_number in 1..=count {
            moded_opcode /= 10;
            let location = match moded_opcode % 10 {
                0 => self.memory[address + parameter_number] as usize,
                1 => address + parameter_number,
                2 => (self.relative_base + self.memory[address + parameter_number]) as usize,
                _ => panic!("Bad mode for opcode {}!", self.memory[address]),
            };
            locations.push(location);
        }

        locations
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day02() {
        let mut interpreter = Interpreter::new("1,9,10,3,2,3,11,0,99,30,40,50");
        interpreter.execute();
        assert_eq!(interpreter.memory()[0], 3500);

        let mut interpreter = Interpreter::new("1,0,0,0,99");
        interpreter.execute();
        assert_eq!(interpreter.memory()[0], 2);

        let mut interpreter = Interpreter::new("2,3,0,3,99");
        interpreter.execute();
        assert_eq!(interpreter.memory()[0], 2);

        let mut interpreter = Interpreter::new("2,4,4,5,99,0");
        interpreter.execute();
        assert_eq!(interpreter.memory()[0], 2);

        let mut interpreter = Interpreter::new("1,1,1,4,99,5,6,0,99");
        interpreter.execute();
        assert_eq!(interpreter.memory()[0], 30);
    }

    #[test]
    fn day05() {
        let mut interpreter = Interpreter::new("3,0,4,0,99");
        interpreter.insert_input(3659);
        interpreter.execute();
        assert_eq!(*interpreter.drain_outputs(), vec![3659]);

        let mut interpreter = Interpreter::new("1002,4,3,4,33");
        interpreter.execute();
        assert_eq!(interpreter.memory[4], 99);

        let mut interpreter = Interpreter::new("1101,100,-1,4,0");
        interpreter.execute();
        assert_eq!(interpreter.memory[4], 99);

        let mut interpreter = Interpreter::new("3,9,8,9,10,9,4,9,99,-1,8");
        interpreter.insert_input(8);
        interpreter.execute();
        assert_eq!(*interpreter.drain_outputs(), vec![1]);
        interpreter.insert_input(9);
        interpreter.execute();
        assert_eq!(*interpreter.drain_outputs(), vec![0]);

        let mut interpreter = Interpreter::new("3,9,7,9,10,9,4,9,99,-1,8");
        interpreter.insert_input(7);
        interpreter.execute();
        assert_eq!(*interpreter.drain_outputs(), vec![1]);
        interpreter.insert_input(8);
        interpreter.execute();
        assert_eq!(*interpreter.drain_outputs(), vec![0]);

        let mut interpreter = Interpreter::new("3,3,1108,-1,8,3,4,3,99");
        interpreter.insert_input(8);
        interpreter.execute();
        assert_eq!(*interpreter.drain_outputs(), vec![1]);
        interpreter.insert_input(9);
        interpreter.execute();
        assert_eq!(*interpreter.drain_outputs(), vec![0]);

        let mut interpreter = Interpreter::new("3,3,1107,-1,8,3,4,3,99");
        interpreter.insert_input(7);
        interpreter.execute();
        assert_eq!(*interpreter.drain_outputs(), vec![1]);
        interpreter.insert_input(8);
        interpreter.execute();
        assert_eq!(*interpreter.drain_outputs(), vec![0]);

        let mut interpreter = Interpreter::new("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
        interpreter.insert_input(0);
        interpreter.execute();
        assert_eq!(*interpreter.drain_outputs(), vec![0]);
        interpreter.insert_input(1);
        interpreter.execute();
        assert_eq!(*interpreter.drain_outputs(), vec![1]);

        let mut interpreter = Interpreter::new("3,3,1105,-1,9,1101,0,0,12,4,12,99,1");
        interpreter.insert_input(0);
        interpreter.execute();
        assert_eq!(*interpreter.drain_outputs(), vec![0]);
        interpreter.insert_input(1);
        interpreter.execute();
        assert_eq!(*interpreter.drain_outputs(), vec![1]);

        let mut interpreter = Interpreter::new(
            "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
        );
        interpreter.insert_input(6);
        interpreter.execute();
        assert_eq!(*interpreter.drain_outputs(), vec![999]);
        interpreter.insert_input(8);
        interpreter.execute();
        assert_eq!(*interpreter.drain_outputs(), vec![1000]);
        interpreter.insert_input(10);
        interpreter.execute();
        assert_eq!(*interpreter.drain_outputs(), vec![1001]);
    }

    #[test]
    fn day07() {
        let mut interpreter = Interpreter::new("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
        let mut next_signal = 0;
        for setting in [4, 3, 2, 1, 0] {
            interpreter.insert_input(setting);
            interpreter.insert_input(next_signal);
            interpreter.execute();
            next_signal = interpreter.get_output().unwrap();
        }
        assert_eq!(next_signal, 43210);

        let mut interpreter = Interpreter::new(
            "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",
        );
        let mut next_signal = 0;
        for setting in [0, 1, 2, 3, 4] {
            interpreter.insert_input(setting);
            interpreter.insert_input(next_signal);
            interpreter.execute();
            next_signal = interpreter.get_output().unwrap();
        }
        assert_eq!(next_signal, 54321);

        let mut interpreter = Interpreter::new(
            "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0",
        );
        let mut next_signal = 0;
        for setting in [1, 0, 4, 3, 2] {
            interpreter.insert_input(setting);
            interpreter.insert_input(next_signal);
            interpreter.execute();
            next_signal = interpreter.get_output().unwrap();
        }
        assert_eq!(next_signal, 65210);

        let interpreter = Interpreter::new(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
        );
        let mut amplifiers = [9, 8, 7, 6, 5]
            .into_iter()
            .map(|setting| {
                let mut interpreter = interpreter.clone();
                interpreter.insert_input(setting);
                interpreter
            })
            .collect::<Vec<_>>();
        let mut next_signal = 0;
        let mut current_amplifier = 0;
        'amplifiers: loop {
            let amplifier = &mut amplifiers[current_amplifier];
            amplifier.insert_input(next_signal);
            let exit_reason = amplifier.execute();
            next_signal = amplifier.get_output().unwrap();
            if exit_reason == ExitReason::NormalExit && current_amplifier == 4 {
                break 'amplifiers;
            }
            current_amplifier = (current_amplifier + 1) % 5;
        }
        assert_eq!(next_signal, 139629729);

        let interpreter = Interpreter::new(
            "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10",
        );
        let mut amplifiers = [9, 7, 8, 5, 6]
            .into_iter()
            .map(|setting| {
                let mut interpreter = interpreter.clone();
                interpreter.insert_input(setting);
                interpreter
            })
            .collect::<Vec<_>>();
        let mut next_signal = 0;
        let mut current_amplifier = 0;
        'amplifiers: loop {
            let amplifier = &mut amplifiers[current_amplifier];
            amplifier.insert_input(next_signal);
            let exit_reason = amplifier.execute();
            next_signal = amplifier.get_output().unwrap();
            if exit_reason == ExitReason::NormalExit && current_amplifier == 4 {
                break 'amplifiers;
            }
            current_amplifier = (current_amplifier + 1) % 5;
        }
        assert_eq!(next_signal, 18216);
    }

    #[test]
    fn day09() {
        let mut interpreter = Interpreter::new("1102,34915192,34915192,7,4,7,99,0");
        interpreter.execute();
        assert_eq!(interpreter.get_output().unwrap(), 1_219_070_632_396_864);

        let mut interpreter = Interpreter::new("104,1125899906842624,99");
        interpreter.execute();
        assert_eq!(interpreter.get_output().unwrap(), 1125899906842624);

        let mut interpreter =
            Interpreter::new("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
        interpreter.execute();
        assert_eq!(*interpreter.drain_outputs(), interpreter.program);
    }
}
