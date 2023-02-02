use std::{collections::HashSet, fs, ops::Sub};

type PosComp = i64;

type Pos = (PosComp, PosComp);

type SensorBeaconPair = (Pos, Pos);

fn main() {
    let path = "resources/input.txt";
    let contents = fs::read_to_string(path).expect("File not found");
    let lines = contents
        .split("\n")
        .map(|line| line.trim())
        .collect::<Vec<&str>>();
    let sensor_beacon_pairs = lines
        .iter()
        .map(|line| {
            let pairs = line
                .split(":")
                .map(|pair| {
                    let nums = pair
                        .trim()
                        .split(",")
                        .map(|num| num.parse().expect(&format!("{} is not a number", num)))
                        .collect::<Vec<PosComp>>();
                    (nums[0], nums[1])
                })
                .collect::<Vec<Pos>>();
            (pairs[0], pairs[1])
        })
        .collect::<Vec<SensorBeaconPair>>();

    // Part 1
    let target_row = 2000000;
    let ranges = get_beacon_exclusion_ranges(&sensor_beacon_pairs, target_row);
    let no_beacon_spaces = ranges
        .iter()
        .map(|range| range.1 - range.0)
        .sum::<PosComp>();
    println!(
        "Number of spaces where a beacon cannot be: {}",
        no_beacon_spaces
    );

    // Part 2
    let min_dim = 0;
    let max_dim = 4000000;
    for y in min_dim..=max_dim {
        let ranges = get_beacon_exclusion_ranges(&sensor_beacon_pairs, y);
        let no_beacon_spaces = ranges
            .iter()
            .map(|range| range.1.min(max_dim) - range.0.max(min_dim))
            .sum::<PosComp>();
        if no_beacon_spaces < max_dim - 1 {
            let mut total_range: HashSet<PosComp> = HashSet::from_iter(min_dim..=max_dim);
            for range in ranges {
                let new_range = HashSet::from_iter(range.0..=range.1);
                total_range = total_range.sub(&new_range);
            }
            let x = *total_range.iter().collect::<Vec<&PosComp>>()[0];
            let tuning_freq = x * 4000000 + y;
            println!("Tuning frequency: {}", tuning_freq);
            break;
        }
    }
}

fn get_beacon_exclusion_ranges(
    sensor_beacon_pairs: &Vec<SensorBeaconPair>,
    target_row: PosComp,
) -> HashSet<Pos> {
    let mut row_ranges = vec![];
    for pair in sensor_beacon_pairs {
        let manhattan_distance = ((pair.0).0 - (pair.1).0).abs() + ((pair.0).1 - (pair.1).1).abs();
        let distance_to_row = (target_row - (pair.0).1).abs();
        if manhattan_distance >= distance_to_row {
            let lateral_distance = manhattan_distance - distance_to_row;
            let row_range = (pair.0 .0 - lateral_distance, pair.0 .0 + lateral_distance);
            row_ranges.push(row_range);
        }
    }
    let mut master_ranges = HashSet::new();
    for row_range in row_ranges {
        master_ranges = insert_into_bounds_set(master_ranges, row_range);
    }
    return master_ranges;
}

fn insert_into_bounds_set(bounds: HashSet<Pos>, item: Pos) -> HashSet<Pos> {
    let mut new_bounds = HashSet::new();
    if bounds.is_empty() {
        new_bounds.insert(item);
        return new_bounds;
    }
    let mut cur_bound = item;
    for bound in bounds.iter() {
        if bound.0 <= cur_bound.0 && cur_bound.1 <= bound.1 {
            return bounds;
        } else if cur_bound.0 <= bound.0 && bound.1 <= cur_bound.1 {
            continue;
        } else if bound.0 <= cur_bound.0 && cur_bound.0 <= bound.1 {
            let new_bound = (bound.0, cur_bound.1);
            cur_bound = new_bound;
        } else if bound.0 <= cur_bound.1 && cur_bound.1 <= bound.1 {
            let new_bound = (cur_bound.0, bound.1);
            cur_bound = new_bound;
        } else {
            new_bounds.insert(*bound);
        }
    }
    new_bounds.insert(cur_bound);
    return new_bounds;
}
