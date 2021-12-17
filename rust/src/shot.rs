use itertools::Itertools;

pub struct Target {
    range_x : (i64, i64),
    range_y : (i64, i64)
}

struct Shot {
    pos : (i64, i64),
    vel : (i64, i64)
}

fn read_range(string : &str) -> Option<(i64, i64)> {
    if let Some((min_s, max_s)) = string.split_once("..") {
        if let Some(min) = min_s.parse::<i64>().ok() {
            if let Some(max) = max_s.parse::<i64>().ok() {
                return Some((min, max))
            }
        }
    }
    None
}

fn in_range(range : (i64, i64), val : i64) -> bool {
    range.0 <= val && val <= range.1
}

impl Target {
    pub fn from_string(string : &str) -> Option<Target> {
        if let Some(main_s) = string.strip_prefix("target area: x=") {
            if let Some((x_s, y_s)) = main_s.split_once(", y=") {
                if let Some(range_x) = read_range(x_s) {
                    if let Some(range_y) = read_range(y_s) {
                        return Some(Target {
                            range_x : range_x,
                            range_y : range_y
                        })
                    }
                }
            }
        }
        None
    }

    fn could_hit(&self, pos : (i64, i64)) -> bool {
        pos.0 <= self.range_x.1 && pos.1 >= self.range_y.0
    }

    fn hits(&self, pos : (i64, i64)) -> bool {
        in_range(self.range_x, pos.0) && 
        in_range(self.range_y, pos.1)
    }
}

impl Shot {
    fn new(dx : i64, dy: i64) -> Shot {
        Shot { pos: (0, 0), vel: (dx, dy) }
    }

    fn simulate(&mut self, target : &Target) -> Option<Vec<(i64, i64)>> {
        let mut pos : Vec<(i64,i64)> = vec!(self.pos);
        while target.could_hit(self.pos) {
            self.pos.0 += self.vel.0;
            self.pos.1 += self.vel.1;
            pos.push(self.pos);
            if self.vel.0 > 1 {
                self.vel.0 -= 1;
            }
            if self.vel.0 < -1 {
                self.vel.0 += 1;
            }
            self.vel.1 -= 1;
            if target.hits(self.pos) {
                return Some(pos);
            }
        }
        None
    }
}

pub fn simulate(dx : i64, dy :i64, target : &Target) {
    let mut shot : Shot = Shot::new(dx, dy);
    let res = shot.simulate(target);
    println!("{:?}", res);
}

fn simulate_y(dx : i64, dy :i64, target : &Target) -> Option<i64> {
    let mut shot : Shot = Shot::new(dx, dy);
    shot.simulate(target).map(
        |pos| pos.iter().map(|p| p.1).max().unwrap_or(0)
    )
}

pub fn find_highest_y(target : &Target) -> i64 {
    let max_v = target.range_x.1;
    (0..max_v).cartesian_product(
        0..max_v*100
    ).filter_map(
        |(dx,dy)| simulate_y(dx, dy, target)
    ).max().unwrap()
}