use std::{collections::HashSet, fs};

const FACE_OFFSETS: [Pos; 6] = [
    (-1, 0, 0),
    (1, 0, 0),
    (0, -1, 0),
    (0, 1, 0),
    (0, 0, -1),
    (0, 0, 1),
];

type PosComp = i8;

type Pos = (PosComp, PosComp, PosComp);

fn main() {
    let path = "resources/input.txt";
    let contents = fs::read_to_string(path).expect("File not found");
    let lines = contents
        .split("\n")
        .map(|line| line.trim())
        .collect::<Vec<&str>>();
    let cubes = lines
        .iter()
        .map(|line| {
            let components = line
                .split(",")
                .map(|comp| comp.parse().unwrap())
                .collect::<Vec<i8>>();
            (components[0], components[1], components[2])
        })
        .collect::<Vec<Pos>>();
    let cubes = cubes.into_iter().collect();

    // Part 1
    let total_exposed_faces = calculate_total_exposed_faces(&cubes);
    println!("Total exposed faces: {}", total_exposed_faces);

    // Part 2
    let outside_air_cubes = calculate_outside_air_cubes(&cubes);
    let outside_exposed_faces = calculate_faces_exposed_to_outside_air(&cubes, &outside_air_cubes);
    println!("Outside exposed faces: {}", outside_exposed_faces);
}

fn calculate_total_exposed_faces(cubes: &HashSet<Pos>) -> usize {
    let mut total_exposed_faces = 0;
    for cube in cubes {
        let mut exposed_faces = 0;
        for offset in &FACE_OFFSETS {
            let offset_pos = (cube.0 + offset.0, cube.1 + offset.1, cube.2 + offset.2);
            if !cubes.contains(&offset_pos) {
                exposed_faces += 1;
            }
        }
        total_exposed_faces += exposed_faces;
    }
    total_exposed_faces
}

fn calculate_outside_air_cubes(cubes: &HashSet<Pos>) -> HashSet<Pos> {
    let mut min_xyz = (PosComp::MAX, PosComp::MAX, PosComp::MAX);
    let mut max_xyz = (PosComp::MIN, PosComp::MIN, PosComp::MIN);
    for cube in cubes {
        min_xyz.0 = min_xyz.0.min(cube.0 - 1);
        min_xyz.1 = min_xyz.1.min(cube.1 - 1);
        min_xyz.2 = min_xyz.2.min(cube.2 - 1);
        max_xyz.0 = max_xyz.0.max(cube.0 + 1);
        max_xyz.1 = max_xyz.1.max(cube.1 + 1);
        max_xyz.2 = max_xyz.2.max(cube.2 + 1);
    }
    let mut air_cube_queue = vec![min_xyz];
    let mut outside_air_cubes = HashSet::new();
    while !air_cube_queue.is_empty() {
        let current_air_cube = air_cube_queue.pop().unwrap();
        let adjacent_air_cubes = FACE_OFFSETS
            .iter()
            .map(|offset| {
                (
                    current_air_cube.0 + offset.0,
                    current_air_cube.1 + offset.1,
                    current_air_cube.2 + offset.2,
                )
            })
            .filter(|adjacent_cube| {
                !&cubes.contains(adjacent_cube)
                    && !outside_air_cubes.contains(adjacent_cube)
                    && !point_is_outside_bounds(adjacent_cube, &min_xyz, &max_xyz)
            })
            .collect::<Vec<Pos>>();
        air_cube_queue.extend(adjacent_air_cubes);
        outside_air_cubes.insert(current_air_cube);
    }
    outside_air_cubes
}

fn calculate_faces_exposed_to_outside_air(
    cubes: &HashSet<Pos>,
    outside_air_cubes: &HashSet<Pos>,
) -> usize {
    let mut total_exposed_faces = 0;
    for cube in cubes {
        let mut exposed_faces = 0;
        for offset in &FACE_OFFSETS {
            if outside_air_cubes.contains(&(
                cube.0 + offset.0,
                cube.1 + offset.1,
                cube.2 + offset.2,
            )) {
                exposed_faces += 1;
            }
        }
        total_exposed_faces += exposed_faces;
    }
    total_exposed_faces
}

fn point_is_outside_bounds(point: &Pos, min_xyz: &Pos, max_xyz: &Pos) -> bool {
    point.0 < min_xyz.0
        || point.0 > max_xyz.0
        || point.1 < min_xyz.1
        || point.1 > max_xyz.1
        || point.2 < min_xyz.2
        || point.2 > max_xyz.2
}
