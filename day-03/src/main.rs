use std::{collections::HashMap, fmt::Debug};

const TOBOGGAN_PATHS: &'static [Path] = &[
    Path { x: 3, y: 1 },
    Path { x: 1, y: 1 },
    Path { x: 5, y: 1 },
    Path { x: 7, y: 1 },
    Path { x: 1, y: 2 },
];

enum GridEntry {
    Tree,
    Empty,
}

struct Path {
    x: usize,
    y: usize,
}

impl Debug for GridEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GridEntry::Tree => write!(f, "#"),
            GridEntry::Empty => write!(f, "."),
        }
    }
}

type TobogganMap = HashMap<(usize, usize), GridEntry>;

fn main() -> Result<(), String> {
    const INPUT: &str = include_str!("input.txt");

    // Parse the map
    let (map, height, width) = parse_map(INPUT)?;

    // Start sledding down
    let trees = count_trees(&map, &TOBOGGAN_PATHS[0], height, width);
    println!("Part 1: {}", trees);

    let tree_multiples: usize = TOBOGGAN_PATHS
        .iter()
        .map(|p| count_trees(&map, &p, height, width))
        .product();
    println!("Part 2: {}", tree_multiples);

    Ok(())
}

fn parse_map(input: &str) -> Result<(TobogganMap, usize, usize), String> {
    let mut map = TobogganMap::new();
    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();
    let mut y = 0;
    let mut x;
    for line in input.lines() {
        x = 0;
        for char in line.chars() {
            match char {
                '.' => {
                    map.insert((x, y), GridEntry::Empty);
                }
                '#' => {
                    map.insert((x, y), GridEntry::Tree);
                }
                _ => {
                    return Err(String::from("Invalid character found in input"));
                }
            }
            x = x + 1;
        }
        y = y + 1;
    }

    Ok((map, height, width))
}

fn count_trees(map: &TobogganMap, path: &Path, height: usize, width: usize) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;
    while y < height {
        if let Some(entry) = map.get(&(x, y)) {
            match entry {
                GridEntry::Tree => {
                    trees += 1;
                }
                GridEntry::Empty => {}
            }
            x = (x + path.x) % width;
            y += path.y;
        } else {
            panic!("Fell outside the map!")
        }
    }
    trees
}
