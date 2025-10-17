pub struct Interpreter {
    initial_state: Vec<usize>,
    noun: Option<usize>,
    verb: Option<usize>,
}

impl Interpreter {
    pub fn new(program: &str) -> Self {
        let initial_state = program
            .split(',')
            .map(|int| int.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        Interpreter {
            initial_state,
            noun: None,
            verb: None,
        }
    }

    pub fn set_noun_verb(&mut self, noun: usize, verb: usize) {
        self.noun = Some(noun);
        self.verb = Some(verb);
    }

    pub fn unset_noun_verb(&mut self) {
        self.noun = None;
        self.verb = None;
    }

    pub fn run(&self) -> usize {
        let mut memory = self.initial_state.clone();

        if let Some(noun) = self.noun {
            memory[1] = noun;
        }
        if let Some(verb) = self.verb {
            memory[2] = verb;
        }
        let mut position = 0;
        loop {
            match memory[position] {
                1 => {
                    let operand1 = memory[position + 1];
                    let operand2 = memory[position + 2];
                    let target = memory[position + 3];
                    memory[target] = memory[operand1] + memory[operand2];

                    position += 4;
                }
                2 => {
                    let operand1 = memory[position + 1];
                    let operand2 = memory[position + 2];
                    let target = memory[position + 3];
                    memory[target] = memory[operand1] * memory[operand2];

                    position += 4;
                }
                99 => return memory[0],
                _ => panic!("Bad opcode!"),
            };
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day02() {
        let interpreter = Interpreter::new("1,9,10,3,2,3,11,0,99,30,40,50");
        assert_eq!(interpreter.run(), 3500);

        let interpreter = Interpreter::new("1,0,0,0,99");
        assert_eq!(interpreter.run(), 2);

        let interpreter = Interpreter::new("2,3,0,3,99");
        assert_eq!(interpreter.run(), 2);

        let interpreter = Interpreter::new("2,4,4,5,99,0");
        assert_eq!(interpreter.run(), 2);

        let interpreter = Interpreter::new("1,1,1,4,99,5,6,0,99");
        assert_eq!(interpreter.run(), 30);

        let day02_input = std::fs::read_to_string("data/day02.txt")
            .unwrap()
            .trim()
            .to_string();
        let mut interpreter = Interpreter::new(&day02_input);
        interpreter.set_noun_verb(98, 20);
        assert_eq!(interpreter.run(), 19690720);
    }
}
