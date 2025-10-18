use std::io::stdin;

use aoc_2019_rust::Interpreter;

fn main() {
    let program = stdin().lines().next().unwrap().unwrap();
    let mut interpreter = Interpreter::new(&program);
    interpreter.insert_input(1);
    interpreter.execute();

    println!("Part 1: {}", interpreter.drain_outputs().last().unwrap());

    interpreter.insert_input(5);
    interpreter.execute();
    println!("Part 2: {}", interpreter.drain_outputs()[0]);
}
