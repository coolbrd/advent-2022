use std::fs;
use regex::Regex;

fn main() {
    let path = "resources/input.txt";
    let contents = fs::read_to_string(path).expect("File not found");
    let sections = contents.split("\n\n").collect::<Vec<&str>>();
    let cubes_transpose = sections[0].split("\n").collect::<Vec<&str>>().iter().rev().skip(1).map(|line| {
        line.chars().collect::<Vec<char>>().chunks(4).map(|spot| {
            let thing = spot[1];
            if thing == ' ' {
                None
            }
            else {
                Some(thing)
            }
        }).collect::<Vec<Option<char>>>()
    }).collect::<Vec<Vec<Option<char>>>>();
    let mut cubes: Vec<Vec<char>> = vec![Vec::new(); cubes_transpose[0].len()];
    cubes_transpose.iter().for_each(|row| {
        row.iter().enumerate().for_each(|(i, cube)| {
            if let Some(val) = cube {
                cubes[i].push(*val);
            }
        });
    });
    let steps = sections[1].split("\n").map(|line| {
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").expect("Invalid regex");
        let moves = re.captures(line).unwrap().iter().skip(1).map(|cap| {
            cap.unwrap().as_str().parse::<u16>().expect(&format!("Unparseable move number: {:?}", cap))
        }).collect::<Vec<u16>>();
        (moves[0], moves[1], moves[2])
    }).collect::<Vec<(u16, u16, u16)>>();
    let mut cubes_p1 = cubes.clone();
    steps.iter().for_each(|step| {
        (0..step.0).for_each(|_| {
            let src = (step.1 - 1) as usize;
            let dest = (step.2 - 1) as usize;
            let temp = cubes_p1[src].pop();
            if let Some(val) = temp {
                cubes_p1[dest].push(val);
            }
        });
    });
    let answer = cubes_p1.iter().map(|row| {
        *row.last().unwrap()
    }).collect::<String>();
    println!("Part 1 answer: {:?}", answer);

    // Part 2
    steps.iter().for_each(|step| {
        let num = step.0 as usize;
        let src = (step.1 - 1) as usize;
        let dest = (step.2 - 1) as usize;
        let src_stack = cubes[src].to_owned();
        let sections = src_stack.split_at(src_stack.len() - num);
        cubes[src] = sections.0.iter().map(|c| c.to_owned()).collect::<Vec<char>>();
        cubes[dest].append(&mut sections.1.iter().map(|c| c.to_owned()).collect::<Vec<char>>());
    });
    let answer = cubes.iter().map(|row| {
        *row.last().unwrap()
    }).collect::<String>();
    println!("Part 2 answer: {:?}", answer);
}
