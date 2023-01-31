use std::{collections::HashSet, fs};

type CPUVal = i32;

enum CPUOp {
    NOP,
    AddX(CPUVal),
}

fn main() {
    let path = "resources/input.txt";
    let contents = fs::read_to_string(path).expect("File not found");
    let lines = contents.split("\n").collect::<Vec<&str>>();
    let instructions = lines
        .iter()
        .map(|line| {
            let parts = line.split(" ").collect::<Vec<&str>>();
            let op = parts[0].trim();
            match op {
                "noop" => CPUOp::NOP,
                "addx" => {
                    let arg = parts[1]
                        .trim()
                        .parse::<CPUVal>()
                        .expect(&format!("Invalid arg for addx: {}", parts[1]));
                    CPUOp::AddX(arg)
                }
                _ => panic!("Unknown op: {}", op),
            }
        })
        .collect::<Vec<CPUOp>>();
    let mut x_cycles = vec![1];
    for instruction in instructions {
        let cur_x = *x_cycles.last().unwrap();
        match instruction {
            CPUOp::NOP => x_cycles.push(cur_x),
            CPUOp::AddX(arg) => {
                x_cycles.push(cur_x);
                x_cycles.push(cur_x + arg);
            }
        }
    }

    // Part 1
    let cycles_of_interest = [20, 60, 100, 140, 180, 220];
    let part_1_sum = calculate_signal_strengths_sum(&x_cycles, &cycles_of_interest);
    println!("Part 1: {}", part_1_sum);

    // Part 2
    let screen_dimensions = (40, 6);
    render_screen(screen_dimensions, &x_cycles);
}

fn calculate_signal_strengths_sum(x_cycles: &Vec<CPUVal>, cycles_of_interest: &[usize]) -> CPUVal {
    cycles_of_interest
        .iter()
        .map(|&cycle| {
            let cur_x = x_cycles[cycle - 1];
            cycle as CPUVal * cur_x
        })
        .sum::<CPUVal>()
}

fn render_screen(screen_dimensions: (usize, usize), x_cycles: &Vec<CPUVal>) {
    let mut cycle = 0;
    for y in 0..screen_dimensions.1 {
        for _ in 0..screen_dimensions.0 {
            let sprite_center = x_cycles[cycle] + (y * screen_dimensions.0) as CPUVal;
            let lit_pixels: HashSet<CPUVal> =
                HashSet::from_iter([sprite_center - 1, sprite_center, sprite_center + 1]);
            if lit_pixels.contains(&(cycle as CPUVal)) {
                print!("#");
            } else {
                print!(".");
            }
            cycle += 1;
        }
        println!("");
    }
}
