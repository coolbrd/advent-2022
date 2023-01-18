use std::{fs, collections::HashSet};

#[derive(Debug)]
enum GustDirection {
    Left,
    Right
}

struct RepeatingSequence<T> {
    sequence: Vec<T>,
    current_index: usize
}

impl<T> RepeatingSequence<T> {
    fn new(sequence: Vec<T>) -> RepeatingSequence<T> {
        RepeatingSequence {
            sequence,
            current_index: 0
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
}

struct Rock {
    body_points: Vec<(u8, u8)>
}

impl Rock {
    fn new(body_points: Vec<(u8, u8)>) -> Rock {
        Rock {
            body_points
        }
    }
}

struct Chamber {
    width: u8,
    points: HashSet<(u8, u64)>,
    highest_point_y: u64
}

impl Chamber {
    fn new(width: u8) -> Chamber {
        Chamber {
            width,
            points: HashSet::new(),
            highest_point_y: 0
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

    fn draw(&self) {
        for y in (0..=self.highest_point_y + 7).rev() {
            for x in 0..self.width {
                if self.points.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn draw_with_rock(&self, rock: &Rock, origin: (u8, u64)) {
        for y in (0..=self.highest_point_y + 7).rev() {
            for x in 0..self.width {
                if self.points.contains(&(x, y)) {
                    print!("#");
                } else if rock.body_points.iter().map(|body_point| {
                    (origin.0 + body_point.0, origin.1 + body_point.1 as u64)
                }).any(|body_point| {
                    body_point.0 == x && body_point.1 == y
                }) {
                    print!("O");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

fn main() {
    let path = "resources/input2.txt";
    let contents = fs::read_to_string(path).expect("File not found");
    let gusts = contents.trim().chars().into_iter().map(|c| {
        match c {
            '<' => GustDirection::Left,
            '>' => GustDirection::Right,
            _ => panic!("Invalid character")
        }
    }).collect::<Vec<GustDirection>>();
    let mut gusts = RepeatingSequence::new(gusts);
    let mut rocks = RepeatingSequence::new([
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (1, 0), (0, 1), (1, 1)]
    ].iter().map(|pieces| {
        Rock::new(pieces.to_vec())
    }).collect::<Vec<Rock>>());
    let mut chamber = Chamber::new(7);
    let target = 1_000_000_000_000;
    for i in 0_u64..target {
        if i % 100000 == 0 {
            println!("{}%", (i as f64 / target as f64) * 100.0);
        }
        let rock = rocks.next_item();
        let mut rock_pos = chamber.get_new_rock_origin();
        loop {
            let gust_direction = gusts.next_item();
            let gust_offset = match gust_direction {
                GustDirection::Left => -1,
                GustDirection::Right => 1
            };
            let mut x_offset = gust_offset;
            let potential_rock_pos = (rock_pos.0 as i8 + x_offset, rock_pos.1);
            for body_point in &rock.body_points {
                let potential_body_point = (potential_rock_pos.0 + body_point.0 as i8, potential_rock_pos.1 + body_point.1 as u64);
                if potential_body_point.0 < 0 {
                    x_offset = 0;
                    break;
                }
                let potential_body_point = (potential_body_point.0 as u8, potential_body_point.1);
                if potential_body_point.0 >= chamber.width || chamber.points.contains(&potential_body_point) {
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
                    let potential_body_point = (potential_rock_pos.0 + body_point.0, potential_rock_pos.1 + body_point.1 as u64);
                    if chamber.points.contains(&potential_body_point) {
                        collision_found = true;
                        break;
                    }
                }
            }
            if collision_found {
                chamber.insert_rock(rock, rock_pos);
                break;
            }
            rock_pos = potential_rock_pos;
        }
    }
    println!("Highest point: {}", chamber.highest_point_y);
}
