use std::{collections::HashSet, fs};

type Map = Vec<Vec<u8>>;
type Pos = (usize, usize);

fn main() {
    let map = get_input(false);
    println!("part 1: {}", part1(&map));
    println!("part 2: {}", part2(&map));
}

fn get_input(toy: bool) -> Map {
    let path = if toy { "toy_input.txt" } else { "input.txt" };
    let s = fs::read_to_string(path).unwrap();
    s.lines()
        .map(|l| l.chars().map(|n| n.to_digit(10).unwrap() as u8).collect())
        .collect()
}

fn part1(map: &Map) -> usize {
    let trailheads = get_trailhead_set(map);
    trailheads
        .iter()
        .map(|&(r, c)| get_reachable_peak_count(map, r, c))
        .sum()
}

fn get_trailhead_set(map: &Map) -> HashSet<Pos> {
    let mut trailheads: HashSet<Pos> = HashSet::new();
    for (r, row) in map.iter().enumerate() {
        for (c, val) in row.iter().enumerate() {
            if *val == 0 {
                trailheads.insert((r, c));
            }
        }
    }
    trailheads
}

fn get_reachable_peak_count(map: &Map, start_r: usize, start_c: usize) -> usize {
    let mut reachable_peaks: HashSet<Pos> = HashSet::new();
    let mut frontier: Vec<Pos> = vec![(start_r, start_c)];

    while let Some(cur_pos) = frontier.pop() {
        let cur_val = map[cur_pos.0][cur_pos.1];
        if cur_val == 9 {
            reachable_peaks.insert(cur_pos);
            continue;
        }
        if cur_pos.0 > 0 && map[cur_pos.0 - 1][cur_pos.1] == cur_val + 1 {
            frontier.push((cur_pos.0 - 1, cur_pos.1))
        }
        if cur_pos.1 > 0 && map[cur_pos.0][cur_pos.1 - 1] == cur_val + 1 {
            frontier.push((cur_pos.0, cur_pos.1 - 1))
        }
        if cur_pos.0 < map.len() - 1 && map[cur_pos.0 + 1][cur_pos.1] == cur_val + 1 {
            frontier.push((cur_pos.0 + 1, cur_pos.1))
        }
        if cur_pos.1 < map[0].len() - 1 && map[cur_pos.0][cur_pos.1 + 1] == cur_val + 1 {
            frontier.push((cur_pos.0, cur_pos.1 + 1))
        }
    }

    reachable_peaks.len()
}

fn part2(map: &Map) -> u32 {
    let trailheads = get_trailhead_set(map);
    trailheads
        .iter()
        .map(|&(r, c)| get_trail_count(map, r, c))
        .sum()
}

fn get_trail_count(map: &Map, start_r: usize, start_c: usize) -> u32 {
    let mut trail_count= 0;
    let mut frontier: Vec<Pos> = vec![(start_r, start_c)];

    while let Some(cur_pos) = frontier.pop() {
        let cur_val = map[cur_pos.0][cur_pos.1];
        if cur_val == 9 {
            trail_count += 1;
            continue;
        }
        if cur_pos.0 > 0 && map[cur_pos.0 - 1][cur_pos.1] == cur_val + 1 {
            frontier.push((cur_pos.0 - 1, cur_pos.1))
        }
        if cur_pos.1 > 0 && map[cur_pos.0][cur_pos.1 - 1] == cur_val + 1 {
            frontier.push((cur_pos.0, cur_pos.1 - 1))
        }
        if cur_pos.0 < map.len() - 1 && map[cur_pos.0 + 1][cur_pos.1] == cur_val + 1 {
            frontier.push((cur_pos.0 + 1, cur_pos.1))
        }
        if cur_pos.1 < map[0].len() - 1 && map[cur_pos.0][cur_pos.1 + 1] == cur_val + 1 {
            frontier.push((cur_pos.0, cur_pos.1 + 1))
        }
    }

    trail_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_answer() {
        let map = get_input(false);
        let result = part1(&map);
        assert_eq!(result, 510);
    }

    #[test]
    fn part2_answer() {
        let map = get_input(false);
        let result = part2(&map);
        assert_eq!(result, 1058);
    }
}