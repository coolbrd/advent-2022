use std::{
    collections::{HashMap, HashSet},
    fs,
};

enum GustDirection {
    Left,
    Right,
}

struct RepeatingSequence<T> {
    sequence: Vec<T>,
    current_index: usize,
}

impl<T> RepeatingSequence<T> {
    fn new(sequence: Vec<T>) -> RepeatingSequence<T> {
        RepeatingSequence {
            sequence,
            current_index: 0,
        }
    }

    fn next_item(&mut self) -> &T {
        let previous_index = self.current_index;
        self.advance_index();
        let item = &self.sequence[previous_index];
        item
    }

    fn advance_index(&mut self) {
        self.current_index = (self.current_index + 1) % self.sequence.len();
    }

    fn reset(&mut self) {
        self.current_index = 0;
    }
}

struct Rock {
    body_points: Vec<(u8, u8)>,
}

impl Rock {
    fn new(body_points: Vec<(u8, u8)>) -> Rock {
        Rock { body_points }
    }
}

struct Chamber {
    width: u8,
    points: HashSet<(u8, u64)>,
    highest_point_y: u64,
}

impl Chamber {
    fn new(width: u8) -> Chamber {
        Chamber {
            width,
            points: HashSet::new(),
            highest_point_y: 0,
        }
    }

    fn drop_rock(
        &mut self,
        rocks: &mut RepeatingSequence<Rock>,
        gusts: &mut RepeatingSequence<GustDirection>,
    ) {
        let rock = rocks.next_item();
        let mut rock_pos = self.get_new_rock_origin();
        loop {
            let gust_direction = gusts.next_item();
            let gust_offset = match gust_direction {
                GustDirection::Left => -1,
                GustDirection::Right => 1,
            };
            let mut x_offset = gust_offset;
            let potential_rock_pos = (rock_pos.0 as i8 + x_offset, rock_pos.1);
            for body_point in &rock.body_points {
                let potential_body_point = (
                    potential_rock_pos.0 + body_point.0 as i8,
                    potential_rock_pos.1 + body_point.1 as u64,
                );
                if potential_body_point.0 < 0 {
                    x_offset = 0;
                    break;
                }
                let potential_body_point = (potential_body_point.0 as u8, potential_body_point.1);
                if potential_body_point.0 >= self.width
                    || self.points.contains(&potential_body_point)
                {
                    x_offset = 0;
                    break;
                }
            }
            rock_pos = ((rock_pos.0 as i8 + x_offset) as u8, rock_pos.1);
            let mut collision_found = false;
            if rock_pos.1 == 0 {
                collision_found = true;
            }
            let mut potential_rock_pos = (rock_pos.0, rock_pos.1);
            if !collision_found {
                potential_rock_pos = (rock_pos.0, rock_pos.1 - 1);
                for body_point in &rock.body_points {
                    let potential_body_point = (
                        potential_rock_pos.0 + body_point.0,
                        potential_rock_pos.1 + body_point.1 as u64,
                    );
                    if self.points.contains(&potential_body_point) {
                        collision_found = true;
                        break;
                    }
                }
            }
            if collision_found {
                self.insert_rock(rock, rock_pos);
                break;
            }
            rock_pos = potential_rock_pos;
        }
    }

    fn get_new_rock_origin(&self) -> (u8, u64) {
        (2, self.highest_point_y + 3)
    }

    fn insert_rock(&mut self, rock: &Rock, origin: (u8, u64)) {
        for body_point in &rock.body_points {
            let point = (origin.0 + body_point.0, origin.1 + body_point.1 as u64);
            self.points.insert(point);
            if (point.1 + 1) > self.highest_point_y {
                self.highest_point_y = point.1 + 1;
            }
        }
    }
}

fn main() {
    let path = "resources/input.txt";
    let contents = fs::read_to_string(path).expect("File not found");
    let gusts = contents
        .trim()
        .chars()
        .into_iter()
        .map(|c| match c {
            '<' => GustDirection::Left,
            '>' => GustDirection::Right,
            _ => panic!("Invalid character"),
        })
        .collect::<Vec<GustDirection>>();
    let mut gusts = RepeatingSequence::new(gusts);
    let mut rocks = RepeatingSequence::new(
        [
            vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
            vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            vec![(0, 0), (1, 0), (0, 1), (1, 1)],
        ]
        .iter()
        .map(|pieces| Rock::new(pieces.to_vec()))
        .collect::<Vec<Rock>>(),
    );

    // Part 1
    let mut chamber = Chamber::new(7);
    let target = 2022;
    for _ in 0_u64..target {
        chamber.drop_rock(&mut rocks, &mut gusts);
    }
    println!("Height after {} rocks: {}", target, chamber.highest_point_y);

    // Part 2
    let mut chamber = Chamber::new(7);
    rocks.reset();
    gusts.reset();
    let mut cycle_heights: HashMap<(usize, usize), Vec<u64>> = HashMap::new();
    let mut cycle_indices: HashMap<(usize, usize), Vec<u64>> = HashMap::new();
    let mut i = 0_u64;
    let cycle_pair;
    loop {
        let current_pair = (rocks.current_index, gusts.current_index);
        chamber.drop_rock(&mut rocks, &mut gusts);
        if cycle_heights.contains_key(&current_pair) {
            let start_indices = cycle_indices.get_mut(&current_pair).unwrap();
            start_indices.push(i);
            let heights = cycle_heights.get_mut(&current_pair).unwrap();
            heights.push(chamber.highest_point_y);
            let differences = heights
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect::<Vec<u64>>();
            if differences.len() > 1 && differences.iter().all(|diff| *diff == differences[0]) {
                cycle_pair = current_pair;
                break;
            }
        } else {
            cycle_heights.insert(current_pair, vec![chamber.highest_point_y]);
            cycle_indices.insert(current_pair, vec![i]);
        }
        i += 1;
    }
    let target = 1_000_000_000_000;
    let cycle_start = cycle_indices.get(&cycle_pair).unwrap()[0];
    let cycle_start_height = cycle_heights.get(&cycle_pair).unwrap()[0];
    let cycle_length = cycle_indices.get(&cycle_pair).unwrap()[1] - cycle_start;
    let cycle_height_diff = cycle_heights.get(&cycle_pair).unwrap()[1] - cycle_start_height;
    let cycling_rocks = target - cycle_start;
    let cycles_total = cycling_rocks / cycle_length;
    let leftover_rocks = cycling_rocks - cycles_total * cycle_length;
    rocks.current_index = cycle_pair.0;
    gusts.current_index = cycle_pair.1;
    let height_before_leftovers = chamber.highest_point_y;
    for _ in 0..leftover_rocks {
        chamber.drop_rock(&mut rocks, &mut gusts);
    }
    let leftover_height = chamber.highest_point_y - height_before_leftovers;
    let answer = cycles_total * cycle_height_diff + cycle_start_height + leftover_height - 1;
    println!("Height after {} rocks: {}", target, answer);
}
