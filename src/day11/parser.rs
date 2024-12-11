pub fn parse_input(input: String) -> Vec<usize> {
    input
        .trim()
        .split(" ")
        .map(|value| value.parse().unwrap())
        .collect()
}
