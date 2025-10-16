use std::io::stdin;

fn main() {
    let fuels = stdin()
        .lines()
        .map_while(Result::ok)
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let total_mass = fuels.iter().map(|fuel| fuel / 3 - 2).sum::<i64>();

    println!("Part 1: {total_mass}");

    let real_mass = fuels
        .iter()
        .map(|fuel| {
            let mut total_mass = 0;
            let mut current_mass = fuel / 3 - 2;
            while current_mass > 0 {
                total_mass += current_mass;
                current_mass = current_mass / 3 - 2;
            }
            total_mass
        })
        .sum::<i64>();

    println!("Part 2: {real_mass}");
}
