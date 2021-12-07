pub struct Crab {
    pub pos : i64
}


fn distance(crabs : &Vec<Crab>, target : i64) -> u64 {
    crabs.iter().map(
        |crab| crab.to_target(target)
    ).sum()
}


fn distance_quad(crabs : &Vec<Crab>, target : i64) -> u64 {
    crabs.iter().map(
        |crab| crab.to_target_quad(target)
    ).sum()
}


pub fn minimum_distance(crabs : &Vec<Crab>) -> u64 {
    let min_pos = crabs.iter().map(|crab| crab.pos).min().unwrap();
    let max_pos = crabs.iter().map(|crab| crab.pos).max().unwrap();
    (min_pos..(max_pos+1)).map(
        |target| distance(crabs, target)
    ).min().unwrap()
}

pub fn minimum_distance_quad(crabs : &Vec<Crab>) -> u64 {
    let min_pos = crabs.iter().map(|crab| crab.pos).min().unwrap();
    let max_pos = crabs.iter().map(|crab| crab.pos).max().unwrap();
    (min_pos..(max_pos+1)).map(
        |target| distance_quad(crabs, target)
    ).min().unwrap()
}

impl Crab {
    fn to_target(&self, target : i64) -> u64 {
        (self.pos - target).abs() as u64
    }

    fn to_target_quad(&self, target : i64) -> u64 {
        let diff = self.to_target(target);
        if diff == 0 {
            0
        } else {
            (diff * (diff + 1)) / 2
        }
    }
}