use std::{fs, collections::HashSet};

type MapPosComp = i32;

type MapPos = (MapPosComp, MapPosComp);

#[derive(Debug)]
enum CardinalDirection {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
struct Blizzard {
    initial_position: MapPos,
    direction: CardinalDirection,
}

#[derive(Debug)]
struct BlizzardMap {
    blizzards: Vec<Blizzard>,
    bounds: (MapPos, MapPos),
}

impl BlizzardMap {
    fn get_blizzard_positions_after_steps(&self, steps: u16) -> HashSet<MapPos> {
        let steps = steps as i32;
        self.blizzards.iter().map(|blizzard| {
            let width = self.get_map_width();
            let height = self.get_map_height();
            match blizzard.direction {
                CardinalDirection::North => {
                    (blizzard.initial_position.0, (blizzard.initial_position.1 - steps).rem_euclid(height))
                },
                CardinalDirection::South => {
                    (blizzard.initial_position.0, (blizzard.initial_position.1 + steps).rem_euclid(height))
                },
                CardinalDirection::East => {
                    ((blizzard.initial_position.0 + steps).rem_euclid(width), blizzard.initial_position.1)
                },
                CardinalDirection::West => {
                    ((blizzard.initial_position.0 - steps).rem_euclid(width), blizzard.initial_position.1)
                }
            }
        }).collect()
    }

    fn get_map_width(&self) -> MapPosComp {
        self.bounds.1.0 - self.bounds.0.0 + 1
    }

    fn get_map_height(&self) -> MapPosComp {
        self.bounds.1.1 - self.bounds.0.1 + 1
    }

    fn draw_map_at_step(&self, step: u16) {
        let blizzard_positions = self.get_blizzard_positions_after_steps(step);
        let width = self.get_map_width();
        let height = self.get_map_height();
        for y in 0..height {
            for x in 0..width {
                let pos = (x, y);
                if blizzard_positions.contains(&pos) {
                    print!("X");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

const PLAYER_MOVEMENT_DIRECTIONS: [MapPos; 5] = [
    (0, 0),
    (0, -1),
    (0, 1),
    (1, 0),
    (-1, 0),
];

fn main() {
    let path = "resources/input.txt";
    let contents = fs::read_to_string(path).expect("File not found");
    let lines = contents.split("\n").map(|line| line.trim()).collect::<Vec<&str>>();
    let blizzards = lines.iter().rev().skip(1).rev().skip(1).enumerate().flat_map(|(y, line)| {
        line.chars().collect::<Vec<char>>().iter().rev().skip(1).rev().skip(1)
            .enumerate().collect::<Vec<(usize, &char)>>().iter().filter(|(_, c)| {
            **c != '.'
        }).map(|(x, c)| {
            let direction = match c {
                '^' => CardinalDirection::North,
                'v' => CardinalDirection::South,
                '>' => CardinalDirection::East,
                '<' => CardinalDirection::West,
                _ => panic!("Invalid direction character: {}", c),
            };
            Blizzard {
                initial_position: (*x as MapPosComp, y as MapPosComp),
                direction,
            }
        }).collect::<Vec<Blizzard>>()
    }).collect::<Vec<Blizzard>>();
    let blizzard_map = BlizzardMap {
        blizzards,
        bounds: ((0, 0), (lines[0].len() as MapPosComp - 3, lines.len() as MapPosComp - 3)),
    };

    // Part 1
    let start_pos = (blizzard_map.bounds.0.0, blizzard_map.bounds.0.1 - 1);
    let end_pos = (blizzard_map.bounds.1.0, blizzard_map.bounds.1.1 + 1);
    let steps_to_end = get_shortest_distance_of_path(&blizzard_map, start_pos, end_pos, 0);
    println!("Part 1: {}", steps_to_end);

    // Part 2
    let steps_from_end_to_start = get_shortest_distance_of_path(&blizzard_map, end_pos, start_pos, steps_to_end);
    let steps_round_trip = steps_to_end + steps_from_end_to_start;
    let steps_from_start_to_end_trip_2 = get_shortest_distance_of_path(&blizzard_map, start_pos, end_pos, steps_round_trip);
    let steps_total = steps_to_end + steps_from_end_to_start + steps_from_start_to_end_trip_2;
    println!("Part 2: {}", steps_total);
}

fn get_shortest_distance_of_path(blizzard_map: &BlizzardMap, start_pos: MapPos, end_pos: MapPos, starting_step: u16) -> u16 {
    let mut current_connected_layer = HashSet::from([start_pos]);
    let mut current_step = 0;
    while !current_connected_layer.contains(&end_pos) {
        let current_layer_blizzards = blizzard_map.get_blizzard_positions_after_steps(current_step + starting_step);
        let mut new_connected_layer = HashSet::new();
        for pos in current_connected_layer {
            for direction in PLAYER_MOVEMENT_DIRECTIONS.iter() {
                let new_pos = (pos.0 + direction.0, pos.1 + direction.1);
                if new_pos == end_pos {
                    return current_step;
                }
                if new_pos.0 < blizzard_map.bounds.0.0 || new_pos.0 > blizzard_map.bounds.1.0 ||
                    new_pos.1 < blizzard_map.bounds.0.1 || new_pos.1 > blizzard_map.bounds.1.1 {
                    continue;
                }
                if !current_layer_blizzards.contains(&new_pos) {
                    new_connected_layer.insert(new_pos);
                }
            }
        }
        if new_connected_layer.is_empty() {
            new_connected_layer.insert(start_pos);
        }
        current_connected_layer = new_connected_layer;
        current_step += 1;
    }
    panic!("No path found from {:?} to {:?}", start_pos, end_pos);
}
