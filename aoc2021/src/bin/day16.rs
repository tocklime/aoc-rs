use aoc_harness::*;
use utils::take_upto_n::TakeUpToN;

aoc_main!(2021 day 16, generator gen, part1 [p1] => 883, part2 [p2] => 1_675_198_555_015, example part1 EG => 16);

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
type NumIter<'a> = Box<&'a mut dyn Iterator<Item = usize>>;

impl Packet {
    fn version_sum(&self) -> usize {
        self.version
            + match &self.message {
                PacketType::LiteralValue(_) => 0,
                PacketType::Operator(v) => v.iter().map(Packet::version_sum).sum(),
            }
    }
    fn evaluate(&self) -> usize {
        match &self.message {
            PacketType::LiteralValue(v) => *v,
            PacketType::Operator(inner) => match self.type_id {
                0 => inner.iter().map(Packet::evaluate).sum(),
                1 => inner.iter().map(Packet::evaluate).product(),
                2 => inner.iter().map(Packet::evaluate).min().unwrap(),
                3 => inner.iter().map(Packet::evaluate).max().unwrap(),
                5 => (inner[0].evaluate() > inner[1].evaluate()).into(),
                6 => (inner[0].evaluate() < inner[1].evaluate()).into(),
                7 => (inner[0].evaluate() == inner[1].evaluate()).into(),
                _ => unreachable!(),
            },
        }
    }
}

fn read_bits_to_n(iter: &mut NumIter, bit_size: usize) -> Option<usize> {
    let mut n = 0;
    for _ in 0..bit_size {
        n = n << 1 | iter.next()?;
    }
    Some(n)
}

fn read_multi_bits(iter: &mut NumIter) -> Option<usize> {
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
fn read_one_packet(iter: &mut NumIter) -> Option<Packet> {
    let version = read_bits_to_n(iter, 3)?;
    let type_id = read_bits_to_n(iter, 3)?;
    let message = if type_id == 4 {
        //literal value packet
        let n = read_multi_bits(iter)?;
        PacketType::LiteralValue(n)
    } else {
        //operator packet
        let length_type_id = iter.next()?;
        if length_type_id == 0 {
            //bits-based packet contents
            let length = read_bits_to_n(iter, 15)?;
            let mut content = TakeUpToN::new(iter, length);
            let mut packets = Vec::new();
            while let Some(p) = read_one_packet(&mut Box::new(&mut content)) {
                packets.push(p);
            }
            PacketType::Operator(packets)
        } else {
            //packet count based packet contents.
            let packet_count = read_bits_to_n(iter, 11)?;
            let inner_packets = read_n_packets(iter, packet_count)?;
            PacketType::Operator(inner_packets)
        }
    };
    Some(Packet {
        version,
        type_id,
        message,
    })
}
fn read_packets(iter: &mut NumIter) -> Vec<Packet> {
    let mut packets = Vec::new();
    while let Some(p) = read_one_packet(iter) {
        packets.push(p);
    }
    packets
}
fn read_n_packets(iter: &mut NumIter, n: usize) -> Option<Vec<Packet>> {
    let mut packets = Vec::new();
    for _ in 0..n {
        packets.push(read_one_packet(iter)?);
    }
    Some(packets)
}

fn gen(input: &str) -> Packet {
    let mut bits_iter = input.trim().chars().flat_map(|c| {
        let n = usize::from_str_radix(&c.to_string(), 16).unwrap();
        [n >> 3 & 1, n >> 2 & 1, n >> 1 & 1, n & 1]
    });
    let packet = read_one_packet(&mut Box::new(&mut bits_iter)).unwrap();
    for x in bits_iter {
        assert_eq!(0, x);
    }
    packet
}
fn p1(input: &Packet) -> usize {
    input.version_sum()
}

fn p2(input: &Packet) -> usize {
    input.evaluate()
}
