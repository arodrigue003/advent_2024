use crate::day24::models::{Gate, System};
use hashbrown::HashMap;
use std::collections::VecDeque;

pub fn solve_part_one(system: &System) -> u64 {
    // First we create a hashmap that store wire values
    let mut wire_values: HashMap<_, _> = system
        .wires
        .iter()
        .map(|wire| (wire.name.clone(), wire.value))
        .collect();

    // Then we create a map from every wire name to a list of associated gates
    let mut wire_destinations: HashMap<String, Vec<&Gate>> = HashMap::new();
    for gate in &system.gates {
        wire_destinations.entry(gate.left.clone()).or_default().push(gate);
        wire_destinations.entry(gate.right.clone()).or_default().push(gate);
    }

    // Add every wire initial value to the signaling list
    let mut signals: VecDeque<&String> = system.wires.iter().map(|wire| &wire.name).collect();

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

    // Build the result value
    let mut result: u64 = 0;
    for (key, value) in wire_values {
        if key.starts_with("z") && value == true {
            // Get the positioning of the value
            let position: u64 = key.as_str()[1..].parse().unwrap();
            result += 1 << position;
        }
    }

    result
}

pub fn solve_part_two(system: &System) -> u32 {
    0
}
