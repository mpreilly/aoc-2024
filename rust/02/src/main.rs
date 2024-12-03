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
    // println!("{:#?}", reports.iter().map(|r| is_safe(r, -1)).collect::<Vec<_>>());
    reports.iter().filter(|r| is_safe(r, -1)).count()
}

fn part2(reports: &Vec<Vec<i32>>) -> usize {
    // println!("{:#?}", reports.iter().map(is_safe2).collect::<Vec<_>>());
    reports.iter().filter(|r| is_safe2(r)).count()
}

fn is_safe(report: &Vec<i32>, skip_index: i32) -> bool {
    if report.len() < 2 {
        return true;
    }
    let start_index = if skip_index == 0 { 1 } else { 0 };
    let mut last_change: i32 = report[start_index + 1] - report[start_index];
    let mut last_num: i32 = report[start_index];
    for i in (start_index + 1)..report.len() {
        if i as i32 == skip_index {
            continue;
        }
        let change = report[i] - last_num;
        if sign(change) != sign(last_change) || change == 0 || change.abs() > 3 {
            return false;
        }
        last_change = change;
        last_num = report[i];
    }
    true
}

fn is_safe_reverse(report: &Vec<i32>, skip_index: i32) -> bool {
    if report.len() < 2 {
        return true;
    }
    let start_index = if skip_index == 0 { 1 } else { 0 };
    let mut last_change: i32 = report[start_index + 1] - report[start_index];
    let mut last_num: i32 = report[start_index];
    for i in ((start_index + 1)..report.len()).rev() {
        if i as i32 == skip_index {
            continue;
        }
        let change = report[i] - last_num;
        if sign(change) != sign(last_change) || change == 0 || change.abs() > 3 {
            return false;
        }
        last_change = change;
        last_num = report[i];
    }
    true
}

fn is_safe2(report: &Vec<i32>) -> bool {
    (0..report.len()).any(|i| is_safe(report, i as i32) || is_safe_reverse(report, i as i32))
}

fn sign(n: i32) -> bool {
    n > 0
}
