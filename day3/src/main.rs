fn oob(width: usize, height: usize, x: isize, y: isize) -> bool {
    x < 0 || y < 0 || x as usize >= width || y as usize >= height
}

fn num_surround(grid: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut poss = Vec::new();
    let width = grid[0].len();
    let height = grid.len();
    for dy in [-1, 0, 1] {
        let mut done_number = false;
        for dx in [-1, 0, 1] {
            let x = x as isize + dx;
            let y = y as isize + dy;
            if oob(width, height, x, y) {
                continue;
            }
            if grid[y as usize][x as usize].is_ascii_digit() {
                if done_number {
                    continue;
                }
                poss.push((x as usize, y as usize));
                done_number = true;
                continue;
            }
            done_number = false
        }
    }
    poss
}

fn part1(input: String) -> impl std::fmt::Display {
    let mut sum = 0;
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    for (y, line) in grid.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if !"0123456789.".contains(*char) {
                for (mut x, y) in num_surround(&grid, x, y) {
                    while x > 0 && grid[y][x - 1].is_ascii_digit() {
                        x -= 1
                    }
                    let mut num = 0;
                    while let Some(d) = grid[y].get(x).map(|d| d.to_digit(10)).flatten() {
                        num *= 10;
                        num += d;
                        x += 1;
                    }
                    sum += num;
                }
            }
        }
    }
    sum
}

fn part2(input: String) -> impl std::fmt::Display {
    let mut sum = 0;
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    for (y, line) in grid.iter().enumerate() {
        for (x, char) in line.iter().copied().enumerate() {
            if char == '*' {
                let mut nums: Vec<u32> = Vec::new();
                for (mut x, y) in num_surround(&grid, x, y) {
                    while x > 0 && grid[y][x - 1].is_ascii_digit() {
                        x -= 1
                    }
                    let mut num = 0;
                    while let Some(d) = grid[y].get(x).map(|d| d.to_digit(10)).flatten() {
                        num *= 10;
                        num += d;
                        x += 1;
                    }
                    nums.push(num);
                }
                if nums.len() == 2 {
                    sum += nums.iter().product::<u32>();
                }
            }
        }
    }
    sum
}

#[test]
fn part1_test() {
    let file = std::fs::read_to_string("input2.txt").unwrap();
    let output = 4361;
    assert_eq!(part1(file).to_string(), output.to_string());
}

#[test]
fn part2_test() {
    let file = std::fs::read_to_string("input2.txt").unwrap();
    let output = 467835;
    assert_eq!(part2(file).to_string(), output.to_string());
}

fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    println!("{}", part1(file.clone()));
    println!("{}", part2(file.clone()));
}
