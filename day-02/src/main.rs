use std::fs;

#[derive(Debug, PartialEq, Clone, Copy)]
enum RPSMove {
    Rock,
    Paper,
    Scissors
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum RPSOutcome {
    P1,
    P2,
    Draw
}

fn main() {
    let path = "resources/input.txt";
    let contents = fs::read_to_string(path).expect("File not found");
    let lines = contents.split("\n").collect::<Vec<&str>>();
    let games = lines.iter().map(|line| line.split(" ").collect::<Vec<&str>>())
                            .map(|game| (game[0], game[1]))
                            .map(|game| -> (RPSMove, RPSMove) {
                                let p1_move = match_rps_move(game.0, "A", "B", "C");
                                let p2_move = match_rps_move(game.1, "X", "Y", "Z");
                                (p1_move, p2_move)
                            }).collect::<Vec<(RPSMove, RPSMove)>>();

    // Part 1
    let results: Vec<(RPSMove, RPSOutcome)> = games.iter().map(|game|
        (game.1, determine_rps_outcome(game.1, game.0)
    )).collect();
    let points = count_game_points(results);
    let total_points: u32 = points.iter().sum();
    println!("Part 1: total points earned: {:?}", total_points);

    // Part 2
    let games = lines.iter().map(|line| line.split(" ").collect::<Vec<&str>>())
                            .map(|game| (game[0], game[1]))
                            .map(|game| -> (RPSMove, RPSOutcome) {
                                let p1_move = match_rps_move(game.0, "A", "B", "C");
                                let outcome = match_rps_outcome(game.1, "Z", "X", "Y");
                                (p1_move, outcome)
                            }).collect::<Vec<(RPSMove, RPSOutcome)>>();
    let results: Vec<(RPSMove, RPSOutcome)> = games.iter().map(|game|
        (determine_required_move(game.0, game.1), game.1)
    ).collect();
    let points = count_game_points(results);
    let total_points: u32 = points.iter().sum();
    println!("Part 2: total points earned: {:?}", total_points);
}

fn match_rps_move(input: &str, rock: &str, paper: &str, scissors: &str) -> RPSMove {
    if input == rock { RPSMove::Rock }
    else if input == paper { RPSMove::Paper }
    else if input == scissors { RPSMove::Scissors }
    else { panic!("Invalid move input!") }
}

fn match_rps_outcome(input: &str, p1: &str, p2: &str, draw: &str) -> RPSOutcome {
    if input == p1 { RPSOutcome::P1 }
    else if input == p2 { RPSOutcome::P2 }
    else if input == draw { RPSOutcome::Draw }
    else { panic!("Invalid outcome input!") }
}

fn determine_rps_outcome(p1: RPSMove, p2: RPSMove) -> RPSOutcome {
    match (p1, p2) {
        (RPSMove::Rock, RPSMove::Rock) => RPSOutcome::Draw,
        (RPSMove::Rock, RPSMove::Paper) => RPSOutcome::P2,
        (RPSMove::Rock, RPSMove::Scissors) => RPSOutcome::P1,
        (RPSMove::Paper, RPSMove::Rock) => RPSOutcome::P1,
        (RPSMove::Paper, RPSMove::Paper) => RPSOutcome::Draw,
        (RPSMove::Paper, RPSMove::Scissors) => RPSOutcome::P2,
        (RPSMove::Scissors, RPSMove::Rock) => RPSOutcome::P2,
        (RPSMove::Scissors, RPSMove::Paper) => RPSOutcome::P1,
        (RPSMove::Scissors, RPSMove::Scissors) => RPSOutcome::Draw
    }
}

fn determine_required_move(p2_move: RPSMove, outcome: RPSOutcome) -> RPSMove {
    match (p2_move, outcome) {
        (RPSMove::Rock, RPSOutcome::P2) => RPSMove::Scissors,
        (RPSMove::Rock, RPSOutcome::Draw) => RPSMove::Rock,
        (RPSMove::Rock, RPSOutcome::P1) => RPSMove::Paper,
        (RPSMove::Paper, RPSOutcome::P2) => RPSMove::Rock,
        (RPSMove::Paper, RPSOutcome::Draw) => RPSMove::Paper,
        (RPSMove::Paper, RPSOutcome::P1) => RPSMove::Scissors,
        (RPSMove::Scissors, RPSOutcome::P2) => RPSMove::Paper,
        (RPSMove::Scissors, RPSOutcome::Draw) => RPSMove::Scissors,
        (RPSMove::Scissors, RPSOutcome::P1) => RPSMove::Rock,
    }
}

fn count_game_points(games: Vec<(RPSMove, RPSOutcome)>) -> Vec<u32> {
    games.iter().map(|result| -> u32 {
        let move_points = match result.0 {
            RPSMove::Rock => 1,
            RPSMove::Paper => 2,
            RPSMove::Scissors => 3,
        };
        let outcome_points = match result.1 {
            RPSOutcome::P1 => 6,
            RPSOutcome::P2 => 0,
            RPSOutcome::Draw => 3,
        };
        move_points + outcome_points
    }).collect::<Vec<u32>>()
}