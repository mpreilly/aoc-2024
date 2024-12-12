use std::{fs, u16};

// length of array to turn input into straight line: 95450
// which means largest file id is 47,725; can fit into u16

// representation of empty block
// messy, but ids start at 0 and we can't do '.' in u16, so use max val
const EMPTY: u16 = u16::MAX;

fn main() {
    let input = get_input(false);
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

fn get_input(toy: bool) -> Vec<u8> {
    let path = if toy { "toy_input.txt" } else { "input.txt" };
    let s = fs::read_to_string(path).unwrap();
    let trimmed = s.trim();
    trimmed
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

fn part1(map: &[u8]) -> u64 {
    let mut blocks = build_block_vec(map);

    let mut next_empty = 0;
    while blocks[next_empty] != EMPTY {
        next_empty += 1;
    }

    let mut cur_block = blocks.len() - 1;
    while cur_block > next_empty {
        let id = blocks[cur_block];
        if id != EMPTY {
            blocks[next_empty] = id;
            blocks[cur_block] = EMPTY;
            while blocks[next_empty] != EMPTY {
                next_empty += 1;
            }
        }
        cur_block -= 1
    }

    calculate_checksum(&blocks)
}

fn build_block_vec(map: &[u8]) -> Vec<u16> {
    let total_space: usize = map.iter().fold(0, |acc, n| acc + (*n as usize));
    let mut blocks = vec![EMPTY; total_space];

    let mut file_id = 0;
    let mut cur_block = 0_usize;
    for (i, n) in map.iter().enumerate() {
        // if it's empty, just advance the cur block bc they're already empty
        if i % 2 != 0 {
            cur_block += *n as usize;
        } else {
            for _ in 0..*n {
                blocks[cur_block] = file_id;
                cur_block += 1;
            }
            file_id += 1;
        }
    }

    blocks
}

fn calculate_checksum(blocks: &[u16]) -> u64 {
    blocks.iter().enumerate().fold(0, |acc, (i, n)| {
        acc + if *n != EMPTY {
            (*n as u64) * (i as u64)
        } else {
            0
        }
    })
}

fn part2(map: &[u8]) -> u64 {
    let mut blocks = build_block_vec(map);

    let mut cur_block = blocks.len() - 1;
    while cur_block > 0 {
        let id = blocks[cur_block];
        if id != EMPTY {
            let file_end = cur_block + 1;
            while cur_block > 0 && blocks[cur_block - 1] == id {
                cur_block -= 1;
            }
            let file_start = cur_block;
            let file_size = file_end - file_start;

            if let Some(empty_start) =
                find_first_large_enough_empty(&blocks, file_size, file_start)
            {
                for i in 0..file_size {
                    blocks[empty_start + i] = blocks[file_start + i];
                    blocks[file_start + i] = EMPTY;
                }
            }
        }
        cur_block = cur_block.saturating_sub(1);
    }

    calculate_checksum(&blocks)
}

fn find_first_large_enough_empty(
    blocks: &[u16],
    size: usize,
    end: usize,
) -> Option<usize> {
    let mut cur_block = 0;
    while cur_block < end {
        let id = blocks[cur_block];
        if id == EMPTY {
            let empty_start = cur_block;
            while blocks[cur_block + 1] == EMPTY {
                cur_block += 1;
            }
            let empty_end = cur_block;
            if (empty_end + 1) - empty_start >= size {
                return Some(empty_start);
            }
        }
        cur_block += 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_answer() {
        let result = part1(&get_input(false));
        assert_eq!(result, 6607511583593);
    }

    #[test]
    fn part2_answer() {
        let result = part2(&get_input(false));
        assert_eq!(result, 6636608781232);
    }
}
