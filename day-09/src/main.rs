use std::{fs, collections::{HashMap, HashSet}};

fn main() {
    let path = "resources/input.txt";
    let contents = fs::read_to_string(path).expect("File not found");
    let lines = contents.split("\n").collect::<Vec<&str>>();
    let movements = lines.iter().map(|line| {
        let mut chars = line.chars();
        let direction = chars.next().unwrap();
        let distance = chars.as_str().trim().parse::<u8>().unwrap();
        (direction, distance)
    }).collect::<Vec<(char, u8)>>();

    let mut head_pos = (0_i16, 0_i16);
    let mut tail_pos = (0_i16, 0_i16);

    let mut tail_visited: HashSet<(i16, i16)> = HashSet::new();
    tail_visited.insert(tail_pos);
    movements.iter().for_each(|(direction, distance)| {
        let movement_offset = match direction {
            'R' => (1, 0),
            'L' => (-1, 0),
            'U' => (0, 1),
            'D' => (0, -1),
            _ => panic!("Invalid direction")
        };
        for _ in 0..*distance {
            let head_pos_last = head_pos;
            head_pos = (head_pos.0 + movement_offset.0, head_pos.1 + movement_offset.1);
            if !tail_is_connected_to_head(head_pos, tail_pos) {
                tail_pos = head_pos_last;
                tail_visited.insert(tail_pos);
            }
        }
    });

    println!("Tail visited: {}", tail_visited.len());
}

fn tail_is_connected_to_head(head_pos: (i16, i16), tail_pos: (i16, i16)) -> bool {
    let direction_offsets: Vec<(i16, i16)> = vec![(1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1), (0, 1), (0, 0)];
    for offset in direction_offsets {
        let offset_pos = (head_pos.0 + offset.0, head_pos.1 + offset.1);
        if tail_pos == offset_pos {
            return true;
        }
    }
    return false;
}
