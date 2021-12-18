use std::fs::read_to_string;
use std::path::Path;

fn fetch_file_path() -> &'static str {
    if Path::new("src/input.txt").exists() {
        "src/input.txt"
    } else {
        "day16/src/input.txt"
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = fetch_file_path();
    println!("part one: {:?}", part_one(input_file)?);
    println!("part two: {:?}", part_two(input_file)?);

    Ok(())
}

fn part_one(input_path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let data = read_to_string(input_path)?;
    let packet = Packet::from_hex(&data)?;
    Ok(packet.count_versions())
}

fn part_two(input_path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let data = read_to_string(input_path)?;
    let packet = Packet::from_hex(&data)?;
    Ok(packet.value())
}

#[derive(Debug, PartialEq)]
pub struct Packet {
    version: u8,
    packet_type: PacketType,
}

impl Packet {
    pub fn value(&self) -> usize {
        self.packet_type.value()
    }
}

#[derive(Debug, PartialEq)]
pub enum PacketType {
    Literal(usize),
    Operator(Operator),
}

impl PacketType {
    pub fn value(&self) -> usize {
        use PacketType::*;
        match self {
            Literal(value) => *value,
            Operator(operator) => operator.value(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Operator {
    id: OperatorType,
    sub_packets: Vec<Packet>,
}

impl Operator {
    pub fn count_versions(&self) -> usize {
        self.sub_packets.iter().map(|x| x.count_versions()).sum()
    }

    pub fn value(&self) -> usize {
        self.id.apply(&self.sub_packets)
    }
}

#[derive(Debug, PartialEq)]
pub enum OperatorType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    Equal,
}

impl From<u8> for OperatorType {
    fn from(input: u8) -> OperatorType {
        use OperatorType::*;
        match input {
            0 => Sum,
            1 => Product,
            2 => Minimum,
            3 => Maximum,
            5 => GreaterThan,
            6 => LessThan,
            7 => Equal,
            _ => unreachable!(),
        }
    }
}

impl OperatorType {
    pub fn apply(&self, packets: &Vec<Packet>) -> usize {
        use OperatorType::*;
        match self {
            Sum => packets.into_iter().map(|x| x.value()).sum(),
            Product => packets.into_iter().map(|x| x.value()).product(),
            Maximum => packets
                .into_iter()
                .map(|x| x.value())
                .max()
                .unwrap_or(usize::MIN),
            Minimum => packets
                .into_iter()
                .map(|x| x.value())
                .min()
                .unwrap_or(usize::MAX),
            GreaterThan => {
                let a = packets
                    .get(0)
                    .expect("greater than expects two sub packets");
                let b = packets
                    .get(1)
                    .expect("greater than expects two sub packets");
                if a.value() > b.value() {
                    1
                } else {
                    0
                }
            }
            LessThan => {
                let a = packets.get(0).expect("less than expects two sub packets");
                let b = packets.get(1).expect("less than expects two sub packets");
                if a.value() < b.value() {
                    1
                } else {
                    0
                }
            }
            Equal => {
                let a = packets.get(0).expect("equal expects two sub packets");
                let b = packets.get(1).expect("equal expects two sub packets");
                if a.value() == b.value() {
                    1
                } else {
                    0
                }
            }
        }
    }
}

fn parse_hex(ch: char) -> Option<&'static [u8]> {
    Some(match ch {
        '0' => &[0, 0, 0, 0],
        '1' => &[0, 0, 0, 1],
        '2' => &[0, 0, 1, 0],
        '3' => &[0, 0, 1, 1],
        '4' => &[0, 1, 0, 0],
        '5' => &[0, 1, 0, 1],
        '6' => &[0, 1, 1, 0],
        '7' => &[0, 1, 1, 1],
        '8' => &[1, 0, 0, 0],
        '9' => &[1, 0, 0, 1],
        'A' => &[1, 0, 1, 0],
        'B' => &[1, 0, 1, 1],
        'C' => &[1, 1, 0, 0],
        'D' => &[1, 1, 0, 1],
        'E' => &[1, 1, 1, 0],
        'F' => &[1, 1, 1, 1],
        _ => return None,
    })
}

impl Packet {
    pub fn from_hex(input: &str) -> Result<Packet, Box<dyn std::error::Error>> {
        let data = input
            .chars()
            .try_fold(Vec::new(), |mut acc, ch| {
                acc.extend_from_slice(parse_hex(ch)?);
                Some(acc)
            })
            .ok_or("invalid hexdata")?;

        Packet::from_bytes(&data)
    }

    pub fn from_bytes(input: &[u8]) -> Result<Packet, Box<dyn std::error::Error>> {
        let (packet, data, eaten_bits) = Self::split_packet(input);

        // trim zeroes
        let amount_of_padding_bits = (8 - (eaten_bits % 8)) % 8;
        let (_padding, data) = data.split_at(amount_of_padding_bits);

        if data.len() != 0 {
            Err("data remaining".into())
        } else {
            Ok(packet)
        }
    }

    pub fn count_versions(&self) -> usize {
        match &self.packet_type {
            PacketType::Operator(operator) => self.version as usize + operator.count_versions(),
            _ => self.version as usize,
        }
    }

    fn to_number(input: &[u8]) -> usize {
        let mut number = 0;
        for (i, bit) in input.iter().rev().enumerate() {
            number += (*bit as usize) * 2usize.pow(i as u32)
        }
        number
    }

    fn split_packet(input: &[u8]) -> (Packet, &[u8], usize) {
        let (version, data) = input.split_at(3);
        let version = Self::to_number(version) as u8;
        let (r#type, data) = data.split_at(3);
        let r#type = Self::to_number(r#type) as u8;
        let (packet_type, data, taken_bits) = match r#type {
            4 => {
                let (literal, data, taken_bits) = Self::split_literal(data);
                let literal = Self::to_number(&literal);
                (PacketType::Literal(literal), data, taken_bits)
            }
            operator_id => {
                let (sub_packets, data, taken_bits) = Self::split_operator(data);
                let operator = Operator {
                    sub_packets,
                    id: OperatorType::from(operator_id),
                };
                (PacketType::Operator(operator), data, taken_bits)
            }
        };

        (
            Packet {
                version,
                packet_type,
            },
            data,
            6 + taken_bits,
        )
    }

    fn split_literal(input: &[u8]) -> (Vec<u8>, &[u8], usize) {
        let mut data = Vec::new();
        let mut has_next = true;
        let mut output = input;
        let mut parts = 0;
        while has_next {
            let (switch, input) = output.split_at(1);
            let (literal_part, input) = input.split_at(4);
            output = input;
            has_next = switch[0] == 1;
            data.extend_from_slice(literal_part);
            parts += 1;
        }

        (data, output, parts * 5)
    }

    fn split_operator(input: &[u8]) -> (Vec<Packet>, &[u8], usize) {
        let (length_type, input) = input.split_at(1);
        let (take_bits, absolute) = if length_type[0] == 1 {
            (11, true)
        } else {
            (15, false)
        };
        let (length, input) = input.split_at(take_bits);
        let length = Self::to_number(length);
        if absolute {
            let mut total_bits_taken = 0;
            let mut sub_packets = Vec::new();
            let mut input = input;
            for _ in 0..length {
                let (packet, output, taken_bits) = Self::split_packet(input);
                total_bits_taken += taken_bits;
                sub_packets.push(packet);
                input = output;
            }
            (sub_packets, input, 1 + take_bits + total_bits_taken)
        } else {
            let (mut sub_packets_bits, input) = input.split_at(length);
            let mut sub_packets = Vec::new();
            while !sub_packets_bits.is_empty() {
                let out = Self::split_packet(sub_packets_bits);
                sub_packets_bits = out.1;
                sub_packets.push(out.0);
            }
            (sub_packets, input, 1 + take_bits + length)
        }
    }
}

#[test]
fn day16_part_one() {
    assert_eq!(927, part_one(fetch_file_path()).unwrap())
}

#[test]
fn day16_part_two() {
    assert_eq!(1725277876501, part_two(fetch_file_path()).unwrap())
}

#[test]
fn packet_literal() {
    let data = "D2FE28";

    let packet = Packet::from_hex(data).unwrap();

    assert_eq!(
        Packet {
            version: 6,
            packet_type: PacketType::Literal(2021)
        },
        packet
    );
}

#[test]
fn packet_operator2() {
    let data = "38006F45291200";

    let packet = Packet::from_hex(data).unwrap();

    assert_eq!(
        Packet {
            version: 1,
            packet_type: PacketType::Operator(Operator {
                id: OperatorType::LessThan,
                sub_packets: vec![
                    Packet {
                        version: 6,
                        packet_type: PacketType::Literal(10)
                    },
                    Packet {
                        version: 2,
                        packet_type: PacketType::Literal(20)
                    },
                ]
            })
        },
        packet
    );
}

#[test]
fn packet_operator3() {
    let data = "EE00D40C823060";

    let packet = Packet::from_hex(data).unwrap();

    assert_eq!(
        Packet {
            version: 7,
            packet_type: PacketType::Operator(Operator {
                id: OperatorType::Maximum,
                sub_packets: vec![
                    Packet {
                        version: 2,
                        packet_type: PacketType::Literal(1)
                    },
                    Packet {
                        version: 4,
                        packet_type: PacketType::Literal(2)
                    },
                    Packet {
                        version: 1,
                        packet_type: PacketType::Literal(3)
                    },
                ]
            })
        },
        packet
    );
}

#[test]
fn count_versions() {
    assert_eq!(
        16,
        Packet::from_hex("8A004A801A8002F478")
            .unwrap()
            .count_versions()
    );
    assert_eq!(
        12,
        Packet::from_hex("620080001611562C8802118E34")
            .unwrap()
            .count_versions()
    );
    assert_eq!(
        23,
        Packet::from_hex("C0015000016115A2E0802F182340")
            .unwrap()
            .count_versions()
    );
    assert_eq!(
        31,
        Packet::from_hex("A0016C880162017C3686B18A3D4780")
            .unwrap()
            .count_versions()
    );
}
