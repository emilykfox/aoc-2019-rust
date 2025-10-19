use std::io::stdin;

use aoc_2019_rust::Interpreter;

fn main() {
    let input = stdin().lines().next().unwrap().unwrap();
    let mut interpreter = Interpreter::new(&input);
    interpreter.insert_input(1);
    interpreter.execute();
    let output = interpreter.drain_outputs();

    assert_eq!(output.len(), 1);
    println!("Part 1: {}", output[0]);

    interpreter.insert_input(2);
    interpreter.execute();
    let output = interpreter.drain_outputs();

    assert_eq!(output.len(), 1);
    println!("Part 2: {}", output[0]);
}
