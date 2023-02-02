use std::fs;

const DECRYPTION_KEY: i64 = 811589153;

fn main() {
    let path = "resources/input.txt";
    let contents = fs::read_to_string(path).expect("File not found");
    let lines = contents
        .split("\n")
        .map(|line| line.trim())
        .collect::<Vec<&str>>();

    // Part 1
    let sequence = lines
        .iter()
        .map(|line| Box::new(line.parse::<i64>().unwrap()))
        .collect::<Vec<Box<i64>>>();
    let mut new_sequence = sequence.iter().collect::<Vec<&Box<i64>>>();
    mix_sequence(&sequence, &mut new_sequence);
    let answer_p1 = calculate_sequence_answer(&new_sequence);
    println!("Code after mixing once: {}", answer_p1);

    // Part 2
    let mix_count = 10;
    let sequence = sequence
        .iter()
        .map(|x| Box::new(**x * DECRYPTION_KEY))
        .collect::<Vec<Box<i64>>>();
    let mut new_sequence = sequence.iter().collect::<Vec<&Box<i64>>>();
    for _ in 0..mix_count {
        mix_sequence(&sequence, &mut new_sequence);
    }
    let answer_p2 = calculate_sequence_answer(&new_sequence);
    println!("Decryption after {} mixings: {}", mix_count, answer_p2);
}

fn mix_sequence<'a>(original_sequence: &'a Vec<Box<i64>>, new_sequence: &mut Vec<&'a Box<i64>>) {
    let len = original_sequence.len() as i64;
    let mut i = 0;
    while i < len {
        let num = &original_sequence[i as usize];
        let num_position_in_new_sequence = new_sequence
            .iter()
            .position(|x| *x as *const _ == num)
            .unwrap();
        let new_position = (num_position_in_new_sequence as i64 + **num) % (len - 1);
        let new_position = if new_position <= 0 {
            (len - 1) + new_position
        } else {
            new_position
        };
        let new_position = new_position as usize;
        new_sequence.remove(num_position_in_new_sequence);
        new_sequence.insert(new_position, &num);
        i += 1;
    }
}

fn calculate_sequence_answer(sequence: &Vec<&Box<i64>>) -> i64 {
    let zero_pos = sequence.iter().position(|x| ***x == 0).unwrap();
    let answer = [1000, 2000, 3000]
        .iter()
        .map(|offset| (zero_pos + offset) % sequence.len() as usize)
        .map(|pos| **sequence[pos])
        .sum::<i64>();
    answer
}
