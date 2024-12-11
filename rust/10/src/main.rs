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

fn part1(map: &Map) -> u32 {
    sum_value(map, get_reachable_peak_count)
}

fn part2(map: &Map) -> u32 {
    sum_value(map, get_trail_count)
}

fn sum_value(map: &Map, value_func: impl Fn(&Map, usize, usize) -> u32) -> u32 {
    let trailheads = get_trailhead_set(map);
    trailheads.iter().map(|&(r, c)| value_func(map, r, c)).sum()
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

// unused, just for kicks
fn get_trailhead_set_functional(map: &Map) -> HashSet<Pos> {
    map.iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &val)| val == 0)
                .map(move |(c, _)| (r, c))
        })
        .collect()
}

fn get_reachable_peak_count(map: &Map, start_r: usize, start_c: usize) -> u32 {
    let mut reachable_peaks: HashSet<Pos> = HashSet::new();
    peak_search(map, start_r, start_c, |pos| {
        reachable_peaks.insert(pos);
    });
    reachable_peaks.len() as u32
}

fn get_trail_count(map: &Map, start_r: usize, start_c: usize) -> u32 {
    let mut trail_count = 0;
    peak_search(map, start_r, start_c, |_| trail_count += 1);
    trail_count
}

fn peak_search(map: &Map, start_r: usize, start_c: usize, mut top_action: impl FnMut(Pos)) {
    let mut frontier: Vec<Pos> = vec![(start_r, start_c)];

    while let Some(cur_pos) = frontier.pop() {
        let cur_val = map[cur_pos.0][cur_pos.1];
        if cur_val == 9 {
            top_action(cur_pos);
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
