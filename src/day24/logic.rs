use crate::day24::models::{FullGateError, Gate, Operation, System, Wire};
use hashbrown::{HashMap, HashSet};
use once_cell::sync::Lazy;
use std::collections::VecDeque;

static RESULT_TRUTH_TABLE: Lazy<Vec<bool>> = Lazy::new(|| vec![false, true, true, false, true, false, false, true]);
static CARRY_TRUTH_TABLE: Lazy<Vec<bool>> = Lazy::new(|| vec![false, false, false, true, false, true, true, true]);

fn build_wire_values_from_wires(wires: &[Wire]) -> HashMap<String, bool> {
    wires.iter().map(|wire| (wire.name.clone(), wire.value)).collect()
}

fn simulate_gates<'a>(
    wire_destinations: &HashMap<String, Vec<&'a Gate>>,
    wire_values: &mut HashMap<String, bool>,
    mut signals: VecDeque<&'a String>,
) {
    while let Some(signal) = signals.pop_front() {
        // For every gate connected to this wire, try to perform an operation
        if let Some(gates) = wire_destinations.get(signal) {
            // signal must have a value by now, it cannot be created otherwise.
            // It is then safe to unwrap for it's value
            let wire_value = wire_values[signal];

            for gate in gates {
                // If output was already generated, don't do anything
                // This should be useless since a wire should only be connected to one output
                if wire_values.contains_key(&gate.result) {
                    continue;
                }

                // Determinate the other wire name
                let other = if &gate.left == signal { &gate.right } else { &gate.left };

                // Try to get its value
                if let Some(other_value) = wire_values.get(other) {
                    // Perform the operation
                    let result = gate.operation.execute(wire_value, *other_value);

                    // Store the result
                    wire_values.insert(gate.result.clone(), result);

                    // Trigger a signal for this wire
                    signals.push_back(&gate.result);
                }
            }
        }
    }
}

fn reconstruct_output(wire_values: &HashMap<String, bool>) -> u64 {
    // Build the result value
    let mut result: u64 = 0;
    for (key, value) in wire_values {
        if key.starts_with("z") && *value == true {
            // Get the positioning of the value
            let position: u64 = key.as_str()[1..].parse().unwrap();
            result += 1 << position;
        }
    }

    result
}

fn get_wire_destinations_from_gates(gates: &[Gate]) -> HashMap<String, Vec<&Gate>> {
    let mut wire_destinations: HashMap<String, Vec<&Gate>> = HashMap::new();
    for gate in gates {
        wire_destinations.entry(gate.left.clone()).or_default().push(gate);
        wire_destinations.entry(gate.right.clone()).or_default().push(gate);
    }
    wire_destinations
}

pub fn solve_part_one(system: &System) -> u64 {
    // First we create a map from every wire name to a list of associated gates
    let wire_destinations = get_wire_destinations_from_gates(&system.gates);

    // Then we create a hashmap that store wire values
    let mut wire_values = build_wire_values_from_wires(&system.wires);

    // Add every wire initial value to the signaling list
    let signals: VecDeque<&String> = system.wires.iter().map(|wire| &wire.name).collect();

    // Simulate running program
    simulate_gates(&wire_destinations, &mut wire_values, signals);

    // Compute the output
    reconstruct_output(&wire_values)
}

pub fn solve_part_two(system: &System) -> u32 {
    // A binary adder always use three values to work (except for the lowest digit):
    // the left operand, the right operand and the carry from the previous bit operation.

    // In order to test our program, we just have to test every adder from the z00 adder to the
    // z44 adder. Since every adder only depends on the carry of the last operation and not
    // of the full operation, giving that we tested the adder for z_n, we can test the adder for
    // z_(n+1) with only 8 inputs.

    // Doing this will allow us to detect which wire contains the carry of this operation. This
    // will allow us to accelerate the simulation of the next target.

    // In order to perform this operation, we make an assumption: the half adder of z00 is correct.
    // TODO: check this assumption and perform necessary checks if required

    // 1. We want a system that we can modify
    let system = system.clone();

    // 1. Create a mapping of current program
    let wire_destinations = get_wire_destinations_from_gates(&system.gates);

    // 2. Detect the carry of the first operation
    let mut carry_wire = system
        .gates
        .iter()
        .filter(|gate| {
            ((&gate.left == "x00" && &gate.right == "y00") || (&gate.left == "y00" && &gate.right == "x00"))
                && gate.operation == Operation::And
        })
        .map(|gate| gate.result.clone())
        .next()
        .unwrap();

    // 3. Check the next full adder
    let mut swapped_wires = vec![];
    for to_test in 1..44 {
        // Check the gate to detect if the full adder for bit to_test is right
        let is_valid = check_full_adder(&wire_destinations, &carry_wire, to_test);

        match is_valid {
            Ok(new_carry_wirer) => carry_wire = new_carry_wirer,
            Err((impacted_gates, errors)) => {
                println!("===We found an error at the full adder {to_test}===");

                // Simple case, we have the right types of gates, the output is reached but is wrong
                // Count the types of each gates
                let mut xor = 0;
                let mut and = 0;
                let mut or = 0;
                for gate in &impacted_gates {
                    match gate.operation {
                        Operation::And => and += 1,
                        Operation::Xor => xor += 1,
                        Operation::Or => or += 1,
                    }
                }

                if errors == vec![FullGateError::OutputBadTruthTable, FullGateError::NoValidCarry]
                    && xor == 2
                    && and == 2
                    && or == 1
                {
                    carry_wire = fix_case_one(to_test, &carry_wire, &impacted_gates, &mut swapped_wires);
                } else {
                    println!("{:#?}", &swapped_wires);
                    println!("{carry_wire}");
                    println!("{to_test}:{:#?}:{:#?}", impacted_gates, errors);
                    unimplemented!()
                }
            }
        }
    }

    0
}

