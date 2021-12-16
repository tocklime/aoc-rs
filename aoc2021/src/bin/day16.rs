use std::str::FromStr;

use aoc_harness::*;

aoc_main!(2021 day 16, part1 [p1], part2 [p2], example part1 EG => 16);

const EG: &str = "8A004A801A8002F478";

#[derive(Debug)]
enum PacketType {
    LiteralValue(usize),
    Operator(Vec<Packet>),
}
#[derive(Debug)]
struct Packet {
    version: usize,
    type_id: usize,
    message: PacketType,
}
impl Packet {
    fn version_sum(&self) -> usize {
        self.version
            + match &self.message {
                PacketType::LiteralValue(_) => 0,
                PacketType::Operator(v) => v.iter().map(|x| x.version_sum()).sum(),
            }
    }
    fn evaluate(&self) -> usize {
        match &self.message {
            PacketType::LiteralValue(v) => *v,
            PacketType::Operator(inner) => match self.type_id {
                0 => inner.iter().map(|x| x.evaluate()).sum(),
                1 => inner.iter().map(|x| x.evaluate()).product(),
                2 => inner.iter().map(|x| x.evaluate()).min().unwrap(),
                3 => inner.iter().map(|x| x.evaluate()).max().unwrap(),
                5 => (inner[0].evaluate() > inner[1].evaluate()).into(),
                6 => (inner[0].evaluate() < inner[1].evaluate()).into(),
                7 => (inner[0].evaluate() == inner[1].evaluate()).into(),
                _ => unreachable!(),
            },
        }
    }
}

fn read_bits_to_n(iter: &mut impl Iterator<Item = usize>, bit_size: usize) -> Option<usize> {
    let mut n = 0;
    for _ in 0..bit_size {
        n = n << 1 | iter.next()?;
    }
    Some(n)
}
fn read_bits_to_vec(iter: &mut impl Iterator<Item = usize>, bit_size: usize) -> Option<Vec<usize>> {
    let mut ans = Vec::with_capacity(bit_size);
    for _ in 0..bit_size {
        ans.push(iter.next()?);
    }
    Some(ans)
}

fn read_multi_bits(iter: &mut impl Iterator<Item = usize>) -> Option<usize> {
    let mut n = 0;
    loop {
        let is_last = iter.next()? == 0;
        let this_num = read_bits_to_n(iter, 4)?;
        n = n << 4 | this_num;
        if is_last {
            break;
        }
    }
    Some(n)
}
fn read_one_packet(iter: &mut impl Iterator<Item = usize>) -> Option<Packet> {
    let version = read_bits_to_n(iter, 3)?;
    let type_id = read_bits_to_n(iter, 3)?;
    let message = match type_id {
        4 => {
            let n = read_multi_bits(iter)?;
            PacketType::LiteralValue(n)
        }
        _ => {
            //operator packet
            let length_type_id = iter.next()?;

            if length_type_id == 0 {
                let length = read_bits_to_n(iter, 15)?;
                let content = read_bits_to_vec(iter, length)?;
                let inner_packets = read_packets(&mut content.into_iter());
                PacketType::Operator(inner_packets)
            } else {
                let packet_count = read_bits_to_n(iter, 11)?;
                let inner_packets = read_n_packets(iter, packet_count)?;
                PacketType::Operator(inner_packets)
            }
        }
    };
    Some(Packet {
        version,
        type_id,
        message,
    })
}
fn read_packets(iter: &mut impl Iterator<Item = usize>) -> Vec<Packet> {
    let mut packets = Vec::new();
    while let Some(p) = read_one_packet(iter) {
        packets.push(p);
    }
    packets
}
fn read_n_packets(iter: &mut impl Iterator<Item = usize>, n: usize) -> Option<Vec<Packet>> {
    let mut packets = Vec::new();
    for _ in 0..n {
        packets.push(read_one_packet(iter)?);
    }
    Some(packets)
}

fn p1(input: &str) -> usize {
    let bits_iter = input
        .trim()
        .chars()
        .flat_map(|c| match c {
            '0' => [0, 0, 0, 0],
            '1' => [0, 0, 0, 1],
            '2' => [0, 0, 1, 0],
            '3' => [0, 0, 1, 1],
            '4' => [0, 1, 0, 0],
            '5' => [0, 1, 0, 1],
            '6' => [0, 1, 1, 0],
            '7' => [0, 1, 1, 1],
            '8' => [1, 0, 0, 0],
            '9' => [1, 0, 0, 1],
            'A' => [1, 0, 1, 0],
            'B' => [1, 0, 1, 1],
            'C' => [1, 1, 0, 0],
            'D' => [1, 1, 0, 1],
            'E' => [1, 1, 1, 0],
            'F' => [1, 1, 1, 1],
            _ => unreachable!(),
        })
        .collect_vec();
    let mut iter = bits_iter.into_iter();
    let packets = read_packets(&mut iter);
    packets.iter().map(|x| x.version_sum()).sum()
}

fn p2(input: &str) -> usize {
    let bits_iter = input
        .trim()
        .chars()
        .flat_map(|c| match c {
            '0' => [0, 0, 0, 0],
            '1' => [0, 0, 0, 1],
            '2' => [0, 0, 1, 0],
            '3' => [0, 0, 1, 1],
            '4' => [0, 1, 0, 0],
            '5' => [0, 1, 0, 1],
            '6' => [0, 1, 1, 0],
            '7' => [0, 1, 1, 1],
            '8' => [1, 0, 0, 0],
            '9' => [1, 0, 0, 1],
            'A' => [1, 0, 1, 0],
            'B' => [1, 0, 1, 1],
            'C' => [1, 1, 0, 0],
            'D' => [1, 1, 0, 1],
            'E' => [1, 1, 1, 0],
            'F' => [1, 1, 1, 1],
            _ => unreachable!(),
        })
        .collect_vec();
    let mut iter = bits_iter.into_iter();
    let packets = read_packets(&mut iter);
    packets[0].evaluate()
}
