pub fn parse_input(input: String) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}
