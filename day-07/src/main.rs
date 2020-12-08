use std::collections::{HashMap, HashSet};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Bag {
    adjective: String,
    color: String,
}

#[derive(Debug)]
struct BagContent {
    number: usize,
    bag: Bag,
}

type BagMap = HashMap<Bag, Option<Vec<BagContent>>>;

fn main() {
    const INPUT: &str = include_str!("input.txt");
    let my_bag = Bag {
        adjective: "shiny".to_owned(),
        color: "gold".to_owned(),
    };
    let mut bagmap: BagMap = HashMap::new();

    for line in INPUT.lines() {
        // remove trailing dot
        let line = line.replace(".", "");
        // remove bag and bags; makes later processing easier
        let line = line.replace("bags", "");
        let line = line.replace("bag", "");
        // split rules to key-value pairs
        let keyval: Vec<&str> = line.split("contain").collect();
        // make key into a bag
        let key = keyval[0].trim().to_owned();
        let mut parts = key.split(" ");
        let adjective = parts.next().unwrap().to_owned();
        let color = parts.next().unwrap().to_owned();
        let bag = Bag { adjective, color };

        let value: Vec<&str> = keyval[1].split(", ").map(|s| s.trim()).collect();
        let value: Vec<String> = value.iter().map(|&s| s.to_owned()).collect();

        // possible types of values:
        // `5 adjective color`
        // `no other`

        // dbg!(&key);
        // dbg!(&value);
        if value.len() == 1 && value[0] == "no other" {
            bagmap.insert(bag, None);
        } else {
            let mut list: Vec<BagContent> = Vec::new();
            for content in value {
                let mut parts = content.split(" ");
                // TODO: remove unwraps...
                let number = parts.next().unwrap().parse().unwrap();
                let adjective = parts.next().unwrap().to_owned();
                let color = parts.next().unwrap().to_owned();
                let content = BagContent {
                    number,
                    bag: Bag { adjective, color },
                };
                list.push(content);
            }
            bagmap.insert(bag, Some(list));
        }
    }

    // dbg!(&graph);

    // Part 1

    let mut can_contain_my_bag: HashSet<Bag> = HashSet::new();
    fn recursive_analyse_can_contain(
        bagmap: &BagMap,
        target_bag: &Bag,
        resultset: &mut HashSet<Bag>,
    ) {
        let contains_bag: Vec<Bag> = find_bags_containing(&bagmap, &target_bag);
        for bag in contains_bag {
            resultset.insert(bag.clone());
            recursive_analyse_can_contain(&bagmap, &bag, resultset);
        }
    }
    recursive_analyse_can_contain(&bagmap, &my_bag, &mut can_contain_my_bag);
    println!("Part 1: {}", can_contain_my_bag.len());

    // Part 2

    fn recursive_analyse_sum(bagmap: &BagMap, target_bag: &Bag) -> usize {
        let mut result = 1;
        let bag_contents = bagmap.get(&target_bag).unwrap();
        if let Some(contents) = bag_contents {
            for bagcontent in contents {
                result += bagcontent.number * recursive_analyse_sum(bagmap, &bagcontent.bag);
            }
        }
        result
    }
    let my_bag_content_size = recursive_analyse_sum(&bagmap, &my_bag);
    // Ignore the starting bag itself
    let my_bag_content_size = my_bag_content_size - 1;
    println!("Part 2: {}", my_bag_content_size);
}

fn find_bags_containing(bagmap: &BagMap, target_bag: &Bag) -> Vec<Bag> {
    bagmap
        .iter()
        .filter_map(|(bag, contents)| {
            if let Some(contents) = contents {
                // Check the list of contents. If we find our bag, return the parent bag.
                for content in contents {
                    if content.bag == *target_bag {
                        return Some(bag.clone());
                    } else {
                        continue;
                    }
                }
                return None;
            } else {
                return None;
            }
        })
        .collect()
}
