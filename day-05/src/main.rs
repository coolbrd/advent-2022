use regex::Regex;
use std::fs;

fn main() {
    let path = "resources/input.txt";
    let contents = fs::read_to_string(path).expect("File not found");
    let input_sections = contents.split("\n\n").collect::<Vec<&str>>();
    let cubes_transpose = input_sections[0]
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .rev()
        .skip(1)
        .map(|line| {
            line.chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|spot| {
                    let potential_crate_letter = spot[1];
                    if potential_crate_letter == ' ' {
                        None
                    } else {
                        Some(potential_crate_letter)
                    }
                })
                .collect::<Vec<Option<char>>>()
        })
        .collect::<Vec<Vec<Option<char>>>>();
    let mut cubes: Vec<Vec<char>> = vec![Vec::new(); cubes_transpose[0].len()];
    cubes_transpose.iter().for_each(|row| {
        row.iter().enumerate().for_each(|(i, cube)| {
            if let Some(val) = cube {
                cubes[i].push(*val);
            }
        });
    });
    let steps = input_sections[1]
        .split("\n")
        .map(|line| {
            let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").expect("Invalid regex");
            let moves = re
                .captures(line)
                .unwrap()
                .iter()
                .skip(1)
                .map(|capture| {
                    capture
                        .unwrap()
                        .as_str()
                        .parse::<u16>()
                        .expect(&format!("Unparseable move number: {:?}", capture))
                })
                .collect::<Vec<u16>>();
            (moves[0], moves[1], moves[2])
        })
        .collect::<Vec<(u16, u16, u16)>>();

    // Part 1
    let mut cubes_p1 = cubes.clone();
    for step in &steps {
        for _ in 0..step.0 {
            let src = (step.1 - 1) as usize;
            let dest = (step.2 - 1) as usize;
            let temp = cubes_p1[src].pop();
            if let Some(val) = temp {
                cubes_p1[dest].push(val);
            }
        }
    }
    let answer_p1 = get_bottom_row(cubes_p1);
    println!("Part 1 answer: {:?}", answer_p1);

    // Part 2
    for step in &steps {
        let num = step.0 as usize;
        let src = (step.1 - 1) as usize;
        let dest = (step.2 - 1) as usize;
        let src_stack = cubes[src].to_owned();
        let sections = src_stack.split_at(src_stack.len() - num);
        cubes[src] = sections
            .0
            .iter()
            .map(|c| c.to_owned())
            .collect::<Vec<char>>();
        cubes[dest].append(
            &mut sections
                .1
                .iter()
                .map(|c| c.to_owned())
                .collect::<Vec<char>>(),
        );
    }
    let answer_p2 = get_bottom_row(cubes);
    println!("Part 2 answer: {:?}", answer_p2);
}

fn get_bottom_row(boxes: Vec<Vec<char>>) -> String {
    boxes.iter().map(|row| *row.last().unwrap()).collect()
}
