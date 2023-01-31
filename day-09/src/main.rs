use std::{collections::HashSet, fs};

const ADJACENT_SPACE_OFFSETS: [(i8, i8); 9] = [
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (0, 0),
];

type RopePosComp = i16;

type RopeSegmentPos = (RopePosComp, RopePosComp);

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Direction {
        match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }

    fn get_offset(&self) -> (i8, i8) {
        match self {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

fn main() {
    let path = "resources/input.txt";
    let contents = fs::read_to_string(path).expect("File not found");
    let lines = contents.split("\n").collect::<Vec<&str>>();
    let movements = lines
        .iter()
        .map(|line| {
            let mut chars = line.chars();
            let direction_char = chars
                .next()
                .expect(&format!("Invalid line, no direction found: {}", line));
            let direction = Direction::from_char(direction_char);
            let distance = chars
                .as_str()
                .trim()
                .parse::<u8>()
                .expect(&format!("Invalid line, non-numeric distance: {}", line));
            (direction, distance)
        })
        .collect::<Vec<(Direction, u8)>>();

    // Part 1
    let tail_visited = create_and_simulate_rope(2, &movements);
    println!("Length 2 tail visited: {}", tail_visited.len());

    // Part 2
    let tail_visited = create_and_simulate_rope(10, &movements);
    println!("Length 10 tail visited: {}", tail_visited.len());
}

fn create_and_simulate_rope(
    length: usize,
    movements: &Vec<(Direction, u8)>,
) -> HashSet<RopeSegmentPos> {
    let mut rope = vec![(0 as RopePosComp, 0 as RopePosComp); length];
    let mut tail_visited = HashSet::new();
    tail_visited.insert(rope[length - 1]);
    for (direction, distance) in movements {
        let movement_offset = direction.get_offset();
        for _ in 0..*distance {
            rope[0] = (
                rope[0].0 + movement_offset.0 as RopePosComp,
                rope[0].1 + movement_offset.1 as RopePosComp,
            );
            for i in 1..length {
                let leading_segment = rope[i - 1];
                let tailing_segment = rope[i];
                if !segments_are_connected(leading_segment, tailing_segment) {
                    rope[i] = determine_next_tailing_segment_pos(leading_segment, tailing_segment);
                    if i == length - 1 {
                        tail_visited.insert(rope[i]);
                    }
                }
            }
        }
    }
    return tail_visited;
}

fn segments_are_connected(seg_1: RopeSegmentPos, seg_2: RopeSegmentPos) -> bool {
    for offset in ADJACENT_SPACE_OFFSETS {
        let offset_pos = (
            seg_1.0 + offset.0 as RopePosComp,
            seg_1.1 + offset.1 as RopePosComp,
        );
        if seg_2 == offset_pos {
            return true;
        }
    }
    return false;
}

fn determine_next_tailing_segment_pos(
    leading_segment: RopeSegmentPos,
    tailing_segment: RopeSegmentPos,
) -> RopeSegmentPos {
    let total_offset = (
        leading_segment.0 - tailing_segment.0,
        leading_segment.1 - tailing_segment.1,
    );
    let clamped_offset = (total_offset.0.clamp(-1, 1), total_offset.1.clamp(-1, 1));
    return (
        tailing_segment.0 + clamped_offset.0,
        tailing_segment.1 + clamped_offset.1,
    );
}
