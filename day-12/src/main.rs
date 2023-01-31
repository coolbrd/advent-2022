use std::{collections::HashSet, fs};

const START_CHAR: char = 'S';
const END_CHAR: char = 'E';

const MOVEMENT_DIRECTIONS: [(i8, i8); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

type MapPosComp = i8;

type MapPos = (MapPosComp, MapPosComp);

fn main() {
    let path = "resources/input.txt";
    let contents = fs::read_to_string(path).expect("File not found");
    let lines = contents
        .split("\n")
        .map(|line| line.trim())
        .collect::<Vec<&str>>();
    let height_map = lines
        .iter()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|b| {
                    if *b == 83 {
                        1
                    } else if *b == 69 {
                        26
                    } else {
                        b - 96
                    }
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    let (end_y, end_line) = lines
        .iter()
        .enumerate()
        .find(|(_, line)| line.contains(END_CHAR))
        .expect("No end found");
    let end_x = end_line.chars().position(|c| c == END_CHAR).unwrap();
    let end = (end_x as MapPosComp, end_y as MapPosComp);

    // Part 1
    let (start_y, start_line) = lines
        .iter()
        .enumerate()
        .find(|(_, line)| line.contains(START_CHAR))
        .expect("No start found");
    let start_x = start_line.chars().position(|c| c == START_CHAR).unwrap();
    let start = (start_x as MapPosComp, start_y as MapPosComp);
    let steps = calculate_min_steps_to_end(&height_map, HashSet::from([start]), end);
    println!("Steps from start {}", steps);

    // Part 2
    let mut all_a_heights = HashSet::new();
    for (i, row) in lines.iter().enumerate() {
        for (j, height) in row.chars().enumerate() {
            if height == 'a' {
                all_a_heights.insert((j as MapPosComp, i as MapPosComp));
            }
        }
    }
    let steps = calculate_min_steps_to_end(&height_map, all_a_heights, end);
    println!("Steps from best a height {}", steps);
}

fn calculate_min_steps_to_end(
    height_map: &Vec<Vec<u8>>,
    starting_frontier: HashSet<MapPos>,
    end: MapPos,
) -> u32 {
    let mut steps = 0;
    let mut visited = starting_frontier.clone();
    let mut frontier = starting_frontier.clone();
    while !visited.contains(&end) {
        let mut new_frontier = HashSet::new();
        for (x, y) in frontier {
            for &(dx, dy) in &MOVEMENT_DIRECTIONS {
                let new_x = x + dx;
                let new_y = y + dy;
                let new_pos = (new_x, new_y);
                if new_x < 0 || new_y < 0 {
                    continue;
                }
                if visited.contains(&new_pos) {
                    continue;
                }
                if new_x as usize >= height_map[0].len() || new_y as usize >= height_map.len() {
                    continue;
                }
                let cur_height = height_map[y as usize][x as usize];
                let dest_height = height_map[new_y as usize][new_x as usize];
                if dest_height <= cur_height + 1 {
                    visited.insert(new_pos);
                    new_frontier.insert(new_pos);
                }
            }
        }
        frontier = new_frontier;
        steps += 1;
    }
    return steps;
}
