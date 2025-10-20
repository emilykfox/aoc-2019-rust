use std::{cmp::Ordering, collections::HashSet, io::stdin};

fn main() {
    let asteroids = stdin()
        .lines()
        .map_while(Result::ok)
        .enumerate()
        .flat_map(|(row_index, line)| {
            line.into_bytes()
                .into_iter()
                .enumerate()
                .filter(|(_, character)| *character == b'#')
                .map(move |(column_index, _)| (column_index as isize, row_index as isize))
        })
        .collect::<HashSet<_>>();

    let mut best_center = (0, 0);
    let mut most_seen = 0;
    for center in asteroids.iter().copied() {
        let count = asteroids
            .iter()
            .copied()
            .filter(|&target| {
                if target == center {
                    false
                } else {
                    let target_row_diff = target.1 - center.1;
                    let target_column_diff = target.0 - center.0;
                    let step_delta = if target_row_diff == 0 {
                        if target_column_diff > 0 {
                            (1, 0)
                        } else {
                            (-1, 0)
                        }
                    } else if target_column_diff == 0 {
                        if target_row_diff > 0 { (0, 1) } else { (0, -1) }
                    } else {
                        let gcd = (1..=target_column_diff.abs())
                            .rev()
                            .find(|&candidate| {
                                target_row_diff % candidate == 0
                                    && target_column_diff % candidate == 0
                            })
                            .unwrap();
                        (target_column_diff / gcd, target_row_diff / gcd)
                    };
                    let mut blocked = false;
                    let mut checking = (center.0 + step_delta.0, center.1 + step_delta.1);
                    while checking != target {
                        if asteroids.contains(&checking) {
                            blocked = true;
                            break;
                        }
                        checking = (checking.0 + step_delta.0, checking.1 + step_delta.1);
                    }
                    !blocked
                }
            })
            .count();
        if count > most_seen {
            best_center = center;
            most_seen = count;
        }
    }
    println!("Part 1: {}", most_seen);

    let mut vaporized_count = 0;
    let mut remaining = asteroids.clone();
    let winner = 'zapping: loop {
        let mut targets = remaining
            .iter()
            .copied()
            .filter(|&target| {
                if target == best_center {
                    false
                } else {
                    let target_row_diff = target.1 - best_center.1;
                    let target_column_diff = target.0 - best_center.0;
                    let step_delta = if target_row_diff == 0 {
                        if target_column_diff > 0 {
                            (1, 0)
                        } else {
                            (-1, 0)
                        }
                    } else if target_column_diff == 0 {
                        if target_row_diff > 0 { (0, 1) } else { (0, -1) }
                    } else {
                        let gcd = (1..=target_column_diff.abs())
                            .rev()
                            .find(|&candidate| {
                                target_row_diff % candidate == 0
                                    && target_column_diff % candidate == 0
                            })
                            .unwrap();
                        (target_column_diff / gcd, target_row_diff / gcd)
                    };
                    let mut blocked = false;
                    let mut checking = (best_center.0 + step_delta.0, best_center.1 + step_delta.1);
                    while checking != target {
                        if remaining.contains(&checking) {
                            blocked = true;
                            break;
                        }
                        checking = (checking.0 + step_delta.0, checking.1 + step_delta.1);
                    }
                    !blocked
                }
            })
            .collect::<Vec<_>>();

        targets.sort_by(|first, second| {
            if first.0 == best_center.0 && first.1 < best_center.1 {
                Ordering::Less
            } else if second.0 == best_center.0 && second.1 < best_center.1 {
                Ordering::Greater
            } else if first.0 > best_center.0 && second.0 <= best_center.0 {
                Ordering::Less
            } else if second.0 > best_center.0 && first.0 <= best_center.0 {
                Ordering::Greater
            } else {
                // both on same side
                if (best_center.1 - first.1) * (second.0 - best_center.0)
                    > (best_center.1 - second.1) * (first.0 - best_center.0)
                {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
        });

        for target in targets {
            remaining.remove(&target);
            vaporized_count += 1;
            if vaporized_count == 200 {
                break 'zapping target;
            }
        }
    };

    println!("Part 2: {}", 100 * winner.0 + winner.1);
}
