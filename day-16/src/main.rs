use std::{fs, collections::{BTreeSet, BTreeMap}};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct ValveState<'a> {
    network: &'a ValveNetwork,
    player_positions: Vec<String>,
    open_valve_names: BTreeSet<String>,
    steps_left: u16,
    pressure_released: u16
}

impl<'a> ValveState<'a> {
    fn get_possible_final_pressure_releases(&self, mut seen: BTreeSet<ValveState<'a>>) -> Vec<u16> {
        if self.is_done() {
            return vec![self.pressure_released];
        }
        let mut successors = self.get_successors();
        let mut final_pressure_releases = Vec::new();
        while !successors.is_empty() {
            let successor = successors.pop().unwrap();
            if seen.contains(&successor) { continue }
            if successor.is_done() {
                final_pressure_releases.push(successor.pressure_released);
            }
            let new_successors = successor.get_successors();
            successors.extend(new_successors);
            seen.insert(successor);
        }
        return final_pressure_releases;
    }

    fn get_successors(&self) -> Vec<ValveState<'a>> {
        match self.player_positions.len() {
            1 => self.get_successors_one_player(),
            2 => self.get_successors_two_player(),
            _ => panic!("Invalid number of players")
        }
    }

    fn get_successors_one_player(&self) -> Vec<ValveState<'a>> {
        let mut successors = Vec::new();
        if self.is_done() { return successors }
        let num_useful_valves = self.get_num_useful_valves();
        if num_useful_valves == 0 { 
            let new_state = ValveState {
                network: self.network,
                player_positions: self.player_positions.clone(),
                open_valve_names: self.open_valve_names.clone(),
                steps_left: 0,
                pressure_released: self.pressure_released + self.get_step_pressure_released() * self.steps_left
            };
            successors.push(new_state);
            return successors;
        }
        let paths_player_1 = self.get_shortest_paths_to_useful_valves(&self.player_positions[0]);
        for (valve_name_player_1, distance_player_1) in paths_player_1 {
        let mut new_open_valve_names = self.open_valve_names.clone();
        new_open_valve_names.insert(valve_name_player_1.to_owned());
        let new_state = ValveState {
            network: self.network,
            player_positions: vec![valve_name_player_1.to_owned()],
            open_valve_names: new_open_valve_names,
            steps_left: self.steps_left - (distance_player_1 + 1),
            pressure_released: self.pressure_released + self.get_step_pressure_released() * (distance_player_1 + 1)
        };
        successors.push(new_state);
        }
        successors
    }
    
    fn get_successors_two_player(&self) -> Vec<ValveState<'a>> {
        let mut successors = Vec::new();
        if self.is_done() { return successors }
        let num_useful_valves = self.get_num_useful_valves();
        if num_useful_valves == 0 { 
            let new_state = ValveState {
                network: self.network,
                player_positions: self.player_positions.clone(),
                open_valve_names: self.open_valve_names.clone(),
                steps_left: 0,
                pressure_released: self.pressure_released + self.get_step_pressure_released() * self.steps_left
            };
            successors.push(new_state);
            return successors;
        }
        let paths_player_1 = self.get_shortest_paths_to_useful_valves(&self.player_positions[0]);
        for (valve_name_player_1, distance_player_1) in paths_player_1 {
            if distance_player_1 == 0 { continue }
            let paths_player_2 = self.get_shortest_paths_to_useful_valves(&self.player_positions[1]);
            for (valve_name_player_2, distance_player_2) in paths_player_2 {
                if distance_player_2 == 0 { continue }
                let mut new_open_valve_names = self.open_valve_names.clone();
                new_open_valve_names.insert(valve_name_player_1.to_owned());
                new_open_valve_names.insert(valve_name_player_2.to_owned());
                let new_state = ValveState {
                    network: self.network,
                    player_positions: vec![valve_name_player_1.to_owned(), valve_name_player_2.to_owned()],
                    open_valve_names: new_open_valve_names,
                    steps_left: self.steps_left - (distance_player_1 + 1),
                    pressure_released: self.pressure_released + self.get_step_pressure_released() * (distance_player_1 + 1)
                };
                successors.push(new_state);
            }
        }
        successors
    }

    fn is_done(&self) -> bool {
        self.steps_left == 0
    }

    fn get_valve(&self, name: &String) -> &Valve {
        self.network.get(name).unwrap()
    }

    fn valve_is_open(&self, valve_name: &String) -> bool {
        self.open_valve_names.contains(valve_name)
    }

    fn get_step_pressure_released(&self) -> u16 {
        self.open_valve_names.iter().map(|valve_name| {
            self.get_valve(valve_name).flow
        }).sum()
    }

    fn get_num_useful_valves(&self) -> u16 {
        self.network.iter().filter(|(valve_name, valve)| {
            !self.valve_is_open(valve_name) && valve.flow > 0
        }).count() as u16
    }

    fn get_shortest_paths_to_useful_valves(&self, position: &String) -> BTreeMap<&String, u16> {
        let current_valve = self.network.get(position).unwrap();
        current_valve.neighbors.iter().filter(|(valve, distance)| {
            (**distance + 1) <= self.steps_left && !self.valve_is_open(*valve) && self.get_valve(*valve).flow > 0
        }).map(|(valve, distance)| {
            (valve, *distance)
        }).collect()
    }
}

