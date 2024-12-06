use std::collections::{HashMap, HashSet};
use std::fs;

type Page = u32;
type Update = Vec<Page>;
type RuleMap = HashMap<Page, HashSet<Page>>;

#[derive(Debug)]
struct Input {
    rules_per_page: RuleMap,
    updates: Vec<Update>,
}

fn main() {
    let input = parse_input(false);
    // println!("input: {:?}", input);

    println!("part 1: {}", part1(&input.updates, &input.rules_per_page))
}

fn parse_input(toy: bool) -> Input {
    let path = if toy { "toy_input.txt" } else { "input.txt" };
    let input = fs::read_to_string(path).unwrap();
    let parts: Vec<&str> = input.split("\n\n").collect();
    Input {
        rules_per_page: parse_rules(parts[0]),
        updates: parse_updates(parts[1]),
    }
}

fn parse_rules(rule_str: &str) -> RuleMap {
    rule_str
        .lines()
        .map(parse_rule_tuple)
        .fold(HashMap::new(), |mut acc, (p1, p2)| {
            acc.entry(p1).or_insert(HashSet::new()).insert(p2);
            acc
        })
}

fn parse_rule_tuple(rule_line: &str) -> (Page, Page) {
    let pages: Vec<Page> = rule_line
        .split("|")
        .map(|p| p.parse::<Page>().unwrap())
        .collect();
    (pages[0], pages[1])
}

fn parse_updates(update_str: &str) -> Vec<Update> {
    update_str
        .lines()
        .map(|l| l.split(",").map(|s| s.parse().unwrap()).collect())
        .collect()
}

fn part1(updates: &Vec<Update>, rule_map: &RuleMap) -> u32 {
    updates
        .iter()
        .filter(|update| is_valid(update, rule_map))
        .map(middle_elem)
        .sum()
}

fn is_valid(update: &Update, rule_map: &RuleMap) -> bool {
    let mut pages_seen: HashSet<Page> = HashSet::new();
    for page in update {
        let maybe_rules = rule_map.get(page);
        if let Some(rules) = maybe_rules {
            // if any of these pages came before this one, it's invalid
            if pages_seen.intersection(rules).count() != 0 {
                return false;
            }
        }
        pages_seen.insert(*page);
    }
    true
}

fn middle_elem(update: &Update) -> u32 {
    update[update.len() / 2]
}

// 47|53
// if an update includes both page number 47 and page number 53,
// then page number 47 must be printed at some point before page number 53

// should be sufficient to say, if I see 47, I just have to make sure
// 53 isn't among the numbers I've already seen.
// otherwise, if 53 is later in the sequence, that's fine.
//        And if it's not, they're not both in the update.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_answer() {
        let input = parse_input(false);
        let result = part1(&input.updates, &input.rules_per_page);
        assert_eq!(result, 4959);
    }
}
