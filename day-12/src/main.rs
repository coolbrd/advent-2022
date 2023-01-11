use std::{fs, collections::HashSet};

fn main() {
    let path = "resources/input.txt";
    let contents = fs::read_to_string(path).expect("File not found");
    let lines = contents.split("\n").collect::<Vec<&str>>();
    let height_map = lines.iter().map(|line| {
        line.as_bytes().iter().map(|b| {
            if *b == 83 {
                0
            }
            else if *b == 69 {
                27
            }
            else {
                b - 96
            }
        }).collect::<Vec<u8>>()
    }).collect::<Vec<Vec<u8>>>();
    let seen_map = vec![vec![false; height_map[0].len()]; height_map.len()];

    let start_y = height_map.iter().position(|row| row.contains(&0)).unwrap();
    let start_x = height_map[start_y].iter().position(|&height| height == 0).unwrap();
    let start = (start_x, start_y);

    let end_y = height_map.iter().position(|row| row.contains(&26)).unwrap();
    let end_x = height_map[end_y].iter().position(|&height| height == 26).unwrap();
    let end = (end_x, end_y);

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    while !visited.contains(&end) {
        print_height_map_seen_map(&lines, &seen_map);
        
    }
}

fn print_height_map_seen_map(height_map: &Vec<&str>, seen_map: &Vec<Vec<bool>>) {
    for (i, row) in height_map.iter().enumerate() {
        for (j, height) in row.chars().enumerate() {
            if seen_map[i][j] {
                print!("#");
            }
            else {
                print!("{}", height);
            }
        }
        println!();
    }
}
