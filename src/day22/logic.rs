/// Mix two values
fn mix(left: usize, right: usize) -> usize {
    left ^ right
}

/// Prune a value
fn prune(value: usize) -> usize {
    value % 16_777_216
}

fn next_secret(mut secret: usize) -> usize {
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

/// Compute the nth iteration of the given secret
fn compute_nth_secret(mut secret: usize, nth: usize) -> usize {
    for _ in 0..nth {
        secret = next_secret(secret);
    }

    secret
}

pub fn solve_part_one(secrets: &[usize]) -> usize {
    secrets.iter().map(|secret|compute_nth_secret(*secret, 2000)).sum()
}

pub fn solve_part_two(secrets: &[usize]) -> u32 {
    0
}
