use std::{fs, collections::VecDeque};

#[derive(Debug, Clone)]
enum MonkeyOp {
    Multiply,
    Add
}

#[derive(Debug, Clone)]
enum MonkeyOpVal {
    Old,
    Val(u16)
}

#[derive(Debug, Clone)]
struct MonkeyAction {
    op: MonkeyOp,
    val: MonkeyOpVal
}

#[derive(Debug, Clone)]
struct Monkey {
    action: MonkeyAction,
    items: VecDeque<u64>,
    test_val: u64,
    dest_if_true: usize,
    dest_if_false: usize
}

fn main() {
    let path = "resources/input.txt";
    let contents = fs::read_to_string(path).expect("File not found");
    let monkeys_content = contents.split("\n\n").collect::<Vec<&str>>();
    let monkeys = monkeys_content.iter().map(|monkey| {
        let mut lines = monkey.split("\n");
        lines.next();
        let items = lines.next().expect(&format!("Incomplete monkey: {}", monkey))
                                .split(":").collect::<Vec<&str>>()[1].split(",")
                                .map(|item| {
                                    item.trim().parse::<u64>().expect(&format!("Invalid item: {}", item))
                                }).collect::<VecDeque<u64>>();
        let action = lines.next().expect(&format!("Incomplete monkey: {}", monkey))
                                 .split(" ").collect::<Vec<&str>>().iter().rev().take(2).map(|s| s.to_owned()).collect::<Vec<&str>>();
        let action = MonkeyAction {
            op: match action[1] {
                "*" => MonkeyOp::Multiply,
                "+" => MonkeyOp::Add,
                _ => panic!("Invalid action: {:?}", action)
            },
            val: action[0].parse::<u16>().map_or(MonkeyOpVal::Old, |v| MonkeyOpVal::Val(v))
        };
        let test_val = lines.next().expect(&format!("Incomplete monkey: {}", monkey))
                            .split(" ").last().expect(&format!("No test value found in monkey: {}", monkey))
                            .parse::<u64>().expect(&format!("Invalid test value: {}", monkey));
        let dest_if_true = lines.next().expect(&format!("Incomplete monkey: {}", monkey))
                                .split(" ").last().expect(&format!("No true dest found in monkey: {}", monkey))
                                .parse::<usize>().expect(&format!("Invalid test value: {}", monkey));
        let dest_if_false = lines.next().expect(&format!("Incomplete monkey: {}", monkey))
                                 .split(" ").last().expect(&format!("No false dest found in monkey: {}", monkey))
                                 .parse::<usize>().expect(&format!("Invalid test value: {}", monkey));
        Monkey {
            action,
            items,
            test_val,
            dest_if_true,
            dest_if_false
        }
    }).collect::<Vec<Monkey>>();

    // Part 1
    let business_1 = perform_monkey_business(20, &mut monkeys.clone(), true);
    println!("Monkey business after 20 rounds: {}", business_1);

    // Part 2
    let business_2 = perform_monkey_business(10000, &mut monkeys.clone(), false);
    println!("Monkey business after 10000 rounds: {}", business_2);
}

fn perform_monkey_business(rounds: usize, monkeys: &mut Vec<Monkey>, worry_degrades: bool) -> u64 {
    let mut inspection_counts = vec![0; monkeys.len()];
    let monkey_multiple = monkeys.iter().map(|monkey| monkey.test_val).fold(1, |acc, val| {
        let mut a = acc;
        let mut b = val;
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        acc * val / a
    });
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let mut thrown_items: Vec<Vec<u64>> = vec![vec![]; monkeys.len()];
            let monkey = &mut monkeys[i];
            while !monkey.items.is_empty() {
                inspection_counts[i] += 1;
                let mut item = monkey.items.pop_front().unwrap();
                if item > monkey_multiple {
                    item = item % monkey_multiple;
                }
                let mod_val = match monkey.action.val {
                    MonkeyOpVal::Old => item,
                    MonkeyOpVal::Val(v) => v as u64
                };
                item = match monkey.action.op {
                    MonkeyOp::Multiply => item * mod_val,
                    MonkeyOp::Add => item + mod_val
                };
                if worry_degrades {
                    item = item / 3;
                }
                if item % monkey.test_val as u64 == 0 {
                    thrown_items[monkey.dest_if_true].push(item);
                }
                else {
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
