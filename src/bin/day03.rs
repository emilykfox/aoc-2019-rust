use std::{collections::HashMap, io::stdin};

fn main() {
    let mut lines = stdin().lines().map_while(Result::ok);
    let directions1 = lines
        .next()
        .unwrap()
        .split(',')
        .map(String::from)
        .collect::<Vec<String>>();

    let mut covered1 = HashMap::<(i64, i64), i64>::new();
    let mut current = (0, 0);
    let mut steps = 0;
    for direction in directions1.iter() {
        let distance = direction[1..].parse::<i64>().unwrap();
        for _ in 1..=distance {
            steps += 1;
            let delta = match direction.chars().next().unwrap() {
                'R' => (1, 0),
                'U' => (0, 1),
                'L' => (-1, 0),
                'D' => (0, -1),
                _ => panic!("Bad direction!"),
            };
            current = (current.0 + delta.0, current.1 + delta.1);
            covered1.entry(current).or_insert(steps);
        }
    }
    let covered1 = covered1;

    let directions2 = lines
        .next()
        .unwrap()
        .split(',')
        .map(String::from)
        .collect::<Vec<String>>();

    let mut covered2 = HashMap::<(i64, i64), i64>::new();
    let mut current = (0, 0);
    let mut steps = 0;
    for direction in directions2.iter() {
        let distance = direction[1..].parse::<i64>().unwrap();
        for _ in 1..=distance {
            steps += 1;
            let delta = match direction.chars().next().unwrap() {
                'R' => (1, 0),
                'U' => (0, 1),
                'L' => (-1, 0),
                'D' => (0, -1),
                _ => panic!("Bad direction!"),
            };
            current = (current.0 + delta.0, current.1 + delta.1);
            covered2.entry(current).or_insert(steps);
        }
    }
    let covered2 = covered2;

    let crossings = covered1
        .keys()
        .copied()
        .filter(|location| covered2.contains_key(location))
        .collect::<Vec<_>>();

    let closest_crossing = crossings
        .iter()
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .unwrap();
    println!("Part 1: {}", closest_crossing);

    let best_steps = crossings
        .into_iter()
        .map(|location| covered1[&location] + covered2[&location])
        .min()
        .unwrap();
    println!("Part 2: {}", best_steps);
}
