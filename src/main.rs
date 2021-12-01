mod sonar;

mod io;

mod challenge {
    use super::io as io;
    use super::sonar as sonar;

    fn challenge_1() {
        let data = io::input_as_list(1);
        let res = sonar::number_increases(&data);
        println!("{}", res);
    }

    fn challenge_2() {
        let data = io::input_as_list(1);
        let res = sonar::sliding_number_increases(&data, 3);
        println!("{}", res);
    }

    pub fn challenge(num : u8) {
        match num {
            1 => challenge_1(),
            2 => challenge_2(),
            _ => () 
        }
    }
}

fn main() {
    challenge::challenge(2);
}