// bucket the antennas into lists by frequency.
// within each frequency, consider all possible pairs of antennas.
// for each pair:
//   calculate their x/y distance.
//   find the points that are that distance in the opposite directions
//     d = dist(a, b) is (b.x - a.x, b.y - a.y)
//     so resonance points are at (b.x + d.x, b.y + d.y)
//       and (a.x - d.x, a.y - d.y)
//   if they're within the bounds of the map, add to set

// get unique combinations only (order doesn't matter)
// for i in 0..(len - 1)
//   for j in (i+1)..len (also avoids pairs with self)

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
        if (self.x == 0 && dx == -1) || (self.y == 0 && dy == -1) {
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

    println!("{}", part1(&map));
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
}
