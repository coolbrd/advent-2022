use std::{fs, collections::VecDeque};

type Packet = Vec<PacketItem>;

#[derive(Debug)]
enum PacketItem {
    Val(u8),
    List(Vec<PacketItem>)
}

fn main() {
    let path = "resources/input2.txt";
    let contents = fs::read_to_string(path).expect("File not found");
    let line_pairs = contents.split("\n\n").map(|line| line.trim()).collect::<Vec<&str>>();
    let packet_pairs = line_pairs.iter().map(|line| line.split("\n").collect::<Vec<&str>>()).map(|pair| {
        let mut packets = pair.iter().map(|line| {
            if let PacketItem::List(mut packet_item) = parse_packet_item(line.trim().chars().collect::<Vec<char>>()) {
                if let PacketItem::List(nested_packet_item) = packet_item.pop().unwrap() {
                    return nested_packet_item
                }
            }
            panic!("Invalid packet data: {}", line);
        }).collect::<VecDeque<Packet>>();
        let packet_1 = packets.pop_front().unwrap();
        let packet_2 = packets.pop_front().unwrap();
        (packet_1, packet_2)
    }).collect::<Vec<(Packet, Packet)>>();
    for (packet_1, packet_2) in packet_pairs.iter() {
        println!("Packet 1: {:?}", packet_1);
        println!("Packet 2: {:?}", packet_2);
        println!("");
    }
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
