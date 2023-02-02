use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Clone, Copy)]
enum CardinalDirection {
    North,
    East,
    South,
    West,
}

impl CardinalDirection {
    fn get_offset(&self) -> (i64, i64) {
        match self {
            CardinalDirection::North => (0, -1),
            CardinalDirection::East => (1, 0),
            CardinalDirection::South => (0, 1),
            CardinalDirection::West => (-1, 0),
        }
    }
}

type ElfPosComp = i64;

type ElfPos = (ElfPosComp, ElfPosComp);

type ElfGroup = HashSet<ElfPos>;

trait CanAddCardinalDirection {
    fn add(self, other: CardinalDirection) -> Self;
}

impl CanAddCardinalDirection for ElfPos {
    fn add(self, other: CardinalDirection) -> Self {
        let offset = other.get_offset();
        (self.0 + offset.0, self.1 + offset.1)
    }
}

struct ElfGroupDispersalGame {
    group: ElfGroup,
    direction_order: Vec<CardinalDirection>,
    round_number: u64,
    elves_moved_last_round: bool,
}

impl ElfGroupDispersalGame {
    fn new(group: ElfGroup) -> Self {
        let direction_order = vec![
            CardinalDirection::North,
            CardinalDirection::South,
            CardinalDirection::West,
            CardinalDirection::East,
        ];
        ElfGroupDispersalGame {
            group,
            direction_order,
            round_number: 0,
            elves_moved_last_round: true,
        }
    }

    fn perform_rounds(&mut self, n: usize) {
        for _ in 0..n {
            self.perform_round();
        }
    }

    fn perform_rounds_until_stable(&mut self) {
        while self.elves_moved_last_round {
            self.perform_round();
        }
    }

    fn perform_round(&mut self) {
        let mut elves_moved_this_round = false;
        let mut tentative_elf_destinations = HashMap::new();
        for elf_pos in self.group.iter() {
            let destination = if self.elf_is_isolated(elf_pos) {
                *elf_pos
            } else {
                elves_moved_this_round = true;
                let mut destination = None;
                for direction in self.direction_order.iter() {
                    if self.elf_can_move_in_direction(elf_pos, direction) {
                        destination = Some(elf_pos.add(*direction));
                        break;
                    }
                }
                destination.unwrap_or(*elf_pos)
            };
            tentative_elf_destinations
                .entry(destination)
                .or_insert(vec![])
                .push(*elf_pos);
        }
        let mut new_group = ElfGroup::new();
        for (destination_pos, elves_going_to_destination) in tentative_elf_destinations.iter() {
            if elves_going_to_destination.len() == 1 {
                new_group.insert(*destination_pos);
            } else {
                for &elf_pos in elves_going_to_destination {
                    new_group.insert(elf_pos);
                }
            }
        }
        self.elves_moved_last_round = elves_moved_this_round;
        self.group = new_group;
        self.advance_to_next_round();
    }

    fn elf_is_isolated(&self, elf: &ElfPos) -> bool {
        let (x, y) = *elf;
        let north = (x, y - 1);
        let north_west = (x - 1, y - 1);
        let west = (x - 1, y);
        let south_west = (x - 1, y + 1);
        let south = (x, y + 1);
        let south_east = (x + 1, y + 1);
        let east = (x + 1, y);
        let north_east = (x + 1, y - 1);
        [
            north, north_west, west, south_west, south, south_east, east, north_east,
        ]
        .iter()
        .all(|pos| !self.group.contains(pos))
    }

    fn elf_can_move_in_direction(&self, elf: &ElfPos, direction: &CardinalDirection) -> bool {
        let (x, y) = *elf;
        match direction {
            CardinalDirection::North => {
                let north = (x, y - 1);
                let north_west = (x - 1, y - 1);
                let north_east = (x + 1, y - 1);
                [north, north_west, north_east]
                    .iter()
                    .all(|pos| !self.group.contains(pos))
            }
            CardinalDirection::East => {
                let east = (x + 1, y);
                let north_east = (x + 1, y - 1);
                let south_east = (x + 1, y + 1);
                [east, north_east, south_east]
                    .iter()
                    .all(|pos| !self.group.contains(pos))
            }
            CardinalDirection::South => {
                let south = (x, y + 1);
                let south_west = (x - 1, y + 1);
                let south_east = (x + 1, y + 1);
                [south, south_west, south_east]
                    .iter()
                    .all(|pos| !self.group.contains(pos))
            }
            CardinalDirection::West => {
                let west = (x - 1, y);
                let north_west = (x - 1, y - 1);
                let south_west = (x - 1, y + 1);
                [west, north_west, south_west]
                    .iter()
                    .all(|pos| !self.group.contains(pos))
            }
        }
    }

    fn advance_to_next_round(&mut self) {
        self.round_number += 1;
        self.direction_order.rotate_left(1);
    }

    fn count_empty_spaces_within_bounds(&self) -> usize {
        let ((min_x, min_y), (max_x, max_y)) = self.get_group_bounds();
        let mut empty_spaces = 0;
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if !self.group.contains(&(x, y)) {
                    empty_spaces += 1;
                }
            }
        }
        empty_spaces
    }

    fn get_group_bounds(&self) -> (ElfPos, ElfPos) {
        let mut min_x = i64::MAX;
        let mut min_y = i64::MAX;
        let mut max_x = i64::MIN;
        let mut max_y = i64::MIN;
        for (x, y) in self.group.iter() {
            if *x < min_x {
                min_x = *x;
            }
            if *y < min_y {
                min_y = *y;
            }
            if *x > max_x {
                max_x = *x;
            }
            if *y > max_y {
                max_y = *y;
            }
        }
        ((min_x, min_y), (max_x, max_y))
    }
}

fn main() {
    let path = "resources/input.txt";
    let contents = fs::read_to_string(path).expect("File not found");
    let lines = contents
        .split("\n")
        .map(|line| line.trim())
        .collect::<Vec<&str>>();
    let group = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some((x as ElfPosComp, y as ElfPosComp))
                } else {
                    None
                }
            })
        })
        .collect::<ElfGroup>();

    // Part 1
    let target_rounds = 10;
    let mut game = ElfGroupDispersalGame::new(group.clone());
    game.perform_rounds(target_rounds);
    let empty_spaces = game.count_empty_spaces_within_bounds();
    println!("Empty spaces after {} rounds: {}", target_rounds, empty_spaces);

    // Part 2
    let mut game = ElfGroupDispersalGame::new(group);
    game.perform_rounds_until_stable();
    let rounds_taken = game.round_number;
    println!("Rounds taken to stabilize: {}", rounds_taken);
}
