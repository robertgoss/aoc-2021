use std::collections::{HashSet, BTreeSet};

use itertools::Itertools;

#[derive(Debug)]
pub struct StateMap {
    octopus_state : Vec<Vec<u8>>
}

impl StateMap {
    pub fn from_lines(lines : Vec<String>) -> StateMap
    {
        StateMap {
            octopus_state : lines.iter().map(
                |line| line.chars().filter_map(|c| c.to_digit(10).map(|n| n as u8)).collect()
            ).collect()
        }
    }

    fn size(&self) -> (usize, usize) {
        if self.octopus_state.is_empty() {
            (0,0)
        } else {
            (self.octopus_state.len(), self.octopus_state.first().unwrap().len())
        }
    }

    pub fn simulate(&mut self, steps : usize) -> usize {
        (0..steps).map(
            |_| self.simulate_once()
        ).sum()
    } 

    pub fn simulate_till_flash(&mut self) -> usize {
        let mut index = 0;
        let (h,w) = self.size();
        let total = h * w;
        while self.simulate_once() < total {
            index += 1
        }
        index + 1
    } 

    fn neighbour_indices(&self, i : usize, j : usize) -> Vec<(usize, usize)> {
        if i > 0 {
            if j > 0 {
                vec!((i-1,j-1), (i-1,j), (i-1,j+1), (i, j-1), (i,j+1), (i+1,j-1), (i+1, j), (i+1,j+1))  
            } else {
                vec!((i-1,j), (i-1,j+1), (i,j+1), (i+1, j), (i+1,j+1))  
            }
        } else {
            if j > 0 {
                vec!((i, j-1), (i,j+1), (i+1,j-1), (i+1, j), (i+1,j+1))  
            } else {
                vec!((i,j+1), (i+1, j), (i+1,j+1))   
            }
        }
    }

    fn increase(&mut self) {
        let (h, w) = self.size();
        (0..h).cartesian_product(0..w).for_each(
            |(i,j)| self.octopus_state[i][j] += 1
        )
    }

    fn ready(&self) -> Vec<(usize, usize)> {
        let (h, w) = self.size();
        (0..h).cartesian_product(0..w).filter(
            |(i,j)| self.octopus_state[*i][*j] > 9
        ).collect()
    }

    fn unflashed(&self, i : usize, j : usize) -> bool {
        self.octopus_state.get(i).and_then(
            |row| row.get(j)
        ).map(
            |val| *val <= 9
        ).unwrap_or(false)
    }

    fn flash(&mut self, flash_i : usize, flash_j : usize) -> Vec<(usize, usize)> {
        let neighbours : Vec<(usize,usize)> = self.neighbour_indices(flash_i, flash_j).into_iter().filter(
            |(i, j)| self.unflashed(*i, *j)
        ).collect();
        neighbours.iter().for_each(
            |(i,j)| self.octopus_state[*i][*j] = std::cmp::min(10, self.octopus_state[*i][*j]+1)
        );
        neighbours.into_iter().filter(
            |(i, j)| !self.unflashed(*i, *j) 
        ).collect()
    }

    fn simulate_once(&mut self) -> usize {
        self.increase();
        let mut flashed : HashSet<(usize, usize)> = HashSet::new();
        let mut to_flash : BTreeSet<(usize, usize)> = BTreeSet::from_iter(self.ready().into_iter());
        while let Some(flash_index) = to_flash.pop_first() {
            for neighbour in self.flash(flash_index.0, flash_index.1) {
                if !flashed.contains(&neighbour) {
                    to_flash.insert(neighbour);
                }
            }
            flashed.insert(flash_index);
        }
        flashed.iter().for_each(
            |(i,j)| self.octopus_state[*i][*j] = 0
        );
        flashed.len()
    } 
}