use crate::day07::models::Equation;

fn can_satisfy_rec_one(equation: &Equation, position: usize, current: u128) -> bool {
    // Check the result if we are at the end
    if position == equation.operands.len() {
        return current == equation.result;
    }

    // Try both options
    can_satisfy_rec_one(equation, position + 1, current * equation.operands[position])
        || can_satisfy_rec_one(equation, position + 1, current + equation.operands[position])
}

fn can_satisfy_one(equation: &Equation) -> bool {
    can_satisfy_rec_one(equation, 1, equation.operands[0])
}

pub fn solve_part_one(equations: &[Equation]) -> u128 {
    equations
        .iter()
        .filter_map(|equation| {
            if can_satisfy_one(equation) {
                Some(equation.result)
            } else {
                None
            }
        })
        .sum()
}

#[inline(always)]
fn concat(left: u128, right: u128) -> u128 {
    left * 10u128.pow(right.ilog10() + 1) + right
}

fn can_satisfy_rec_two(equation: &Equation, position: usize, current: u128) -> bool {
    // Check the result if we are at the end
    if position == equation.operands.len() {
        return current == equation.result;
    }

    // Try both options
    can_satisfy_rec_two(equation, position + 1, current * equation.operands[position])
        || can_satisfy_rec_two(equation, position + 1, current + equation.operands[position])
        || can_satisfy_rec_two(equation, position + 1, concat(current, equation.operands[position]))
}

fn can_satisfy_two(equation: &Equation) -> bool {
    can_satisfy_rec_two(equation, 1, equation.operands[0])
}

pub fn solve_part_two(equations: &[Equation]) -> u128 {
    equations
        .iter()
        .filter_map(|equation| {
            if can_satisfy_two(equation) {
                Some(equation.result)
            } else {
                None
            }
        })
        .sum()
}
