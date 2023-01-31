use std::{collections::HashSet, fs};

fn main() {
    let path = "resources/input.txt";
    let contents = fs::read_to_string(path).expect("File not found");
    let lines = contents.split("\n").collect::<Vec<&str>>();

    // Part 1
    let priorities = lines
        .iter()
        .map(|line| {
            let compartments = line.split_at(line.len() / 2);
            let compartments = vec![compartments.0, compartments.1]
                .iter()
                .map(|compartment| HashSet::from_iter(compartment.chars().into_iter()))
                .collect::<Vec<HashSet<_>>>();
            let intersection = &compartments[0] & &compartments[1];
            let intersection =
                **intersection
                    .iter()
                    .collect::<Vec<&char>>()
                    .get(0)
                    .expect(&format!(
                        "Rucksack has non-intersecting compartments: {:?}",
                        compartments
                    ));
            char_to_priority(intersection) as u32
        })
        .collect::<Vec<u32>>();
    let dupe_sum = priorities.iter().sum::<u32>();
    println!("Total priority of intersecting items: {}", dupe_sum);

    // Part 2
    let badges = lines
        .chunks(3)
        .map(|group| {
            let sacks = group
                .iter()
                .map(|sack| HashSet::from_iter(sack.chars().into_iter()))
                .collect::<Vec<HashSet<char>>>();
            let intersection = &(&sacks[0] & &sacks[1]) & &sacks[2];
            let intersection =
                **intersection
                    .iter()
                    .collect::<Vec<&char>>()
                    .get(0)
                    .expect(&format!(
                        "No intersection found between sack triplet: {:?}",
                        sacks
                    ));
            char_to_priority(intersection) as u32
        })
        .collect::<Vec<u32>>();
    let badge_sum = badges.iter().sum::<u32>();
    println!("Total priority of badges: {}", badge_sum);
}

fn char_to_priority(c: char) -> u8 {
    let byte_val = *c.to_string().as_bytes().get(0).unwrap();
    if byte_val >= 97 {
        return byte_val - 96;
    }
    return byte_val - 38;
}
