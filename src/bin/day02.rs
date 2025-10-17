use aoc_2019_rust::Program;

use std::io::stdin;

fn main() {
    let program = Program::new(
        stdin()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .split(',')
            .map(|int| int.parse::<usize>().unwrap())
            .collect::<Vec<usize>>(),
    );

    println!("Part 1: {}", program.run(12, 2));

    'loops: for noun in 0..=99 {
        for verb in 0..=99 {
            if program.run(noun, verb) == 19690720 {
                println!("Part 2: {}", 100 * noun + verb);
                break 'loops;
            }
        }
    }
}
