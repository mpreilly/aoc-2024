use std::fs;
use rayon::prelude::*;

type Stone = u64;

fn main() {
    let stones = get_input(false);
    // println!("part 1: {}", part1_par(stones.clone()));
    println!("part 2: {}", part2_par(stones.clone()));
}

fn get_input(toy: bool) -> Vec<Stone> {
    let path = if toy { "toy_input.txt" } else { "input.txt" };
    let input = fs::read_to_string(path).unwrap();
    input.split_whitespace().map(|n| n.parse().unwrap()).collect()
}

fn part1(mut stones: Vec<Stone>) -> usize {
    for _ in 0..25 {
        let mut i = 0;
        while i < stones.len() {
            let stone = stones[i];
            if stone == 0 {
                stones[i] = 1;
                i += 1;
            } else {
                let stone_str = stone.to_string();
                if stone_str.len() % 2 == 0 {
                    let stone_1: Stone = stone_str[..(stone_str.len() / 2)].parse().unwrap();
                    let stone_2: Stone = stone_str[(stone_str.len() / 2)..].parse().unwrap();
                    stones.splice(i..i+1, [stone_1, stone_2].into_iter());
                    i += 2;
                } else {
                    stones[i] = stone * 2024;
                    i += 1;
                }
            }
        }
    }
    stones.len()
}

fn part1_opt(mut stones: Vec<Stone>) -> usize {
    for _ in 0..25 {
        for i in 0..stones.len() {
            let stone = stones[i];
            if stone == 0 {
                stones[i] = 1;
            } else {
                let stone_str = stone.to_string();
                if stone_str.len() % 2 == 0 {
                    let stone_1: Stone = stone_str[..(stone_str.len() / 2)].parse().unwrap();
                    let stone_2: Stone = stone_str[(stone_str.len() / 2)..].parse().unwrap();
                    stones[i] = stone_1;
                    stones.push(stone_2);
                } else {
                    stones[i] = stone * 2024;
                }
            }
        }
    }
    stones.len()
}

fn part2(mut stones: Vec<Stone>) -> usize {
    for blink in 0..75 {
        println!("blink: {}; stones.len() = {}", blink, stones.len());
        // take size at start so it's not changed by adding stones to the end
        for i in 0..stones.len() {
            let stone = stones[i];
            if stone == 0 {
                stones[i] = 1;
            } else {
                let stone_str = stone.to_string();
                if stone_str.len() % 2 == 0 {
                    let stone_1: Stone = stone_str[..(stone_str.len() / 2)].parse().unwrap();
                    let stone_2: Stone = stone_str[(stone_str.len() / 2)..].parse().unwrap();
                    stones[i] = stone_1;
                    stones.push(stone_2);
                } else {
                    stones[i] = stone * 2024;
                }
            }
        }
    }
    stones.len()
}

fn part1_par(mut stones: Vec<Stone>) -> usize {
    for blink in 0..25 {
        println!("blink: {}; stones.len() = {}", blink, stones.len());
        stones = (0..stones.len())
            .into_par_iter()
            .fold(Vec::new, 
                |mut acc, i| {
                    let stone = stones[i];
                    if stone == 0 {
                        acc.push(1)
                    } else {
                        let stone_str = stone.to_string();
                        if stone_str.len() % 2 == 0 {
                            let stone_1: Stone = stone_str[..(stone_str.len() / 2)].parse().unwrap();
                            let stone_2: Stone = stone_str[(stone_str.len() / 2)..].parse().unwrap();
                            acc.push(stone_1);
                            acc.push(stone_2);
                        } else {
                            acc.push(stone * 2024);
                        }
                    }
                    acc
            })
            .reduce(Vec::new,
                |mut acc, v| {acc.extend(v); acc});
    }
    stones.len()
}

fn part2_par(mut stones: Vec<Stone>) -> usize {
    for blink in 0..75 {
        println!("blink: {}; stones.len() = {}", blink, stones.len());
        stones = (0..stones.len())
            .into_par_iter()
            .fold(Vec::new, 
                |mut acc, i| {
                    let stone = stones[i];
                    if stone == 0 {
                        acc.push(1)
                    } else {
                        let stone_str = stone.to_string();
                        if stone_str.len() % 2 == 0 {
                            let stone_1: Stone = stone_str[..(stone_str.len() / 2)].parse().unwrap();
                            let stone_2: Stone = stone_str[(stone_str.len() / 2)..].parse().unwrap();
                            acc.push(stone_1);
                            acc.push(stone_2);
                        } else {
                            acc.push(stone * 2024);
                        }
                    }
                    acc
            })
            .reduce(Vec::new,
                |mut acc, v| {acc.extend(v); acc});
    }
    stones.len()
}
