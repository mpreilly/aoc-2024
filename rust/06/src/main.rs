use std::{collections::HashSet, fs};

// "up" is +y! flipping the array to start.

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Pos {
    x: usize, 
    y: usize
}

impl Pos {
    fn add(&self, change_x: i32, change_y: i32) -> Option<Pos> {
        // None if the move takes us off the map
        if (self.x == 0 && change_x == -1) || (self.y == 0 && change_y == -1) {
            return None;
        }
        
        Some(Pos {
            x: ((self.x as i32) + change_x) as usize,
            y: ((self.y as i32) + change_y) as usize
        })
    }
}

#[derive(Debug)]
struct Guard {
    pos: Pos,
    direction: (i32, i32) // if heading up, it's going +0 x, +1 y, so (0, 1)
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
                _ => panic!("unknown symbol")
            }
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
            _ => panic!("unknown direction")
        }
    }

    fn advance(&mut self) {
        self.pos = self.next().unwrap();
    }
}

#[derive(Debug)]
struct Map {
    guard: Guard,
    obstacles: HashSet<Pos>,
    max_x: usize,
    max_y: usize
}

impl Map {
    fn from_string(s: &str) -> Map {
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

        Map {
            guard: guard.unwrap(),
            obstacles,
            max_x: s.lines().next().unwrap().len() - 1,
            max_y: s.lines().count() - 1
        }
    }

    fn move_guard(&mut self) -> bool {

        // at most 4 possible directions. maybe the guard is trapped (??)
        for _ in 0..4 {
            if let Some(new_pos) = self.guard.next() {
                if new_pos.x > self.max_x || new_pos.y > self.max_y {
                    return false;
                }
                if self.obstacles.contains(&new_pos) {
                    self.guard.rotate();
                } else {
                    self.guard.advance();
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

fn main() {
    let input = get_input(false);
    let map = Map::from_string(&input);
    println!("{}", part1(map))
}

fn part1(mut map: Map) -> usize {
    let mut guard_positions: HashSet<Pos> = HashSet::new();
    guard_positions.insert(map.guard.pos);

    // seems like cycles are not possible
    while map.move_guard() {
        guard_positions.insert(map.guard.pos);
    }

    guard_positions.len()
}

fn get_input(toy: bool) -> String {
    let path = if toy { "toy_input.txt" } else { "input.txt" };
    fs::read_to_string(path).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_answer() {
        let input = get_input(false);
        let map = Map::from_string(&input);
        let result = part1(map);
        assert_eq!(result, 5453);
    }
}