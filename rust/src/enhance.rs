use std::collections::HashSet;

pub struct Map {
    lookup_vec : Vec<bool>,
    lit_pixels : HashSet<(i64,i64)>,
    inverted : bool
}

fn read_lookup(string : &str) -> Vec<bool> {
    string.chars().map(
        |ch| ch == '#'
    ).collect()
}

fn read_map<'a,I>(lines : I) -> HashSet<(i64,i64)> 
  where I : Iterator<Item = &'a str> 
{
    let mut map : HashSet<(i64,i64)> = HashSet::new();
    for (i, line) in lines.enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if ch == '#' {
                map.insert((i as i64,j as i64));
            }
        }
    }
    map
}

impl Map {
    pub fn from_string(string : &str) -> Option<Map> {
        if let Some(first) = string.lines().next() {
            return Some(Map {
                lookup_vec :read_lookup(first),
                lit_pixels : read_map(string.lines().skip(2)),
                inverted : false
            })
        }
        None
    }

    pub fn enhance(&mut self, count : usize) {
        for _ in 0..count {
            self.enhance_once();
        }
    }

    fn ranges_x(&self) -> (i64,i64) {
        (
            *self.lit_pixels.iter().map(|(i,_)| i).min().unwrap(),
            *self.lit_pixels.iter().map(|(i,_)| i).max().unwrap()+1
        )
    }

    fn ranges_y(&self) -> (i64,i64) {
        (
            *self.lit_pixels.iter().map(|(_,j)| j).min().unwrap(),
            *self.lit_pixels.iter().map(|(_,j)| j).max().unwrap()+1
        )
    }

    fn ranges(&self) -> ((i64,i64), (i64,i64)) {
        (self.ranges_x(), self.ranges_y())
    }

    fn get(&self, i : i64, j : i64) -> bool {
        if self.inverted {
            !self.lit_pixels.contains(&(i,j))
        } else {
            self.lit_pixels.contains(&(i,j))
        }
    }

    pub fn get_bits(&self, i : i64, j : i64) -> [bool; 9] {
        [
            self.get(i-1, j-1),
            self.get(i-1, j),
            self.get(i-1, j+1),
            self.get(i, j-1),
            self.get(i, j),
            self.get(i, j+1),
            self.get(i+1, j-1),
            self.get(i+1, j),
            self.get(i+1, j+1),
        ]
    }

    pub fn get_index(&self, i : i64, j : i64) -> usize {
        let mut index : usize = 0;
        for bit in self.get_bits(i,j) {
            index *= 2;
            if bit {
                index += 1;
            }
        }
        index
    }

    fn enhance_once(&mut self) {
        let (range_x, range_y) = self.ranges();
        let mut next_lit : HashSet<(i64,i64)> = HashSet::new();
        let next_inverted = self.lookup_vec[0] && !self.inverted;
        for i in (range_x.0-3) .. (range_x.1+3) {
            for j in (range_y.0-3) .. (range_y.1+3) {
                let index = self.get_index(i, j);
                if next_inverted {
                    if !*self.lookup_vec.get(index).unwrap() {
                        next_lit.insert((i, j));
                    } 
                } else {
                    if *self.lookup_vec.get(index).unwrap() {
                        next_lit.insert((i, j));
                    } 
                }
            }
        }
        self.lit_pixels = next_lit;
        self.inverted = next_inverted;
    }

    pub fn lit(&self) -> usize {
        assert!(!self.inverted, "Infinite");
        self.lit_pixels.len()
    }
}