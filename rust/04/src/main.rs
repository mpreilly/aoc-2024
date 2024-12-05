use std::fs;

fn main() {
    let grid = parse_grid(false);

    println!("part1: {}", part1(&grid));
    println!("part2: {}", part2(&grid));
}

fn parse_grid(toy: bool) -> Vec<Vec<char>> {
    let path = if toy { "toy_input.txt" } else { "input.txt" };
    let input = fs::read_to_string(path).unwrap();
    input.lines().map(|l| l.chars().collect()).collect()
}

fn part1(grid: &Vec<Vec<char>>) -> i32 {
    let mut xmas_count = 0;
    let mas = vec!['M', 'A', 'S'];

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            // println!("grid: {:?}", grid);
            // println!("grid[r]: {:?}", grid[r]);
            // println!("grid[r][c]: {:?}", grid[r][c]);
            // println!("grid[r][c] == 'X': {:?}", grid[r][c] == 'X');
            if grid[r][c] == 'X' {
                xmas_count += search_all_dirs(grid, &mas[..], r as i32, c as i32)
            }
        }
    }
    xmas_count
}

fn search_all_dirs(grid: &Vec<Vec<char>>, search_str: &[char], start_r: i32, start_c: i32) -> i32 {
    let mut count = 0;
    for r_move in -1..=1 {
        for c_move in -1..=1 {
            if r_move == 0 && c_move == 0 {
                continue;
            }
            if search(
                grid,
                search_str,
                start_r + r_move,
                start_c + c_move,
                r_move,
                c_move,
            ) {
                count += 1;
            }
        }
    }
    count
}

fn search(
    grid: &Vec<Vec<char>>,
    search_str: &[char],
    start_r: i32,
    start_c: i32,
    r_move: i32,
    c_move: i32,
) -> bool {
    if search_str.is_empty() {
        return true;
    }
    if start_r < 0
        || start_r as usize >= grid.len()
        || start_c < 0
        || start_c as usize >= grid[0].len()
    {
        return false;
    }
    grid[start_r as usize][start_c as usize] == search_str[0]
        && search(
            grid,
            &search_str[1..],
            start_r + r_move,
            start_c + c_move,
            r_move,
            c_move,
        )
}

fn part2(grid: &Vec<Vec<char>>) -> i32 {
    let mut xmas_count = 0;

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == 'A' && check_mas_crossing(grid, r, c) {
                xmas_count += 1;
            }
        }
    }
    xmas_count
}

fn check_mas_crossing(grid: &Vec<Vec<char>>, start_r: usize, start_c: usize) -> bool {
    let mas = vec!['M', 'A', 'S'];

    // if A is at the edge, can't have anything outside
    if start_r == 0
        || start_r >= grid.len() - 1
        || start_c == 0
        || start_c >= grid[0].len() - 1
    {
        return false;
    }

    // need two of the four possible mas-layouts:
    // starting r-1, c-1 and moving r+1 c+1
    // r-1, c+1 -> r+1, c-1
    // r+1, c-1 -> r-1, c+1
    // r+1, r+1 -> r-1, c-1
    let mut mas_count = 0;

    for r_start in [-1, 1] {
        for c_start in [-1, 1] {
            if grid[(start_r as i32 + r_start) as usize][(start_c as i32 + c_start) as usize] == 'M' && grid[(start_r as i32 - r_start) as usize][(start_c as i32 - c_start) as usize] == 'S' {
                mas_count += 1;
            }
        }
    }

    mas_count == 2
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_answer() {
        let result = part1(&parse_grid(false));
        assert_eq!(result, 2603);
    }

    #[test]
    fn part2_answer() {
        let result = part2(&parse_grid(false));
        assert_eq!(result, 1965);
    }
}
