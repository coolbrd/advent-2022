use std::fs;

fn main() {
    let path = "resources/input.txt";
    let contents = fs::read_to_string(path).expect("File not found");
    let lines = contents.split("\n").collect::<Vec<&str>>();
    let ranges = lines
        .iter()
        .map(|line| {
            let pair = line
                .split(",")
                .map(|range| {
                    let nums = range
                        .split("-")
                        .map(|num| {
                            num.parse::<u16>()
                                .expect(&format!("Encountered non-numeric range bound: {}", num))
                        })
                        .collect::<Vec<u16>>();
                    (nums[0], nums[1])
                })
                .collect::<Vec<(u16, u16)>>();
            (pair[0], pair[1])
        })
        .collect::<Vec<((u16, u16), (u16, u16))>>();
    
    // Part 1
    let complete_overlaps = ranges
        .iter()
        .filter(|range| {
            (range.0.0 >= range.1.0 && range.0.1 <= range.1.1)
                || (range.1.0 >= range.0.0 && range.1.1 <= range.0.1)
        })
        .collect::<Vec<&((u16, u16), (u16, u16))>>();
    println!("Number of complete overlaps: {:?}", complete_overlaps.len());

    // Part 2
    let overlaps = ranges
        .iter()
        .filter(|range| {
            (range.0.0 >= range.1.0 && range.0.0 <= range.1.1)
                || (range.0.1 >= range.1.0 && range.0.1 <= range.1.1)
                || (range.1.0 >= range.0.0 && range.1.0 <= range.0.1)
                || (range.1.1 >= range.0.0 && range.1.1 <= range.0.1)
        })
        .collect::<Vec<&((u16, u16), (u16, u16))>>();
    println!("Number of partial overlaps: {:?}", overlaps.len());
}
