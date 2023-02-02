use std::{
    fs,
    ops::{Add, Sub},
};

type ResourceValue = u16;

#[derive(Clone, Copy)]
struct ResourceAmount {
    ore: ResourceValue,
    clay: ResourceValue,
    obsidian: ResourceValue,
    geode: ResourceValue,
}

impl Add for ResourceAmount {
    type Output = ResourceAmount;

    fn add(self, other: ResourceAmount) -> ResourceAmount {
        ResourceAmount {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
            geode: self.geode + other.geode,
        }
    }
}

impl Sub for ResourceAmount {
    type Output = ResourceAmount;

    fn sub(self, other: ResourceAmount) -> ResourceAmount {
        ResourceAmount {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
            geode: self.geode - other.geode,
        }
    }
}

struct Blueprint {
    ore_robot_cost: ResourceAmount,
    clay_robot_cost: ResourceAmount,
    obsidian_robot_cost: ResourceAmount,
    geode_robot_cost: ResourceAmount,
}

#[derive(Clone)]
struct RobotState {
    time_left: u8,
    ore_robots: ResourceValue,
    clay_robots: ResourceValue,
    obsidian_robots: ResourceValue,
    geode_robots: ResourceValue,
    resources: ResourceAmount,
}

#[derive(Clone)]
enum RobotTypes {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl RobotState {
    fn new(time_left: u8) -> RobotState {
        RobotState {
            time_left,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            resources: ResourceAmount {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
        }
    }

    fn get_possible_final_geodes(&self, blueprint: &Blueprint) -> Vec<ResourceValue> {
        if self.is_done() {
            return vec![self.resources.geode];
        }
        let mut successors = self.get_successors(blueprint);
        let mut final_geode_counts = Vec::new();
        while !successors.is_empty() {
            let successor = successors.pop().unwrap();
            if successor.is_done() {
                final_geode_counts.push(successor.resources.geode);
            } else {
                let new_successors = successor.get_successors(blueprint);
                successors.extend(new_successors);
            }
        }
        final_geode_counts
    }

    fn is_done(&self) -> bool {
        self.time_left == 0
    }

    fn get_successors(&self, blueprint: &Blueprint) -> Vec<RobotState> {
        let mut successors = Vec::new();
        if self.is_done() {
            return successors;
        }
        let buildable_robot_types = self.get_useful_buildable_robot_types(blueprint);
        for robot_type in &buildable_robot_types {
            match robot_type {
                RobotTypes::Ore => successors.push(self.get_build_ore_robot_successor(blueprint)),
                RobotTypes::Clay => successors.push(self.get_build_clay_robot_successor(blueprint)),
                RobotTypes::Obsidian => successors.push(self.get_build_obsidian_robot_successor(blueprint)),
                RobotTypes::Geode => successors.push(self.get_build_geode_robot_successor(blueprint)),
            }
        }
        let mut current_do_nothing_successor = self.get_do_nothing_successor();
        let mut current_buildable_robot_types =
            current_do_nothing_successor.get_useful_buildable_robot_types(blueprint);
        while current_buildable_robot_types.len() == buildable_robot_types.len()
            && !current_do_nothing_successor.is_done()
        {
            current_do_nothing_successor = current_do_nothing_successor.get_do_nothing_successor();
            current_buildable_robot_types =
                current_do_nothing_successor.get_useful_buildable_robot_types(blueprint);
        }
        successors.push(current_do_nothing_successor);
        return successors;
    }

    fn get_useful_buildable_robot_types(&self, blueprint: &Blueprint) -> Vec<RobotTypes> {
        let mut need_more_ore_robots =
            self.get_current_ore_output() < blueprint.geode_robot_cost.ore;
        let mut need_more_clay_robots =
            self.get_current_clay_output() < blueprint.geode_robot_cost.clay;
        let need_more_obsidian_robots =
            self.get_current_obsidian_output() < blueprint.geode_robot_cost.obsidian;
        if need_more_obsidian_robots {
            need_more_ore_robots =
                self.get_current_ore_output() < blueprint.obsidian_robot_cost.ore;
            need_more_clay_robots =
                self.get_current_clay_output() < blueprint.obsidian_robot_cost.clay;
            if need_more_clay_robots {
                need_more_ore_robots =
                    self.get_current_ore_output() < blueprint.clay_robot_cost.ore;
            }
        }
        let robot_types = self.get_buildable_robot_types(blueprint);
        robot_types
            .iter()
            .filter(|robot_type| match robot_type {
                RobotTypes::Ore => need_more_ore_robots,
                RobotTypes::Clay => need_more_clay_robots,
                RobotTypes::Obsidian => need_more_obsidian_robots,
                RobotTypes::Geode => true,
            })
            .cloned()
            .collect()
    }

    fn get_buildable_robot_types(&self, blueprint: &Blueprint) -> Vec<RobotTypes> {
        let mut robot_types = Vec::new();
        if self.can_build_ore_robot(&blueprint.ore_robot_cost) {
            robot_types.push(RobotTypes::Ore);
        }
        if self.can_build_clay_robot(&blueprint.clay_robot_cost) {
            robot_types.push(RobotTypes::Clay);
        }
        if self.can_build_obsidian_robot(&blueprint.obsidian_robot_cost) {
            robot_types.push(RobotTypes::Obsidian);
        }
        if self.can_build_geode_robot(&blueprint.geode_robot_cost) {
            robot_types.push(RobotTypes::Geode);
        }
        robot_types
    }

    fn get_do_nothing_successor(&self) -> RobotState {
        RobotState {
            time_left: self.time_left - 1,
            ore_robots: self.ore_robots,
            clay_robots: self.clay_robots,
            obsidian_robots: self.obsidian_robots,
            geode_robots: self.geode_robots,
            resources: self.resources + self.get_current_resource_output(),
        }
    }

    fn get_build_ore_robot_successor(&self, blueprint: &Blueprint) -> RobotState {
        RobotState {
            time_left: self.time_left - 1,
            ore_robots: self.ore_robots + 1,
            clay_robots: self.clay_robots,
            obsidian_robots: self.obsidian_robots,
            geode_robots: self.geode_robots,
            resources: self.resources + self.get_current_resource_output()
                - blueprint.ore_robot_cost,
        }
    }

    fn get_build_clay_robot_successor(&self, blueprint: &Blueprint) -> RobotState {
        RobotState {
            time_left: self.time_left - 1,
            ore_robots: self.ore_robots,
            clay_robots: self.clay_robots + 1,
            obsidian_robots: self.obsidian_robots,
            geode_robots: self.geode_robots,
            resources: self.resources + self.get_current_resource_output()
                - blueprint.clay_robot_cost,
        }
    }

    fn get_build_obsidian_robot_successor(&self, blueprint: &Blueprint) -> RobotState {
        RobotState {
            time_left: self.time_left - 1,
            ore_robots: self.ore_robots,
            clay_robots: self.clay_robots,
            obsidian_robots: self.obsidian_robots + 1,
            geode_robots: self.geode_robots,
            resources: self.resources + self.get_current_resource_output()
                - blueprint.obsidian_robot_cost,
        }
    }

    fn get_build_geode_robot_successor(&self, blueprint: &Blueprint) -> RobotState {
        RobotState {
            time_left: self.time_left - 1,
            ore_robots: self.ore_robots,
            clay_robots: self.clay_robots,
            obsidian_robots: self.obsidian_robots,
            geode_robots: self.geode_robots + 1,
            resources: self.resources + self.get_current_resource_output()
                - blueprint.geode_robot_cost,
        }
    }

    fn can_build_ore_robot(&self, cost: &ResourceAmount) -> bool {
        self.resources.ore >= cost.ore
    }

    fn can_build_clay_robot(&self, cost: &ResourceAmount) -> bool {
        self.resources.ore >= cost.ore
    }

    fn can_build_obsidian_robot(&self, cost: &ResourceAmount) -> bool {
        self.resources.ore >= cost.ore && self.resources.clay >= cost.clay
    }

    fn can_build_geode_robot(&self, cost: &ResourceAmount) -> bool {
        self.resources.ore >= cost.ore && self.resources.obsidian >= cost.obsidian
    }

    fn get_current_ore_output(&self) -> ResourceValue {
        self.ore_robots
    }

    fn get_current_clay_output(&self) -> ResourceValue {
        self.clay_robots
    }

    fn get_current_obsidian_output(&self) -> ResourceValue {
        self.obsidian_robots
    }

    fn get_current_geode_output(&self) -> ResourceValue {
        self.geode_robots
    }

    fn get_current_resource_output(&self) -> ResourceAmount {
        ResourceAmount {
            ore: self.get_current_ore_output(),
            clay: self.get_current_clay_output(),
            obsidian: self.get_current_obsidian_output(),
            geode: self.get_current_geode_output(),
        }
    }
}

fn main() {
    let path = "resources/input.txt";
    let contents = fs::read_to_string(path).expect("File not found");
    let lines = contents
        .split("\n")
        .map(|line| line.trim())
        .collect::<Vec<&str>>();
    let blueprints = lines
        .iter()
        .map(|line| {
            let costs = line
                .split(":")
                .map(|robot_cost| {
                    robot_cost
                        .split(",")
                        .map(|cost_component| cost_component.parse().unwrap())
                        .collect()
                })
                .collect::<Vec<Vec<ResourceValue>>>();
            let ore_robot_cost = ResourceAmount {
                ore: costs[0][0],
                clay: 0,
                obsidian: 0,
                geode: 0,
            };
            let clay_robot_cost = ResourceAmount {
                ore: costs[1][0],
                clay: 0,
                obsidian: 0,
                geode: 0,
            };
            let obsidian_robot_cost = ResourceAmount {
                ore: costs[2][0],
                clay: costs[2][1],
                obsidian: 0,
                geode: 0,
            };
            let geode_robot_cost = ResourceAmount {
                ore: costs[3][0],
                clay: 0,
                obsidian: costs[3][1],
                geode: 0,
            };
            Blueprint {
                ore_robot_cost,
                clay_robot_cost,
                obsidian_robot_cost,
                geode_robot_cost,
            }
        })
        .collect::<Vec<Blueprint>>();

    // Part 1
    let initial_state = RobotState::new(24);
    let mut total_quality = 0;
    for (i, blueprint) in blueprints.iter().enumerate() {
        let blueprint_number = i + 1;
        let final_geode_counts = initial_state.get_possible_final_geodes(blueprint);
        let max_geode_count = *final_geode_counts.iter().max().unwrap();
        let quality = max_geode_count as usize * blueprint_number;
        total_quality += quality;
    }
    println!("Total quality of all blueprints: {}", total_quality);

    // Part 2
    let initial_state = RobotState::new(32);
    let mut highest_geode_counts = vec![];
    for blueprint in blueprints[0..=2].iter() {
        let final_geode_counts = initial_state.get_possible_final_geodes(blueprint);
        let max_geode_count = final_geode_counts.iter().max().unwrap();
        highest_geode_counts.push(*max_geode_count as usize);
    }
    let geode_count_product = highest_geode_counts.iter().product::<usize>();
    println!("Product of first three blueprints: {}", geode_count_product);
}
