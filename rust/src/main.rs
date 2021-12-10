#![feature(option_result_contains)]
#![feature(map_first_last)]

mod sonar;
mod movement;
mod binary;
mod bingo;
mod vents;
mod fish;
mod crabs;
mod displays;
mod heights;
mod brackets;

mod io;

mod challenge {
    use super::io as io;
    use super::sonar as sonar;
    use super::movement as movement;
    use super::binary as binary;
    use super::vents as vents;
    use super::fish as fish;
    use super::crabs as crabs;
    use super::displays as displays;
    use super::brackets as brackets;

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

    fn challenge_5() {
        let data = io::input_from_binary(3);
        let gamma = binary::gamma(&data);
        let epsilon = binary::epsilon(&data);
        println!("{}", gamma * epsilon);
    }

    fn challenge_6() {
        let data = io::input_from_binary(3);
        let oxygen = binary::oxygen(&data);
        let carbon = binary::carbon(&data);
        println!("{}", oxygen * carbon);
    }

    fn challenge_7() {
        let mut data = io::input_as_game(4);
        let res = data.play_first();
        println!("{}", res);
    }

    fn challenge_8() {
        let mut data = io::input_as_game(4);
        let res = data.play_last();
        println!("{}", res);
    }

    fn challenge_9() {
        let data = io::input_as_vents(5);
        let res = vents::get_overlap_num_cardinal(&data);
        println!("{}", res);
    }

    fn challenge_10() {
        let data = io::input_as_vents(5);
        let res = vents::get_overlap_num(&data);
        println!("{}", res);
    }

    fn challenge_11() {
        let data = io::input_as_fish(6);
        let res = fish::count_after(&data, 79);
        println!("{}", res);
    }

    fn challenge_12() {
        let data = io::input_as_fish(6);
        let res = fish::count_after(&data, 255);
        println!("{}", res);
    }

    fn challenge_13() {
        let data = io::input_as_crabs(7);
        let res = crabs::minimum_distance(&data);
        println!("{}", res);
    }

    fn challenge_14() {
        let data = io::input_as_crabs(7);
        let res = crabs::minimum_distance_quad(&data);
        println!("{}", res);
    }

    fn challenge_15() {
        let data = io::input_as_displays(8);
        let res = displays::count_easy_digits(&data);
        println!("{}", res);
    }

    fn challenge_16() {
        let data = io::input_as_displays(8);
        let res : usize = data.iter().map(
            |entry| entry.output_num()
        ).sum();
        println!("{}", res);
    }

    fn challenge_17() {
        let data = io::input_as_heightmap(9);
        let res = data.total_risk();
        println!("{}", res);
    }

    fn challenge_18() {
        let data = io::input_as_heightmap(9);
        let res : usize = data.largest_basins(3).iter().product();
        println!("{}", res);
    }

    fn challenge_19() {
        let data = io::input_as_lines(10);
        let res : usize = brackets::parse_score(&data);
        println!("{}", res);
    }

    fn challenge_20() {
        let data = io::input_as_lines(10);
        let res : usize = brackets::parse_complete_score(&data);
        println!("{}", res);
    }

    pub fn challenge(num : u8) {
        match num {
            1 => challenge_1(),
            2 => challenge_2(),
            3 => challenge_3(),
            4 => challenge_4(),
            5 => challenge_5(),
            6 => challenge_6(),
            7 => challenge_7(),
            8 => challenge_8(),
            9 => challenge_9(),
            10 => challenge_10(),
            11 => challenge_11(),
            12 => challenge_12(),
            13 => challenge_13(),
            14 => challenge_14(),
            15 => challenge_15(),
            16 => challenge_16(),
            17 => challenge_17(),
            18 => challenge_18(),
            19 => challenge_19(),
            20 => challenge_20(),
            _ => () 
        }
    }
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    let ver = args.get(1).unwrap().parse::<u8>().unwrap();
    challenge::challenge(ver);
}