fn fix_case_one(
    to_test: i32,
    carry_wire: &String,
    impacted_gates: &HashSet<Gate>,
    swapped_wires: &mut Vec<String>,
) -> String {
    // We now that gates of this full adder or mixed up, we need to correct them
    let x_wire = format!("x{:<02}", to_test);
    let y_wire = format!("y{:<02}", to_test);
    let result_wire = format!("z{:<02}", to_test);

    // Get the list of wires
    let mut impacted_wires = HashSet::new();
    for gate in impacted_gates {
        impacted_wires.insert(gate.left.clone());
        impacted_wires.insert(gate.right.clone());
        impacted_wires.insert(gate.result.clone());
    }

    // The carry should be the only wire that is not used as an input
    let mut correct_carry = None;
    for wire in &impacted_wires {
        if wire != &x_wire && wire != &y_wire && wire != carry_wire && wire != &result_wire {
            let mut is_correct = true;
            for gate in impacted_gates {
                if &gate.left == wire || &gate.right == wire {
                    is_correct = false;
                    break;
                }
            }
            if is_correct == true {
                correct_carry = Some(wire.clone());
                break;
            }
        }
    }
    // extract the value
    let correct_carry = if let Some(correct_carry) = correct_carry {
        correct_carry
    } else {
        unreachable!()
    };

    // Correct carry must be the output of the "or" gate. We take our existing our
    // gate to get valid input
    let (carry_or_gate, correct_carry_or_gate) = impacted_gates
        .iter()
        .filter(|gate| gate.operation == Operation::Or)
        .map(|gate| {
            (
                gate.clone(),
                Gate {
                    left: gate.left.clone(),
                    operation: Operation::Or,
                    right: gate.right.clone(),
                    result: correct_carry.clone(),
                },
            )
        })
        .next()
        .unwrap();

    // Now we search for the xor gate that uses the carry. It allows use to
    // determinate the correct output of the xor gate between x and y
    let (carry_xor_gate, correct_carry_xor_gate) = impacted_gates
        .iter()
        .filter(|gate| gate.operation == Operation::Xor && (gate.left == *carry_wire || gate.right == *carry_wire))
        .map(|gate| {
            (
                gate.clone(),
                Gate {
                    left: gate.left.clone(),
                    operation: Operation::Xor,
                    right: gate.right.clone(),
                    result: result_wire.clone(),
                },
            )
        })
        .next()
        .unwrap();

    // Now we search for the valid first xor gate. It is the gate that output one
    // of the correct_carry_xor_gate input
    let (input_xor_gate, correct_input_xor_gate) = impacted_gates
        .iter()
        .filter(|gate| {
            gate.operation == Operation::Xor
                && (gate.result == correct_carry_xor_gate.left || gate.result == correct_carry_xor_gate.right)
        })
        .map(|gate| {
            (
                gate.clone(),
                Gate {
                    left: gate.left.clone(),
                    operation: Operation::Xor,
                    right: gate.right.clone(),
                    result: if correct_carry_xor_gate.left == *carry_wire {
                        correct_carry_xor_gate.right.clone()
                    } else {
                        correct_carry_xor_gate.left.clone()
                    },
                },
            )
        })
        .next()
        .unwrap();

    // we get the two end gates
    let and_gates: Vec<Gate> = impacted_gates
        .iter()
        .filter(|gate| gate.operation == Operation::And)
        .map(|gate| gate.clone())
        .collect();

    // We now that there are two end gates from the test at the beginning
    // One of them must have the correct output or it not possible to decide how
    // to correct them
    let (correct_and_gate_1, correct_and_gate_2) = if and_gates[0].result == correct_carry_or_gate.left
        || and_gates[0].result == correct_carry_or_gate.right
    {
        // The first gate has the correct result, use the other input for the second gate
        (
            and_gates[0].clone(),
            Gate {
                left: and_gates[1].left.clone(),
                operation: Operation::And,
                right: and_gates[1].left.clone(),
                result: if and_gates[0].result == correct_carry_or_gate.left {
                    correct_carry_or_gate.right.clone()
                } else {
                    correct_carry_or_gate.left.clone()
                },
            },
        )
    } else if and_gates[1].result == correct_carry_or_gate.left || and_gates[1].result == correct_carry_or_gate.right {
        // The second gate has the correct result, use the other input for the first gate
        (
            Gate {
                left: and_gates[0].left.clone(),
                operation: Operation::And,
                right: and_gates[0].left.clone(),
                result: if and_gates[1].result == correct_carry_or_gate.left {
                    correct_carry_or_gate.right.clone()
                } else {
                    correct_carry_or_gate.left.clone()
                },
            },
            and_gates[1].clone(),
        )
    } else {
        unreachable!()
    };

    // Add gate wrong output to the list of swapped wire.
    // Since, we now the valid carry and we use it directly, we don't even have
    // to update the program for this case
    if correct_carry_or_gate.result != carry_or_gate.result {
        swapped_wires.push(carry_or_gate.result.clone());
    }
    if correct_carry_xor_gate.result != carry_xor_gate.result {
        swapped_wires.push(carry_xor_gate.result.clone());
    }
    if correct_input_xor_gate.result != input_xor_gate.result {
        swapped_wires.push(input_xor_gate.result.clone());
    }
    if correct_and_gate_1.result != and_gates[0].result {
        swapped_wires.push(and_gates[0].result.clone());
    }
    if correct_and_gate_2.result != and_gates[1].result {
        swapped_wires.push(and_gates[1].result.clone());
    }
    correct_carry
}

