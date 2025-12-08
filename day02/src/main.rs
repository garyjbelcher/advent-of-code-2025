use fancy_regex::Regex;

fn solve_part_1(input: &str) -> i64 {
    let mut sum: i64 = 0;
    let ranges = input.split(",").collect::<Vec<&str>>();

    for range in ranges {
        let parts: Vec<&str> = range.split('-').collect();
        let lower: i64 = parts[0].parse().unwrap();
        let upper: i64 = parts[1].parse().unwrap();

        for i in lower..=upper {
            let i_str = i.to_string();
            if i_str.len() % 2 == 0 {
                let left_half = &i_str[0..(i_str.len() / 2)];
                let right_half = &i_str[(i_str.len() / 2)..i_str.len()];
                if left_half == right_half {
                    sum += i;
                }
            }
        }
    }

    return sum;
}

fn solve_part_2(input: &str) -> i64 {
    let mut sum: i64 = 0;
    let ranges = input.split(",").collect::<Vec<&str>>();
    let re = Regex::new(r"^(\d+?)\1+$").unwrap();

    for range in ranges {
        let parts: Vec<&str> = range.split('-').collect();
        let lower: i64 = parts[0].parse().unwrap();
        let upper: i64 = parts[1].parse().unwrap();

        for i in lower..=upper {
            let i_str = i.to_string();
            if re.is_match(&i_str).unwrap() {
                sum += i;
            }
        }
    }

    return sum;
}

fn main() {
    let input = std::fs::read_to_string("resources/day2.txt").expect("Failed to read input file!");

    println!(
        "What do you get if you add up all of the invalid IDs? {}",
        solve_part_1(&input)
    );
    println!(
        "What do you get if you add up all of the invalid IDs using these new rules? {}",
        solve_part_2(&input)
    );
}
