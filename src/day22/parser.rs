pub fn parse_input(input: String) -> Vec<u64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}
