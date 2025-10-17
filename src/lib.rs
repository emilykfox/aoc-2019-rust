pub struct Program {
    initial_state: Vec<usize>,
}

impl Program {
    pub fn new(initial_state: Vec<usize>) -> Self {
        Program { initial_state }
    }

    pub fn run(&self, noun: usize, verb: usize) -> usize {
        let mut memory = self.initial_state.clone();

        memory[1] = noun;
        memory[2] = verb;
        let mut position = 0;
        while memory[position] != 99 {
            let operand1 = memory[position + 1];
            let operand2 = memory[position + 2];
            let target = memory[position + 3];
            match memory[position] {
                1 => memory[target] = memory[operand1] + memory[operand2],
                2 => memory[target] = memory[operand1] * memory[operand2],
                _ => panic!("Bad opcode!"),
            };

            position += 4;
        }

        memory[0]
    }
}
