fn solve_part_1(input: &str) -> i32 {

    let mut count = 0;
    let mut dial_position = 50;
    
    let lines = input.lines();
    
    for line in lines {
        let direction = &line[0..1];
        let rotations: i32 = line[1..].parse().unwrap();

        match direction {
            "R" => {
                dial_position = (dial_position + (rotations % 100) + 100) % 100;
            },
            "L" => {
                dial_position = (dial_position + (-1 * rotations % 100) + 100) % 100;
            },
            _ => panic!("Invalid direction!"),
        }

        if dial_position == 0 {
            count += 1;
        }
    }
    
    return count;
}

fn solve_part_2(input: &str) -> i32 {
    let mut count = 0;
    let mut dial_position = 50;
    
    let lines = input.lines();
    
    for line in lines {
        let direction = &line[0..1];
        let rotations: i32 = line[1..].parse().unwrap();
        let rotation;

        match direction {
            "R" => {
                rotation = rotations;
            },
            "L" => {
                rotation = -1 * rotations;
            },
            _ => panic!("Invalid direction!"),
        }

        for _ in 0..rotation.abs() {
            dial_position = (dial_position + (rotation.signum()) + 100) % 100;
            if dial_position == 0 {
                count += 1;
            }
        }
    }
    
    return count;
}

fn main() {
    let input = std::fs::read_to_string("resources/day1.txt").expect("Failed to read input file!");

    println!("What's the actual password to open the door? {}", solve_part_1(&input));
    println!("What is the password to open the door? {}", solve_part_2(&input));
}
