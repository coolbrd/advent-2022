use std::{fs, collections::{HashMap, HashSet}};

type ValveIdentifier = u8;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ValveState {
    player_transits: (Option<PlayerTransit>, Option<PlayerTransit>),
    open_valves: Vec<bool>,
    steps_left: u16,
    pressure_released: u16
}

type PlayerTransit = (ValveIdentifier, u16);

impl ValveState {
    fn new(network: &ValveNetwork, steps_left: u16, is_two_player: bool) -> ValveState {
        ValveState {
            player_transits: (Some((0, 0)), if is_two_player { Some((0, 0)) } else { None }),
            open_valves: vec![false; network.len()],
            steps_left,
            pressure_released: 0
        }
    }

    fn get_max_possible_released_pressure(&self, network: &ValveNetwork) -> u16 {
        let releases = self.get_possible_final_pressure_releases(network, HashSet::new());
        return *releases.iter().max().unwrap();
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

        let mut player_positions = (None, None);
        let mut updated_player_transits = self.player_transits.clone();
        let mut updated_open_valves = self.open_valves.clone();
        if let Some(player_1_transit) = self.player_transits.0 {
            if player_1_transit.1 == 0 {
                player_positions.0 = Some(player_1_transit.0);
                updated_player_transits.0 = None;
                updated_open_valves[player_1_transit.0 as usize] = true;
            }
        }
        if let Some(player_2_transit) = self.player_transits.1 {
            if player_2_transit.1 == 0 {
                player_positions.1 = Some(player_2_transit.0);
                updated_player_transits.1 = None;
                updated_open_valves[player_2_transit.0 as usize] = true;
            }
        }

        let player_1_paths = if let Some(player_1_position) = player_positions.0 {
            self.get_shortest_paths_to_useful_valves(network, player_1_position)
        } else {
            HashMap::new()
        };
        let player_2_paths = if let Some(player_2_position) = player_positions.1 {
            self.get_shortest_paths_to_useful_valves(network, player_2_position)
        } else {
            HashMap::new()
        };

        if self.all_valves_open() || (player_1_paths.is_empty() && player_2_paths.is_empty()) {
            let successor = ValveState {
                player_transits: updated_player_transits,
                open_valves: updated_open_valves.clone(),
                steps_left: 0,
                pressure_released: self.pressure_released + self.get_step_pressure_released(&updated_open_valves, network) * self.steps_left
            };
            successors.push(successor);
            return successors;
        }

        let mut destination_combinations = vec![];
        if player_1_paths.is_empty() {
            destination_combinations = player_2_paths.iter().map(|(valve, distance)| {
                ((None, Some(*valve)), (None, Some(*distance)))
            }).collect();
        }
        else if player_2_paths.is_empty() {
            destination_combinations = player_1_paths.iter().map(|(valve, distance)| {
                ((Some(*valve), None), (Some(*distance), None))
            }).collect();
        }
        else {
            for (p1_valve, p1_distance) in &player_1_paths {
                for (p2_valve, p2_distance) in &player_2_paths {
                    destination_combinations.push(((Some(*p1_valve), Some(*p2_valve)), (Some(*p1_distance), Some(*p2_distance))));
                }
            }
        }

        for combination in destination_combinations {
            let mut new_player_transits = updated_player_transits.clone();
            if let Some(player_1_destination) = combination.0.0 {
                new_player_transits.0 = Some((player_1_destination, combination.1.0.unwrap() + 1));
            }
            if let Some(player_2_destination) = combination.0.1 {
                new_player_transits.1 = Some((player_2_destination, combination.1.1.unwrap() + 1));
            }
            let steps_until_next_arrival = new_player_transits.0.unwrap_or((u8::MAX, u16::MAX)).1.min(new_player_transits.1.unwrap_or((u8::MAX, u16::MAX)).1);
            if new_player_transits.0.is_some() {
                new_player_transits.0 = Some((new_player_transits.0.unwrap().0, new_player_transits.0.unwrap().1 - steps_until_next_arrival));
            }
            if new_player_transits.1.is_some() {
                new_player_transits.1 = Some((new_player_transits.1.unwrap().0, new_player_transits.1.unwrap().1 - steps_until_next_arrival));
            }
            let successor = ValveState {
                player_transits: new_player_transits,
                open_valves: updated_open_valves.clone(),
                steps_left: self.steps_left - steps_until_next_arrival,
                pressure_released: self.pressure_released + self.get_step_pressure_released(&updated_open_valves, network) * steps_until_next_arrival
            };
            successors.push(successor);
        }
        return successors;
    }

    fn is_done(&self) -> bool {
        self.steps_left == 0
    }

    fn all_valves_open(&self) -> bool {
        self.open_valves.iter().all(|&valve| valve)
    }

    fn valve_is_open(&self, valve_id: ValveIdentifier) -> bool {
        self.open_valves[valve_id as usize]
    }

    fn get_step_pressure_released(&self, open_valves: &Vec<bool>, network: &ValveNetwork) -> u16 {
        let mut sum = 0;
        for (valve_id, valve) in network.iter() {
            if open_valves[*valve_id as usize] {
                sum += valve.flow;
            }
        }
        return sum;
    }

    fn get_shortest_paths_to_useful_valves(&self, network: &ValveNetwork, position: ValveIdentifier) -> HashMap<ValveIdentifier, u16> {
        let current_valve = network.get(&position).unwrap();
        current_valve.neighbors.iter().filter(|(valve, distance)| {
            **distance != 0 && (**distance + 1) <= self.steps_left && !self.valve_is_open(**valve) && network.get_valve(**valve).flow > 0
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

struct Valve {
    flow: u16,
    neighbors: HashMap<ValveIdentifier, u16>
}

fn main() {
    let path = "resources/input.txt";
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
    let initial_state = ValveState::new(&network, 30, false);
    let max_release = initial_state.get_max_possible_released_pressure(&network);
    println!("Max release after 30 minutes with one player: {}", max_release);

    // Part 2
    let initial_state = ValveState::new(&network, 26, true);
    let max_release = initial_state.get_max_possible_released_pressure(&network);
    println!("Max release after 26 minutes with two players: {}", max_release);
}
