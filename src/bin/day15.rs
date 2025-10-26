use core::time;
use std::{
    collections::{HashMap, VecDeque},
    io::stdin,
    thread,
};

const MINX: i64 = -21;
const MAXX: i64 = 19;
const MINY: i64 = -21;
const MAXY: i64 = 19;

const SEARCH_WAIT: u64 = 10;
const PAUSE: u64 = 3000;
const OXYGEN_WAIT: u64 = 30;

use aoc_2019_rust::intcode;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tile {
    Floor,
    Wall,
    OxygenSystem,
    Oxygenated,
}

fn main() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    let program = stdin().lines().next().unwrap().unwrap();
    let mut interpreter = intcode::Interpreter::new(&program);

    println!("Part 1:");
    let mut map = HashMap::new();
    map.insert((0, 0), Tile::Floor);
    build_map((0, 0), &mut interpreter, &mut map);

    let mut distances = HashMap::new();
    distances.insert((0, 0), 0);
    let mut queue = VecDeque::new();
    queue.push_back((0, 0));
    let mut oxygen_distance = i64::MAX;
    let mut oxygen_location = (0, 0);
    while let Some(current) = queue.pop_front() {
        let distance = distances[&current];
        if map[&current] == Tile::OxygenSystem {
            oxygen_distance = distance;
            oxygen_location = current;
        }
        for delta in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let to = (current.0 + delta.0, current.1 + delta.1);
            if map[&to] != Tile::Wall && !distances.contains_key(&to) {
                distances.insert(to, distance + 1);
                queue.push_back(to);
            }
        }
    }

    println!("Distance is {oxygen_distance}.");

    let duration = time::Duration::from_millis(PAUSE);
    thread::sleep(duration);
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    println!("Part 2:");
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();
    distances.insert(oxygen_location, 0);
    map.insert(oxygen_location, Tile::Oxygenated);
    queue.push_back(oxygen_location);
    let mut max_distance = i64::MIN;
    while let Some(current) = queue.pop_front() {
        let distance = distances[&current];
        if distance > max_distance {
            max_distance = distance;
            print!("{esc}[3;1H", esc = 27 as char);
            draw_map(&map, None);
            let duration = time::Duration::from_millis(OXYGEN_WAIT);
            thread::sleep(duration);
        }
        for delta in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let to = (current.0 + delta.0, current.1 + delta.1);
            if map[&to] != Tile::Wall && !distances.contains_key(&to) {
                distances.insert(to, distance + 1);
                map.insert(to, Tile::Oxygenated);
                queue.push_back(to);
            }
        }
    }

    println!("Oxygen took {max_distance} minutes to distribute.");
}

fn build_map(
    from: (i64, i64),
    interpreter: &mut intcode::Interpreter,
    map: &mut HashMap<(i64, i64), Tile>,
) {
    print!("{esc}[3;1H", esc = 27 as char);
    draw_map(map, Some(from));
    let duration = time::Duration::from_millis(SEARCH_WAIT);
    thread::sleep(duration);
    for (movement, counter, delta) in [
        (1, 2, (0, -1)),
        (2, 1, (0, 1)),
        (3, 4, (-1, 0)),
        (4, 3, (1, 0)),
    ] {
        let to = (from.0 + delta.0, from.1 + delta.1);
        #[allow(clippy::map_entry)]
        if !map.contains_key(&to) {
            interpreter.insert_input(movement);
            interpreter.execute();
            let output = interpreter.get_output().unwrap();
            if output == 0 {
                map.insert(to, Tile::Wall);
            } else {
                if output == 2 {
                    map.insert(to, Tile::OxygenSystem);
                } else {
                    map.insert(to, Tile::Floor);
                }
                build_map(to, interpreter, map);
                interpreter.insert_input(counter);
                interpreter.execute();
                _ = interpreter.get_output();
            }
        }
    }
    print!("{esc}[3;1H", esc = 27 as char);
    draw_map(map, Some(from));
    let duration = time::Duration::from_millis(SEARCH_WAIT);
    thread::sleep(duration);
}

fn draw_map(map: &HashMap<(i64, i64), Tile>, current: Option<(i64, i64)>) {
    let mut grid = vec![vec!["⬛️"; (MAXX - MINX + 1) as usize]; (MAXY - MINY + 1) as usize];
    for ((x, y), tile) in map.iter() {
        grid[(*y - MINY) as usize][(*x - MINX) as usize] = match *tile {
            Tile::Floor => "🟦",
            Tile::Wall => "🟥",
            Tile::OxygenSystem => "🎈",
            Tile::Oxygenated => "🌫️",
        };
        if let Some(location) = current {
            grid[(location.1 - MINY) as usize][(location.0 - MINX) as usize] = "🤖";
        }
    }

    for row in grid {
        let line = row.into_iter().collect::<String>();
        println!("{}", line);
    }
}
