#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Display {
    bits : Vec<bool>
}

impl Display {
    pub fn from_string(string : &str) -> Display {
        Display { bits: vec!(
            string.contains('a'),
            string.contains('b'),
            string.contains('c'),
            string.contains('d'),
            string.contains('e'),
            string.contains('f'),
            string.contains('g')
         ) }
    }

    fn bits_set(&self) -> usize {
        self.bits.iter().filter(
            |bit| **bit
        ).count()
    } 

    fn intersection(&self, other : &Self) -> Display {
        let new_bits = self.bits.iter().zip(other.bits.iter()).map(
            |(bit1, bit2)| *bit1 && *bit2
        ).collect();
        Display { bits : new_bits }
    }

    fn is_easy_digit(&self) -> bool {
       self.is_one() || self.is_four() || self.is_seven() || self.is_eight()
    }

    fn is_zero(&self, six : &Self, nine : &Self) -> bool {
        self.bits_set() == 6 && *self != *six && *self != *nine
    }

    fn is_one(&self) -> bool {
        self.bits_set() == 2
    }

    fn is_two(&self, three : &Self, four : &Self) -> bool {
        self.bits_set() == 5 && *self != *three && self.intersection(four).bits_set() == 2
    }

    fn is_three(&self, one : &Self) -> bool {
        self.bits_set() == 5 && self.intersection(one) == *one
    }

    fn is_four(&self) -> bool {
        self.bits_set() == 4
    }

    fn is_five(&self, three : &Self, four : &Self) -> bool {
        self.bits_set() == 5 && *self != *three && self.intersection(four).bits_set() == 3
    }

    fn is_six(&self, one : &Self) -> bool {
        self.bits_set() == 6 && self.intersection(one) != *one
    }

    fn is_seven(&self) -> bool {
        self.bits_set() == 3
    }

    fn is_eight(&self) -> bool {
        self.bits_set() == 7
    }

    fn is_nine(&self, four : &Self) -> bool {
        self.bits_set() == 6 && self.intersection(four) == *four
    }
}

pub struct Entry {
    patterns : [Display; 10],
    output : [Display; 4]
}

impl Entry {
    pub fn from_string(string : &str) -> Option<Entry> {
        if let Some((pattern_s, output_s)) = string.split_once(" | ") {
            let pattern_v : Vec<Display> = pattern_s.split(" ").map(
                |str| Display::from_string(str)
            ).collect();
            let output_v : Vec<Display> = output_s.split(" ").map(
                |str| Display::from_string(str)
            ).collect();
            if let (Ok(patterns), Ok(output)) = (pattern_v.try_into(), output_v.try_into()) {
                return Some(Entry{patterns : patterns, output : output});
            }
        }
        None
    }

    fn count_easy_digits(&self) -> usize {
        self.output.iter().filter(
            |digit| digit.is_easy_digit()
        ).count()
    }

    fn map_digits(&self) -> [Display; 10] {
        let one = self.patterns.iter().filter(|d| d.is_one()).next().unwrap().clone();
        let four = self.patterns.iter().filter(|d| d.is_four()).next().unwrap().clone();
        let seven = self.patterns.iter().filter(|d| d.is_seven()).next().unwrap().clone();
        let eight= self.patterns.iter().filter(|d| d.is_eight()).next().unwrap().clone();
        let nine = self.patterns.iter().filter(|d| d.is_nine(&four)).next().unwrap().clone();
        let three = self.patterns.iter().filter(|d| d.is_three(&one)).next().unwrap().clone();
        let six = self.patterns.iter().filter(|d| d.is_six(&one)).next().unwrap().clone();
        let zero = self.patterns.iter().filter(|d| d.is_zero(&six, &nine)).next().unwrap().clone();
        let two = self.patterns.iter().filter(|d| d.is_two(&three, &four)).next().unwrap().clone();
        let five = self.patterns.iter().filter(|d| d.is_five(&three, &four)).next().unwrap().clone();
        [zero, one, two, three, four, five, six, seven, eight, nine]
    }

    fn output_nums(&self) -> Vec<usize> {
        let map = self.map_digits();
        self.output.iter().map(
            |digit| map.iter().position(|test| *test == *digit).unwrap()
        ).collect()
    }

    pub fn output_num(&self) -> usize {
        let mut res : usize = 0;
        let mut base : usize = 1;
        for d in self.output_nums().into_iter().rev() {
            res += base * d;
            base *= 10;
        }
        res
    }
}

pub fn count_easy_digits(entries : &Vec<Entry>) -> usize {
    entries.iter().map(
        |entry| entry.count_easy_digits()
    ).sum()
}