use crate::day19::models::Onsen;

pub fn parse_input(input: String) -> Onsen {
    let mut towels: Vec<String> = vec![];
    let mut designs: Vec<String> = vec![];

    for (i, line) in input.lines().enumerate() {
        if i == 0 {
            // First line
            towels = line.split(", ").map(|towel| towel.to_string()).collect();
        } else if !line.is_empty() {
            // Skip empty lines
            designs.push(line.to_string());
        }
    }

    Onsen::new(towels, designs)
}
