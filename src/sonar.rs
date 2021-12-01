pub fn number_increases(readings : &Vec<i64>) -> usize {
    let pairs = readings.iter().zip(readings.iter().skip(1));
    pairs.filter(|(x,y)| *x < *y).count()
}


pub fn sliding_number_increases(readings : &Vec<i64>, window_size : usize) -> usize {
    let sliding_windows : Vec<i64> = readings.windows(window_size).map(
        |slice| slice.iter().sum()
    ).collect();
    number_increases(&sliding_windows)
}