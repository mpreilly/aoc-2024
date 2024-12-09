use gcd::Gcd;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Eq, Hash, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn dist(&self, other: &Pos) -> (i32, i32) {
        (
            other.x as i32 - self.x as i32,
            other.y as i32 - self.y as i32,
        )
    }

    fn add(&self, dx: i32, dy: i32) -> Option<Pos> {
        // None if the move takes us negative
        if (dx < 0 && self.x < dx.unsigned_abs() as usize)
            || (dy < 0 && self.y < dy.unsigned_abs() as usize)
        {
            return None;
        }

        Some(Pos {
            x: ((self.x as i32) + dx) as usize,
            y: ((self.y as i32) + dy) as usize,
        })
    }
}

struct Map {
    max_x: usize,
    max_y: usize,
    antennas_by_frequency: HashMap<char, Vec<Pos>>,
}

impl Map {
    fn from_string(s: &str) -> Map {
        let mut antennas_by_frequency: HashMap<char, Vec<Pos>> = HashMap::new();

        // flip lines so +y is "up" on grid
        for (y, line) in s.lines().rev().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '.' => {}
                    _ => antennas_by_frequency
                        .entry(c)
                        .or_default()
                        .push(Pos { x, y }),
                }
            }
        }

        Map {
            max_x: s.lines().next().unwrap().len() - 1,
            max_y: s.lines().count() - 1,
            antennas_by_frequency,
        }
    }

    fn contains(&self, p: &Pos) -> bool {
        p.x <= self.max_x && p.y <= self.max_y
    }
}

fn main() {
    let input = get_input(false);
    let map = Map::from_string(&input);

    println!("part 1: {}", part1(&map));
    println!("part 2: {}", part2(&map));
}

fn get_input(toy: bool) -> String {
    let path = if toy { "toy_input.txt" } else { "input.txt" };
    fs::read_to_string(path).unwrap()
}

fn part1(map: &Map) -> usize {
    let mut antinode_locations: HashSet<Pos> = HashSet::new();

    for antennas in map.antennas_by_frequency.values() {
        for (i, a1) in antennas[..(antennas.len() - 1)].iter().enumerate() {
            for a2 in antennas[(i + 1)..].iter() {
                let (dx, dy) = a1.dist(a2);
                if let Some(antinode1) = a2.add(dx, dy) {
                    if map.contains(&antinode1) {
                        antinode_locations.insert(antinode1);
                    }
                }
                if let Some(antinode2) = a1.add(-dx, -dy) {
                    if map.contains(&antinode2) {
                        antinode_locations.insert(antinode2);
                    }
                }
            }
        }
    }

    antinode_locations.len()
}

fn part2(map: &Map) -> usize {
    let mut antinode_locations: HashSet<Pos> = HashSet::new();

    for antennas in map.antennas_by_frequency.values() {
        for (i, a1) in antennas[..(antennas.len() - 1)].iter().enumerate() {
            for a2 in antennas[(i + 1)..].iter() {
                let (dx, dy) = get_smallest_step_in_line(a1, a2);
                add_points_in_line(map, &mut antinode_locations, a1, dx, dy, true);
                add_points_in_line(map, &mut antinode_locations, a1, dx, dy, false);
            }
        }
    }

    antinode_locations.len()
}

// to get all that are "exactly in line", we should get the distances down to their smallest change.
// e.g. (+1, -2) is the smallest we can go, because we can't divide that 1.
// divide both changes by the greatest common factor.
fn get_smallest_step_in_line(a1: &Pos, a2: &Pos) -> (i32, i32) {
    let (dx, dy) = a1.dist(a2);
    let gcd = (dx.unsigned_abs()).gcd(dy.unsigned_abs());
    (dx / (gcd as i32), dy / (gcd as i32))
}

fn add_points_in_line(
    map: &Map,
    set: &mut HashSet<Pos>,
    a1: &Pos,
    dx: i32,
    dy: i32,
    forward: bool,
) {
    let direction = if forward { 1 } else { -1 };
    let mut jump_num = 0;
    while {
        if let Some(antinode) = a1.add(dx * jump_num * direction, dy * jump_num * direction) {
            if map.contains(&antinode) {
                set.insert(antinode);
                true
            } else {
                false
            }
        } else {
            false
        }
    } {
        jump_num += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_answer() {
        let input = get_input(false);
        let map = Map::from_string(&input);
        let result = part1(&map);
        assert_eq!(result, 400);
    }

    #[test]
    fn part2_answer() {
        let input = get_input(false);
        let map = Map::from_string(&input);
        let result = part2(&map);
        assert_eq!(result, 1280);
    }
}
