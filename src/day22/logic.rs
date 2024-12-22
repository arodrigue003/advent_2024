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

#[inline(always)]
fn convert_changes_to_offset(a: i64, b: i64, c: i64, d: i64) -> usize {
    (50653 * a + 1369 * b + 37 * c + d + 468540) as usize
}

pub fn solve_part_two(secrets: &[i64]) -> i64 {
    // Since every change is computed from two values that range from 0 to 9, every change value
    // ranges from -18 to 18.
    // Knowing that, we can transform every change (a, b, c, d) into a unique offset in a big vector
    // with the transformation (a+9)*37.pow(3) + (b+9)*37.pow(2) + (c+9)*37.pow(1) + c + 9.
    // This is equivalent to 50653*a + 1369*b + 37*c + d + 468540
    // Vec max offset is reached when a, b, c and d all equal 9 so 937080 (we will add 1 for the size)

    // Store the list of profits
    let mut sequences_profit = vec![0; 937_080];

    for secret in secrets {
        // Check if we already had this sequence for this secret
        let mut sequence_already_seen = vec![false; 937_080];

        for (a, b, c, d, e) in Secret::new(*secret).take(2000).tuple_windows() {
            let offset = convert_changes_to_offset(b % 10 - a % 10, c % 10 - b % 10, d % 10 - c % 10, e % 10 - d % 10);
            // If we didn't see the sequence already
            if sequence_already_seen[offset] == false {
                sequence_already_seen[offset] = true;
                sequences_profit[offset] += e % 10;
            }
        }
    }

    // return the highest value
    sequences_profit.into_iter().max().unwrap()
    // res as i64
}
