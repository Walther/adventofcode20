use std::collections::HashSet;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    // Different groups are separated by two newlines
    let declarations_data: Vec<String> = INPUT.split("\n\n").map(|s| s.to_owned()).collect();
    // Each group may have multiple members
    let declarations_data: Vec<Vec<String>> = declarations_data
        .iter()
        .map(|p| p.split("\n").map(|s| s.to_owned()).collect())
        .collect();
    let mut anyone_sum = 0;
    let mut everyone_sum = 0;
    for group in declarations_data {
        let mut declarations: HashSet<char> = HashSet::new();
        // Part 1
        for member in group.iter() {
            for letter in member.chars() {
                declarations.insert(letter);
            }
        }
        anyone_sum += declarations.len();
        // Part 2
        for &letter in declarations.iter() {
            if group.iter().all(|member| member.contains(letter)) {
                everyone_sum += 1;
            }
        }
    }
    println!("Part 1: {}", anyone_sum);
    println!("Part 2: {}", everyone_sum);
}
