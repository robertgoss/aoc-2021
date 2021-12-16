#![feature(option_result_contains)]
#![feature(map_first_last)]
#![feature(iter_intersperse)]

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
mod octopus;
mod caves;
mod folding;
mod polymer;
mod pathfind;
mod packet;

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
    use super::pathfind::Pathfinder;

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

    fn challenge_21() {
        let mut data = io::input_as_octopus_states(11);
        let res : usize = data.simulate(100);
        println!("{}", res);
    }

    fn challenge_22() {
        let mut data = io::input_as_octopus_states(11);
        let res : usize = data.simulate_till_flash();
        println!("{}", res);
    }

    fn challenge_23() {
        let data = io::input_as_cave_systen(12);
        let res : usize = data.number_paths();
        println!("{}", res);
    }

    fn challenge_24() {
        let data = io::input_as_cave_systen(12);
        let res : usize = data.number_paths_single_reentry();
        println!("{}", res);
    }

    fn challenge_25() {
        let mut data = io::input_as_folding(13);
        data.fold_first();
        let res : usize = data.number_dots();
        println!("{}", res);
    }

    fn challenge_26() {
        let mut data = io::input_as_folding(13);
        data.fold();
        data.display();
    }

    fn challenge_27() {
        let data = io::input_as_polymer(14);
        let (max, min) = data.calculate_common(10);
        let res = max - min;
        println!("{}", res);
    }

    fn challenge_28() {
        let data = io::input_as_polymer(14);
        let (max, min) = data.calculate_common(40);
        let res = max - min;
        println!("{}", res);
    }

    fn challenge_29() {
        let data = io::input_as_risk_map(15);
        let res = data.safest_path();
        println!("{}", res);
    }

    fn challenge_30() {
        let data = io::input_as_risk_map(15);
        let enlarged = data.enlarge(5);
        let res = enlarged.safest_path();
        println!("{}", res);
    }

    fn challenge_31() {
        let data = io::input_as_packet(16);
        let res = data.version_sum();
        println!("{}", res);
    }

    fn challenge_32() {
        let data = io::input_as_packet(16);
        let res = data.evaluate();
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
            21 => challenge_21(),
            22 => challenge_22(),
            23 => challenge_23(),
            24 => challenge_24(),
            25 => challenge_25(),
            26 => challenge_26(),
            27 => challenge_27(),
            28 => challenge_28(),
            29 => challenge_29(),
            30 => challenge_30(),
            31 => challenge_31(),
            32 => challenge_32(),
            _ => () 
        }
    }
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    let ver = args.get(1).unwrap().parse::<u8>().unwrap();
    challenge::challenge(ver);
}