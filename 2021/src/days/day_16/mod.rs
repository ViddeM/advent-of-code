use crate::days::common::Day;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
struct BITS {
    version: u8,
    packet_type: PacketType,
}

impl Display for BITS {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.packet_type {
            PacketType::Literal(n) => write!(f, "{}", n),
            PacketType::Operator(op, parts) => {
                let mut s = String::new();
                for (i, part) in parts.iter().enumerate() {
                    match part.packet_type {
                        PacketType::Literal(_) => s.push_str(&part.to_string()),
                        PacketType::Operator(_, _) => s.push_str(&format!("({})", part)),
                    }
                    if i < parts.len() - 1 {
                        s.push_str(&op.to_string())
                    }
                }
                write!(f, "{}", s)
            }
        }
    }
}

#[derive(Debug, Clone)]
enum PacketType {
    Literal(u64),
    Operator(OperatorType, Vec<BITS>),
}

#[derive(Debug, Clone)]
enum OperatorType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl Display for OperatorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OperatorType::Sum => write!(f, " + "),
            OperatorType::Product => write!(f, " * "),
            OperatorType::Minimum => write!(f, " `min` "),
            OperatorType::Maximum => write!(f, " `max` "),
            OperatorType::GreaterThan => write!(f, " > "),
            OperatorType::LessThan => write!(f, " < "),
            OperatorType::EqualTo => write!(f, " == "),
        }
    }
}

impl OperatorType {
    fn parse(num: u8) -> OperatorType {
        match num {
            0 => OperatorType::Sum,
            1 => OperatorType::Product,
            2 => OperatorType::Minimum,
            3 => OperatorType::Maximum,
            5 => OperatorType::GreaterThan,
            6 => OperatorType::LessThan,
            7 => OperatorType::EqualTo,
            n => panic!("Invalid operator type number {}", n),
        }
    }
}

fn parse(input: &str) -> BITS {
    let mut digits: Vec<u8> =
        input
            .trim()
            .chars()
            .map(|c| c.to_digit(16).unwrap())
            .fold(Vec::new(), |mut v, d| {
                for i in 0..4 {
                    v.push(((d >> (3 - i)) & 1) as u8);
                }

                v
            });

    digits.reverse(); // Popping reads from back so we need to reverse the vector.
    parse_packet(&mut digits)
}

#[derive(Debug, Clone)]
enum ReadMode {
    Version(u8),             // Remaining bits to read for version
    TypeId(u8),              // Remaining bits to read for typeid
    Literal(u8, bool), // Remaining bits to read for the current literal block and if we should keep reading after this block.
    LengthTypeId,      // Just 1 bit so no need for number
    TotalLengthSubPack(u32), // Remaining bits to read for total length number.
    NumSubPack(u32),   // Remaining bits to read for number of sub-packages number.
}

enum LengthTypeId {
    TotalLength(u32),
    NumSubPackages(u32),
}

fn parse_packets(digits: &mut Vec<u8>, length_type: LengthTypeId) -> Vec<BITS> {
    match length_type {
        LengthTypeId::TotalLength(n) => {
            let init_digits_len = digits.len();
            let mut packets = Vec::new();
            loop {
                packets.push(parse_packet(digits));
                let bits_read = (init_digits_len - digits.len()) as u32;
                if bits_read > n {
                    panic!("Read too many bits for subpackage of type 'Total Length'!")
                } else if bits_read == n {
                    return packets;
                }
            }
        }
        LengthTypeId::NumSubPackages(n) => {
            let mut packets = Vec::new();
            for _ in 0..n {
                packets.push(parse_packet(digits));
            }
            return packets;
        }
    }
}

