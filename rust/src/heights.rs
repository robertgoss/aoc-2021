use std::collections::{HashSet, BTreeSet};

use itertools::Itertools;

#[derive(Debug)]
pub struct HeightMap {
    heights : Vec<Vec<u8>>
}

impl HeightMap {
    pub fn from_lines(lines : Vec<String>) -> HeightMap
    {
        HeightMap {
            heights : lines.iter().map(
                |line| line.chars().filter_map(|c| c.to_digit(10).map(|n| n as u8)).collect()
            ).collect()
        }
    }

    fn size(&self) -> (usize, usize) {
        if self.heights.is_empty() {
            (0,0)
        } else {
            (self.heights.len(), self.heights.first().unwrap().len())
        }
    }

    fn get(&self, i : usize, j : usize) -> Option<u8> {
        self.heights.get(i).and_then(
            |row| row.get(j).cloned()
        )
    }

    fn neighbour_indices(&self, i : usize, j : usize) -> Vec<(usize, usize)> {
        if i > 0 {
            if j > 0 {
                vec!((i-1, j),(i, j-1),(i+1, j),(i, j+1))  
            } else {
                vec!((i-1, j),(i+1, j),(i, j+1))  
            }
        } else {
            if j > 0 {
                vec!((i, j-1),(i+1, j),(i, j+1))  
            } else {
                vec!((i+1, j),(i, j+1))  
            }
        }
    }

    fn neighbours(&self, i : usize, j : usize) -> Vec<u8> {
        self.neighbour_indices(i, j).into_iter().filter_map(
            |(i,j)| self.get(i,j)
        ).collect()
    }

    fn risk(&self, i : usize, j : usize) -> Option<usize> {
        if let Some(val) = self.get(i, j) {
            let low_point = self.neighbours(i, j).into_iter().all(
                |neighbour_val| neighbour_val > val
            );
            if low_point {
                Some(val as usize+1)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn non_edge_neighbours(&self, i : usize, j : usize) -> Vec<(usize, usize)> {
        self.neighbour_indices(i, j).into_iter().filter(
            |(n_i, n_j)| self.get(*n_i, *n_j).filter(|val| *val != 9).is_some()
        ).collect()
    }
    

    fn low_points(&self) -> Vec<(usize, usize)> {
        let (h, w) = self.size();
        (0..h).cartesian_product(0..w).filter(
            |(i,j)| self.risk(*i, *j).is_some()
        ).collect()
    }

    fn basin_size(&self, i : usize, j : usize) -> usize {
        let mut marked : HashSet<(usize, usize)> = HashSet::new();
        let mut working_set : BTreeSet<(usize, usize)> = BTreeSet::new();
        working_set.insert((i,j));
        while let Some(elem) = working_set.pop_first() {
            for neighbour in self.non_edge_neighbours(elem.0, elem.1) {
                if !marked.contains(&neighbour) {
                    working_set.insert(neighbour);
                }
            }
            marked.insert(elem);
        }
        marked.len()
    }

    pub fn largest_basins(&self, num : usize) -> Vec<usize> {
        self.low_points().into_iter().map(
            |(i,j)| self.basin_size(i, j)
        ).sorted().rev().take(num).collect()
    }

    pub fn total_risk(&self) -> usize {
        let (h, w) = self.size();
        (0..h).cartesian_product(0..w).filter_map(
            |(i,j)| self.risk(i,j)
        ).sum()
    }
}