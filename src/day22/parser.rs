pub fn parse_input(input: String) -> Vec<i64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}
