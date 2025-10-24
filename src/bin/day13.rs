use core::time;
use std::{collections::HashMap, io::stdin, thread};

use aoc_2019_rust::intcode;

fn main() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    let program = stdin().lines().next().unwrap().unwrap();
    let mut interpreter = intcode::Interpreter::new(&program);

    let mut map = HashMap::new();

    interpreter.execute();
    let output = interpreter.drain_outputs();

    let mut max_column = 0;
    let mut max_row = 0;
    for chunk in output.chunks(3) {
        let x = chunk[0] as usize;
        let y = chunk[1] as usize;
        map.insert((x, y), chunk[2]);
        max_column = max_column.max(x);
        max_row = max_row.max(y);
    }

    let num_blocks = map.values().filter(|tile| **tile == 2).count();

    println!("Part 1: {}", num_blocks);
    println!("Part 2:");

    let mut grid = vec![vec![" "; max_column + 1]; max_row + 1];
    for ((x, y), tile) in map.iter() {
        grid[*y][*x] = match *tile {
            0 => "⬛️",
            1 => "🟥",
            2 => "🟦",
            3 => "🏓",
            4 => "🟣",
            _ => panic!("Bad tile!"),
        };
    }
    print!("{esc}[3;1H", esc = 27 as char);
    for row in grid {
        println!("{}", row.into_iter().collect::<String>());
    }

    interpreter.reset();
    interpreter.memory_mut()[0] = 2;
    let mut next_input = 0;
    let mut score = 0;
    let mut exit_reason = intcode::ExitReason::NoInput;
    while exit_reason != intcode::ExitReason::NormalExit {
        interpreter.insert_input(next_input);
        exit_reason = interpreter.execute();
        let output = interpreter.drain_outputs();
        let mut paddle_x = 0;
        let mut ball_x = 0;
        for chunk in output.chunks(3) {
            if chunk[0] == -1 {
                score = chunk[2];
                continue;
            }
            let x = chunk[0] as usize;
            let y = chunk[1] as usize;
            map.insert((x, y), chunk[2]);
            if chunk[2] == 3 {
                paddle_x = x;
            } else if chunk[2] == 4 {
                ball_x = x;
            }
        }

        if paddle_x > ball_x {
            next_input = -1;
        } else if paddle_x < ball_x {
            next_input = 1;
        }

        let mut grid = vec![vec![" "; max_column + 1]; max_row + 1];
        for ((x, y), tile) in map.iter() {
            grid[*y][*x] = match *tile {
                0 => "⬛️",
                1 => "🟥",
                2 => "🟦",
                3 => "🏓",
                4 => "🟣",
                _ => panic!("Bad tile!"),
            };
        }

        let ten_millis = if score < 1000 {
            time::Duration::from_millis(40)
        } else if score < 2000 {
            time::Duration::from_millis(30)
        } else if score < 5000 {
            time::Duration::from_millis(20)
        } else if score < 7000 {
            time::Duration::from_millis(10)
        } else if score < 11300 {
            time::Duration::from_millis(1)
        } else {
            time::Duration::from_millis(10)
        };
        thread::sleep(ten_millis);

        print!("{esc}[3;1H", esc = 27 as char);
        for (index, row) in grid.into_iter().enumerate() {
            if index == 2 {
                println!("{} {}", row.into_iter().collect::<String>(), score);
            } else {
                println!("{}", row.into_iter().collect::<String>());
            }
        }
    }

    println!("Part 2: {}", score);
}
