#[derive(PartialEq, Eq, Clone, Copy)]
enum Bracket {
    Paren,
    Curly,
    Angle,
    Square
}

impl Bracket {
    fn is_open(ch : char) -> Option<Bracket> {
        match ch {
            '(' => Some(Bracket::Paren),
            '{' => Some(Bracket::Curly),
            '<' => Some(Bracket::Angle),
            '[' => Some(Bracket::Square),
            _ => None
        }
    }
    fn is_closed(ch : char) -> Option<Bracket> {
        match ch {
            ')' => Some(Bracket::Paren),
            '}' => Some(Bracket::Curly),
            '>' => Some(Bracket::Angle),
            ']' => Some(Bracket::Square),
            _ => None
        }
    }
    fn corrupt_score(&self) -> usize {
        match self {
            Bracket::Paren => 3,
            Bracket::Square => 57,
            Bracket::Curly => 1197,
            Bracket::Angle => 25137
        }
    }
    fn completetion_score(&self) -> usize {
        match self {
            Bracket::Paren => 1,
            Bracket::Square => 2,
            Bracket::Curly => 3,
            Bracket::Angle => 4
        }
    }
}

enum ParseResult {
    Corrupt(Bracket),
    Incomplete(Vec<Bracket>)
}

impl ParseResult {
    fn corrupt(&self) -> Option<Bracket> {
        match self {
            ParseResult::Corrupt(bracket) => Some(*bracket),
            _ => None
        }
    }

    fn incomplete(&self) -> Option<Vec<Bracket>> {
        match self {
            ParseResult::Incomplete(brackets) => Some(brackets.clone()),
            _ => None
        }
    }
}

fn attempt_parse(string : &str) -> ParseResult {
    let mut stack : Vec<Bracket> = Vec::new();
    for char in string.chars() {
        if let Some(bracket) = Bracket::is_open(char) {
            stack.push(bracket);
        } 
        if let Some(bracket) = Bracket::is_closed(char) {
            if let Some(expected) = stack.pop() {
                if expected != bracket {
                    return ParseResult::Corrupt(bracket);
                }
            }
        } 
    }
    ParseResult::Incomplete(stack)
}

fn completetion_score(brackets : &Vec<Bracket>) -> usize {
    let mut score : usize = 0;
    for bracket in brackets.iter().rev() {
        score *= 5;
        score += bracket.completetion_score();
    }
    score
}

pub fn parse_score(lines : &Vec<String>) -> usize {
    lines.iter().filter_map(
        |line| attempt_parse(line).corrupt().map(
            |bracket| bracket.corrupt_score()
        )
    ).sum()
}

pub fn parse_complete_score(lines : &Vec<String>) -> usize {
    let mut scores : Vec<usize> = lines.iter().filter_map(
        |line| attempt_parse(line).incomplete().map(
            |brackets| completetion_score(&brackets)
        )
    ).collect();
    scores.sort();
    scores[scores.len() / 2]
}