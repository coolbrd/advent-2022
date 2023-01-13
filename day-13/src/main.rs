use std::{fs, collections::VecDeque};

#[derive(PartialEq, Debug)]
enum PacketItem {
    Val(u8),
    List(Vec<PacketItem>)
}

#[derive(PartialEq, Debug)]
enum PacketOrderCorrectness {
    Correct,
    Incorrect,
    Unknown
}

fn main() {
    let path = "resources/input.txt";
    let contents = fs::read_to_string(path).expect("File not found");
    let line_pairs = contents.split("\n\n").map(|line| line.trim()).collect::<Vec<&str>>();
    let packet_pairs = line_pairs.iter().map(|line| line.split("\n").collect::<Vec<&str>>()).map(|pair| {
        let mut packets = pair.iter().map(|line| {
            if let PacketItem::List(packet_item) = parse_packet_item(line.trim().chars().collect::<Vec<char>>()) {
                return PacketItem::List(packet_item);
            }
            panic!("Invalid packet data: {}", line);
        }).collect::<VecDeque<PacketItem>>();
        let packet_1 = packets.pop_front().unwrap();
        let packet_2 = packets.pop_front().unwrap();
        (packet_1, packet_2)
    }).collect::<Vec<(PacketItem, PacketItem)>>();

    let mut index_sum = 0;
    packet_pairs.iter().enumerate().for_each(|(i, pair)| {
        let (packet_1, packet_2) = pair;
        println!("Packet 1: {:?}", packet_1);
        println!("Packet 2: {:?}", packet_2);
        let comparison = check_packet_items_correctly_ordered(packet_1, packet_2);
        if comparison == PacketOrderCorrectness::Correct {
            index_sum += i + 1;
        }
        println!("Comparison: {:?}", comparison);
        println!("");
    });
    println!("Index sum: {}", index_sum);

    // Part 2
    let mut all_packets = packet_pairs.iter().map(|pair| {
        let (packet_1, packet_2) = pair;
        return vec![packet_1, packet_2];
    }).flatten().collect::<Vec<&PacketItem>>();
    let divider_packets = vec![
        PacketItem::List(vec![PacketItem::List(vec![PacketItem::List(vec![PacketItem::Val(2)])])]),
        PacketItem::List(vec![PacketItem::List(vec![PacketItem::List(vec![PacketItem::Val(6)])])])];
    all_packets.extend(divider_packets.iter());
    all_packets.sort_by(|left, right| {
        let comparison = check_packet_items_correctly_ordered(left, right);
        match comparison {
            PacketOrderCorrectness::Correct => {
                return std::cmp::Ordering::Less;
            },
            PacketOrderCorrectness::Incorrect => {
                return std::cmp::Ordering::Greater;
            },
            PacketOrderCorrectness::Unknown => {
                return std::cmp::Ordering::Equal;
            }
        }
    });
    let div_1_pos = all_packets.iter().position(|packet| **packet == divider_packets[0]).unwrap();
    let div_2_pos = all_packets.iter().position(|packet| **packet == divider_packets[1]).unwrap();
    let key = (div_1_pos + 1) * (div_2_pos + 1);
    println!("Key: {}", key);
}

fn parse_packet_item(packet_data: Vec<char>) -> PacketItem {
    let mut packet_items = Vec::new();
    let mut cur_buffer = Vec::new();
    let mut cur_depth = 0_i16;
    for i in 0..packet_data.len() {
        let c = packet_data[i];
        match c {
            '[' => {
                cur_buffer.push(c);
                cur_depth += 1;
            },
            ']' => {
                cur_buffer.push(c);
                cur_depth -= 1;
                if cur_depth < 0 {
                    panic!("Invalid packet item data, unmatched closing bracket: {}", packet_data.iter().collect::<String>());
                }
                else if cur_depth == 0 {
                    let nested_packet_item_buffer = cur_buffer.clone()[1..cur_buffer.len()-1].to_vec();
                    packet_items.push(parse_packet_item(nested_packet_item_buffer));
                    cur_buffer.clear();
                }
            },
            ',' => {
                if cur_depth == 0 {
                    if cur_buffer.len() > 0 {
                        packet_items.push(PacketItem::Val(cur_buffer.iter().collect::<String>()
                                    .parse::<u8>().expect(&format!(
                                        "Invalid packet item data, expected number, got: {}",
                                        cur_buffer.iter().collect::<String>()))));
                    }
                    cur_buffer.clear();
                }
                else {
                    cur_buffer.push(c);
                }
            },
            '0'..='9' => {
                cur_buffer.push(c);
            },
            _ => {
                panic!("Invalid packet item data, unexpected character: '{}' in '{}'", c, packet_data.iter().collect::<String>());
            }
        }
        if i == packet_data.len() - 1 {
            if cur_depth == 0 {
                if cur_buffer.len() > 0 {
                    packet_items.push(PacketItem::Val(cur_buffer.iter().collect::<String>()
                                .parse::<u8>().expect(&format!(
                                    "Invalid packet item data, expected number, got: {}",
                                    cur_buffer.iter().collect::<String>()))));
                }
                cur_buffer.clear();
            }
            else {
                panic!("Invalid packet item data, unmatched opening bracket: {}", packet_data.iter().collect::<String>());
            }
        }
    }
    return PacketItem::List(packet_items);
}

fn check_packet_items_correctly_ordered(left_item: &PacketItem, right_item: &PacketItem) -> PacketOrderCorrectness {
    match (left_item, right_item) {
        (PacketItem::Val(left_val), PacketItem::Val(right_val)) => {
            if left_val < right_val {
                return PacketOrderCorrectness::Correct;
            }
            else if left_val > right_val {
                return PacketOrderCorrectness::Incorrect;
            }
            return PacketOrderCorrectness::Unknown;
        },
        (PacketItem::List(left_list), PacketItem::List(right_list)) => {
            for i in 0..left_list.len() {
                if i >= right_list.len() {
                    return PacketOrderCorrectness::Incorrect;
                }
                let sub_item_ordered = check_packet_items_correctly_ordered(&left_list[i], &right_list[i]);
                match sub_item_ordered {
                    PacketOrderCorrectness::Correct => {
                        return PacketOrderCorrectness::Correct;
                    },
                    PacketOrderCorrectness::Incorrect => {
                        return PacketOrderCorrectness::Incorrect;
                    },
                    PacketOrderCorrectness::Unknown => {
                        continue;
                    }
                }
            }
            if left_list.len() < right_list.len() {
                return PacketOrderCorrectness::Correct;
            }
            return PacketOrderCorrectness::Unknown;
        },
        (PacketItem::Val(left_val), PacketItem::List(_)) => {
            let nested_item = PacketItem::List(vec![PacketItem::Val(*left_val)]);
            let ordered = check_packet_items_correctly_ordered(&nested_item, right_item);
            return ordered;
        },
        (PacketItem::List(_), PacketItem::Val(right_val)) => {
            let nested_item = PacketItem::List(vec![PacketItem::Val(*right_val)]);
            let ordered = check_packet_items_correctly_ordered(left_item, &nested_item);
            return ordered;
        }
    }
}
