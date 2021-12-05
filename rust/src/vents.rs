use itertools::{iproduct};

#[derive(Clone, Debug)]
pub struct Vent {
    start : (i64, i64),
    end : (i64, i64)
}

fn parse_pos(string : &str) -> Option<(i64, i64)> {
    if let Some( (x_str , y_str) ) = string.split_once(",") {
        if let Ok(x) = x_str.parse::<i64>() {
            if let Ok(y) = y_str.parse::<i64>() {
                return Some( (x,y) );
            }
        }
    }
    return None;
}

impl Vent {
    pub fn from_string(string : &str) -> Option<Vent> {
        if let Some( (start_str, end_str) ) = string.split_once(" -> ") {
            if let Some( start ) = parse_pos(start_str) {
                if let Some( end ) = parse_pos(end_str) {
                    return Some(Vent{start : start, end : end})
                }
            }
        }
        return None;
    }

    fn is_cardinal(&self) -> bool {
        self.start.0 == self.end.0 || self.start.1 == self.end.1
    }

    fn hits_point(&self, x : i64, y : i64) -> bool {
        if self.start.0 == self.end.0 {
            let min_y = std::cmp::min(self.start.1, self.end.1);
            let max_y = std::cmp::max(self.start.1, self.end.1);
            return x == self.start.0 && y <= max_y && y >= min_y; 
        }
        if self.start.0 < self.end.0 {
            let delta_x = x - self.start.0;
            let delta_y = ((self.end.1 - self.start.1) * delta_x) / (self.end.0 - self.start.0);
            let interp_y = self.start.1 + delta_y;
            x >= self.start.0 && x <= self.end.0 && interp_y == y
        } else {
            let delta_x = x - self.end.0;
            let delta_y = ((self.start.1 - self.end.1) * delta_x) / (self.start.0 - self.end.0);
            let interp_y = self.end.1 + delta_y;
            x >= self.end.0 && x <= self.start.0 && interp_y == y
        }
    }
}

fn x_range(vents : &Vec<Vent>) -> (i64, i64) {
    let x_min = vents.iter().map(
        |vent| std::cmp::min(vent.start.0, vent.end.0)
    ).min().unwrap();
    let x_max = vents.iter().map(
        |vent| std::cmp::max(vent.start.0, vent.end.0)
    ).max().unwrap();
    (x_min, x_max)
}

fn y_range(vents : &Vec<Vent>) -> (i64, i64) {
    let y_min = vents.iter().map(
        |vent| std::cmp::min(vent.start.1, vent.end.1)
    ).min().unwrap();
    let y_max = vents.iter().map(
        |vent| std::cmp::max(vent.start.1, vent.end.1)
    ).max().unwrap();
    (y_min, y_max)
}

fn has_overlap(vents : &Vec<Vent>, x: i64, y: i64) -> bool {
    vents.iter().filter(
        |vent| vent.hits_point(x, y)
     ).count() > 1
}

pub fn get_overlap_num(vents : &Vec<Vent>) -> usize {
    let (x_min, x_max) = x_range(vents);
    let (y_min, y_max) = y_range(vents);
    iproduct!(x_min..(x_max+1), y_min..(y_max+1)).filter(
        |(x,y)| has_overlap(vents, *x, *y)
    ).count()
}

pub fn get_overlap_num_cardinal(vents : &Vec<Vent>) -> usize {
    let filtered : Vec<Vent> = vents.iter().filter(
        |vent| vent.is_cardinal()
    ).cloned().collect();
    get_overlap_num(&filtered)
}