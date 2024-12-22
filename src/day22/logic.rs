use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

/// Mix two values
fn mix(left: i64, right: i64) -> i64 {
    left ^ right
}

/// Prune a value
fn prune(value: i64) -> i64 {
    value % 16_777_216
}

fn next_secret(mut secret: i64) -> i64 {
    // Calculate the result of multiplying the secret number by 64.
    // Then, mix this result into the secret number. Finally, prune the secret number
    secret = prune(mix(secret, secret * 64));

    // Calculate the result of dividing the secret number by 32.
    // Round the result down to the nearest integer.
    // Then, mix this result into the secret number.
    // Finally, prune the secret number.
    secret = prune(mix(secret, secret / 32));

    // Calculate the result of multiplying the secret number by 2048.
    // Then, mix this result into the secret number.
    // Finally, prune the secret number.
    secret = prune(mix(secret, secret * 2048));

    secret
}

pub struct Secret {
    is_first: bool,
    secret: i64,
}

impl Secret {
    pub fn new(secret: i64) -> Self {
        Self { is_first: true, secret }
    }
}

impl Iterator for Secret {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_first {
            self.is_first = false;
        } else {
            self.secret = next_secret(self.secret);
        }

        Some(self.secret)
    }
}

pub fn solve_part_one(secrets: &[i64]) -> i64 {
    secrets
        .iter()
        .map(|secret| Secret::new(*secret).skip(2000).next().unwrap())
        .sum()
}

pub fn solve_part_two(secrets: &[i64]) -> i64 {
    // Store the list of profits
    let mut sequences_profit: HashMap<_, i64> = HashMap::new();

    for secret in secrets {
        // Check if we already had this sequence for this secret
        let mut sequence_already_seen = HashSet::new();

        for (a, b, c, d, e) in Secret::new(*secret).take(2000).tuple_windows() {
            let sequence = [b % 10 - a % 10, c % 10 - b % 10, d % 10 - c % 10, e % 10 - d % 10];
            // If we didn't see the sequence already
            if !sequence_already_seen.contains(&sequence) {
                sequence_already_seen.insert(sequence.clone());
                *sequences_profit.entry(sequence).or_default() += e % 10;
            }
        }
    }

    // return the highest value
    sequences_profit.into_values().max().unwrap()
}
