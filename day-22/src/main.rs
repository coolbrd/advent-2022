use std::{fs, collections::HashMap};

type MapPosComp = i64;

type MapPos = (MapPosComp, MapPosComp);

type MonkeyMap = HashMap<(MapPosComp, MapPosComp), MapSpace>;

#[derive(Debug, Clone)]
struct MapSpace {
    neighbors: HashMap<Direction, MapPos>,
    is_wall: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
        self.map.get(&self.get_next_destination_map_position()).unwrap()
    }

    fn get_next_destination_map_position(&self) -> MapPos {
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
            self.position = self.get_next_destination_map_position();
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

#[derive(Debug)]
enum PlayerAction {
    MoveForward(usize),
    TurnLeft,
    TurnRight,
}

fn main() {
    let path = "resources/input2.txt";
    let contents = fs::read_to_string(path).expect("File not found");
    let parts = contents.split("\n\n").map(|line| line.trim_end()).collect::<Vec<&str>>();

    let actions = parse_actions(parts[1]);
    
    // Part 1
    let wrapping_monkey_map = parse_wrapping_monkey_map(parts[0]);
    let starting_map_position = get_starting_map_position(&wrapping_monkey_map);
    let mut player = Player { map: &wrapping_monkey_map, position: *starting_map_position, facing: Direction::East };
    player.perform_actions(&actions);
    let password = calculate_password(&player);
    println!("Password: {}", password);

    let cube_side_length = ((wrapping_monkey_map.len() / 6) as f64).sqrt() as usize;
    println!("Cube side length: {}", cube_side_length);
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
                neighbors.insert(*direction, new_position);
                continue;
            }
            let mut opposite_boundary_position = *map_position;
            let mut next_opposite_boundary_position = opposite_boundary_position;
            while flat_monkey_map.contains_key(&next_opposite_boundary_position) {
                opposite_boundary_position = next_opposite_boundary_position;
                next_opposite_boundary_position = (opposite_boundary_position.0 - x_offset as MapPosComp, opposite_boundary_position.1 - y_offset as MapPosComp);
            }
            neighbors.insert(*direction, opposite_boundary_position);
        }
        (*map_position, MapSpace { neighbors, is_wall: *is_wall })
    }).collect::<MonkeyMap>();
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