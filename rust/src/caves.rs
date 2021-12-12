use std::collections::HashMap;
use std::collections::HashSet;
use petgraph::graphmap::UnGraphMap;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Cavern {
    id : u32,
    large : bool
}

impl Cavern {
    fn from_string(string : &str, nodemap : &mut HashMap<String, Cavern>, current_max : &mut u32) -> Cavern {
        let is_large = string.chars().next().unwrap_or('a').is_ascii_uppercase();
        let cave = *nodemap.entry(string.to_string()).or_insert_with(
            || {
                let node_id = *current_max;
                *current_max += 1;
                Cavern {id : node_id, large : is_large}
            }
        );
        cave
    }
}
pub struct CaveSystem {
    nodemap : HashMap<String, Cavern>,
    graph : UnGraphMap<Cavern, ()>
}

impl CaveSystem {
    pub fn from_lines(lines : Vec<String>) -> CaveSystem {
        let mut nodemap : HashMap<String, Cavern> = HashMap::new();
        let mut current_max : u32 = 0;
        let edges : Vec<(Cavern, Cavern)> = lines.into_iter().filter_map(
            |line| {
                if let Some((node1, node2)) = line.split_once("-") {
                let cave1 = Cavern::from_string(node1, &mut nodemap, &mut current_max);
                let cave2 = Cavern::from_string(node2, &mut nodemap, &mut current_max);
                Some( (cave1, cave2) )
                } else {
                    None
                }
            }
        ).collect();
        CaveSystem {
            nodemap : nodemap,
            graph : UnGraphMap::from_edges(
                edges.into_iter()
            )
        }
    }

    pub fn number_paths(&self) -> usize {
        let mut restrictions : HashSet<Cavern> = HashSet::new();
        let start = *self.nodemap.get("start").unwrap();
        let end = *self.nodemap.get("end").unwrap();
        self.number_paths_restricted(start, end, &mut restrictions)
    }

    pub fn number_paths_single_reentry(&self) -> usize {
        let mut restrictions : HashSet<Cavern> = HashSet::new();
        let start = *self.nodemap.get("start").unwrap();
        let end = *self.nodemap.get("end").unwrap();
        self.number_paths_restricted_reentry(start, end, &mut restrictions, start)
    }

    fn number_paths_restricted(&self, from : Cavern, to : Cavern, restrictions : &mut HashSet<Cavern>) -> usize {
        if from == to {
            return 1;
        }
        let mut sum = 0;
        for neighbour in self.graph.neighbors(from) {
            if !restrictions.contains(&neighbour) {
                if !from.large {
                    restrictions.insert(from);
                    sum += self.number_paths_restricted(neighbour, to, restrictions);
                    restrictions.remove(&from);
                } else {
                    sum += self.number_paths_restricted(neighbour, to, restrictions);
                }
            }
        }
        sum
    }

    fn number_paths_restricted_reentry(&self, from : Cavern, to : Cavern, restrictions : &mut HashSet<Cavern>, start : Cavern) -> usize {
        if from == to {
            return 1;
        }
        let mut sum = 0;
        for neighbour in self.graph.neighbors(from) {
            if !restrictions.contains(&neighbour) {
                if !from.large {
                    restrictions.insert(from);
                    sum += self.number_paths_restricted_reentry(neighbour, to, restrictions, start);
                    restrictions.remove(&from);
                } else {
                    sum += self.number_paths_restricted_reentry(neighbour, to, restrictions, start);
                }
            } else {
                if neighbour != start {
                    restrictions.remove(&neighbour);
                    if !from.large {
                        restrictions.insert(from);
                        sum += self.number_paths_restricted(neighbour, to, restrictions);
                        restrictions.remove(&from);
                    } else {
                        sum += self.number_paths_restricted(neighbour, to, restrictions);
                    }
                    restrictions.insert(neighbour);
                }
            }
        }
        sum
    }
}