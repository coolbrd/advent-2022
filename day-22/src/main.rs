use std::{fs, collections::HashMap};

type MapPosComp = i64;

type MapPos = (MapPosComp, MapPosComp);

type MonkeyMap = HashMap<(MapPosComp, MapPosComp), MapSpace>;

#[derive(Clone)]
struct MapSpace {
    neighbors: HashMap<Direction, (MapPos, Direction)>,
    is_wall: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn get_offset(&self) -> (i8, i8) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
            Direction::East => (1, 0),
        }
    }
}

struct Player<'a> {
    map: &'a MonkeyMap,
    position: MapPos,
    facing: Direction,
}

impl Player<'_> {
    fn perform_actions(&mut self, actions: &Vec<PlayerAction>) {
        for action in actions {
            match action {
                PlayerAction::MoveForward(num) => self.move_forward(*num),
                PlayerAction::TurnLeft => self.turn_left(),
                PlayerAction::TurnRight => self.turn_right(),
            }
        }
    }

    fn get_current_map_space(&self) -> &MapSpace {
        self.map.get(&self.position).unwrap()
    }

    fn get_next_destination_map_space(&self) -> &MapSpace {
        self.map.get(&self.get_next_destination_map_position().0).unwrap()
    }

    fn get_next_destination_map_position(&self) -> (MapPos, Direction) {
        *self.get_current_map_space().neighbors.get(&self.facing).unwrap()
    }

    fn get_facing_num(&self) -> u8 {
        match self.facing {
            Direction::East => 0,
            Direction::South => 1,
            Direction::West => 2,
            Direction::North => 3,
        }
    }

    fn move_forward(&mut self, distance: usize) {
        for _ in 0..distance {
            let next_map_space = self.get_next_destination_map_space();
            if next_map_space.is_wall {
                break;
            }
            let destination_pos = self.get_next_destination_map_position();
            self.position = destination_pos.0;
            self.facing = destination_pos.1;
        }
    }

    fn turn_left(&mut self) {
        self.facing = match self.facing {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::East => Direction::North,
        };
    }

    fn turn_right(&mut self) {
        self.facing = match self.facing {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::East => Direction::South,
        };
    }
}

enum PlayerAction {
    MoveForward(usize),
    TurnLeft,
    TurnRight,
}

fn main() {
    let path = "resources/input.txt";
    let contents = fs::read_to_string(path).expect("File not found");
    let parts = contents.split("\n\n").map(|line| line.trim_end()).collect::<Vec<&str>>();
    let actions = parse_actions(parts[1]);

    // Part 1
    let wrapping_monkey_map = parse_wrapping_monkey_map(parts[0]);
    let starting_map_position = get_starting_map_position(&wrapping_monkey_map);
    let mut player = Player { map: &wrapping_monkey_map, position: *starting_map_position, facing: Direction::East };
    player.perform_actions(&actions);
    let password = calculate_password(&player);
    println!("Wrapping map password: {}", password);

    // Part 2
    // Not smart enough to generalize this
    let input_cube_side_transition_map = [
        vec![
            (Direction::North, (5, Direction::East)),
            (Direction::East, (1, Direction::East)),
            (Direction::South, (2, Direction::South)),
            (Direction::West, (3, Direction::East))
        ],
        vec![
            (Direction::North, (5, Direction::North)),
            (Direction::East, (4, Direction::West)),
            (Direction::South, (2, Direction::West)),
            (Direction::West, (0, Direction::West))
        ],
        vec![
            (Direction::North, (0, Direction::North)),
            (Direction::East, (1, Direction::North)),
            (Direction::South, (4, Direction::South)),
            (Direction::West, (3, Direction::South))
        ],
        vec![
            (Direction::North, (2, Direction::East)),
            (Direction::East, (4, Direction::East)),
            (Direction::South, (5, Direction::South)),
            (Direction::West, (0, Direction::East))
        ],
        vec![
            (Direction::North, (2, Direction::North)),
            (Direction::East, (1, Direction::West)),
            (Direction::South, (5, Direction::West)),
            (Direction::West, (3, Direction::West))
        ],
        vec![
            (Direction::North, (3, Direction::North)),
            (Direction::East, (4, Direction::North)),
            (Direction::South, (1, Direction::South)),
            (Direction::West, (0, Direction::South))
        ],
    ].iter().map(|v| {
        v.iter().cloned().collect::<HashMap<Direction, (usize, Direction)>>()
    }).collect::<Vec<HashMap<Direction, (usize, Direction)>>>();
    let cube_side_length = ((wrapping_monkey_map.len() / 6) as f64).sqrt() as usize;
    let cubic_monkey_map = parse_cubic_monkey_map(parts[0], cube_side_length, input_cube_side_transition_map);
    let starting_map_position = get_starting_map_position(&cubic_monkey_map);
    let mut player = Player { map: &cubic_monkey_map, position: *starting_map_position, facing: Direction::East };
    player.perform_actions(&actions);
    let password = calculate_password(&player);
    println!("Cubic map password: {}", password);
}

