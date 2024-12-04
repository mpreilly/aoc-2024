use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    re.find_iter(input)
        .map(|m| m.as_str())
        .fold(0, |acc, s| acc + mul(s))
}

fn mul(mul_str: &str) -> i32 {
    let re = Regex::new(r"[0-9]{1,3}").unwrap();
    re.find_iter(mul_str)
        .map(|m| m.as_str())
        .fold(1, |acc, s| acc * s.parse::<i32>().unwrap())
}

fn part2(input: &str) -> i32 {
    let re = Regex::new(r"(mul\([0-9]{1,3},[0-9]{1,3}\))|(do\(\))|(don't\(\))").unwrap();
    let mut enabled = true;
    re.find_iter(input).map(|m| m.as_str()).fold(0, |acc, s| {
        acc + match s {
            "do()" => {
                enabled = true;
                0
            }
            "don't()" => {
                enabled = false;
                0
            }
            _ => {
                if enabled {
                    mul(s)
                } else {
                    0
                }
            }
        }
    })
}
