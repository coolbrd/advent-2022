use std::{fs, collections::HashSet};

const FACE_OFFSETS: [(i8, i8, i8); 6] = [(-1, 0, 0), (1, 0, 0), (0, -1, 0), (0, 1, 0), (0, 0, -1), (0, 0, 1)];

fn main() {
    let path = "resources/input.txt";
    let contents = fs::read_to_string(path).expect("File not found");
    let lines = contents.split("\n").map(|line| line.trim()).collect::<Vec<&str>>();
    let cubes = lines.iter().map(|line| {
        let components = line.split(",").map(|comp| comp.parse().unwrap()).collect::<Vec<i8>>();
        (components[0], components[1], components[2])
    }).collect::<Vec<(i8, i8, i8)>>();
    let cubes: HashSet<(i8, i8, i8)> = cubes.into_iter().collect();

    // Part 1
    let total_exposed_faces = calculate_total_exposed_faces(&cubes);
    println!("Total exposed faces: {}", total_exposed_faces);

    // Part 2
    let outside_air_cubes = calculate_outside_air_cubes(&cubes);
    let outside_exposed_faces = calculate_faces_exposed_to_outside_air(&cubes, &outside_air_cubes);
    println!("Outside exposed faces: {}", outside_exposed_faces);
}

fn calculate_total_exposed_faces(cubes: &HashSet<(i8, i8, i8)>) -> usize {
    let mut total_exposed_faces = 0;
    for cube in cubes {
        let mut exposed_faces = 0;
        for offset in &FACE_OFFSETS {
            if !cubes.contains(&(cube.0 + offset.0, cube.1 + offset.1, cube.2 + offset.2)) {
                exposed_faces += 1;
            }
        }
        total_exposed_faces += exposed_faces;
    }
    total_exposed_faces
}

fn calculate_outside_air_cubes(cubes: &HashSet<(i8, i8, i8)>) -> HashSet<(i8, i8, i8)> {
    let mut min_xyz = (i8::MAX, i8::MAX, i8::MAX);
    let mut max_xyz = (i8::MIN, i8::MIN, i8::MIN);
    for cube in cubes {
        min_xyz.0 = min_xyz.0.min(cube.0 - 1);
        min_xyz.1 = min_xyz.1.min(cube.1 - 1);
        min_xyz.2 = min_xyz.2.min(cube.2 - 1);
        max_xyz.0 = max_xyz.0.max(cube.0 + 1);
        max_xyz.1 = max_xyz.1.max(cube.1 + 1);
        max_xyz.2 = max_xyz.2.max(cube.2 + 1);
    }
    let mut air_cube_queue = vec![min_xyz];
    let mut outside_air_cubes: HashSet<(i8, i8, i8)> = HashSet::new();
    while !air_cube_queue.is_empty() {
        let current_air_cube = air_cube_queue.pop().unwrap();
        let adjacent_air_cubes = FACE_OFFSETS.iter().map(|offset| {
            (current_air_cube.0 + offset.0, current_air_cube.1 + offset.1, current_air_cube.2 + offset.2)
        }).filter(|adjacent_cube| {
            !&cubes.contains(adjacent_cube) && !outside_air_cubes.contains(adjacent_cube) && !point_is_outside_bounds(adjacent_cube, &min_xyz, &max_xyz)
        }).collect::<Vec<(i8, i8, i8)>>();
        air_cube_queue.extend(adjacent_air_cubes);
        outside_air_cubes.insert(current_air_cube);
    }
    outside_air_cubes
}

fn calculate_faces_exposed_to_outside_air(cubes: &HashSet<(i8, i8, i8)>, outside_air_cubes: &HashSet<(i8, i8, i8)>) -> usize {
    let mut total_exposed_faces = 0;
    for cube in cubes {
        let mut exposed_faces = 0;
        for offset in &FACE_OFFSETS {
            if outside_air_cubes.contains(&(cube.0 + offset.0, cube.1 + offset.1, cube.2 + offset.2)) {
                exposed_faces += 1;
            }
        }
        total_exposed_faces += exposed_faces;
    }
    total_exposed_faces
}

fn point_is_outside_bounds(point: &(i8, i8, i8), min_xyz: &(i8, i8, i8), max_xyz: &(i8, i8, i8)) -> bool {
    point.0 < min_xyz.0 || point.0 > max_xyz.0 || point.1 < min_xyz.1 || point.1 > max_xyz.1 || point.2 < min_xyz.2 || point.2 > max_xyz.2
}