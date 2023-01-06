use std::{fs, collections::HashSet};

fn main() {
    let path = "resources/input.txt";
    let contents = fs::read_to_string(path).expect("File not found");
    let lines = contents.split("\n").collect::<Vec<&str>>();
    let trees = lines.iter().map(|line| {
        line.trim().chars().map(|c| c.to_string().parse::<u8>().unwrap()).collect::<Vec<u8>>()
    }).collect::<Vec<Vec<u8>>>();
    
    // Part 1
    let mut visible_trees: HashSet<(usize, usize)> = HashSet::new();
    trees.iter().enumerate().for_each(|(i, tree_line)| {
        let mut tallest: Option<u8> = None;
        tree_line.iter().enumerate().for_each(|(j, tree)| {
            tallest = Some(update_tallest_tree(tallest, *tree, &mut visible_trees, i, j));
        });
        let mut tallest: Option<u8> = None;
        tree_line.iter().rev().enumerate().for_each(|(j_rev, tree)| {
            let j = tree_line.len() - j_rev - 1;
            tallest = Some(update_tallest_tree(tallest, *tree, &mut visible_trees, i, j));
        });
    });
    (0..trees[0].len()).for_each(|j| {
        let tree_line = trees.iter().map(|tree_row| tree_row[j]).collect::<Vec<u8>>();
        let mut tallest: Option<u8> = None;
        tree_line.iter().enumerate().for_each(|(i, tree)| {
            tallest = Some(update_tallest_tree(tallest, *tree, &mut visible_trees, i, j));
        });
        let mut tallest: Option<u8> = None;
        tree_line.iter().rev().enumerate().for_each(|(i_rev, tree)| {
            let i = tree_line.len() - i_rev - 1;
            tallest = Some(update_tallest_tree(tallest, *tree, &mut visible_trees, i, j));
        });
    });
    println!("Total visible trees: {}", visible_trees.len());

    // Part 2
}

fn update_tallest_tree(tallest: Option<u8>, tree: u8, visible_trees: &mut HashSet<(usize, usize)>, i: usize, j: usize) -> u8 {
    if let Some(tallest_val) = tallest {
        if tree > tallest_val {
            visible_trees.insert((i, j));
        }
        else {
            return tallest_val;
        }
    }
    else {
        visible_trees.insert((i, j));
    }
    return tree;
}
