use std::{collections::HashSet, fs};

const SAND_ORIGIN: (i32, i32) = (500, 0);
const SAND_MOVEMENT_DIRECTIONS: [(i8, i8); 3] = [(0, 1), (-1, 1), (1, 1)];

type PosComp = i32;

type MapPos = (PosComp, PosComp);

type SegmentedLine = Vec<MapPos>;

fn main() {
    let path = "resources/input.txt";
    let contents = fs::read_to_string(path).expect("File not found");
    let lines = contents
        .split("\n")
        .map(|line| line.trim())
        .collect::<Vec<&str>>();
    let segments = lines
        .iter()
        .map(|line| {
            line.split("->")
                .map(|vertex| {
                    let point = vertex
                        .trim()
                        .split(",")
                        .map(|comp| {
                            comp.parse::<PosComp>()
                                .expect(&format!("Unexpected vertex component value: {}", comp))
                        })
                        .collect::<Vec<PosComp>>();
                    (point[0], point[1])
                })
                .collect::<SegmentedLine>()
        })
        .collect::<Vec<SegmentedLine>>();

    let mut walls = HashSet::new();
    for segment in segments {
        for window in segment.windows(2) {
            let last_vertex = window[0];
            let current_vertex = window[1];
            let x_range = get_range(last_vertex.0, current_vertex.0);
            let y_range = get_range(last_vertex.1, current_vertex.1);
            for x in x_range {
                walls.insert((x, last_vertex.1));
            }
            for y in y_range {
                walls.insert((last_vertex.0, y));
            }
        }
    }
    let lowest_wall_y = walls.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let floor_y = lowest_wall_y + 2;

    let mut settled_sand = HashSet::new();
    let mut bottom_reached_at = None;
    let mut end_reached = false;
    while !end_reached {
        let mut moving_sand = SAND_ORIGIN;
        loop {
            let mut moved = false;
            for dir in SAND_MOVEMENT_DIRECTIONS {
                let dest = (
                    moving_sand.0 + dir.0 as PosComp,
                    moving_sand.1 + dir.1 as PosComp,
                );
                if moving_sand.1 < floor_y - 1
                    && !walls.contains(&dest)
                    && !settled_sand.contains(&dest)
                {
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

fn get_range(p1: PosComp, p2: PosComp) -> Vec<PosComp> {
    if p1 < p2 {
        return (p1..(p2 + 1)).collect();
    }
    return (p2..(p1 + 1)).rev().collect();
}
