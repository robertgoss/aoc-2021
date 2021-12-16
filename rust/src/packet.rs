
fn read_bit<'a>(string : &'a str) -> (bool, &'a str) {
    let rest = &string[1..];
    if string.chars().next().unwrap() == '1' {
        (true, rest)
    } else {
        (false,rest)
    }
}

fn read_number_3<'a>(string : &'a str) -> (u8, &'a str) {
    (u8::from_str_radix(&string[0..3], 2).unwrap(),  &string[3..])
}

fn read_number_4<'a>(string : &'a str) -> (u8, &'a str) {
    (u8::from_str_radix(&string[0..4], 2).unwrap(),  &string[4..])
}

fn read_number_11<'a>(string : &'a str) -> (u16, &'a str) {
    (u16::from_str_radix(&string[0..11], 2).unwrap(),  &string[11..])
}

fn read_number_15<'a>(string : &'a str) -> (u16, &'a str) {
    (u16::from_str_radix(&string[0..15], 2).unwrap(),  &string[15..])
}

fn read_number_literal<'a>(string : &'a str) -> (u64, &'a str) {
    let mut total : u64 = 0;
    let mut remaining = string;
    let mut last_packet : bool = false;
    while !last_packet {
        let bit_res = read_bit(remaining);
        last_packet = !bit_res.0;
        remaining = bit_res.1;
        let num_res =  read_number_4(remaining);
        let num : u8 = num_res.0;
        remaining = num_res.1;
        total  = total * 16 + (num as u64);
    }
    (total ,remaining)
}

fn read_number_length<'a>(string : &'a str) -> ((u16, &'a str), bool) {
    let (packet_length, num_s) = read_bit(string);
    if packet_length {
        (read_number_11(num_s), true)
    } else {
        (read_number_15(num_s), false)
    }
}

fn read_packets<'a>(string : &'a str, maximum_length : u16, length_type : bool) -> (Vec<Packet>, &'a str) {
    if length_type {
        read_packets_packets(string, maximum_length)
    } else {
        read_packets_bits(string, maximum_length)
    }
}

fn read_packets_bits<'a>(string : &'a str, maximum_length : u16) -> (Vec<Packet>, &'a str) {
    let mut remaining = &string[..(maximum_length as usize)];
    let mut packets : Vec<Packet> = Vec::new();
    while !remaining.is_empty() {
        let packet_res = Packet::from_string(remaining);
        packets.push(packet_res.0);
        remaining = packet_res.1;
    }
    (packets, &string[(maximum_length as usize)..])
}

fn read_packets_packets<'a>(string : &'a str, maximum_length : u16) -> (Vec<Packet>, &'a str) {
    let mut remaining = string;
    let packets : Vec<Packet> =
    (0..maximum_length).map(
        |_| {
            let packet_res = Packet::from_string(remaining);
            remaining = packet_res.1;
            packet_res.0
        }
    ).collect();
    (packets, remaining)
}

#[derive(Debug)]
enum Operator {
    Sum,
    Product,
    Min,
    Max,
    GreaterThan,
    LessThan,
    Equal
}


#[derive(Debug)]
enum PacketContents {
    Literal(u64),
    Operator(Operator, Vec<Packet>)
}

#[derive(Debug)]
pub struct Packet {
    version : u8,
    data : PacketContents
}

pub fn packet_from_hex_string(string : & str) -> Packet {
    let binary_bits : Vec<&str> = string.chars().map(
        |ch| match ch {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => "ERROR"
        }
    ).collect();
    let binary_string = binary_bits.join("");
    Packet::from_string(&binary_string).0
}

impl Packet {
    fn from_string<'a>(string : &'a str) -> (Packet, &'a str) {
        let (version, contents_s) = read_number_3(string);
        let (contents, rest) = PacketContents::from_string(contents_s);
        let packet = Packet {version : version, data : contents};
        (packet, rest)
    }

    pub fn version_sum(&self) -> usize {
        self.version as usize + self.data.version_sum()
    }

    pub fn evaluate(&self) -> u64 {
        self.data.evaluate()
    }
}

impl PacketContents {
    fn from_string<'a>(string : &'a str) -> (PacketContents, &'a str) {
        let (operator, data_s) = read_number_3(string);
        if operator == 4 {
            let (literal, rest) = read_number_literal(data_s);
            (PacketContents::Literal(literal), rest)
        } else {
            let ((length, sub_packets), length_type) = read_number_length(data_s);
            let (packets, rest) = read_packets(sub_packets, length, length_type);
            (PacketContents::Operator(Operator::from_u8(operator), packets), rest)
        }
    }

    fn version_sum(&self) -> usize {
        match self {
            PacketContents::Literal(_) => 0,
            PacketContents::Operator(_, children) => 
                children.iter().map(|child| child.version_sum()).sum()
        }
    }

    fn evaluate(&self) -> u64 {
        match self {
            PacketContents::Literal(val) => *val,
            PacketContents::Operator(op, children) => {
                let values : Vec<u64> = children.iter().map(
                    |child| child.evaluate()
                ).collect();
                op.operate(values)
            }
        }
    }
}

impl Operator {
    fn from_u8(val : u8) -> Operator {
        match val {
            0 => Operator::Sum,
            1 => Operator::Product,
            2 => Operator::Min,
            3 => Operator::Max,
            5 => Operator::GreaterThan,
            6 => Operator::LessThan,
            7 => Operator::Equal,
            _ => panic!("Bad value")
        }
    }

    fn operate(&self, values : Vec<u64>) -> u64 {
        match self {
            Operator::Sum => values.iter().sum(),
            Operator::Product => values.iter().product(),
            Operator::Min => *values.iter().min().unwrap_or(&0),
            Operator::Max => *values.iter().max().unwrap_or(&0),
            Operator::LessThan => {
                if values[0] < values[1] { 1 } else { 0 }
            }
            Operator::GreaterThan => {
                if values[0] > values[1] { 1 } else { 0 }
            }
            Operator::Equal => {
                if values[0] == values[1] { 1 } else { 0 }
            }
        }
    }
}