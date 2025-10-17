pub struct Interpreter {
    initial_state: Vec<isize>,
    noun: Option<isize>,
    verb: Option<isize>,
    input: Option<isize>,

    memory: Vec<isize>,
    prev_outputs: Vec<isize>,
}

impl Interpreter {
    pub fn new(program: &str) -> Self {
        let initial_state = program
            .split(',')
            .map(|int| int.parse::<isize>().expect(int))
            .collect::<Vec<isize>>();
        Interpreter {
            initial_state,
            noun: None,
            verb: None,
            input: None,
            memory: Vec::new(),
            prev_outputs: Vec::new(),
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

    pub fn set_input(&mut self, input: isize) {
        self.input = Some(input);
    }

    pub fn unset_input(&mut self) {
        self.input = None;
    }

    pub fn prev_outputs(&self) -> &Vec<isize> {
        &self.prev_outputs
    }

    // Returns the final value at address 0.
    pub fn run(&mut self) -> isize {
        self.memory = self.initial_state.clone();
        self.prev_outputs = Vec::new();

        if let Some(noun) = self.noun {
            self.memory[1] = noun;
        }
        if let Some(verb) = self.verb {
            self.memory[2] = verb;
        }
        let mut instr_ptr = 0;
        loop {
            match self.memory[instr_ptr] % 100 {
                1 => {
                    let parameters = self.get_parameters(instr_ptr, 2);
                    let target = self.memory[instr_ptr + 3] as usize;
                    self.memory[target] = parameters[0] + parameters[1];

                    instr_ptr += 4;
                }
                2 => {
                    let parameters = self.get_parameters(instr_ptr, 2);
                    let target = self.memory[instr_ptr + 3] as usize;
                    self.memory[target] = parameters[0] * parameters[1];

                    instr_ptr += 4;
                }
                3 => {
                    let target = self.memory[instr_ptr + 1] as usize;
                    self.memory[target] = self.input.expect("No input set!");

                    instr_ptr += 2;
                }
                4 => {
                    let parameters = self.get_parameters(instr_ptr, 1);
                    self.prev_outputs.push(parameters[0]);

                    instr_ptr += 2;
                }
                5 => {
                    let parameters = self.get_parameters(instr_ptr, 2);
                    if parameters[0] != 0 {
                        instr_ptr = parameters[1] as usize;
                    } else {
                        instr_ptr += 3;
                    }
                }
                6 => {
                    let parameters = self.get_parameters(instr_ptr, 2);
                    if parameters[0] == 0 {
                        instr_ptr = parameters[1] as usize;
                    } else {
                        instr_ptr += 3;
                    }
                }
                7 => {
                    let parameters = self.get_parameters(instr_ptr, 2);
                    let target = self.memory[instr_ptr + 3] as usize;
                    self.memory[target] = (parameters[0] < parameters[1]) as isize;
                    instr_ptr += 4;
                }
                8 => {
                    let parameters = self.get_parameters(instr_ptr, 2);
                    let target = self.memory[instr_ptr + 3] as usize;
                    self.memory[target] = (parameters[0] == parameters[1]) as isize;
                    instr_ptr += 4;
                }
                99 => return self.memory[0],
                _ => panic!("Bad opcode {}!", self.memory[instr_ptr]),
            };
        }
    }

    fn get_parameters(&self, address: usize, count: usize) -> Vec<isize> {
        let mut parameters = Vec::with_capacity(count);

        let mut moded_opcode = self.memory[address] / 10;
        for parameter_number in 1..=count {
            moded_opcode /= 10;
            let parameter = match moded_opcode % 10 {
                0 => self.memory[self.memory[address + parameter_number] as usize],
                1 => self.memory[address + parameter_number],
                _ => panic!("Bad mode for opcode {}!", self.memory[address]),
            };
            parameters.push(parameter);
        }

        parameters
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day02() {
        let mut interpreter = Interpreter::new("1,9,10,3,2,3,11,0,99,30,40,50");
        assert_eq!(interpreter.run(), 3500);

        let mut interpreter = Interpreter::new("1,0,0,0,99");
        assert_eq!(interpreter.run(), 2);

        let mut interpreter = Interpreter::new("2,3,0,3,99");
        assert_eq!(interpreter.run(), 2);

        let mut interpreter = Interpreter::new("2,4,4,5,99,0");
        assert_eq!(interpreter.run(), 2);

        let mut interpreter = Interpreter::new("1,1,1,4,99,5,6,0,99");
        assert_eq!(interpreter.run(), 30);
    }

    #[test]
    fn day05() {
        let mut interpreter = Interpreter::new("3,0,4,0,99");
        interpreter.set_input(3659);
        interpreter.run();
        assert_eq!(*interpreter.prev_outputs(), vec![3659]);

        let mut interpreter = Interpreter::new("1002,4,3,4,33");
        interpreter.run();
        assert_eq!(interpreter.memory[4], 99);

        let mut interpreter = Interpreter::new("1101,100,-1,4,0");
        interpreter.run();
        assert_eq!(interpreter.memory[4], 99);

        let mut interpreter = Interpreter::new("3,9,8,9,10,9,4,9,99,-1,8");
        interpreter.set_input(8);
        interpreter.run();
        assert_eq!(*interpreter.prev_outputs(), vec![1]);
        interpreter.set_input(9);
        interpreter.run();
        assert_eq!(*interpreter.prev_outputs(), vec![0]);

        let mut interpreter = Interpreter::new("3,9,7,9,10,9,4,9,99,-1,8");
        interpreter.set_input(7);
        interpreter.run();
        assert_eq!(*interpreter.prev_outputs(), vec![1]);
        interpreter.set_input(8);
        interpreter.run();
        assert_eq!(*interpreter.prev_outputs(), vec![0]);

        let mut interpreter = Interpreter::new("3,3,1108,-1,8,3,4,3,99");
        interpreter.set_input(8);
        interpreter.run();
        assert_eq!(*interpreter.prev_outputs(), vec![1]);
        interpreter.set_input(9);
        interpreter.run();
        assert_eq!(*interpreter.prev_outputs(), vec![0]);

        let mut interpreter = Interpreter::new("3,3,1107,-1,8,3,4,3,99");
        interpreter.set_input(7);
        interpreter.run();
        assert_eq!(*interpreter.prev_outputs(), vec![1]);
        interpreter.set_input(8);
        interpreter.run();
        assert_eq!(*interpreter.prev_outputs(), vec![0]);

        let mut interpreter = Interpreter::new("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
        interpreter.set_input(0);
        interpreter.run();
        assert_eq!(*interpreter.prev_outputs(), vec![0]);
        interpreter.set_input(1);
        interpreter.run();
        assert_eq!(*interpreter.prev_outputs(), vec![1]);

        let mut interpreter = Interpreter::new("3,3,1105,-1,9,1101,0,0,12,4,12,99,1");
        interpreter.set_input(0);
        interpreter.run();
        assert_eq!(*interpreter.prev_outputs(), vec![0]);
        interpreter.set_input(1);
        interpreter.run();
        assert_eq!(*interpreter.prev_outputs(), vec![1]);

        let mut interpreter = Interpreter::new(
            "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
        );
        interpreter.set_input(6);
        interpreter.run();
        assert_eq!(*interpreter.prev_outputs(), vec![999]);
        interpreter.set_input(8);
        interpreter.run();
        assert_eq!(*interpreter.prev_outputs(), vec![1000]);
        interpreter.set_input(10);
        interpreter.run();
        assert_eq!(*interpreter.prev_outputs(), vec![1001]);
    }
}
