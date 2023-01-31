use std::{collections::HashSet, fs};

fn main() {
    let path = "resources/input.txt";
    let stream = fs::read_to_string(path).expect("File not found");

    // Part 1
    let packet_start = find_first_unique_marker(&stream, 4).unwrap();
    println!("Start of packet at {}", packet_start);

    // Part 2
    let message_start = find_first_unique_marker(&stream, 14).unwrap();
    println!("Start of message at {}", message_start);
}

fn find_first_unique_marker(stream: &String, length: usize) -> Option<usize> {
    for (i, segment) in stream
        .chars()
        .collect::<Vec<char>>()
        .windows(length)
        .enumerate()
    {
        let char_set = segment.iter().collect::<HashSet<&char>>();
        if char_set.len() == length {
            return Some(i + length);
        }
    }
    return None;
}
