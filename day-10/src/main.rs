use std::{fs, collections::HashSet};

enum CPUOp {
    NOP,
    AddX(i32)
}

fn main() {
    let path = "resources/input.txt";
    let contents = fs::read_to_string(path).expect("File not found");
    let lines = contents.split("\n").collect::<Vec<&str>>();
    let instructions = lines.iter().map(|line| {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let op = parts[0].trim();
        match op {
            "noop" => CPUOp::NOP,
            "addx" => {
                let arg = parts[1].trim().parse::<i32>().unwrap();
                CPUOp::AddX(arg)
            }
            _ => panic!("Unknown op: {}", op)
        }
    }).collect::<Vec<CPUOp>>();
    let mut x_cycles = vec![1_i32];
    instructions.iter().for_each(|instr| {
        let cur_x = *x_cycles.last().unwrap();
        match instr {
            CPUOp::NOP => x_cycles.push(cur_x),
            CPUOp::AddX(arg) => {
                x_cycles.push(cur_x);
                x_cycles.push(cur_x + arg);
            }
        }
    });

    // Part 1
    let cycles_of_interest = vec![20, 60, 100, 140, 180, 220];
    let part_1_sum = cycles_of_interest.iter().map(|cycle| {
        let cur_x = x_cycles[cycle - 1];
        *cycle as i32 * cur_x
    }).sum::<i32>();
    println!("Part 1: {}", part_1_sum);

    // Part 2
    let screen_dims = (40, 6);
    let mut cycle = 0;
    (0..screen_dims.1).for_each(|y| {
        (0..screen_dims.0).for_each(|_| {
            let sprite_center = x_cycles[cycle] + y * screen_dims.0;
            let lit_pixels: HashSet<i32> = HashSet::from_iter(vec![sprite_center - 1, sprite_center, sprite_center + 1]);
            if lit_pixels.contains(&(cycle as i32)) {
                print!("#");
            } else {
                print!(".");
            }
            cycle += 1;
        });
        println!("");
    });
}
