use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("file read fail");
    let (mut l1, mut l2): (Vec<usize>, Vec<usize>)  = parse_lists(&input);
    l1.sort();
    l2.sort();

    let part1_answer = part1(&l1, &l2);
    println!("part1: {}", part1_answer);

    let part2_answer = part2(&l1, &l2);
    println!("part2: {}", part2_answer);
}

fn parse_lists(input: &str) -> (Vec<usize>, Vec<usize>) {
    input.lines().map(line_to_tuple).unzip()
}

fn line_to_tuple(line: &str) -> (usize, usize) {
    let num_vec: Vec<&str> = line.split_whitespace().collect();
    let err_msg = "num parse fail";
    (num_vec[0].parse().expect(err_msg), num_vec[1].parse().expect(err_msg))
}

fn part1(l1: &[usize], l2: &[usize]) -> usize {
    l1.iter().zip(l2).fold(0, |acc, (a, b)| acc + dist(a, b))
}

fn dist(a: &usize, b: &usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn part2(l1:&[usize], l2: &[usize]) -> usize {
    let l2_freq_map: HashMap<usize, usize> = build_freq_map(l2);
    l1.iter().fold(0, |acc, n| acc + (n * l2_freq_map.get(n).unwrap_or(&0)))
}

fn build_freq_map(l: &[usize]) -> HashMap<usize, usize> {
    l.iter().fold(HashMap::new(), |mut map, n| {
        map.insert(*n, map.get(n).map_or_else(|| 0, |n| *n) + 1);
        map
    })
}
