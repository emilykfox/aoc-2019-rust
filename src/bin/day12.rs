use std::{collections::HashMap, io::stdin};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Moon {
    x: i64,
    y: i64,
    z: i64,
    vx: i64,
    vy: i64,
    vz: i64,
}

fn main() {
    let re = regex::Regex::new(r#"x=(?<x>-?\d+), y=(?<y>-?\d+), z=(?<z>-?\d+)"#).unwrap();

    let mut moons = stdin()
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            let captures = re.captures(&line).expect(&line);
            Moon {
                x: captures["x"].parse().unwrap(),
                y: captures["y"].parse().unwrap(),
                z: captures["z"].parse().unwrap(),
                vx: 0,
                vy: 0,
                vz: 0,
            }
        })
        .collect::<Vec<_>>();

    let mut seen_statesx = HashMap::from([(
        moons
            .iter()
            .map(|moon| (moon.x, moon.vx))
            .collect::<Vec<_>>(),
        0,
    )]);
    let mut seen_statesy = HashMap::from([(
        moons
            .iter()
            .map(|moon| (moon.y, moon.vy))
            .collect::<Vec<_>>(),
        0,
    )]);
    let mut seen_statesz = HashMap::from([(
        moons
            .iter()
            .map(|moon| (moon.z, moon.vz))
            .collect::<Vec<_>>(),
        0,
    )]);
    let mut periodx = None;
    let mut periody = None;
    let mut periodz = None;
    let mut first_repeatingx = None;
    let mut first_repeatingy = None;
    let mut first_repeatingz = None;

    let mut total_energy = 0;
    for step in 1.. {
        let current_moons = moons.clone();
        for moon in moons.iter_mut() {
            for current_moon in current_moons.iter() {
                if current_moon.x > moon.x {
                    moon.vx += 1;
                }
                if current_moon.x < moon.x {
                    moon.vx -= 1;
                }
                if current_moon.y > moon.y {
                    moon.vy += 1;
                }
                if current_moon.y < moon.y {
                    moon.vy -= 1;
                }
                if current_moon.z > moon.z {
                    moon.vz += 1;
                }
                if current_moon.z < moon.z {
                    moon.vz -= 1;
                }
            }

            moon.x += moon.vx;
            moon.y += moon.vy;
            moon.z += moon.vz;
        }

        if periodx.is_none() {
            let allx = moons
                .iter()
                .map(|moon| (moon.x, moon.vx))
                .collect::<Vec<_>>();
            if let Some(prev_step) = seen_statesx.get(&allx) {
                periodx = Some(step - prev_step);
                first_repeatingx = Some(*prev_step);
            } else {
                seen_statesx.insert(allx, step);
            }
        }
        if periody.is_none() {
            let ally = moons
                .iter()
                .map(|moon| (moon.y, moon.vy))
                .collect::<Vec<_>>();
            if let Some(prev_step) = seen_statesy.get(&ally) {
                periody = Some(step - prev_step);
                first_repeatingy = Some(*prev_step);
            } else {
                seen_statesy.insert(ally, step);
            }
        }
        if periodz.is_none() {
            let allz = moons
                .iter()
                .map(|moon| (moon.z, moon.vz))
                .collect::<Vec<_>>();
            if let Some(prev_step) = seen_statesz.get(&allz) {
                periodz = Some(step - prev_step);
                first_repeatingz = Some(*prev_step);
            } else {
                seen_statesz.insert(allz, step);
            }
        }

        if step == 1000 {
            total_energy = moons
                .iter()
                .map(|moon| {
                    (moon.x.abs() + moon.y.abs() + moon.z.abs())
                        * (moon.vx.abs() + moon.vy.abs() + moon.vz.abs())
                })
                .sum::<i64>();
        }

        if periodx.is_some() && periody.is_some() && periodz.is_some() {
            break;
        }
    }

    println!("Part 1: {}", total_energy);

    let starts_repeating = first_repeatingx
        .unwrap()
        .max(first_repeatingy.unwrap())
        .max(first_repeatingz.unwrap());

    let periodx = periodx.unwrap();
    let periody = periody.unwrap();
    let periodz = periodz.unwrap();

    let lcm: i64 = num::integer::lcm(num::integer::lcm(periodx, periody), periodz);

    let answer = starts_repeating + lcm;

    println!("Part 2: {}", answer);
}
