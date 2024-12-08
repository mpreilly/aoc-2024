use rayon::prelude::*;
use std::{collections::HashSet, fs};

// "up" is +y! flipping the array to start.

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn add(&self, change_x: i32, change_y: i32) -> Option<Pos> {
        // None if the move takes us off the map
        if (self.x == 0 && change_x == -1) || (self.y == 0 && change_y == -1) {
            return None;
        }

        Some(Pos {
            x: ((self.x as i32) + change_x) as usize,
            y: ((self.y as i32) + change_y) as usize,
        })
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Guard {
    pos: Pos,
    direction: (i32, i32), // if heading up, it's going +0 x, +1 y, so (0, 1)
}

impl Guard {
    fn from_char(c: char, pos: Pos) -> Guard {
        Guard {
            pos,
            direction: match c {
                '^' => (0, 1),
                '>' => (1, 0),
                'v' => (0, -1),
                '<' => (-1, 0),
                _ => panic!("unknown symbol"),
            },
        }
    }

    // what *would* the next pos be, if we kept going
    fn next(&self) -> Option<Pos> {
        self.pos.add(self.direction.0, self.direction.1)
    }

    fn rotate(&mut self) {
        self.direction = match self.direction {
            (0, 1) => (1, 0),
            (1, 0) => (0, -1),
            (0, -1) => (-1, 0),
            (-1, 0) => (0, 1),
            _ => panic!("unknown direction"),
        }
    }

    fn advance(&mut self) {
        self.pos = self.next().unwrap();
    }
}

#[derive(Debug)]
struct Map {
    obstacles: HashSet<Pos>,
    max_x: usize,
    max_y: usize,
}

impl Map {
    fn move_guard(&self, guard: &mut Guard, maybe_extra_obstacle: Option<Pos>) -> bool {
        // if we're not actually using extra_obstacle, put it off the map
        let extra_obstacle = maybe_extra_obstacle.unwrap_or(Pos {
            x: self.max_x + 10,
            y: self.max_y + 10,
        });

        // at most 4 possible directions. maybe the guard is trapped (??)
        for _ in 0..4 {
            if let Some(new_pos) = guard.next() {
                if new_pos.x > self.max_x || new_pos.y > self.max_y {
                    return false;
                }
                if self.obstacles.contains(&new_pos) || new_pos == extra_obstacle {
                    guard.rotate();
                } else {
                    guard.advance();
                    return true;
                }
            } else {
                // this cat's off the map
                return false;
            }
        }

        false
    }
}

struct State {
    map: Map,
    guard: Guard,
}

impl State {
    fn from_string(s: &str) -> State {
        // flip lines so that +y is "up"
        let lines = s.lines().rev();
        let mut obstacles: HashSet<Pos> = HashSet::new();
        let mut guard: Option<Guard> = None;
        for (y, line) in lines.enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '.' => {}
                    '#' => {
                        obstacles.insert(Pos { x, y });
                    }
                    _ => {
                        guard = Some(Guard::from_char(c, Pos { x, y }));
                    }
                }
            }
        }

        State {
            map: Map {
                obstacles,
                max_x: s.lines().next().unwrap().len() - 1,
                max_y: s.lines().count() - 1,
            },
            guard: guard.unwrap(),
        }
    }
}

fn main() {
    let input = get_input(false);
    let state = State::from_string(&input);

    println!("{}", part1(&state.map, state.guard));
    println!("{}", part2_rayon(&state.map, state.guard));
}

fn get_input(toy: bool) -> String {
    let path = if toy { "toy_input.txt" } else { "input.txt" };
    fs::read_to_string(path).unwrap()
}

fn part1(map: &Map, mut guard: Guard) -> usize {
    let mut guard_positions: HashSet<Pos> = HashSet::new();
    guard_positions.insert(guard.pos);

    // seems like cycles are not possible
    while map.move_guard(&mut guard, None) {
        guard_positions.insert(guard.pos);
    }

    guard_positions.len()
}

// diff approach: the grid isn't that big. Check all the positions
fn part2(map: &Map, guard: Guard) -> usize {
    let mut new_obstacle_positions: HashSet<Pos> = HashSet::new();
    let start_guard_pos = guard.pos;

    for x in 0..=map.max_x {
        for y in 0..=map.max_y {
            let pos = Pos { x, y };
            if pos == start_guard_pos || map.obstacles.contains(&pos) {
                continue;
            }
            // guard implements copy, so this will give it a fresh guard instance to move around
            // without impacting subsequent iterations.
            if check_cycle(map, guard, pos) {
                new_obstacle_positions.insert(pos);
            }
        }
    }

    new_obstacle_positions.len()
}

fn part2_rayon(map: &Map, guard: Guard) -> usize {
    all_possible_positions(map, guard.pos)
        .par_iter()
        .filter(|&&pos| check_cycle(map, guard, pos))
        .count()
}

fn all_possible_positions(map: &Map, start_guard_pos: Pos) -> Vec<Pos> {
    let mut positions = Vec::new();
    for x in 0..=map.max_x {
        for y in 0..=map.max_y {
            let pos = Pos { x, y };
            if pos != start_guard_pos && !map.obstacles.contains(&pos) {
                positions.push(pos);
            }
        }
    }
    positions
}

fn check_cycle(map: &Map, mut guard: Guard, extra_obstacle: Pos) -> bool {
    let mut past_guard_states: HashSet<Guard> = HashSet::new();
    past_guard_states.insert(guard);

    while map.move_guard(&mut guard, Some(extra_obstacle)) {
        if past_guard_states.contains(&guard) {
            return true;
        }
        past_guard_states.insert(guard);
    }

    false
}

#[cfg(test)]
mod tests {
    // cargo test --release -- --nocapture 
    // to get better performance (12s -> 1s for part2_rayon) and to let it print

    use super::*;
    use std::time::Instant;

    #[test]
    fn part1_answer() {
        let input = get_input(false);
        let state = State::from_string(&input);
        let result = part1(&state.map, state.guard);
        assert_eq!(result, 5453);
    }

    #[test]
    fn part2_for_loop() {
        let input = get_input(false);
        let state = State::from_string(&input);

        let start = Instant::now();
        let result = part2(&state.map, state.guard);
        let duration = start.elapsed();
        println!("part2 duration (non-rayon): {:?}", duration);

        assert_eq!(result, 2188);
    }

    #[test]
    fn part2_with_rayon() {
        let input = get_input(false);
        let state = State::from_string(&input);

        let start = Instant::now();
        let result = part2_rayon(&state.map, state.guard);
        let duration = start.elapsed();
        println!("part2 duration (rayon parallelized): {:?}", duration);

        assert_eq!(result, 2188);
    }
}
