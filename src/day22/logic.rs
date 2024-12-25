use itertools::Itertools;

/// Mix two values
#[inline(always)]
fn mix(left: u64, right: u64) -> u64 {
    left ^ right
}

/// Prune a value
#[inline(always)]
fn prune(value: u64) -> u64 {
    value % 16_777_216
}

fn next_secret(mut secret: u64) -> u64 {
    // Calculate the result of multiplying the secret number by 64.
    // Then, mix this result into the secret number. Finally, prune the secret number
    secret = prune(mix(secret, secret << 6));

    // Calculate the result of dividing the secret number by 32.
    // Round the result down to the nearest integer.
    // Then, mix this result into the secret number.
    // Finally, prune the secret number.
    secret = prune(mix(secret, secret >> 5));

    // Calculate the result of multiplying the secret number by 2048.
    // Then, mix this result into the secret number.
    // Finally, prune the secret number.
    secret = prune(mix(secret, secret << 11));

    secret
}

pub struct Secret {
    is_first: bool,
    secret: u64,
}

impl Secret {
    pub fn new(secret: u64) -> Self {
        Self { is_first: true, secret }
    }
}

impl Iterator for Secret {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_first {
            self.is_first = false;
        } else {
            self.secret = next_secret(self.secret);
        }

        Some(self.secret)
    }
}

pub fn solve_part_one(secrets: &[u64]) -> u64 {
    secrets
        .iter()
        .map(|secret| Secret::new(*secret).skip(2000).next().unwrap())
        .sum()
}

#[inline(always)]
fn convert_changes_to_offset(a: u64, b: u64, c: u64, d: u64) -> usize {
    (6859 * a + 361 * b + 19 * c + d + 65160) as usize
}

pub fn solve_part_two(secrets: &[u64]) -> u64 {
    // Since every change is computed from two values that range from 0 to 9, every change value
    // ranges from -9 to 9.
    // Knowing that, we can transform every change (a, b, c, d) into a unique offset in a big vector
    // with the transformation (a+9)*19.pow(3) + (b+9)*19.pow(2) + (c+9)*19.pow(1) + c + 18.
    // This is equivalent to 6859*a + 361*b + 19*c + d + 65160
    // Vec max offset is reached when a, b, c and d all equal 9 so 130_320 (we will add 1 for the size)

    // Store the list of profits
    let mut sequences_profit: Vec<(usize, u64)> = vec![(0, 0); 2 << 17];

    for (i, secret) in secrets.iter().enumerate() {
        for (a, b, c, d, e) in Secret::new(*secret).take(2000).tuple_windows() {
            let offset = convert_changes_to_offset(b % 10 - a % 10, c % 10 - b % 10, d % 10 - c % 10, e % 10 - d % 10);
            // If we didn't see the sequence already
            if sequences_profit[offset].0 < i + 1 {
                sequences_profit[offset] = (i + 1, sequences_profit[offset].1 + (e % 10));
            }
        }
    }

    // return the highest value
    sequences_profit.into_iter().map(|(_, v)| v).max().unwrap()
}
