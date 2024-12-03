use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("File read error");
    let reports: Vec<Vec<i32>> = input.lines().map(parse_line).collect();
    
    let part1_answer = part1(&reports);
    println!("part 1: {}", part1_answer);

    let part2_answer = part2(&reports);
    println!("part 2: {}", part2_answer);
}

fn parse_line(line: &str) -> Vec<i32> {
    line.split_whitespace().map(|n| n.parse().unwrap()).collect()
}

fn part1(reports: &Vec<Vec<i32>>) -> usize {
    let is_safe_list: Vec<bool> = reports.iter().map(is_safe).collect();
    // println!("{:#?}", is_safe_list);
    is_safe_list.iter().filter(|&&r| r).count()
}

fn part2(reports: &Vec<Vec<i32>>) -> usize {
    let is_safe_list: Vec<bool> = reports.iter().map(is_safe2).collect();
    // println!("{:#?}", is_safe_list);
    is_safe_list.iter().filter(|&&r| r).count()
}

fn is_safe(report: &Vec<i32>) -> bool {
    let mut last_change: Option<i32> = None;
    for i in 1..report.len() {
        let change = report[i] - report[i-1];
        if change == 0 || change.abs() > 3 {
            return false;
        }
        if let Some(lc) = last_change {
            if sign(change) != sign(lc) {
                return false;
            }
        }
        last_change = Some(change);
    }
    true
}

fn is_safe2(report: &Vec<i32>) -> bool {
    (0..report.len()).any(|i| is_safe(&copy_and_remove_index(report, i)))
}

fn copy_and_remove_index(report: &Vec<i32>, index: usize) -> Vec<i32> {
    let mut new_vec = report.clone();
    new_vec.remove(index);
    new_vec
}

fn sign(n: i32) -> bool {
    n > 0
}
