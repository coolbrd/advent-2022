use std::{collections::VecDeque, fs};

#[derive(Clone)]
enum MonkeyOp {
    Multiply,
    Add,
}

impl MonkeyOp {
    fn from_str(s: &str) -> Self {
        match s {
            "*" => MonkeyOp::Multiply,
            "+" => MonkeyOp::Add,
            _ => panic!("Invalid action: {}", s),
        }
    }
}

#[derive(Clone)]
enum MonkeyOpVal {
    Old,
    Val(u16),
}

impl MonkeyOpVal {
    fn from_str(s: &str) -> Self {
        match s {
            "old" => MonkeyOpVal::Old,
            _ => MonkeyOpVal::Val(
                s.parse::<u16>()
                    .expect(&format!("Invalid action value: {}", s)),
            ),
        }
    }
}

#[derive(Clone)]
struct MonkeyAction {
    op: MonkeyOp,
    val: MonkeyOpVal,
}

#[derive(Clone)]
struct Monkey {
    action: MonkeyAction,
    items: VecDeque<u64>,
    test_val: u64,
    dest_if_true: usize,
    dest_if_false: usize,
}

impl Monkey {
    fn from_str(monkey_content: &str) -> Self {
        let mut lines = monkey_content.split("\n");
        lines.next();
        let items = lines
            .next()
            .expect(&format!("Incomplete monkey: {}", monkey_content))
            .split(":")
            .collect::<Vec<&str>>()
            .get(1)
            .expect(&format!(
                "Incomplete monkey, has no starting items: {}",
                monkey_content
            ))
            .split(",")
            .map(|item| {
                item.trim()
                    .parse::<u64>()
                    .expect(&format!("Invalid item: {}", item))
            })
            .collect::<VecDeque<u64>>();
        let action = lines
            .next()
            .expect(&format!("Incomplete monkey: {}", monkey_content))
            .split(" ")
            .collect::<Vec<&str>>()
            .iter()
            .rev()
            .take(2)
            .map(|s| s.to_owned())
            .collect::<Vec<&str>>();
        let (val, op) = (
            action
                .get(0)
                .expect(&format!("No monkey action value found: {:?}", action))
                .to_owned(),
            action
                .get(1)
                .expect(&format!("No monkey action operation found: {:?}", action))
                .to_owned(),
        );
        let action = MonkeyAction {
            op: MonkeyOp::from_str(op),
            val: MonkeyOpVal::from_str(val),
        };
        let test_val = lines
            .next()
            .expect(&format!(
                "Incomplete monkey, no test line found: {}",
                monkey_content
            ))
            .split(" ")
            .last()
            .expect(&format!(
                "No test value found in monkey: {}",
                monkey_content
            ))
            .parse::<u64>()
            .expect(&format!("Invalid test value: {}", monkey_content));
        let dest_if_true = lines
            .next()
            .expect(&format!(
                "Incomplete monkey, no destination if true line found: {}",
                monkey_content
            ))
            .split(" ")
            .last()
            .expect(&format!(
                "No true destination found in monkey: {}",
                monkey_content
            ))
            .parse::<usize>()
            .expect(&format!("Invalid test value: {}", monkey_content));
        let dest_if_false = lines
            .next()
            .expect(&format!(
                "Incomplete monkey, no destination is false line found: {}",
                monkey_content
            ))
            .split(" ")
            .last()
            .expect(&format!(
                "No false destination found in monkey: {}",
                monkey_content
            ))
            .parse::<usize>()
            .expect(&format!("Invalid test value: {}", monkey_content));
        Monkey {
            action,
            items,
            test_val,
            dest_if_true,
            dest_if_false,
        }
    }
}

fn main() {
    let path = "resources/input.txt";
    let contents = fs::read_to_string(path).expect("File not found");
    let monkeys_content = contents.split("\n\n").collect::<Vec<&str>>();
    let monkeys = monkeys_content
        .iter()
        .map(|monkey_content| Monkey::from_str(monkey_content))
        .collect::<Vec<Monkey>>();

    // Part 1
    let rounds_to_perform_p1 = 20;
    let business_1 = perform_monkey_business(rounds_to_perform_p1, &mut monkeys.clone(), 3);
    println!(
        "Monkey business after {} rounds: {}",
        rounds_to_perform_p1, business_1
    );

    // Part 2
    let rounds_to_perform_p2 = 10000;
    let business_2 = perform_monkey_business(rounds_to_perform_p2, &mut monkeys.clone(), 1);
    println!(
        "Monkey business after {} rounds: {}",
        rounds_to_perform_p2, business_2
    );
}

fn perform_monkey_business(
    rounds: usize,
    monkeys: &mut Vec<Monkey>,
    worry_decay_factor: u8,
) -> u64 {
    let mut inspection_counts = vec![0; monkeys.len()];
    let monkey_multiple = calculate_monkey_lcm(monkeys);
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let mut thrown_items: Vec<Vec<u64>> = vec![vec![]; monkeys.len()];
            let monkey = &mut monkeys[i];
            while !monkey.items.is_empty() {
                inspection_counts[i] += 1;
                let mut item = monkey.items.pop_front().unwrap();
                let mod_val = match monkey.action.val {
                    MonkeyOpVal::Old => item,
                    MonkeyOpVal::Val(v) => v as u64,
                };
                item = match monkey.action.op {
                    MonkeyOp::Multiply => item * mod_val,
                    MonkeyOp::Add => item + mod_val,
                };
                item %= monkey_multiple;
                item /= worry_decay_factor as u64;
                if item % monkey.test_val == 0 {
                    thrown_items[monkey.dest_if_true].push(item);
                } else {
                    thrown_items[monkey.dest_if_false].push(item);
                }
            }
            for (j, items) in thrown_items.iter().enumerate() {
                monkeys[j].items.extend(items);
            }
        }
    }
    inspection_counts.sort();
    return inspection_counts.iter().rev().take(2).product::<u64>();
}

fn calculate_monkey_lcm(monkeys: &Vec<Monkey>) -> u64 {
    monkeys
        .iter()
        .map(|monkey| monkey.test_val)
        .fold(1, |acc, val| {
            let mut a = acc;
            let mut b = val;
            while b != 0 {
                let temp = b;
                b = a % b;
                a = temp;
            }
            acc * val / a
        })
}
