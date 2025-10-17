use std::io::stdin;

fn main() {
    let input = stdin().lines().next().unwrap().unwrap();
    let mut parts = input.split('-');
    let min = parts.next().unwrap().parse::<usize>().unwrap();
    let max = parts.next().unwrap().parse::<usize>().unwrap();

    let num_passwords = (min..=max)
        .map(|password| password.to_string().bytes().collect::<Vec<_>>())
        .filter(|password| {
            let mut prev = 0;
            let mut found_double = false;
            for current in password {
                if *current == prev {
                    found_double = true;
                }
                if *current < prev {
                    return false;
                }
                prev = *current;
            }
            found_double
        })
        .count();

    println!("Part 1: {}", num_passwords);

    let num_passwords = (min..=max)
        .map(|password| password.to_string().bytes().collect::<Vec<_>>())
        .filter(|password| {
            let mut prev = 0;
            let mut finding_double = false;
            let mut too_much = false;
            let mut found_double = false;
            for current in password {
                if *current == prev {
                    if !finding_double {
                        finding_double = true;
                    } else {
                        too_much = true;
                    }
                } else {
                    if finding_double && !too_much {
                        found_double = true;
                    }
                    finding_double = false;
                    too_much = false;
                }
                if *current < prev {
                    return false;
                }
                prev = *current;
            }

            if finding_double && !too_much {
                found_double = true;
            }

            found_double
        })
        .count();

    println!("Part 2: {}", num_passwords);
}
