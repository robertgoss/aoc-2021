use itertools::Itertools;

#[derive(Clone)]
pub enum Snailfish {
    Literal(usize),
    Pair(Box<Snailfish>, Box<Snailfish>)
}

fn read_number<'a>(string : &'a str) -> Option<(usize,&'a str)> {
    let prefix = string.chars().take_while(|c| c.is_numeric()).collect::<String>();
    if let Some(num) = prefix.parse::<usize>().ok() {
        return Some((
            num, 
            string.trim_start_matches(|c : char| c.is_numeric())
        ));
    }
    None
}

fn read_char<'a>(string : &'a str) -> Option<(char, &'a str)> {
    string.chars().next().map(
        |ch| (ch, &string[1..])
    )
}

pub fn sum(nums : Vec<Snailfish>) -> usize {
    let mut num = nums[0].clone();
    for fish in nums.iter().skip(1) {
        num = num.add(fish);
    }
    num.magnitude()
}

pub fn greatest_magnitude(nums : Vec<Snailfish>) -> usize {
    nums.iter().cartesian_product(nums.iter()).map(
        |(x,y)| x.add(y).magnitude()
    ).max().unwrap()
}

impl Snailfish {
    pub fn from_string(string : &str) -> Option<Snailfish> {
        Snailfish::from_string_parital(string).map(|res| res.0) 
    }

    fn get_val(&self) -> Option<usize> {
        if let Snailfish::Literal(val) = self {
            return Some(*val);
        }
        None
    }

    fn from_string_parital<'a>(string : &'a str) -> Option<(Snailfish,&'a str)> {
        if let Some((ch, _)) = read_char(string) {
            if ch == '[' {
                return Snailfish::pair_from_string(string);
            } else {
                return Snailfish::literal_from_string(string);
            }
        } 
        None
    }

    fn literal_from_string<'a>(string : &'a str) -> Option<(Snailfish,&'a str)> {
        read_number(string).map(
            |(num, rest)| (Snailfish::Literal(num), rest)
        )
    }

    fn pair_from_string<'a>(string : &'a str) -> Option<(Snailfish,&'a str)> {
        if let Some(('[', first_s)) = read_char(string) {
            if let Some((first_f, mid_s)) = Snailfish::from_string_parital(first_s) {
                if let Some((',', second_s)) = read_char(mid_s) {
                    if let Some((second_f, end_s)) = Snailfish::from_string_parital(second_s) {
                        if let Some((']', rest)) = read_char(end_s) {
                            return Some((
                                Snailfish::Pair(Box::new(first_f), Box::new(second_f)),
                                rest
                            ));
                        }
                    }
                }
            }
        }
        None
    }

    fn reduce(&mut self) {
        let mut exploded = true;
        let mut split = true;
        while exploded || split {
            split = false;
            exploded = self.try_explode(0).is_some();
            if !exploded {
                split = self.try_split();
            }
        }
    }

    fn try_explode(&mut self, depth : usize) -> Option<(Option<usize>, Option<usize>)> {
        if let Snailfish::Pair(left, right) = self {
            if depth == 4 {
                let left_val = left.get_val().unwrap();
                let right_val = right.get_val().unwrap();
                *self = Snailfish::Literal(0);
                return Some((Some(left_val), Some(right_val)));
            } else {
                if let Some((left_add, right_add)) = left.try_explode(depth + 1) {
                    if let Some(right_val) = right_add {
                        right.add_leftmost(right_val);
                    }
                    return Some((left_add, None));
                }
                if let Some((left_add, right_add)) = right.try_explode(depth + 1) {
                    if let Some(left_val) = left_add {
                        left.add_rightmost(left_val);
                    }
                    return Some((None, right_add));
                }
            }
        }
        None
    }

    fn add_leftmost(&mut self, add : usize) {
        match self {
            Self::Literal(val) => {
                *val += add;
            },
            Self::Pair(left, _) => {
                left.add_leftmost(add);
            }
        }
    }

    fn add_rightmost(&mut self, add : usize) {
        match self {
            Self::Literal(val) => {
                *val += add;
            },
            Self::Pair(_, right) => {
                right.add_rightmost(add);
            }
        }
    }

    fn try_split(&mut self) -> bool {
        if let Some(val) = self.get_val() {
            if val >= 10 {
                let left = Snailfish::Literal(val / 2);
                let right = Snailfish::Literal((val+1) / 2);
                *self = Snailfish::Pair(
                    Box::new(left),
                    Box::new(right)
                );
                return true;
            }
        } else {
            if let Snailfish::Pair(left, right) = self {
                if left.try_split() {
                    return true;
                }
                if right.try_split() {
                    return true;
                }
            }
        }
        false
    }

    fn add(&self, right : &Self) -> Self {
        let mut pair = Snailfish::Pair(
            Box::new(self.clone()),
            Box::new(right.clone())
        );
        pair.reduce();
        pair
    }

    fn magnitude(&self) -> usize {
        match self {
            Self::Literal(val) => *val,
            Self::Pair(l, r) => 
                3*l.magnitude() + 2 * r.magnitude()
        }
    }
}