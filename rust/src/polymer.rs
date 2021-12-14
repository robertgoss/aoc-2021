use itertools::Itertools;
use std::collections::HashMap;

fn combine_maps(map1 : &HashMap<char, usize>, map2 : &HashMap<char, usize>) -> HashMap<char, usize> {
    let mut new_map = map1.clone();
    for (k, v) in map2.iter() {
        let counter = new_map.entry(*k).or_insert(0);
        *counter += *v;
    }
    new_map
}  

#[derive(PartialEq,Eq,PartialOrd, Ord,Hash)]
struct PairCacheKey {
    fst : char,
    snd : char,
    steps : usize
}

struct PairCache {
    rules : Vec<Rule>,
    cache : HashMap<PairCacheKey, HashMap<char, usize>>
}

impl PairCache {
    fn new(rules : Vec<Rule>) -> PairCache {
        PairCache {
            rules : rules,
            cache : HashMap::new()
        }
    }

    fn count(&mut self, fst : char, snd : char, steps : usize) -> HashMap<char, usize> {
        let key = PairCacheKey {fst : fst, snd : snd, steps : steps};
        if let Some(cached_res) = self.cache.get(&key) {
            return cached_res.clone();
        }
        let res = self.count_impl(fst, snd, steps);
        self.cache.insert(key, res.clone());
        res
    }

    fn count_impl(&mut self, fst : char, snd : char, steps : usize) -> HashMap<char, usize> {
        if steps > 0 {
            if let Some(mid) = self.apply(fst, snd) {
                let mut res = combine_maps(
                    &self.count(fst, mid, steps - 1),
                    &self.count(mid, snd, steps - 1),
                );
                // Remove overlap
                let overlap_counter = res.entry(mid).or_insert(1);
                *overlap_counter -= 1;
                return res;
            }
        }
        if fst == snd {
            HashMap::from_iter([(fst, 2)])
        } else {
            HashMap::from_iter([(fst, 1), (snd, 1)])
        }
    }

    fn apply(&self, fst : char, snd : char) -> Option<char> {
        self.rules.iter().filter_map(
            |rule| rule.apply(fst, snd)
        ).next()
    }
}

#[derive(Clone)]
struct Rule {
    input : (char, char),
    output : char
}

impl Rule {
    fn from_string(string : &str) -> Option<Rule> {
        if let Some( (fst,snd) ) = string.split_once(" -> ") {
            if let Some(in1) = fst.chars().next() {
                if let Some(in2) = fst.chars().nth(1) {
                    if let Some(out) = snd.chars().next() {
                        return Some(Rule{ input : (in1,in2), output : out });
                    }
                }
            }
        }
        None
    }

    fn apply(&self, fst : char, snd : char) -> Option<char> {
        if fst == self.input.0 && snd == self.input.1 {
            Some(self.output)
        } else {
            None
        }
    }
}

pub struct PolymerProgram {
    template : String,
    rules : Vec<Rule>
}

impl PolymerProgram {
    pub fn from_string(string : &str) -> Option<PolymerProgram> {
        string.split_once("\n\n").map(
            |(template, rules_s)| {  
                let rules :Vec<Rule> = rules_s.split('\n').filter_map(
                    |rule_s| Rule::from_string(rule_s)
                ).collect();
                PolymerProgram {template: template.to_string(), rules : rules}
            }
        )
    }

    pub fn calculate_common(&self, steps : usize) -> (usize, usize) {
        let map = self.calculate_counts(steps);
        (*map.values().max().unwrap(), *map.values().min().unwrap())
    }

    fn calculate_counts(&self, steps : usize) -> HashMap<char, usize> {
        let mut counter : PairCache = PairCache::new(self.rules.clone());
        let mut counts = self.template.chars().tuple_windows().map(
            |(fst, snd)| counter.count(fst, snd, steps)
        ).reduce(
            |map1, map2| combine_maps(&map1, &map2)  
        ).unwrap();
        // Remove overlaps
        self.template.chars().tuple_windows().for_each(
            |(_,mid,  _)| {
                let overlap_counter = counts.entry(mid).or_insert(1);
                *overlap_counter -= 1;
            }
        );
        counts
    }
}