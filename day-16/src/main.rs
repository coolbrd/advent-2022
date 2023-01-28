use std::{fs, collections::{HashMap, HashSet}};

type ValveIdentifier = u8;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
struct ValveState {
    player_positions: Vec<ValveIdentifier>,
    open_valves: Vec<bool>,
    steps_left: u16,
    pressure_released: u16
}

impl ValveState {
    fn new(network: &ValveNetwork, steps_left: u16, player_count: usize) -> ValveState {
        ValveState {
            player_positions: vec![0; player_count],
            open_valves: vec![false; network.len()],
            steps_left,
            pressure_released: 0
        }
    }

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
        let paths_player_1 = self.get_shortest_paths_to_useful_valves(network, self.player_positions[0]);
        if paths_player_1.len() == 0 {
            let new_state = ValveState {
                player_positions: self.player_positions.clone(),
                open_valves: self.open_valves.clone(),
                steps_left: 0,
                pressure_released: self.pressure_released + self.get_step_pressure_released(network) * self.steps_left
            };
            successors.push(new_state);
            return successors;
        }
        for (valve_id_player_1, distance_player_1) in paths_player_1 {
            let mut new_open_valves = self.open_valves.clone();
            new_open_valves[valve_id_player_1 as usize] = true;
            let new_state = ValveState {
                player_positions: vec![valve_id_player_1],
                open_valves: new_open_valves,
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
        self.open_valves[valve_id as usize]
    }

    fn get_step_pressure_released(&self, network: &ValveNetwork) -> u16 {
        let mut sum = 0;
        for (valve_id, valve) in network.iter() {
            if self.valve_is_open(*valve_id) {
                sum += valve.flow;
            }
        }
        return sum;
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
    let mut valve_ids = named_valves.iter().collect::<Vec<(&String, &(u16, Vec<String>))>>();
    valve_ids.sort_by(|a, b| a.0.cmp(b.0));
    let valve_ids = valve_ids.iter().enumerate().map(move |(i, (name, _))| {
        ((*name).clone(), i as ValveIdentifier)
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
    let lines = contents.split("\n").map(|line| line.trim()).collect::<Vec<&str>>();
    let named_valves = lines.iter().map(|line| {
        let mut components = line.split(":").map(|s| s.to_owned()).collect::<Vec<String>>();
        let neighbor_names = components.pop().unwrap().split(",").map(|s| s.to_owned()).collect::<Vec<String>>();
        let flow = components.pop().unwrap().parse::<u16>().unwrap();
        let name = components.pop().unwrap();
        (name, (flow, neighbor_names))
    }).collect::<HashMap<String, (u16, Vec<String>)>>();
    let network = compute_valve_network(named_valves);

    // Part 1
    let initial_state = ValveState::new(&network, 30, 1);
    let releases = initial_state.get_possible_final_pressure_releases(&network, HashSet::new());
    let max_release = releases.iter().max().unwrap();
    println!("Max release after 30 minutes with one player: {}", max_release);
}
