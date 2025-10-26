use std::{collections::HashMap, io::stdin};

struct Reaction<'a> {
    inputs: HashMap<&'a str, u64>,
    output: (&'a str, u64),
}

fn main() {
    let lines = stdin().lines().map(Result::unwrap).collect::<Vec<_>>();

    let chemical_re = regex::Regex::new(r#"(?<number>\d+) (?<chemical>\w+)"#).unwrap();
    let reactions = lines
        .iter()
        .map(|line| {
            let mut pieces = line.split("=>");
            let (input_str, output_str) = (pieces.next().unwrap(), pieces.next().unwrap());
            let inputs = chemical_re
                .captures_iter(input_str)
                .map(|captures| {
                    (
                        captures.name("chemical").unwrap().as_str(),
                        captures["number"].parse().unwrap(),
                    )
                })
                .collect();
            let output_capture = chemical_re.captures(output_str).unwrap();
            let output = (
                output_capture.name("chemical").unwrap().as_str(),
                output_capture["number"].parse().unwrap(),
            );
            Reaction { inputs, output }
        })
        .collect::<Vec<_>>();

    let mut input_for = HashMap::<&str, Vec<&Reaction>>::new();
    for reaction in reactions.iter() {
        for input in reaction.inputs.iter() {
            input_for.entry(input.0).or_default().push(reaction);
        }
    }

    let mut amount_needed = HashMap::new();
    amount_needed.insert("FUEL", 1);

    fn compute_needed<'a>(
        of: &'a str,
        input_for: &HashMap<&str, Vec<&'a Reaction>>,
        table: &mut HashMap<&'a str, u64>,
    ) -> u64 {
        if !table.contains_key(&of) {
            let mut total = 0;
            if let Some(reactions) = input_for.get(&of) {
                for reaction in reactions.iter() {
                    let output_needed = compute_needed(reaction.output.0, input_for, table);
                    let output_per = reaction.output.1;
                    let reactions_needed = output_needed.div_ceil(output_per);
                    let input_per = reaction.inputs[&of];
                    total += input_per * reactions_needed;
                }
            }
            table.insert(of, total);
        }
        table[of]
    }

    let needed = compute_needed("ORE", &input_for, &mut amount_needed);
    println!("Part 1: {needed}");

    let mut min = 1u64;
    let mut max = 1000000000000u64;
    while max - min > 0 {
        let mid = (max - min) / 2 + min;
        let mut amount_needed = HashMap::new();
        amount_needed.insert("FUEL", mid);
        let needed = compute_needed("ORE", &input_for, &mut amount_needed);
        if needed > 1000000000000u64 {
            max = mid - 1;
        } else {
            min = mid;
        }
    }

    println!("Part 2: {min}");
}
