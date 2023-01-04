use std::{fs, collections::HashSet};

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
    for i in length..stream.len() {
        let segment = &stream[(i - length)..i];
        let char_set: HashSet<char> = HashSet::from_iter(segment.chars().into_iter());
        if char_set.len() == length {
            return Some(i);
        }
    }
    return None;
}
