use core::time;
use std::{collections::HashSet, io::stdin, thread};

use aoc_2019_rust::{ExitReason, Interpreter};

fn main() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let program = stdin().lines().next().unwrap().unwrap();
    let mut interpreter = Interpreter::new(&program);

    let mut white = HashSet::new();
    let mut painted = HashSet::new();
    let mut location = (0, 0);
    let mut direction = (0, 1);
    let mut exit_reason = ExitReason::NoInput;
    while exit_reason != ExitReason::NormalExit {
        interpreter.insert_input(if white.contains(&location) { 1 } else { 0 });
        exit_reason = interpreter.execute();

        let new_color = interpreter.get_output().unwrap();
        if new_color == 1 {
            white.insert(location);
        } else {
            white.remove(&location);
        }
        painted.insert(location);

        let turn = interpreter.get_output().unwrap();
        direction = match turn {
            0 => match direction {
                (0, 1) => (-1, 0),
                (1, 0) => (0, 1),
                (0, -1) => (1, 0),
                (-1, 0) => (0, -1),
                _ => panic!("Bad direction!"),
            },
            1 => match direction {
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                (-1, 0) => (0, 1),
                _ => panic!("Bad direction!"),
            },
            _ => panic!("Bad turn!"),
        };

        location = (location.0 + direction.0, location.1 + direction.1);
    }

    println!("Part 1: {}", painted.len());
    println!("Part 2:");

    let mut location = (0, 0);
    const MIN_ROW: isize = -5;
    const MAX_ROW: isize = 0;
    const MIN_COLUMN: isize = 0;
    const MAX_COLUMN: isize = 42;

    let mut white = HashSet::new();
    white.insert(location);

    print!("{esc}[3;1H", esc = 27 as char);
    let mut grid = vec![
        vec!["⬛️"; (1 + MAX_COLUMN - MIN_COLUMN) as usize];
        (3 + MAX_ROW - MIN_ROW) as usize
    ];
    for location in white.iter() {
        grid[(location.1 - MIN_ROW + 1) as usize][(location.0 - MIN_COLUMN) as usize] = "⬜️";
    }
    grid[(location.1 - MIN_ROW + 1) as usize][(location.0 - MIN_COLUMN) as usize] = "🤖";

    for row in grid.into_iter().rev() {
        let line = row.into_iter().collect::<String>();
        println!("{}", &line);
    }

    let mut direction = (0, 1);
    let mut exit_reason = ExitReason::NoInput;
    while exit_reason != ExitReason::NormalExit {
        interpreter.insert_input(if white.contains(&location) { 1 } else { 0 });
        exit_reason = interpreter.execute();

        let new_color = interpreter.get_output().unwrap();
        if new_color == 1 {
            white.insert(location);
        } else {
            white.remove(&location);
        }
        painted.insert(location);

        let turn = interpreter.get_output().unwrap();
        direction = match turn {
            0 => match direction {
                (0, 1) => (-1, 0),
                (1, 0) => (0, 1),
                (0, -1) => (1, 0),
                (-1, 0) => (0, -1),
                _ => panic!("Bad direction!"),
            },
            1 => match direction {
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                (-1, 0) => (0, 1),
                _ => panic!("Bad direction!"),
            },
            _ => panic!("Bad turn!"),
        };

        location = (location.0 + direction.0, location.1 + direction.1);

        let ten_millis = time::Duration::from_millis(40);
        thread::sleep(ten_millis);
        print!("{esc}[3;1H", esc = 27 as char);
        let mut grid = vec![
            vec!["⬛️"; (1 + MAX_COLUMN - MIN_COLUMN) as usize];
            (3 + MAX_ROW - MIN_ROW) as usize
        ];
        for location in white.iter() {
            grid[(location.1 - MIN_ROW + 1) as usize][(location.0 - MIN_COLUMN) as usize] = "⬜️";
        }
        grid[(location.1 - MIN_ROW + 1) as usize][(location.0 - MIN_COLUMN) as usize] = "🤖";

        for row in grid.into_iter().rev() {
            let line = row.into_iter().collect::<String>();
            println!("{}", &line);
        }
    }
}
