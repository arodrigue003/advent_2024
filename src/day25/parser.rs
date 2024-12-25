use itertools::Itertools;

pub fn parse_input(input: String) -> (Vec<u64>, Vec<u64>) {
    input
        .lines()
        .chunk_by(|line| !line.is_empty())
        .into_iter()
        .filter_map(|(to_use, chunks)| if to_use == true { Some(chunks) } else { None })
        .map(|chunks| {
            chunks.into_iter().fold(0, |acc, line| {
                (acc << 8)
                    + line
                        .chars()
                        .fold(0, |acc, char| (acc << 1) + if char == '#' { 1 } else { 0 })
            })
        })
        .partition(|value| value & 1 == 1)
}