type ValveNetwork = BTreeMap<String, Valve>;

fn compute_valve_network(valves: BTreeMap<String, (u16, Vec<String>)>) -> ValveNetwork {
    let mut network: ValveNetwork = BTreeMap::new();
    for (current_valve_name, (current_valve_flow, _)) in &valves {
        let mut paths: BTreeMap<String, u16> = BTreeMap::new();
        let mut queue: Vec<(String, u16)> = Vec::new();
        queue.push((current_valve_name.to_owned(), 0));
        while !queue.is_empty() {
            let (valve_name, distance) = queue.pop().unwrap();
            if paths.contains_key(&valve_name) { 
                if *paths.get(&valve_name).unwrap() <= distance {
                    continue;
                }
            }
            paths.insert(valve_name.to_owned(), distance);
            let valve = valves.get(&valve_name).unwrap();
            for neighbor_name in &valve.1 {
                queue.push((neighbor_name.to_owned(), distance + 1));
            }
        }
        let new_valve = Valve {
            name: current_valve_name.to_owned(),
            flow: *current_valve_flow,
            neighbors: paths
        };
        network.insert(current_valve_name.to_owned(), new_valve);
    }
    return network;
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Valve {
    name: String,
    flow: u16,
    neighbors: BTreeMap<String, u16>
}

fn main() {
    let path = "resources/input.txt";
    let contents = fs::read_to_string(path).expect("File not found");
    let lines = contents.split("\n").map(|line| line.trim()).collect::<Vec<&str>>();
    let valves = lines.iter().map(|line| {
        let mut components = line.split(":").map(|s| s.to_owned()).collect::<Vec<String>>();
        let neighbor_names = components.pop().unwrap().split(",").map(|s| s.to_owned()).collect::<Vec<String>>();
        let flow = components.pop().unwrap().parse::<u16>().unwrap();
        let name = components.pop().unwrap();
        (name, (flow, neighbor_names))
    }).collect::<BTreeMap<String, (u16, Vec<String>)>>();
    let network = compute_valve_network(valves);
    let first_valve_name = "AA".to_owned();
    let initial_state = ValveState {
        network: &network,
        player_positions: vec![first_valve_name],
        open_valve_names: BTreeSet::new(),
        steps_left: 30,
        pressure_released: 0
    };
    //let releases = initial_state.get_possible_final_pressure_releases(BTreeSet::new());
    //let max_release = releases.iter().max().unwrap();
    //println!("Max release: {}", max_release);

    let first_valve_name = "AA".to_owned();
    let initial_state = ValveState {
        network: &network,
        player_positions: vec![first_valve_name.to_owned(), first_valve_name],
        open_valve_names: BTreeSet::new(),
        steps_left: 10,
        pressure_released: 0
    };
    let releases = initial_state.get_possible_final_pressure_releases(BTreeSet::new());
    let max_release = releases.iter().max().unwrap();
    println!("Max release: {}", max_release);
}