fn parse_packet(digits: &mut Vec<u8>) -> BITS {
    let mut mode = ReadMode::Version(3);
    let mut version = 0;
    let mut p_type = 0;
    let mut literal = 0;
    let mut length = 0;

    while let Some(digit) = digits.pop() {
        match mode {
            ReadMode::Version(n) => {
                version = version | (digit << (n - 1));
                if n == 1 {
                    mode = ReadMode::TypeId(3);
                } else {
                    mode = ReadMode::Version(n - 1);
                }
            }
            ReadMode::TypeId(n) => {
                p_type = p_type | (digit << (n - 1));
                mode = if n == 1 {
                    match p_type {
                        4 => ReadMode::Literal(5, false),
                        _ => ReadMode::LengthTypeId,
                    }
                } else {
                    ReadMode::TypeId(n - 1)
                }
            }
            ReadMode::Literal(n, is_last) => {
                mode = match n {
                    5 => ReadMode::Literal(4, digit == 0),
                    _ => {
                        literal = literal | ((digit as u64) << (n - 1));
                        if n == 1 {
                            if is_last {
                                return BITS {
                                    version,
                                    packet_type: PacketType::Literal(literal),
                                };
                            } else {
                                literal = literal << 4;
                                ReadMode::Literal(5, false)
                            }
                        } else {
                            ReadMode::Literal(n - 1, is_last)
                        }
                    }
                }
            }
            ReadMode::LengthTypeId => {
                if digit == 0 {
                    mode = ReadMode::TotalLengthSubPack(15);
                } else {
                    mode = ReadMode::NumSubPack(11);
                }
            }
            ReadMode::TotalLengthSubPack(n) => {
                length = length | ((digit as u32) << (n - 1));
                if n == 1 {
                    let sub_packages = parse_packets(digits, LengthTypeId::TotalLength(length));
                    return BITS {
                        version,
                        packet_type: PacketType::Operator(
                            OperatorType::parse(p_type),
                            sub_packages,
                        ),
                    };
                } else {
                    mode = ReadMode::TotalLengthSubPack(n - 1);
                }
            }
            ReadMode::NumSubPack(n) => {
                length = length | ((digit as u32) << (n - 1));
                if n == 1 {
                    let sub_packages = parse_packets(digits, LengthTypeId::NumSubPackages(length));
                    return BITS {
                        version,
                        packet_type: PacketType::Operator(
                            OperatorType::parse(p_type),
                            sub_packages,
                        ),
                    };
                } else {
                    mode = ReadMode::NumSubPack(n - 1)
                }
            }
        }
    }

    panic!(
        "Failed to parse input, mode: {:?}, remaining:\n{:?}",
        mode, digits
    );
}

pub struct Day16 {}

impl Day for Day16 {
    fn parse_bench(&self, input: &str) -> () {
        parse(input);
    }

    fn part_1(&self, input: &str) -> String {
        let bits = parse(input);
        sum_version_numbers(bits).to_string()
    }

    fn part_2(&self, input: &str) -> String {
        let bits = parse(input);
        calc_val(bits).to_string()
    }
}

fn sum_version_numbers(bits: BITS) -> u32 {
    (bits.version as u32)
        + match bits.packet_type {
            PacketType::Operator(_, sub) => sub
                .into_iter()
                .fold(0, |sum, p| sum + sum_version_numbers(p)),
            _ => 0,
        }
}

fn calc_val(bits: BITS) -> u64 {
    let v = match bits.packet_type {
        PacketType::Literal(n) => n as u64,
        PacketType::Operator(ref op, ref sub) => {
            let mut vals = sub.into_iter().map(|s| calc_val(s.clone()));
            match &op {
                OperatorType::Sum => vals.fold(0, |sum, v| sum + (v as u64)),
                OperatorType::Product => vals.fold(1, |sum, v| sum * v),
                OperatorType::Minimum => {
                    vals.fold(u64::MAX, |min, v| if v < min { v } else { min })
                }
                OperatorType::Maximum => vals.fold(0, |max, v| if v > max { v } else { max }),
                OperatorType::GreaterThan => {
                    let first = vals.next().unwrap();
                    let second = vals.next().unwrap();
                    if first > second {
                        1
                    } else {
                        0
                    }
                }
                OperatorType::LessThan => {
                    let first = vals.next().unwrap();
                    let second = vals.next().unwrap();
                    if first < second {
                        1
                    } else {
                        0
                    }
                }
                OperatorType::EqualTo => {
                    let first = vals.next().unwrap();
                    let second = vals.next().unwrap();
                    if first == second {
                        1
                    } else {
                        0
                    }
                }
            }
        }
    };

    v
}