fn parse_wrapping_monkey_map(map_string: &str) -> MonkeyMap {
    let flat_monkey_map = map_string.split("\n").enumerate().map(|(y, line)| {
        line.chars().enumerate().collect::<Vec<(usize, char)>>().iter().filter(|(_, c)| *c != ' ').map(|(x, c)| {
            let is_wall = *c == '#';
            ((*x as MapPosComp, y as MapPosComp), is_wall)
        }).collect::<Vec<(MapPos, bool)>>()
    }).flatten().collect::<HashMap<MapPos, bool>>();
    let monkey_map = flat_monkey_map.iter().map(|(map_position, is_wall)| {
        let mut neighbors = HashMap::new();
        for direction in [Direction::North, Direction::South, Direction::West, Direction::East].iter() {
            let (x_offset, y_offset) = direction.get_offset();
            let new_position = (map_position.0 + x_offset as MapPosComp, map_position.1 + y_offset as MapPosComp);
            if flat_monkey_map.contains_key(&new_position) {
                neighbors.insert(*direction, (new_position, *direction));
                continue;
            }
            let mut opposite_boundary_position = *map_position;
            let mut next_opposite_boundary_position = opposite_boundary_position;
            while flat_monkey_map.contains_key(&next_opposite_boundary_position) {
                opposite_boundary_position = next_opposite_boundary_position;
                next_opposite_boundary_position = (opposite_boundary_position.0 - x_offset as MapPosComp, opposite_boundary_position.1 - y_offset as MapPosComp);
            }
            neighbors.insert(*direction, (opposite_boundary_position, *direction));
        }
        (*map_position, MapSpace { neighbors, is_wall: *is_wall })
    }).collect::<MonkeyMap>();
    return monkey_map;
}

