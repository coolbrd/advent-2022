use std::cmp;
use std::fs;

fn main() {
    let path = "resources/input.txt";
    let contents = fs::read_to_string(path).expect("File not found");
    let lines = contents.split("\n\n").collect::<Vec<&str>>();
    let elves: Vec<Vec<u32>> = lines
        .iter()
        .map(|cal_list_str| -> Vec<u32> {
            cal_list_str
                .split("\n")
                .collect::<Vec<&str>>()
                .iter()
                .map(|cal| cal.parse().expect("Unable to parse calorie input"))
                .collect()
        })
        .collect();
    let elf_totals: Vec<u32> = elves.iter().map(|elf| elf.iter().sum()).collect();

    // Part 1
    let mut max_val: Option<u32> = None;
    let mut max_idx: Option<usize> = None;
    elf_totals.iter().enumerate().for_each(|elf| -> () {
        let cur_elf_val = *elf.1;
        max_val = Some(cmp::max(cur_elf_val, max_val.unwrap_or(cur_elf_val)));
        max_idx = Some(elf.0);
    });
    let max_val = max_val.expect("No max value found. Elf list likely empty.");
    let max_idx = max_idx.expect("No max index found. Elf list likely empty.");
    println!(
        "Elf #{} has the highest calorie count, {}",
        max_idx, max_val
    );

    // Part 2
    let mut elf_totals = elf_totals.clone();
    elf_totals.sort();
    let three_highest = elf_totals
        .iter()
        .rev()
        .take(3)
        .copied()
        .collect::<Vec<u32>>();
    let three_highest_sum: u32 = three_highest.iter().sum();
    println!(
        "The three highest calorie counts are {:?}, totalling to {}",
        three_highest, three_highest_sum
    );
}