/// Check if the full adder for bit "to_test" is valid
fn check_full_adder<'a>(
    wire_destinations: &HashMap<String, Vec<&Gate>>,
    carry_wire: &String,
    to_test: i32,
) -> Result<String, (HashSet<Gate>, Vec<FullGateError>)> {
    // We should have the following truth table
    // | a | b | carry | out | carry out |
    // |---|---|-------|-----|-----------|
    // | 0 | 0 | 0     | 0   | 0         |
    // | 0 | 0 | 1     | 1   | 0         |
    // | 0 | 1 | 0     | 1   | 0         |
    // | 0 | 1 | 1     | 0   | 1         |
    // | 1 | 0 | 0     | 1   | 0         |
    // | 1 | 0 | 1     | 0   | 1         |
    // | 1 | 1 | 0     | 0   | 1         |
    // | 1 | 1 | 1     | 1   | 1         |

    // Create a an b inputs.
    let x = format!("x{:<02}", to_test);
    let y = format!("y{:<02}", to_test);

    let mut wire_values_result: HashMap<String, Vec<bool>> = HashMap::new();
    for a in [false, true] {
        for b in [false, true] {
            for carry in [false, true] {
                // Create a wire_values set for this problem
                let mut wire_values: HashMap<_, _> = [(x.clone(), a), (y.clone(), b), (carry_wire.clone(), carry)]
                    .into_iter()
                    .collect();

                // Create initial signals
                let signals: VecDeque<_> = [&x, &y, &carry_wire].into_iter().collect();

                // Run it
                simulate_gates(&wire_destinations, &mut wire_values, signals);

                // Append the result to the list of results
                for (key, value) in wire_values {
                    // We ignore x, y, and carry_wire values
                    if key != x && key != y && key != *carry_wire {
                        wire_values_result.entry(key).or_default().push(value);
                    }
                }
            }
        }
    }

    // Check truth table for the output
    let mut errors = vec![];
    let expected_result = format!("z{:<02}", to_test);
    if let Some(value) = wire_values_result.get(&expected_result) {
        // check truth table
        if *value != *RESULT_TRUTH_TABLE {
            errors.push(FullGateError::OutputBadTruthTable);
        }
    } else {
        // Check if we can determinate where the output is
        let mut detected_result = None;
        for (key, value) in &wire_values_result {
            if *value == *RESULT_TRUTH_TABLE {
                if detected_result.is_none() {
                    detected_result = Some(key.clone())
                } else {
                    errors.push(FullGateError::MultiplePossibleResult);
                }
            }
        }
        errors.push(FullGateError::WrongOutput(detected_result));
    }

    // Try to detect the new carry wire
    let mut new_carry_wire = None;
    for (key, value) in &wire_values_result {
        if *value == *CARRY_TRUTH_TABLE {
            if new_carry_wire.is_none() {
                new_carry_wire = Some(key.clone());
            } else {
                errors.push(FullGateError::MultiplePossibleCarry);
            }
        }
    }
    if let Some(new_carry_wire) = new_carry_wire {
        Ok(new_carry_wire)
    } else {
        errors.push(FullGateError::NoValidCarry);

        // Get every impacted logic gate from this adder
        let mut impacted_gates = HashSet::new();
        for (wire, gates) in wire_destinations {
            if wire_values_result.contains_key(wire) || wire == &x || wire == &y || wire == &expected_result {
                for gate in gates {
                    if wire_values_result.contains_key(&gate.result)
                        || gate.result == x
                        || gate.result == y
                        || gate.result == expected_result
                    {
                        impacted_gates.insert((*gate).clone());
                    }
                }
            }
        }
        Err((impacted_gates, errors))
    }
}
