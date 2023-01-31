use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let path = "resources/input.txt";
    let contents = fs::read_to_string(path).expect("File not found");
    let lines = contents.split("\n").collect::<Vec<&str>>();
    let trees = lines
        .iter()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();
    let mut visible_trees: HashSet<(usize, usize)> = HashSet::new();
    let mut left_score_components: HashMap<(usize, usize), u16> = HashMap::new();
    let mut right_score_components: HashMap<(usize, usize), u16> = HashMap::new();
    let mut down_score_components: HashMap<(usize, usize), u16> = HashMap::new();
    let mut up_score_components: HashMap<(usize, usize), u16> = HashMap::new();
    for (i, tree_line) in trees.iter().enumerate() {
        let mut tallest: Option<u8> = None;
        let mut distance_to_nearest_tree_of_at_least_height: Vec<Option<u16>> = vec![None; 10];
        tree_line.iter().enumerate().for_each(|(j, &tree)| {
            tallest = Some(update_visible_trees(
                tallest,
                tree,
                &mut visible_trees,
                i,
                j,
            ));
            update_score_component(
                &mut left_score_components,
                &mut distance_to_nearest_tree_of_at_least_height,
                tree,
                i,
                j,
                j as u16,
            );
        });
        let mut tallest: Option<u8> = None;
        let mut distance_to_nearest_tree_of_at_least_height: Vec<Option<u16>> = vec![None; 10];
        tree_line
            .iter()
            .rev()
            .enumerate()
            .for_each(|(j_rev, &tree)| {
                let j = tree_line.len() - j_rev - 1;
                tallest = Some(update_visible_trees(
                    tallest,
                    tree,
                    &mut visible_trees,
                    i,
                    j,
                ));
                update_score_component(
                    &mut right_score_components,
                    &mut distance_to_nearest_tree_of_at_least_height,
                    tree,
                    i,
                    j,
                    j_rev as u16,
                );
            });
    }
    for j in 0..trees[0].len() {
        let tree_line = trees
            .iter()
            .map(|tree_row| tree_row[j])
            .collect::<Vec<u8>>();
        let mut tallest: Option<u8> = None;
        let mut distance_to_nearest_tree_of_at_least_height: Vec<Option<u16>> = vec![None; 10];
        tree_line.iter().enumerate().for_each(|(i, tree)| {
            tallest = Some(update_visible_trees(
                tallest,
                *tree,
                &mut visible_trees,
                i,
                j,
            ));
            update_score_component(
                &mut down_score_components,
                &mut distance_to_nearest_tree_of_at_least_height,
                *tree,
                i,
                j,
                i as u16,
            );
        });
        let mut tallest: Option<u8> = None;
        let mut distance_to_nearest_tree_of_at_least_height: Vec<Option<u16>> = vec![None; 10];
        tree_line
            .iter()
            .rev()
            .enumerate()
            .for_each(|(i_rev, tree)| {
                let i = tree_line.len() - i_rev - 1;
                tallest = Some(update_visible_trees(
                    tallest,
                    *tree,
                    &mut visible_trees,
                    i,
                    j,
                ));
                update_score_component(
                    &mut up_score_components,
                    &mut distance_to_nearest_tree_of_at_least_height,
                    *tree,
                    i,
                    j,
                    i_rev as u16,
                );
            });
    }

    // Part 1
    println!("Total visible trees: {}", visible_trees.len());

    // Part 2
    let mut highest_score = 0;
    for i in 0..trees.len() {
        for j in 0..trees[0].len() {
            let left = *left_score_components.get(&(i, j)).unwrap_or(&0);
            let right = *right_score_components.get(&(i, j)).unwrap_or(&0);
            let down = *down_score_components.get(&(i, j)).unwrap_or(&0);
            let up = *up_score_components.get(&(i, j)).unwrap_or(&0);
            let score = (left as u32) * (right as u32) * (down as u32) * (up as u32);
            if score > highest_score {
                highest_score = score;
            }
        }
    }
    println!("Highest score: {}", highest_score);
}

fn update_visible_trees(
    tallest: Option<u8>,
    tree: u8,
    visible_trees: &mut HashSet<(usize, usize)>,
    i: usize,
    j: usize,
) -> u8 {
    if let Some(tallest_val) = tallest {
        if tree > tallest_val {
            visible_trees.insert((i, j));
        } else {
            return tallest_val;
        }
    } else {
        visible_trees.insert((i, j));
    }
    return tree;
}

fn update_score_component(
    score_component: &mut HashMap<(usize, usize), u16>,
    distance_to_nearest_tree_of_at_least_height: &mut Vec<Option<u16>>,
    tree: u8,
    i: usize,
    j: usize,
    default: u16,
) {
    let comp = distance_to_nearest_tree_of_at_least_height[tree as usize].unwrap_or(default);
    score_component.insert((i, j), comp);
    for k in 0..(tree + 1) as usize {
        distance_to_nearest_tree_of_at_least_height[k] = Some(1);
    }
    for k in (tree + 1) as usize..10 {
        if let Some(distance) = distance_to_nearest_tree_of_at_least_height[k] {
            distance_to_nearest_tree_of_at_least_height[k] = Some(distance + 1);
        }
    }
}
