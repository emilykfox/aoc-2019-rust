use std::io::stdin;

use aoc_2019_rust::Interpreter;

fn main() {
    let program = stdin().lines().next().unwrap().unwrap();
    let mut interpreter = Interpreter::new(&program);
    interpreter.set_input(1);
    interpreter.run();

    println!("Part 1: {}", interpreter.get_outputs().last().unwrap());

    interpreter.set_input(5);
    interpreter.run();
    println!("Part 2: {}", interpreter.get_outputs()[0]);
}
