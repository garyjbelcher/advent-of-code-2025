fn solve_part_1(input: &str) -> i32 {
    let mut sum = 0;

    for line in input.lines() {
        let mut candidates: Vec<i32> = Vec::new();
        let batteries_needed = 2;
        let mut deletions = line.len() - batteries_needed;

        for i in 0..line.len() {
            let digit: i32 = line[i..i + 1].parse().unwrap();
            if candidates.is_empty() {
                candidates.push(digit);
                continue;
            }

            if *candidates.last().unwrap() < digit {
                // Digit is BIGGER.
                for j in (0..candidates.len()).rev() {
                    if deletions == 0 {
                        break;
                    }
                    if candidates[j] < digit {
                        candidates.remove(j);
                        deletions -= 1;
                    }
                }
                candidates.push(digit);
            } else if *candidates.last().unwrap() == digit {
                // Digit is THE SAME.
                if candidates.len() < batteries_needed {
                    candidates.push(digit);
                } else {
                    deletions -= 1;
                }
            } else {
                // Digit is SMALLER.
                if candidates.len() < batteries_needed {
                    candidates.push(digit);
                } else {
                    deletions -= 1;
                }
            }
        }

        let final_number: i32 = candidates
            .iter()
            .map(|d| d.to_string())
            .collect::<String>()
            .parse()
            .unwrap();

        sum += final_number;
    }

    return sum;
}

fn solve_part_2(input: &str) -> i64 {
    let mut sum = 0;

    for line in input.lines() {
        let mut candidates: Vec<i32> = Vec::new();
        let batteries_needed = 12;
        let mut deletions = line.len() - batteries_needed;

        for i in 0..line.len() {
            let digit: i32 = line[i..i + 1].parse().unwrap();
            if candidates.is_empty() {
                candidates.push(digit);
                continue;
            }

            if *candidates.last().unwrap() < digit {
                // Digit is BIGGER.
                for j in (0..candidates.len()).rev() {
                    if deletions == 0 {
                        break;
                    }
                    if candidates[j] < digit {
                        candidates.remove(j);
                        deletions -= 1;
                    }
                }
                candidates.push(digit);
            } else if *candidates.last().unwrap() == digit {
                // Digit is THE SAME.
                if candidates.len() < batteries_needed {
                    candidates.push(digit);
                } else {
                    deletions -= 1;
                }
            } else {
                // Digit is SMALLER.
                if candidates.len() < batteries_needed {
                    candidates.push(digit);
                } else {
                    deletions -= 1;
                }
            }
        }

        let final_number: i64 = candidates
            .iter()
            .map(|d| d.to_string())
            .collect::<String>()
            .parse()
            .unwrap();

        sum += final_number;
    }

    return sum;
}

fn main() {
    let input = std::fs::read_to_string("resources/day3.txt").expect("Failed to read input file!");

    println!("What is the total output joltage? {}", solve_part_1(&input));
    println!(
        "What is the new total output joltage? {}",
        solve_part_2(&input)
    );
}
