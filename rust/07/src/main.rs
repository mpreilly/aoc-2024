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

    println!("part 1: {}", solution(&equations, false));
    println!("part 2: {}", solution(&equations, true));
}

fn get_input(toy: bool) -> String {
    let path = if toy { "toy_input.txt" } else { "input.txt" };
    fs::read_to_string(path).unwrap()
}

fn solution(equations: &[Equation], is_part_2: bool) -> u64 {
    equations
        .iter()
        .filter(|e| is_valid(e, is_part_2))
        .map(|e| e.result)
        .sum()
}

fn is_valid(equation: &Equation, is_part_2: bool) -> bool {
    can_hit_result(
        equation.result,
        equation.operands[0],
        &equation.operands[1..],
        is_part_2,
    )
}

fn can_hit_result(desired_result: u64, operand1: u64, remaining: &[u64], is_part_2: bool) -> bool {
    if operand1 > desired_result {
        return false;
    }

    if remaining.is_empty() {
        return operand1 == desired_result;
    }

    let operand2 = remaining[0];
    can_hit_result(
        desired_result,
        operand1 + operand2,
        &remaining[1..],
        is_part_2,
    ) || can_hit_result(
        desired_result,
        operand1 * operand2,
        &remaining[1..],
        is_part_2,
    ) || (is_part_2
        && can_hit_result(
            desired_result,
            format!("{}{}", operand1, operand2).parse().unwrap(),
            &remaining[1..],
            is_part_2,
        ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_answer() {
        let input = get_input(false);
        let equations: Vec<Equation> = input.lines().map(Equation::from_string).collect();
        let result = solution(&equations, false);
        assert_eq!(result, 1545311493300);
    }

    #[test]
    fn part2_answer() {
        let input = get_input(false);
        let equations: Vec<Equation> = input.lines().map(Equation::from_string).collect();
        let result = solution(&equations, true);
        assert_eq!(result, 169122112716571);
    }
}
