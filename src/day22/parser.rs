pub fn parse_input(input: String) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}
