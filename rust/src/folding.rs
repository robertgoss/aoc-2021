use std::collections::HashSet;

struct Paper {
    dots : HashSet<(i64, i64)>
}

fn fold_x_pt(x : i64, y : i64, val : i64) -> (i64, i64) {
    if x > val {
        (2*val - x, y)
    } else {
        (x,y)
    }
}

fn fold_y_pt(x : i64, y : i64, val : i64) -> (i64, i64) {
    if y > val {
        (x, 2*val - y)
    } else {
        (x,y)
    }
}

impl Paper {
    fn from_string(string : &str) -> Paper {
        let dots : HashSet<(i64, i64)> = string.split('\n').filter_map(
            |line| line.split_once(',')
        ).filter_map(
            |(x_s,y_s)| x_s.parse::<i64>().ok().and_then(
                |x| y_s.parse::<i64>().ok().map(|y| (x,y))
            )
        ).collect();
        Paper {dots : dots}
    }

    fn fold_x(&mut self, val :i64) {
        let new_dots : HashSet<(i64,i64)> = self.dots.iter().map(
            |(x,y)| fold_x_pt(*x, *y, val)
        ).collect();
        self.dots = new_dots;
    }

    fn fold_y(&mut self, val :i64) {
        let new_dots : HashSet<(i64,i64)> = self.dots.iter().map(
            |(x,y)| fold_y_pt(*x, *y, val)
        ).collect();
        self.dots = new_dots;
    }

    fn fold(&mut self, fold : &Fold) {
        match fold {
            Fold::X(val) => self.fold_x(*val),
            Fold::Y(val) => self.fold_y(*val)
        }
    }

    fn size(&self) -> (i64, i64) {
        let w = *self.dots.iter().map(
            |(x,_)| x
        ).max().unwrap_or(&0);
        let h = *self.dots.iter().map(
            |(_,y)| y
        ).max().unwrap_or(&0);
        (w,h)
    }

    fn display(&self) {
        let (w,h) = self.size();
        for y in 0..(h+1) {
            let line : String = (0..(w+1)).map(
                |x| {
                    if self.dots.contains(&(x,y)) {
                        '#'
                    } else {
                        '.'
                    }
                }
            ).collect();
            println!("{}", line);
        }
    }
}

enum Fold {
    X(i64),
    Y(i64)
}

impl Fold {
    fn from_string(string : &str) -> Option<Fold> {
        if let Some( (dir_s, val_s) ) = string.split_once('=') {
            if let Some(val) = val_s.parse::<i64>().ok() {
                if dir_s.ends_with('x') {
                    return Some(Fold::X(val))
                } else {
                    return Some(Fold::Y(val))
                }
            }
        }
        None
    }
}

pub struct Instructions {
    paper : Paper,
    folds : Vec<Fold>
}

impl Instructions {
    pub fn from_string(string : &str) -> Option<Instructions> {
        string.split_once("\n\n").map(
            |(paper_s, folds_s)| {  
                let paper = Paper::from_string(paper_s);
                let folds :Vec<Fold> = folds_s.split('\n').filter_map(
                    |fold_s| Fold::from_string(fold_s)
                ).collect();
                Instructions {paper: paper, folds : folds}
            }
        )
    }

    pub fn fold_first(&mut self) {
        self.paper.fold(self.folds.first().unwrap())
    }

    pub fn fold(&mut self) {
        for fold in self.folds.iter() {
            self.paper.fold(fold);
        }
    }

    pub fn display(&self) {
        self.paper.display();
    }

    pub fn number_dots(&self) -> usize {
        self.paper.dots.len()
    }
}