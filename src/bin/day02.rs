use aoc_2019_rust::Interpreter;

use std::io::stdin;

fn main() {
    let mut interpreter = Interpreter::new(&stdin().lines().next().unwrap().unwrap());

    interpreter.set_noun_verb(12, 2);
    println!("Part 1: {}", interpreter.run());

    'loops: for noun in 0..=99 {
        for verb in 0..=99 {
            interpreter.set_noun_verb(noun, verb);
            if interpreter.run() == 19690720 {
                println!("Part 2: {}", 100 * noun + verb);
                break 'loops;
            }
        }
    }
}
