use std::{ops::Range, collections::HashSet};

use itertools::iproduct;


#[derive(Debug, Clone, PartialEq, Eq)]
struct Cube {
    x_range : (i64,i64),
    y_range : (i64,i64),
    z_range : (i64,i64)
}

fn read_cube_line(string : &str) -> Option<(bool, Cube)> {
    string.split_once(" ").and_then(
        |(sign_s, cube_s)| Cube::from_string(cube_s).map(|cube| (sign_s == "on", cube))
    )
}

fn read_range(string : &str) -> Option<(i64,i64)> {
    if let Some((_,range_s)) = string.split_once("=") {
        if let Some((min_s, max_s)) = range_s.split_once("..") {
            if let Some(min_v) = min_s.parse::<i64>().ok() {
                if let Some(max_v) = max_s.parse::<i64>().ok() {
                   return Some((min_v, max_v+1));
                }
            }
        }
    }
    None
}

fn intersect_range((x,y) : (i64,i64), (a,b) : (i64,i64)) -> Option<(i64,i64)> {
    let min_v = std::cmp::max(x,a);
    let max_v = std::cmp::min(y,b);
    if min_v <= max_v {
        Some((min_v, max_v))
    } else {
        None
    }
}

fn split_range((x,y) : (i64,i64), (a,b) : (i64,i64)) -> Vec<(i64, i64)> {
    let mut ranges : Vec<(i64, i64)> = vec!((a,b));
    if x < a {
        ranges.push((x, a));
    }
    if b < y {
        ranges.push((b, y));
    }
    ranges
}

impl Cube {
    fn from_string(string : &str) -> Option<Cube> {
        let ranges : Vec<(i64,i64)> = string.split(",").filter_map(
            |range_s| read_range(range_s)
        ).collect();
        if ranges.len() == 3 {
            Some(Cube{
                x_range : ranges[0],
                y_range : ranges[1],
                z_range : ranges[2]
            })
        } else {
            None
        }
    }

    fn size(&self) -> usize {
        let size_x = (self.x_range.1 -self.x_range.0) as usize;
        let size_y = (self.y_range.1 -self.y_range.0) as usize;
        let size_z = (self.z_range.1 -self.z_range.0) as usize;
        size_x * size_y * size_z
    }

    fn intersect(&self, other : &Cube) -> Option<Cube> {
        if let Some(range_x) = intersect_range(self.x_range, other.x_range) {
            if let Some(range_y) = intersect_range(self.y_range, other.y_range) {
                if let Some(range_z) = intersect_range(self.z_range, other.z_range) {
                    return Some(Cube{
                        x_range : range_x,
                        y_range : range_y,
                        z_range : range_z
                    })
                }
            }
        }
        None
    }

    fn range_x(&self) -> Range<i64> {
        self.x_range.0 .. self.x_range.1
    }

    fn range_y(&self) -> Range<i64> {
        self.y_range.0 .. self.y_range.1
    }

    fn range_z(&self) -> Range<i64> {
        self.z_range.0 .. self.z_range.1
    }

    fn split_x(&self, overlap_range : &(i64, i64)) -> Vec<Cube> {
        split_range(self.x_range, *overlap_range).into_iter().map(
            |x_range| Cube { x_range : x_range, y_range : self.y_range, z_range : self.z_range }
        ).collect()
    }

    fn split_y(&self, overlap_range : &(i64, i64)) -> Vec<Cube> {
        split_range(self.y_range, *overlap_range).into_iter().map(
            |y_range| Cube { y_range : y_range, x_range : self.x_range, z_range : self.z_range }
        ).collect()
    }

    fn split_z(&self, overlap_range : &(i64, i64)) -> Vec<Cube> {
        split_range(self.z_range, *overlap_range).into_iter().map(
            |z_range| Cube { z_range : z_range, y_range : self.y_range, x_range : self.x_range }
        ).collect()
    }

    fn remove(&self, to_remove : &Cube) -> Vec<Cube> {
        if let Some(overlap) = self.intersect(to_remove) {
            let mut sub_cubes : Vec<Cube> = Vec::new();
            for cube_x in self.split_x(&overlap.x_range) {
                for cube_y in cube_x.split_y(&overlap.y_range) {
                    for cube_z in cube_y.split_z(&overlap.z_range) {
                        if cube_z != overlap {
                            sub_cubes.push(cube_z)
                        }
                    }
                }
            }
            sub_cubes
        } else {
            vec!(self.clone())
        }
    }
}

fn remove_cube(cubes : &Vec<Cube>, to_remove : &Cube) -> Vec<Cube> {
    cubes.iter().map(
        |cube| cube.remove(to_remove)
    ).flatten().collect()
}

#[derive(Clone,Debug)]
pub struct CubeSet {
    steps : Vec<(bool, Cube)>
}

impl CubeSet {
    pub fn from_lines<'a,I>(lines : I) -> CubeSet
        where I : Iterator<Item = &'a String>
    {
        let steps = lines.filter_map(
            |line| read_cube_line(line)
        ).collect();
        CubeSet{
            steps : steps
        }
    }

    pub fn limit(&mut self) {
        let area = Cube {
            x_range : (-50, 51),
            y_range : (-50, 51),
            z_range : (-50, 51)
        };
        *self = self.limit_to(&area);
    } 

    fn limit_to(&self, area : &Cube) -> CubeSet {
        let new_steps = self.steps.iter().filter_map(
            |(on, cube)| area.intersect(cube).map(
                |new_cube| (*on, new_cube)
            )
        ).collect();
        CubeSet { steps: new_steps }
    }

    pub fn switched_on_basic(&self) -> usize {
        let mut lit : HashSet<(i64,i64,i64)> = HashSet::new();
        for (on, cube) in &self.steps {
            for pt in iproduct!(cube.range_x(), cube.range_y(), cube.range_z()) {
                if *on {
                    lit.insert(pt);
                } else {
                    lit.remove(&pt);
                }
            }
        }
        lit.len()
    }

    pub fn switched_on(&self) -> usize {
        let mut simple_cubes : Vec<Cube> = Vec::new();
        for (on, cube) in &self.steps {
            simple_cubes = remove_cube(&simple_cubes, cube);
            if *on {
                simple_cubes.push(cube.clone());
            }
        }
        simple_cubes.iter().map(
            |cube| cube.size()
        ).sum()
    }
}
