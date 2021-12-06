use itertools::Itertools;
use std::convert::TryInto;


struct Board {
    elements : [[Option<u32>; 5]; 5]
}

pub struct Game {
    numbers : Vec<u32>,
    boards : Vec<Board>
}

fn parse_row(line : &String) -> Option<[Option<u32>; 5]> {
    let row : Vec<Option<u32>> = line.split(" ").map(
        |element| element.parse::<u32>().ok()
    ).filter(
        |res| res.is_some()
    ).collect();
    row.try_into().ok()
}

impl Board {
    pub fn from_lines<'a, I>(lines_iter : I) -> Option<Board> 
        where I : Iterator<Item = &'a String>
    {
        let elements : Vec<[Option<u32>;5]> = lines_iter.filter_map(
            |line| parse_row(line)
        ).collect();
        if let Some(array) = elements.try_into().ok() {
            Some(Board {elements : array} )
        } else {
            None
        }
    }

    fn mark_number(&mut self, number : u32) {
        for row in self.elements.iter_mut() {
            for elem in row.iter_mut() {
                if elem.contains(&number) {
                    *elem = None
                }
            }
        }
    }

    fn has_win(&self) -> Option<u32> {
        for i in 0..5 {
            if (0..5).all(|j| self.elements[i][j].is_none())  {
                return Some(self.score());
            }
        }
        for j in 0..5 {
            if (0..5).all(|i| self.elements[i][j].is_none())  {
                return Some(self.score());
            }
        }
        None
    }

    fn score(&self) -> u32 {
        self.elements.iter().map(
            |row| row.iter().map(
                |elem| elem.unwrap_or(0)
              ).sum::<u32>()
        ).sum()
    }
}

impl Game {
    pub fn from_lines(lines : Vec<String>) -> Option<Game> {
        if let Some(numbers) = lines.iter().next().map(
            |line| line.split(',').filter_map(
                |part| part.parse::<u32>().ok()
            ).rev().collect()
        ) {
            let boards = lines.iter().skip(1).chunks(6).into_iter().filter_map(
                |board_lines| Board::from_lines(board_lines.skip(1))
            ).collect();
            Some (Game {numbers : numbers, boards: boards})
        } else {
            None
        }
    }

    fn mark_number(&mut self, number : u32) -> Vec<u32> {
        let mut new_scores : Vec<u32> = Vec::new();
        for board in self.boards.iter_mut() {
            let already_won = board.has_win().is_some();
            board.mark_number(number);
            if !already_won {
                if let Some(new_score) = board.has_win() {
                    new_scores.push(new_score);
                }
            }
        }
        new_scores
    }

    fn no_boards(&self) -> bool {
        self.boards.iter().all(
            |board| board.has_win().is_some()
        )
    }

    pub fn play_first(&mut self) -> u32 {
        while let Some(number) = self.numbers.pop() {
            let new_scores = self.mark_number(number);
            if new_scores.len() > 0 {
                return new_scores[0] * number;
            }
        }
        0
    }

    pub fn play_last(&mut self) -> u32 {
        while let Some(number) = self.numbers.pop() {
            let new_scores = self.mark_number(number);
            if self.no_boards() {
                return new_scores[0] * number;
            }
        }
        0
    }
}