use std::{fs, collections::HashSet};

fn main() {
    let path = "resources/input.txt";
    let contents = fs::read_to_string(path).expect("File not found");
    let lines = contents.split("\n").collect::<Vec<&str>>();
    // Part 1
    let priorities = lines.iter().map(|line| {
        let compartments = line.split_at(line.len() / 2);
        let compartments = vec![compartments.0, compartments.1].iter().map(|compartment| {
            HashSet::from_iter(compartment.chars().into_iter())
        }).collect::<Vec<HashSet<char>>>();
        let intersection = **compartments[0].intersection(&compartments[1]).collect::<Vec<&char>>().get(0).expect(
            &format!("Rucksack has non-intersecting compartments: {:?}", compartments));
        char_to_priority(intersection)
    }).collect::<Vec<u8>>();
    let dupe_sum: u32 = priorities.iter().map(|val| u32::from(*val)).sum();
    println!("Total priority of intersecting items: {}", dupe_sum);

    // Part 2
    let badges = lines.chunks(3).map(|group| {
        let sacks = group.iter().map(|sack| {
            HashSet::from_iter(sack.chars().into_iter())
        }).collect::<Vec<HashSet<char>>>();
        let intersection = &(&sacks[0] & &sacks[1]) & &sacks[2];
        let intersection = **intersection.iter().collect::<Vec<&char>>().get(0).expect(
            &format!("No intersection found between sack triplet: {:?}", sacks)
        );
        char_to_priority(intersection)
    }).collect::<Vec<u8>>();
    let badge_sum: u32 = badges.iter().map(|val| u32::from(*val)).sum();
    println!("Total priority of badges: {}", badge_sum);
}

fn char_to_priority(c: char) -> u8 {
    let byte_val = *c.to_string().as_bytes().get(0).expect(
        &format!("char_to_priority was given a character that couldn't be reduced to a byte value: '{}'", c));
    if byte_val >= 97 {
        return byte_val - 96;
    }
    return byte_val - 38;
}