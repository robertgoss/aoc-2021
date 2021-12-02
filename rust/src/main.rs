mod sonar;
mod movement;

mod io;

mod challenge {
    use super::io as io;
    use super::sonar as sonar;
    use super::movement as movement;

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

    fn challenge_3() {
        let data = io::input_as_commands(2);
        let res = movement::move_ship(&data);
        println!("{}", res.0 * res.1);
    }

    fn challenge_4() {
        let data = io::input_as_commands(2);
        let res = movement::move_ship_aim(&data);
        println!("{}", res.0 * res.1);
    }

    pub fn challenge(num : u8) {
        match num {
            1 => challenge_1(),
            2 => challenge_2(),
            3 => challenge_3(),
            4 => challenge_4(),
            _ => () 
        }
    }
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    let ver = args.get(1).unwrap().parse::<u8>().unwrap();
    challenge::challenge(ver);
}