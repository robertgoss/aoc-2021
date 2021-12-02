#[derive(Copy, Clone, Debug)]
enum Direction {
    Forward,
    Up,
    Down
}

pub struct Command {
    dir : Direction,
    mov : u64
}

impl Direction {
    pub fn from_string(string : &str) -> Option<Direction> {
        match string  {
            "forward" => Some(Direction::Forward),
            "up" => Some(Direction::Up),
            "down" => Some(Direction::Down),
            _ => None
        }
    }

    fn move_pos(self, (x,y) : (i64, i64), mov: u64) -> (i64,i64) {
        match self {
            Direction::Down => (x, y + mov as i64),
            Direction::Up => (x, y - mov as i64),
            Direction::Forward => (x + mov as i64, y),

        }
    }

    fn move_ship(self, ship : &Ship, mov: i64) -> Ship {
        let (x,y) = ship.pos;
        match self {
            Direction::Down => Ship {pos : ship.pos, aim : ship.aim + mov},
            Direction::Up => Ship {pos : ship.pos, aim : ship.aim - mov},
            Direction::Forward => {
                let pos = (x + mov, y + (mov * ship.aim));
                Ship {pos : pos, aim : ship.aim}
            },

        }
    }
}

struct Ship{
    pos : (i64,i64),
    aim : i64
}

impl Command {
    pub fn from_string(string : &str) -> Option<Command> {
        if let Some((first,last)) = string.split_once(" ") {
            if let Some(dir) = Direction::from_string(first) {
                if let Ok(mov) = last.parse::<u64>() {
                    return Some(Command{dir, mov});
                }
            }
        }
        None
    }

    fn move_pos(&self, pos : (i64, i64)) -> (i64,i64) {
        self.dir.move_pos(pos, self.mov)
    }

    fn move_ship(&self, ship : &Ship) -> Ship {
        self.dir.move_ship(ship, self.mov as i64)
    }
}

pub fn move_ship(commands : &Vec<Command>) -> (i64, i64) {
    let mut pos : (i64,i64) = (0, 0);
    commands.iter().for_each(
        |command| { pos = command.move_pos(pos) }
    );
    pos
}

pub fn move_ship_aim(commands : &Vec<Command>) -> (i64, i64) {
    let mut ship = Ship {pos : (0,0), aim :0 };
    commands.iter().for_each(
        |command| { ship = command.move_ship(&ship) }
    );
    ship.pos
}