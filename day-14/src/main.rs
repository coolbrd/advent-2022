use std::{fs, collections::HashSet};

const SAND_ORIGIN: (i32, i32) = (500, 0);
const SAND_MOVEMENT_DIRECTIONS: [(i8, i8); 3] = [(0, 1), (-1, 1), (1, 1)];

fn main() {
    let path = "resources/input.txt";
    let contents = fs::read_to_string(path).expect("File not found");
    let lines = contents.split("\n").map(|line| line.trim()).collect::<Vec<&str>>();
    let segments = lines.iter().map(|line| {
        line.split("->").map(|vertex| {
            let point = vertex.trim().split(",").map(|comp| {
                comp.parse::<i32>().expect(&format!("Unexpected vertex component value: {}", comp))
            }).collect::<Vec<i32>>();
            (point[0], point[1])
        }).collect::<Vec<(i32, i32)>>()
    }).collect::<Vec<Vec<(i32, i32)>>>();
    let mut walls: HashSet<(i32, i32)> = HashSet::new();
    segments.iter().for_each(|segment| {
        for i in 1..segment.len() {
            let last_vertex = segment[i - 1];
            let current_vertex = segment[i];
            let x_range = get_range(last_vertex.0, current_vertex.0);
            let y_range = get_range(last_vertex.1, current_vertex.1);
            for x in x_range {
                walls.insert((x, last_vertex.1));
            }
            for y in y_range {
                walls.insert((last_vertex.0, y));
            }
        }
    });
    let lowest_wall_y = walls.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let floor_y = lowest_wall_y + 2;
    let mut settled_sand: HashSet<(i32, i32)> = HashSet::new();
    let mut bottom_reached_at: Option<usize> = None;
    let mut end_reached = false;
    while !end_reached {
        let mut moving_sand = SAND_ORIGIN;
        loop {
            let mut moved = false;
            for dir in SAND_MOVEMENT_DIRECTIONS {
                let dest = (moving_sand.0 + dir.0 as i32, moving_sand.1 + dir.1 as i32);
                if moving_sand.1 < floor_y - 1 && !walls.contains(&dest) && !settled_sand.contains(&dest) {
                    moving_sand = dest;
                    moved = true;
                    break;
                }
            }
            if !moved {
                settled_sand.insert(moving_sand);
                break;
            }
            if bottom_reached_at.is_none() {
                if moving_sand.1 > lowest_wall_y {
                    bottom_reached_at = Some(settled_sand.len());
                }
            }
        }
        if settled_sand.contains(&SAND_ORIGIN) {
            end_reached = true;
        }
    }
    // Part 1
    println!("Bottom reached at: {}", bottom_reached_at.unwrap());

    // Part 2
    println!("Settled sand: {}", settled_sand.len());
}

fn get_range(p1: i32, p2: i32) -> Vec<i32> {
    if p1 < p2 {
        return (p1..(p2 + 1)).collect();
    } else {
        return (p2..(p1 + 1)).rev().collect();
    }
}

fn print_cave(walls: &HashSet<(i32, i32)>, sand: &HashSet<(i32, i32)>) {
    let single_wall = walls.iter().next().unwrap();
    let mut min_x = single_wall.0;
    let mut max_x = single_wall.0;
    let mut min_y = 0;
    let mut max_y = single_wall.1;
    walls.iter().for_each(|(x, y)| {
        if *x < min_x {
            min_x = *x;
        }
        if *x > max_x {
            max_x = *x;
        }
        if *y < min_y {
            min_y = *y;
        }
        if *y > max_y {
            max_y = *y;
        }
    });
    for y in min_y..(max_y + 1) {
        for x in min_x..(max_x + 1) {
            if sand.contains(&(x, y)) {
                print!("o");
            } 
            else if walls.contains(&(x, y)) {
                print!("#");
            }
            else {
                print!(".");
            }
        }
        println!();
    }
}
