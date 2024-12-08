use std::fs;

struct Equation {
    result: u64,
    operands: Vec<u64>,
}

impl Equation {
    fn from_string(s: &str) -> Equation {
        let parts: Vec<&str> = s.split(": ").collect();
        Equation {
            result: parts[0].parse().unwrap(),
            operands: parts[1].split(" ").map(|n| n.parse().unwrap()).collect(),
        }
    }
}

fn main() {
    let input = get_input(false);
    let equations: Vec<Equation> = input.lines().map(Equation::from_string).collect();

    println!("part 1: {}", part1(&equations))
}

fn get_input(toy: bool) -> String {
    let path = if toy { "toy_input.txt" } else { "input.txt" };
    fs::read_to_string(path).unwrap()
}

fn part1(equations: &[Equation]) -> u64 {
    equations
        .iter()
        .filter(|e| is_valid(e))
        .map(|e| e.result)
        .sum()
}

fn is_valid(equation: &Equation) -> bool {
    can_hit_result(
        equation.result,
        equation.operands[0],
        &equation.operands[1..],
    )
}

fn can_hit_result(desired_result: u64, operand1: u64, remaining: &[u64]) -> bool {
    if operand1 > desired_result {
        return false;
    }

    if remaining.is_empty() {
        return operand1 == desired_result;
    }

    let operand2 = remaining[0];
    can_hit_result(desired_result, operand1 + operand2, &remaining[1..])
        || can_hit_result(desired_result, operand1 * operand2, &remaining[1..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_answer() {
        let input = get_input(false);
        let equations: Vec<Equation> = input.lines().map(Equation::from_string).collect();
        let result = part1(&equations);
        assert_eq!(result, 1545311493300);
    }
}