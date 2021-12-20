use std::str::FromStr;

#[allow(dead_code)]
pub const URL: &str = "https://adventofcode.com/2021/day/16/input";

struct BitReader {
    b: Vec<u8>,
    curr: usize,
}

impl BitReader {
    fn new(b: Vec<u8>) -> Self {
        BitReader { b, curr: 0 }
    }

    fn curr(&self) -> usize {
        self.curr
    }

    fn read(&mut self, amount: usize) -> u8 {
        let bits = u8::BITS as usize;
        assert!(amount <= bits);

        let (byte_i, bit_i) = (self.curr / bits, self.curr % bits);

        let res = if bit_i + amount <= bits {
            (self.b[byte_i] << bit_i) >> (bits - amount)
        } else {
            ((self.b[byte_i] << bit_i) | (self.b[byte_i+1] >> (bits - bit_i))) >> (bits - amount)
        };
        self.curr += amount;
        res
    }

    fn read_u32(&mut self, amount: usize) -> u32 {
        let bits = u8::BITS as usize;
        assert!(amount <= u32::BITS as usize);

        let (bytes, rem_bits) = (amount / bits, amount % bits);

        let mut res = 0;
        for _ in 0..bytes {
            res = (res << 8) | (self.read(8) as u32);
        }
        res = (res << rem_bits) | (self.read(rem_bits) as u32);
        res
    }
}

impl FromStr for BitReader {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes: Vec<u8> = (0..s.len()).step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i+2], 16).unwrap())
            .collect();

        Ok(BitReader::new(bytes))
    }
}

enum OpType {
    Sum,
    Product,
    Min,
    Max,
    Gt,
    Lt,
    Eq,
}

enum Payload {
    Literal(u64),
    Op(OpType, Vec<Packet>)
}

impl Payload {
    fn from(typ: u8, b: &mut BitReader) -> Self {
        match typ {
            4 => {
                let mut value: u64 = 0;
                loop {
                    let group = b.read(5);
                    value = (value << 4) | (group & 0x0F) as u64;
                    if (group >> 4) == 0 {
                        break;
                    }
                }
                Self::Literal(value)
            },
            op_typ => {
                let mut subpackets = Vec::new();

                let len_type = b.read(1);
                match len_type {
                    0 => {
                        let len_in_bits = b.read_u32(15) as usize;
                        let end = b.curr() + len_in_bits;
                        while b.curr() < end {
                            subpackets.push(Packet::from(b));
                        }
                    },
                    _ => {
                        let len_in_packets = b.read_u32(11);
                        for _ in 0..len_in_packets {
                            subpackets.push(Packet::from(b))
                        }
                    }
                }
                match op_typ {
                    0 => Self::Op(OpType::Sum, subpackets),
                    1 => Self::Op(OpType::Product, subpackets),
                    2 => Self::Op(OpType::Min, subpackets),
                    3 => Self::Op(OpType::Max, subpackets),
                    5 => Self::Op(OpType::Gt, subpackets),
                    6 => Self::Op(OpType::Lt, subpackets),
                    7 => Self::Op(OpType::Eq, subpackets),
                    _ => panic!("hmmm"),
                }
            }
        }
    }
}

struct Packet {
    version: u8,
    payload: Payload,
}

impl Packet {
    fn from(b: &mut BitReader) -> Self {
        let header = b.read(6);

        let version = header >> 3;
        let typ = header & 0x07;

        let payload = Payload::from(typ, b);
        Packet{ version, payload }
    }

    fn eval(&self) -> u64 {
        match &self.payload {
            Payload::Literal(v) => *v,
            Payload::Op(op_type, subpackets) => {
                match op_type {
                    OpType::Sum => subpackets.iter().fold(0, |acc, p| acc + p.eval()),
                    OpType::Product => subpackets.iter().fold(1, |acc, p| acc * p.eval()),
                    OpType::Min => subpackets.iter().map(|p| p.eval()).min().unwrap(),
                    OpType::Max => subpackets.iter().map(|p| p.eval()).max().unwrap(),
                    OpType::Gt => if subpackets[0].eval() > subpackets[1].eval() {1} else {0},
                    OpType::Lt => if subpackets[0].eval() < subpackets[1].eval() {1} else {0},
                    OpType::Eq => if subpackets[0].eval() == subpackets[1].eval() {1} else {0},
                }
            }
        }
    }
}


#[allow(dead_code)]
pub fn solve1(text: &str) -> u64 {
    let mut b: BitReader = text.trim().parse().unwrap();
    let packet = Packet::from(&mut b);

    let mut sum = 0;
    let mut stack = vec![packet];
    while !stack.is_empty() {
        let p = stack.pop().unwrap();
        sum += p.version as u64;
        match p.payload {
            Payload::Op(_, packets) => packets.into_iter().for_each(|p| stack.push(p)),
            _ => (),
        }
    }
    sum
}


#[allow(dead_code)]
pub fn solve2(text: &str) -> u64 {
    let mut b: BitReader = text.trim().parse().unwrap();
    let packet = Packet::from(&mut b);
    packet.eval()
}

