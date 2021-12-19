use std::collections::HashSet;

use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Scan {
    centre : (i64,i64,i64),
    points : Vec<(i64, i64, i64)>
}

fn read_point(string : &str) -> Option<(i64, i64, i64)> {
    let mut parts = string.split(',');
    if let Some(x) = parts.next().and_then(|x_s| x_s.parse::<i64>().ok()) {
        if let Some(y) = parts.next().and_then(|y_s| y_s.parse::<i64>().ok()) {
            if let Some(z) = parts.next().and_then(|z_s| z_s.parse::<i64>().ok()) {
                return Some((x,y,z))
            }
        }
    }
    None
}

fn vec_mult(scale : i64, (x,y,z) : &(i64,i64,i64)) -> (i64,i64,i64) {
    (scale * x, scale * y, scale * z)
}

fn vec_sum((x,y,z) : (i64,i64,i64), (a,b,c) : (i64,i64,i64)) -> (i64,i64,i64) {
    (x+a, y+b, z+c)
}

fn vec_diff((x,y,z) : (i64,i64,i64), (a,b,c) : (i64,i64,i64)) -> (i64,i64,i64) {
    (x-a, y-b, z-c)
}

fn manhatten_distance(a : (i64,i64,i64), b : (i64,i64,i64)) -> i64 {
    let (x,y,z) = vec_diff(a, b);
    x.abs() + y.abs() + z.abs()
}


fn vec_rotation(
    (x,y,z) : &(i64,i64,i64), 
    x_axis : &(i64,i64,i64), 
    y_axis : &(i64,i64,i64), 
    z_axis : &(i64,i64,i64)
) -> (i64,i64,i64) {
    vec_sum(
        vec_sum(
            vec_mult(*x, x_axis),
            vec_mult(*y, y_axis)
        ),
        vec_mult(*z, z_axis)
    )
}


impl Scan {
    fn from_string(string : &str) -> Scan {
        let points : Vec<(i64,i64,i64)> = string.lines().skip(1).filter_map(
            |line| read_point(line)
        ).collect();
        Scan{ points : points, centre : (0,0,0) }
    }

    fn rotation(
        &self, 
        x_axis : (i64,i64,i64), 
        y_axis : (i64,i64,i64), 
        z_axis : (i64,i64,i64)
    ) -> Scan {
        Scan { 
            points: self.points.iter().map(
                |pt| vec_rotation(pt, &x_axis, &y_axis, &z_axis)
            ).collect(),
            centre : self.centre
        }
    }

    fn rotations(&self) -> Vec<(((i64,i64,i64),(i64,i64,i64),(i64,i64,i64)),Scan)> {
        let mut rots : Vec<(((i64,i64,i64),(i64,i64,i64),(i64,i64,i64)),Scan)> = Vec::new();
        let axes = [(1,0,0), (0,1,0), (0,0,1)];
        let signs = [-1, 1];
        for x_axis in axes {
            for y_axis in axes {
                for z_axis in axes {
                    if x_axis != y_axis && x_axis != z_axis && y_axis != z_axis {
                        for x_sign in signs {
                            for y_sign in signs {
                                for z_sign in signs {
                                    rots.push((
                                        (vec_mult(x_sign, &x_axis),vec_mult(y_sign, &y_axis),vec_mult(z_sign, &z_axis))
                                        ,self.rotation(
                                    vec_mult(x_sign, &x_axis),
                                    vec_mult(y_sign, &y_axis),
                                    vec_mult(z_sign, &z_axis)
                                    )
                                ))
                            }
                            }
                        }
                    }
                }

            }
        } 
        rots
    }

    fn translation(&self, vec : (i64,i64,i64)) -> Scan {
        Scan { 
            points: self.points.iter().map(|pt| vec_sum(*pt, vec)).collect(),
            centre: vec_sum(self.centre, vec) 
        }
    }

    fn translations(&self, other : &HashSet<(i64,i64,i64)>) -> Vec<((i64,i64,i64),Scan)> {
        let diffs : HashSet<(i64,i64,i64)> = 
          HashSet::from_iter(
            self.points.iter().cartesian_product(
                other.iter()
            ).map(|(a , b)| vec_diff(*b, *a))
        );
        diffs.into_iter().map(
            |diff| (diff, self.translation(diff))
        ).collect()
    }

    fn overlaps(&self, pts : &HashSet<(i64,i64,i64)>) -> bool {
        self.points.iter().filter(
            |pt| pts.contains(*pt)
        ).count() >= 12
    }

    fn any_overlaps(&self, set : &HashSet<(i64,i64,i64)>) -> Option<Scan> {
        for (rot, rotation) in self.rotations() {
            for (vec, translation) in rotation.translations(set) {
                if translation.overlaps(&set) {
                    println!("Found with vec, rot: {:?} {:?}", vec, rot);
                    return Some(translation);
                }
            }
        }
        None
    }

    fn find_consistent(&self, scans : &Vec<Scan>) -> Option<Scan> {
        self.any_overlaps(&unique_point_set(scans))
    }
}

pub fn scans_from_string(string : &str) -> Vec<Scan> {
    string.split("\n\n").map(
        |chunk| Scan::from_string(chunk)
    ).collect()
}

pub fn make_consistent(scans : Vec<Scan>) -> Vec<Scan> {
    let mut consistent_set = vec![scans.first().unwrap().clone()];
    let mut working_set : HashSet<Scan> = HashSet::from_iter(
        scans.iter().skip(1).cloned()
    );
    while !working_set.is_empty() {
        let mut found_res : Option<Scan> = None;
        let mut found_initial : Option<Scan> = None;
        for attempt in working_set.iter() {
            if let Some(res) = attempt.find_consistent(&consistent_set) {
                found_res = Some(res);
                found_initial = Some(attempt.clone());
                break;
            }
        }
        working_set.remove(&found_initial.unwrap());
        consistent_set.push(found_res.unwrap());
    }
    consistent_set
}

pub fn unique_point_set(scans : &Vec<Scan>) -> HashSet<(i64,i64,i64)> {
    let mut pts : HashSet<(i64,i64,i64)> = HashSet::new();
    for scan in scans {
        for pt in &scan.points {
            pts.insert(*pt);
        }
    }
    pts
}

pub fn max_distance(scans : &Vec<Scan>) -> i64 {
    scans.iter().cartesian_product(scans.iter()).map(
        |(a,b)| manhatten_distance(a.centre,b.centre)
    ).max().unwrap()
}

pub fn unique_points(scans : &Vec<Scan>) -> usize {
    unique_point_set(scans).len()
}
