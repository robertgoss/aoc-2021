use std::{ops::RangeFrom, collections::HashMap};

use itertools::iproduct;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Game {
    player : usize,
    player_score: [usize; 2],
    player_position: [usize; 2]
}

impl Game {
    pub fn from_string(string : &str) -> Game {
        let position : Vec<usize> = string.lines().filter_map(
            |line| line.split_once(": ").and_then(
                |(_,rest)| rest.parse::<usize>().ok()
            )
        ).collect();
        Game {
            player : 0,
            player_position : [
                position[0] - 1,
                position[1] - 1
            ],
            player_score : [0, 0]
        }
    }

    pub fn losing_score(&self) -> usize {
        *self.player_score.iter().min().unwrap()
    }

    fn wining_score(&self) -> usize {
        *self.player_score.iter().max().unwrap()
    }

    pub fn play(&mut self) -> usize {
        let mut dice : RangeFrom<usize> = 1..;
        while self.wining_score() < 1000 {
            self.turn(&mut dice);
        }
        dice.next().unwrap() - 1
    }

    fn turn<I>(&mut self, dice : &mut I) 
      where I : Iterator<Item = usize>
    {
        let player = self.player;
        self.player_position[player] += self.roll(dice);
        self.player_position[player] = self.player_position[player] % 10;
        self.player_score[player] += self.player_position[player] + 1;
        self.player = if player == 1 { 0 } else { 1 };
    }

    fn roll<I>(&mut self, dice : &mut I)  -> usize
      where I : Iterator<Item = usize>
    {
        (0..3).map(
            |_| dice.next().unwrap()
        ).sum()
    }

    fn dirac_moves(&self) -> Vec<Game> {
        let next_player = if self.player == 0 {1} else {0};
        iproduct!(1..4, 1..4, 1..4).map(
            |(i, j, k)| {
                let mut game = self.clone();
                game.player = next_player;
                game.player_position[self.player] += i + j + k;
                game.player_position[self.player] = game.player_position[self.player] % 10;
                game.player_score[self.player] += game.player_position[self.player] + 1;
                game
            }
        ).collect()
    }
}

pub fn dirac_results(starting_game : &Game) -> (usize, usize) {
    let mut cache : HashMap<Game, (usize,usize)> = HashMap::new();
    dirac_results_cached(starting_game.clone(), &mut cache)
} 

fn dirac_results_cached(
    game : Game,
    cache : &mut HashMap<Game, (usize,usize)>
) -> (usize, usize) {
    if let Some(cached) = cache.get(&game) {
        return *cached;
    }
    if game.wining_score() >= 21 {
        if game.player_score[0] > game.player_score[1] {
            return (1,0)
        } else {
            return (0,1)
        }
    }
    let mut res = (0, 0);
    for next_game in game.dirac_moves() {
        let sub_res = dirac_results_cached(next_game, cache);
        res.0 += sub_res.0;
        res.1 += sub_res.1;
    }
    cache.insert(game, res);
    res
} 
