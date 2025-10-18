use itertools::Itertools;
use std::io::stdin;

use aoc_2019_rust::{ExitReason, Interpreter};

fn main() {
    let original_interpreter = Interpreter::new(&stdin().lines().next().unwrap().unwrap());

    let mut interpreter = original_interpreter.clone();

    let highest_signal = [0, 1, 2, 3, 4]
        .into_iter()
        .permutations(5)
        .map(|permutation| {
            interpreter.clear_inputs();
            interpreter.drain_outputs();
            let mut next_signal = 0;
            for setting in permutation {
                interpreter.insert_input(setting);
                interpreter.insert_input(next_signal);
                interpreter.execute();
                next_signal = interpreter.get_output().unwrap();
            }
            next_signal
        })
        .max()
        .unwrap();
    println!("Part 1: {}", highest_signal);

    let highest_signal = [5, 6, 7, 8, 9]
        .into_iter()
        .permutations(5)
        .map(|permutation| {
            let mut amplifiers = permutation
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
                if exit_reason == ExitReason::Reached99 && current_amplifier == 4 {
                    break 'amplifiers;
                }
                current_amplifier = (current_amplifier + 1) % 5;
            }
            next_signal
        })
        .max()
        .unwrap();
    println!("Part 2: {}", highest_signal);
}
