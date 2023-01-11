use std::{fs, collections::HashSet};

const MOVEMENT_DIRECTIONS: [(i8, i8); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn main() {
    let path = "resources/input.txt";
    let contents = fs::read_to_string(path).expect("File not found");
    let lines = contents.split("\n").map(|line| line.trim()).collect::<Vec<&str>>();
    let height_map = lines.iter().map(|line| {
        line.as_bytes().iter().map(|b| {
            if *b == 83 {
                1
            }
            else if *b == 69 {
                26
            }
            else {
                b - 96
            }
        }).collect::<Vec<u8>>()
    }).collect::<Vec<Vec<u8>>>();

    let start_y = lines.iter().position(|line| line.contains("S")).unwrap();
    let start_x = lines[start_y].chars().position(|c| c == 'S').unwrap();
    let start = (start_x, start_y);

    let end_y = lines.iter().position(|line| line.contains("E")).unwrap();
    let end_x = lines[end_y].chars().position(|c| c == 'E').unwrap();
    let end = (end_x, end_y);
    
    let steps = navigate_terrain(&height_map, HashSet::from([start]), end);
    println!("Steps from start {}", steps);

    let mut all_a_heights: HashSet<(usize, usize)> = HashSet::new();
    for (i, row) in lines.iter().enumerate() {
        for (j, height) in row.chars().enumerate() {
            if height == 'a' {
                all_a_heights.insert((j, i));
            }
        }
    }
    let steps = navigate_terrain(&height_map, all_a_heights, end);
    println!("Steps from best a height {}", steps);
}

fn navigate_terrain(height_map: &Vec<Vec<u8>>, starting_frontier: HashSet<(usize, usize)>, end: (usize, usize)) -> u32 {
    let mut steps = 0_u32;
    let mut visited = starting_frontier.clone();
    let mut frontier = starting_frontier.clone();
    while !visited.contains(&end) {
        let mut new_frontier = HashSet::new();
        frontier.iter().for_each(|(x, y)| {
            MOVEMENT_DIRECTIONS.iter().for_each(|&(dx, dy)| {
                let new_x = *x as i8 + dx;
                if new_x < 0 { return }
                let new_x = new_x as usize;
                let new_y = *y as i8 + dy;
                if new_y < 0 { return }
                let new_y = new_y as usize;
                let new_pos = (new_x, new_y);
                if visited.contains(&new_pos) { return }
                if new_x >= height_map[0].len() || new_y >= height_map.len() { return }
                let cur_height = height_map[*y][*x];
                let dest_height = height_map[new_y][new_x];
                if dest_height <= cur_height + 1 {
                    visited.insert(new_pos);
                    new_frontier.insert(new_pos);
                }
            });
        });
        frontier = new_frontier;
        steps += 1;
    }
    return steps;
}

fn print_height_map_seen_map(height_map: &Vec<&str>, seen: &HashSet<(usize, usize)>) {
    for (i, row) in height_map.iter().enumerate() {
        for (j, height) in row.chars().enumerate() {
            if seen.contains(&(j, i)) {
                print!("#");
            }
            else {
                print!("{}", height);
            }
        }
        println!();
    }
}
