fn check_n(grid: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    (row > 0 && grid[row - 1][col] == '@') as i32
}

fn check_ne(grid: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    (row > 0 && col < grid[0].len() - 1 && grid[row - 1][col + 1] == '@') as i32
}

fn check_e(grid: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    (col < grid[0].len() - 1 && grid[row][col + 1] == '@') as i32
}

fn check_se(grid: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    (row < grid.len() - 1 && col < grid[0].len() - 1 && grid[row + 1][col + 1] == '@') as i32
}

fn check_s(grid: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    (row < grid.len() - 1 && grid[row + 1][col] == '@') as i32
}

fn check_sw(grid: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    (row < grid.len() - 1 && col > 0 && grid[row + 1][col - 1] == '@') as i32
}

fn check_w(grid: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    (col > 0 && grid[row][col - 1] == '@') as i32
}

fn check_nw(grid: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    (row > 0 && col > 0 && grid[row - 1][col - 1] == '@') as i32
}

fn solve_part_1(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut count = 0;

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] != '@' {
                continue;
            }

            let mut sum = 0;

            sum += check_n(&grid, row, col);
            sum += check_ne(&grid, row, col);
            sum += check_e(&grid, row, col);
            sum += check_se(&grid, row, col);
            sum += check_s(&grid, row, col);
            sum += check_sw(&grid, row, col);
            sum += check_w(&grid, row, col);
            sum += check_nw(&grid, row, col);

            if sum < 4 {
                count += 1;
            }
        }
    }

    return count;
}

fn scan(grid: &mut Vec<Vec<char>>) -> i32 {
    let mut removed = 0;

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] != '@' {
                continue;
            }

            let mut sum = 0;

            sum += check_n(&grid, row, col);
            sum += check_ne(&grid, row, col);
            sum += check_e(&grid, row, col);
            sum += check_se(&grid, row, col);
            sum += check_s(&grid, row, col);
            sum += check_sw(&grid, row, col);
            sum += check_w(&grid, row, col);
            sum += check_nw(&grid, row, col);

            if sum < 4 {
                grid[row][col] = '.';
                removed += 1;
            }
        }
    }
    
    return removed;
}

fn solve_part_2(input: &str) -> i32 {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut count = 0;

    loop {
        let removed = scan(&mut grid);
        
        if removed == 0 {
            break;
        }
        
        count += removed;
    }

    return count;
}

fn main() {
    let input = std::fs::read_to_string("resources/day4.txt").expect("Failed to read input file!");

    println!(
        "How many rolls of paper can be accessed by a forklift? {}",
        solve_part_1(&input)
    );

    println!(
        "How many rolls of paper in total can be removed by the Elves and their forklifts? {}",
        solve_part_2(&input)
    );
}