fn parse_cubic_monkey_map(map_string: &str, side_length: usize, transition_table: Vec<HashMap<Direction, (usize, Direction)>>) -> MonkeyMap {
    let mut current_face = 0;
    let mut cubic_faces = vec![vec![vec![]; side_length]; 6];
    for (i, row_chunk) in map_string.split("\n").collect::<Vec<&str>>().chunks(side_length).enumerate() {
        for j in 0..(row_chunk[0].len() / side_length) {
            let mut increase_face = false;
            for (ri, row) in row_chunk.iter().enumerate() {
                let row_slice = &row[side_length * j..side_length * (j + 1)];
                if row_slice.chars().into_iter().any(|c| c != ' ') {
                    increase_face = true;
                }
                if increase_face {
                    for (ci, c) in row_slice.chars().into_iter().enumerate() {
                        let is_wall = c == '#';
                        let x = j * side_length + ci;
                        let y = i * side_length + ri;
                        cubic_faces[current_face][ri].push(((x as i64, y as i64), is_wall));
                    }
                }
            }
            if increase_face {
                current_face += 1;
            }
        }
    }
    let mut monkey_map = HashMap::new();
    for (fi, face) in cubic_faces.iter().enumerate() {
        for (y, row) in face.iter().enumerate() {
            for (x, pos) in row.iter().enumerate() {
                let mut neighbors = HashMap::new();
                for direction in [Direction::North, Direction::South, Direction::West, Direction::East].iter() {
                    let (x_offset, y_offset) = direction.get_offset();
                    let new_position_within_face = (x as i32 + x_offset as i32, y as i32 + y_offset as i32);
                    if new_position_within_face.0 < 0 || new_position_within_face.0 >= side_length as i32 || new_position_within_face.1 < 0 || new_position_within_face.1 >= side_length as i32 {
                        let (new_face, new_direction) = transition_table[fi][direction];
                        let other_face_pos = match (direction, new_direction) {
                            (Direction::North, Direction::North) => (x, side_length - 1),
                            (Direction::North, Direction::South) => (side_length - x - 1, 0),
                            (Direction::North, Direction::West) => (side_length - 1, side_length - x - 1),
                            (Direction::North, Direction::East) => (0, x),
                            (Direction::South, Direction::North) => (side_length - x - 1, side_length - 1),
                            (Direction::South, Direction::South) => (x, 0),
                            (Direction::South, Direction::West) => (side_length - 1, x),
                            (Direction::South, Direction::East) => (0, side_length - x - 1),
                            (Direction::West, Direction::North) => (side_length - y - 1, side_length - 1),
                            (Direction::West, Direction::South) => (y, 0),
                            (Direction::West, Direction::West) => (side_length - 1, y),
                            (Direction::West, Direction::East) => (0, side_length - y - 1),
                            (Direction::East, Direction::North) => (y, side_length - 1),
                            (Direction::East, Direction::South) => (side_length - y - 1, 0),
                            (Direction::East, Direction::West) => (side_length - 1, side_length - y - 1),
                            (Direction::East, Direction::East) => (0, y),
                        };
                        let new_pos = cubic_faces[new_face][other_face_pos.1][other_face_pos.0];
                        neighbors.insert(*direction, (new_pos.0, new_direction));
                        continue;
                    }
                    let new_pos = cubic_faces[fi][new_position_within_face.1 as usize][new_position_within_face.0 as usize];
                    neighbors.insert(*direction, (new_pos.0, *direction));
                }
                monkey_map.insert(pos.0, MapSpace { neighbors, is_wall: pos.1 });
            }
        }
    }
    return monkey_map;
}

fn parse_actions(action_char_sequence: &str) -> Vec<PlayerAction> {
    let mut actions = vec![];
    let mut cur_num = vec![];
    let max_index = action_char_sequence.len() - 1;
    for (i, c) in action_char_sequence.chars().enumerate() {
        if c.is_digit(10) {
            cur_num.push(c);
            if i == max_index {
                let num = cur_num.iter().collect::<String>().parse().unwrap();
                cur_num = vec![];
                actions.push(PlayerAction::MoveForward(num));
            }
        } else {
            let num = cur_num.iter().collect::<String>().parse().unwrap();
            cur_num = vec![];
            actions.push(PlayerAction::MoveForward(num));
            let action = match c {
                'L' => PlayerAction::TurnLeft,
                'R' => PlayerAction::TurnRight,
                _ => panic!("Unknown direction"),
            };
            actions.push(action);
        }
    }
    return actions;
}

fn get_starting_map_position(monkey_map: &MonkeyMap) -> &MapPos {
    let mut starting_map_positions = monkey_map.iter().filter(|(map_pos, map_space)| {
        map_pos.1 == 0 && !map_space.is_wall
    }).collect::<Vec<(&MapPos, &MapSpace)>>();
    starting_map_positions.sort_by(|(map_pos_a, _), (map_pos_b, _)| {
        map_pos_a.0.cmp(&map_pos_b.0)
    });
    let starting_map_position = starting_map_positions.first().unwrap().0;
    return starting_map_position;
}

fn calculate_password(player: &Player) -> i64 {
    let (row, col, facing) = (player.position.1 + 1, player.position.0 + 1, player.get_facing_num());
    let password = row * 1000 + col * 4 + facing as i64;
    return password;
}