use std::{fs, collections::HashSet};

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

    let tail_visited = simulate_rope(&movements, 2);
    println!("Length 2 tail visited: {}", tail_visited.len());

    let tail_visited = simulate_rope(&movements, 10);
    println!("Length 10 tail visited: {}", tail_visited.len());
}

fn simulate_rope(movements: &Vec<(char, u8)>, length: usize) -> HashSet<(i16, i16)> {
    let mut rope = vec![(0_i16, 0_i16); length];
    let mut tail_visited: HashSet<(i16, i16)> = HashSet::new();
    tail_visited.insert(rope[length - 1]);
    movements.iter().for_each(|(direction, distance)| {
        let movement_offset = match direction {
            'R' => (1, 0),
            'L' => (-1, 0),
            'U' => (0, 1),
            'D' => (0, -1),
            _ => panic!("Invalid direction")
        };
        for _ in 0..*distance {
            rope[0] = (rope[0].0 + movement_offset.0, rope[0].1 + movement_offset.1);
            (1..length).for_each(|i| {
                let leading_segment = rope[i - 1];
                let tailing_segment = rope[i];
                if !segments_are_connected(leading_segment, tailing_segment) {
                    rope[i] = determine_next_tailing_segment_pos(leading_segment, tailing_segment);
                    if i == length - 1 {
                        tail_visited.insert(rope[i]);
                    }
                }
            });
        }
    });
    return tail_visited;
}

fn segments_are_connected(seg_1: (i16, i16), seg_2: (i16, i16)) -> bool {
    let direction_offsets: Vec<(i16, i16)> = vec![(1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1), (0, 1), (0, 0)];
    for offset in direction_offsets {
        let offset_pos = (seg_1.0 + offset.0, seg_1.1 + offset.1);
        if seg_2 == offset_pos {
            return true;
        }
    }
    return false;
}

fn determine_next_tailing_segment_pos(leading_segment: (i16, i16), tailing_segment: (i16, i16)) -> (i16, i16) {
    let total_offset = (leading_segment.0 - tailing_segment.0, leading_segment.1 - tailing_segment.1);
    let clamped_offset = (total_offset.0.clamp(-1, 1), total_offset.1.clamp(-1, 1));
    return (tailing_segment.0 + clamped_offset.0, tailing_segment.1 + clamped_offset.1);
}

fn draw_rope(rope: &Vec<(i16, i16)>) {
    let mut min_x = -3;
    let mut max_x = 3;
    let mut min_y = -3;
    let mut max_y = 3;
    rope.iter().for_each(|(x, y)| {
        if x < &min_x {
            min_x = *x;
        }
        if x > &max_x {
            max_x = *x;
        }
        if y < &min_y {
            min_y = *y;
        }
        if y > &max_y {
            max_y = *y;
        }
    });
    (min_y..(max_y + 1)).for_each(|y| {
        (min_x..(max_x + 1)).for_each(|x| {
            if rope.contains(&(x, y)) {
                print!("X");
            } else {
                print!(".");
            }
        });
        println!("");
    });
}
