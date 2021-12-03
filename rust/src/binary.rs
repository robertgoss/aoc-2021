pub fn gamma(data : &Vec<u64>) -> u64 {
    let mut res = 0;
    for i in 0..64 {
        if one_most_common(i, data) {
            res += 1 << i;
        }
    }
    res
}

pub fn epsilon(data : &Vec<u64>) -> u64 {
    let mut res = 0;
    for i in 0..(max_index(data)+1) {
        if !one_most_common(i, data) {
            res += 1 << i;
        }
    }
    res
}

pub fn oxygen(data : &Vec<u64>) -> u64 {
    let mut filtered = data.clone();
    for i in (0..(max_index(data)+1)).rev() {
        if one_most_common(i, &filtered) {
            filtered = filtered.into_iter().filter(
                |n| bit_set(i, *n)
            ).collect();
        } else {
            filtered = filtered.into_iter().filter(
                |n| !bit_set(i, *n)
            ).collect();
        }
        if filtered.len() == 1 {
            return filtered[0]
        }
    }
    0
}

pub fn carbon(data : &Vec<u64>) -> u64 {
    let mut filtered = data.clone();
    for i in (0..(max_index(data)+1)).rev() {
        if one_most_common(i, &filtered) {
            filtered = filtered.into_iter().filter(
                |n| !bit_set(i, *n)
            ).collect();
        } else {
            filtered = filtered.into_iter().filter(
                |n| bit_set(i, *n)
            ).collect();
        }
        if filtered.len() == 1 {
            return filtered[0]
        }
    }
    0
}

fn max_index(data : &Vec<u64>) -> u8 {
    data.iter().map(
        |n| max_bit(*n)
    ).max().unwrap_or(0)
}

fn one_most_common(i : u8, data: &Vec<u64>) -> bool {
    let num = data.iter().filter(
        |n| bit_set(i, **n)
    ).count();
    num >= (data.len()+1) / 2
}

fn max_bit(n : u64) -> u8 {
    (0..64).filter(
        |i| bit_set(*i, n)
    ).max().unwrap_or(0)
}

fn bit_set(i:u8, n : u64) -> bool {
    let mask = 1 << i;
    n & mask != 0
}