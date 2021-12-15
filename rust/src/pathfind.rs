use petgraph::graphmap::DiGraphMap;
use petgraph::algo::dijkstra;
use itertools::Itertools;

pub struct RiskMap {
    risks : Vec<Vec<u8>>
}

pub struct EnlargedRiskMap {
    underlying : RiskMap,
    repeat : usize
}

pub trait Pathfinder {
    fn size(&self) -> (usize, usize);
    fn get(&self, i : usize, j : usize) -> Option<u8>;


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

    fn neighbours(&self, i : usize, j : usize) -> Vec<(usize, usize, u8)> {
        self.neighbour_indices(i, j).into_iter().filter_map(
            |(i,j)| self.get(i,j).map(|val| (i,j,val))
        ).collect()
    }

    fn risk_graph(&self) -> DiGraphMap<(usize,usize), usize> {
        let (w, h) = self.size();
        let mut graph : DiGraphMap<(usize,usize), usize> = DiGraphMap::new();
        (0..w).cartesian_product(0..h).for_each(
            |pos| { graph.add_node(pos); }
        );
        (0..w).cartesian_product(0..h).for_each(
            |(i,j)| {
                self.neighbours(i, j).into_iter().for_each(
                    |(i_end, j_end, val)| { 
                        graph.add_edge((i,j), (i_end, j_end), val as usize); 
                    }
                )
            }
        );
        graph
    }

    fn safest_path(&self) -> usize {
        let graph = self.risk_graph();
        let (w,h) = self.size();
        let end = (w-1, h-1);
        let path_risks = dijkstra(
            &graph, 
            (0,0), 
            Some(end), 
            |edge| *edge.2
        );
        *path_risks.get(&end).unwrap()
    }
}

impl RiskMap {
    pub fn from_lines(lines : Vec<String>) -> RiskMap
    {
        RiskMap {
            risks : lines.iter().map(
                |line| line.chars().filter_map(|c| c.to_digit(10).map(|n| n as u8)).collect()
            ).collect()
        }
    }

    pub fn enlarge(self, repeat : usize) -> EnlargedRiskMap {
        EnlargedRiskMap {
            underlying : self,
            repeat : repeat
        }
    }
}

impl Pathfinder for RiskMap {
    fn get(&self, i : usize, j : usize) -> Option<u8> {
        self.risks.get(i).and_then(
            |row| row.get(j).cloned()
        )
    }


    fn size(&self) -> (usize, usize) {
        if self.risks.is_empty() {
            (0,0)
        } else {
            (self.risks.len(), self.risks.first().unwrap().len())
        }
    }
}

fn wrap_around(val : u8) -> u8 {
    if val <= 9 {
        val
    } else {
        val - 9
    }
}

impl Pathfinder for EnlargedRiskMap {
    fn get(&self, i : usize, j : usize) -> Option<u8> {
        let (w, h) = self.underlying.size();
        let shift_i = (i / w) as u8;
        let shift_j = (j / h) as u8;
        self.underlying.get(i % w,j % h).map(
            |val| wrap_around(val + shift_i + shift_j)
        )
    }


    fn size(&self) -> (usize, usize) {
        let (w, h) = self.underlying.size();
        (w * self.repeat, h * self.repeat)
    }
}

