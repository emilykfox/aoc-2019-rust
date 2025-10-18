use std::io::stdin;

fn main() {
    let sif = stdin().lines().next().unwrap().unwrap();

    const WIDTH: usize = 25;
    const HEIGHT: usize = 6;
    const DIGITS_PER_LAYER: usize = WIDTH * HEIGHT;

    let mut layers = Vec::new();
    for layer_num in 0..(sif.len() / DIGITS_PER_LAYER) {
        let layer = sif
            [DIGITS_PER_LAYER * layer_num..(DIGITS_PER_LAYER * layer_num + DIGITS_PER_LAYER)]
            .chars()
            .map(|digit| digit.to_digit(10).unwrap())
            .collect::<Vec<_>>();
        layers.push(layer);
    }

    let mut fewest = usize::MAX;
    let mut best = 0;
    for layer in layers.iter() {
        let num0 = layer.iter().filter(|&&digit| digit == 0).count();
        if num0 < fewest {
            let num1 = layer.iter().filter(|&&digit| digit == 1).count();
            let num2 = layer.iter().filter(|&&digit| digit == 2).count();
            fewest = num0;
            best = num1 * num2;
        }
    }
    println!("Part 1: {}", best);

    let mut image = [["⬛️"; WIDTH]; HEIGHT];
    for layer in layers.iter().rev() {
        for row in 0..HEIGHT {
            for column in 0..WIDTH {
                image[row][column] = match layer[row * WIDTH + column] {
                    0 => "⬛️",
                    1 => "⬜️",
                    2 => image[row][column],
                    _ => panic!("Bad layer digit!"),
                };
            }
        }
    }

    println!("Part 2:");
    for line in image.into_iter() {
        let line_string = line.into_iter().flat_map(str::chars).collect::<String>();
        println!("{}", line_string);
    }
}
