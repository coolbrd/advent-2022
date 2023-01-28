use std::{fs, collections::{HashMap, HashSet}};

type ValveIdentifier = u8;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
struct ValveState {
    player_positions: Vec<ValveIdentifier>,
    open_valve_names: Vec<ValveIdentifier>,
    steps_left: u16,
    pressure_released: u16
}

impl ValveState {
    fn get_possible_final_pressure_releases(&self, network: &ValveNetwork, mut seen: HashSet<ValveState>) -> Vec<u16> {
        if self.is_done() {
            return vec![self.pressure_released];
        }
        let mut successors = self.get_successors(network);
        let mut final_pressure_releases = Vec::new();
        while !successors.is_empty() {
            let successor = successors.pop().unwrap();
            if seen.contains(&successor) { continue }
            if successor.is_done() {
                final_pressure_releases.push(successor.pressure_released);
            }
            let new_successors = successor.get_successors(network);
            successors.extend(new_successors);
            seen.insert(successor);
        }
        return final_pressure_releases;
    }

    fn get_successors(&self, network: &ValveNetwork) -> Vec<ValveState> {
        let mut successors = Vec::new();
        if self.is_done() { return successors }
        let num_useful_valves = self.get_num_useful_valves(network);
        if num_useful_valves == 0 { 
            let new_state = ValveState {
                player_positions: self.player_positions.clone(),
                open_valve_names: self.open_valve_names.clone(),
                steps_left: 0,
                pressure_released: self.pressure_released + self.get_step_pressure_released(network) * self.steps_left
            };
            successors.push(new_state);
            return successors;
        }
        let paths_player_1 = self.get_shortest_paths_to_useful_valves(network, self.player_positions[0]);
        for (valve_name_player_1, distance_player_1) in paths_player_1 {
        let mut new_open_valve_names = self.open_valve_names.clone();
        new_open_valve_names.push(valve_name_player_1);
        let new_state = ValveState {
            player_positions: vec![valve_name_player_1],
            open_valve_names: new_open_valve_names,
            steps_left: self.steps_left - (distance_player_1 + 1),
            pressure_released: self.pressure_released + self.get_step_pressure_released(network) * (distance_player_1 + 1)
        };
        successors.push(new_state);
        }
        successors
    }

    fn is_done(&self) -> bool {
        self.steps_left == 0
    }

    fn valve_is_open(&self, valve_id: ValveIdentifier) -> bool {
        self.open_valve_names.contains(&valve_id)
    }

    fn get_step_pressure_released(&self, network: &ValveNetwork) -> u16 {
        self.open_valve_names.iter().map(|valve_id| {
            network.get_valve(*valve_id).flow
        }).sum()
    }

    fn get_num_useful_valves(&self, network: &ValveNetwork) -> u16 {
        network.iter().filter(|(valve_id, valve)| {
            !self.valve_is_open(**valve_id) && valve.flow > 0
        }).count() as u16
    }

    fn get_shortest_paths_to_useful_valves(&self, network: &ValveNetwork, position: ValveIdentifier) -> HashMap<ValveIdentifier, u16> {
        let current_valve = network.get(&position).unwrap();
        current_valve.neighbors.iter().filter(|(valve, distance)| {
            (**distance + 1) <= self.steps_left && !self.valve_is_open(**valve) && network.get_valve(**valve).flow > 0
        }).map(|(valve, distance)| {
            (*valve, *distance)
        }).collect()
    }
}

type ValveNetwork = HashMap<ValveIdentifier, Valve>;

trait HasValve {
    fn get_valve(&self, valve_id: ValveIdentifier) -> &Valve;
}

impl HasValve for ValveNetwork {
    fn get_valve(&self, valve_id: ValveIdentifier) -> &Valve {
        self.get(&valve_id).unwrap()
    }
}

fn compute_valve_network(named_valves: HashMap<String, (u16, Vec<String>)>) -> ValveNetwork {
    let mut network: ValveNetwork = HashMap::new();
    let valve_ids = named_valves.iter().enumerate().map(|(i, (name, _))| {
        (name.to_owned(), i as ValveIdentifier)
    }).collect::<HashMap<String, ValveIdentifier>>();
    for (current_valve_name, (current_valve_flow, _)) in &named_valves {
        let current_valve_id = valve_ids.get(current_valve_name).unwrap();
        let mut paths = HashMap::new();
        let mut queue = Vec::new();
        queue.push((current_valve_name.to_owned(), 0));
        while !queue.is_empty() {
            let (valve_name, distance) = queue.pop().unwrap();
            let valve_id = valve_ids.get(&valve_name).unwrap();
            if paths.contains_key(valve_id) { 
                if *paths.get(valve_id).unwrap() <= distance {
                    continue;
                }
            }
            paths.insert(*valve_id, distance);
            let valve = named_valves.get(&valve_name).unwrap();
            for neighbor_name in &valve.1 {
                queue.push((neighbor_name.to_owned(), distance + 1));
            }
        }
        let new_valve = Valve {
            flow: *current_valve_flow,
            neighbors: paths
        };
        network.insert(*current_valve_id, new_valve);
    }
    return network;
}

#[derive(Debug)]
struct Valve {
    flow: u16,
    neighbors: HashMap<ValveIdentifier, u16>
}

fn main() {
    let path = "resources/input2.txt";
    let contents = fs::read_to_string(path).expect("File not found");
    let mut lines = contents.split("\n").map(|line| line.trim()).collect::<Vec<&str>>();
    lines.sort();
    let named_valves = lines.iter().map(|line| {
        let mut components = line.split(":").map(|s| s.to_owned()).collect::<Vec<String>>();
        let neighbor_names = components.pop().unwrap().split(",").map(|s| s.to_owned()).collect::<Vec<String>>();
        let flow = components.pop().unwrap().parse::<u16>().unwrap();
        let name = components.pop().unwrap();
        (name, (flow, neighbor_names))
    }).collect::<HashMap<String, (u16, Vec<String>)>>();
    let network = compute_valve_network(named_valves);

    // Part 1
    let initial_state = ValveState {
        player_positions: vec![0],
        open_valve_names: vec![],
        steps_left: 30,
        pressure_released: 0
    };
    let releases = initial_state.get_possible_final_pressure_releases(&network, HashSet::new());
    let max_release = releases.iter().max().unwrap();
    println!("Max release: {}", max_release);
}