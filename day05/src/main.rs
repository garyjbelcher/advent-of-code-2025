#[derive(Clone)]
struct Range {
    end: i64,
    start: i64,
}

fn solve_part_1(fresh_ingredient_ranges: &Vec<Range>, available_ingredient_ids: &Vec<i64>) -> i32 {
    let mut count = 0;

    for id in available_ingredient_ids {
        for range in fresh_ingredient_ranges {
            if *id >= range.start && *id <= range.end {
                count += 1;
                break;
            }
        }
    }

    return count;
}

fn solve_part_2(fresh_ingredient_ranges: &mut Vec<Range>) -> i64 {
    fresh_ingredient_ranges.sort_by_key(|r| r.start);

    let mut merged_ranges = Vec::new();
    let mut current_range = fresh_ingredient_ranges[0].clone();

    for next_range in fresh_ingredient_ranges.into_iter().skip(1) {
        // Check if the next range overlaps or touches the current one.
        if next_range.start <= current_range.end.saturating_add(1) {
            // Extend the end of the current range if the next one is further.
            current_range.end = current_range.end.max(next_range.end);
        } else {
            // There is no overlap, so push the current range and start a new one.
            merged_ranges.push(current_range);
            current_range = next_range.clone();
        }
    }

    merged_ranges.push(current_range);

    return merged_ranges.iter().map(|r| r.end - r.start + 1).sum();
}

fn main() {
    let input = std::fs::read_to_string("resources/day5.txt").expect("Failed to read input file!");

    let mut fresh_ingredient_ranges: Vec<Range> = Vec::new();
    let mut available_ingredient_ids: Vec<i64> = Vec::new();

    for line in input.lines() {
        if line.contains('-') {
            let split: Vec<&str> = line.split('-').collect();
            let start: i64 = split[0].parse().unwrap();
            let end: i64 = split[1].parse().unwrap();
            fresh_ingredient_ranges.push(Range { end, start });
        } else if !line.is_empty() {
            available_ingredient_ids.push(line.parse::<i64>().unwrap());
        }
    }

    println!(
        "How many of the available ingredient IDs are fresh? {}",
        solve_part_1(&fresh_ingredient_ranges, &available_ingredient_ids)
    );
    println!(
        "How many ingredient IDs are considered to be fresh according to the fresh ingredient ID ranges? {}",
        solve_part_2(&mut fresh_ingredient_ranges)
    );
}
