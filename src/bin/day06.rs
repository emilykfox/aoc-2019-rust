use std::{collections::HashMap, io::stdin};

fn main() {
    let lines = stdin().lines().map_while(Result::ok).collect::<Vec<_>>();

    let mut orbits_around = HashMap::<_, Vec<_>>::new();
    let mut orbiting = HashMap::new();
    for line in lines.iter() {
        let (center, orbiter) = (&line[0..3], &line[4..]);
        orbits_around.entry(center).or_default().push(orbiter);
        orbiting.insert(orbiter, center);
    }

    let mut num_orbits = HashMap::new();
    let mut object_stack = Vec::new();
    for &center in orbits_around
        .keys()
        .filter(|&&center| !orbiting.contains_key(center))
    {
        num_orbits.insert(center, 0);
        object_stack.push(center);
    }

    while let Some(object) = object_stack.pop() {
        for &orbiter in orbits_around.get(&object).unwrap_or(&Vec::new()).iter() {
            num_orbits.insert(orbiter, num_orbits[object] + 1);
            object_stack.push(orbiter);
        }
    }

    let total_orbits = num_orbits.values().sum::<usize>();
    println!("Part 1: {}", total_orbits);

    let mut santa_distances = HashMap::new();
    let mut current = orbiting.get("SAN");
    let mut distance = 0;
    while let Some(&inner_current) = current {
        santa_distances.insert(inner_current, distance);
        current = orbiting.get(inner_current);
        distance += 1;
    }

    let mut current = orbiting.get("YOU");
    let mut distance = 0;
    'you: while let Some(&inner_current) = current {
        if let Some(santa_distance) = santa_distances.get(inner_current) {
            println!("Part 2: {}", distance + santa_distance);
            break 'you;
        }

        current = orbiting.get(inner_current);
        distance += 1;
    }
}
