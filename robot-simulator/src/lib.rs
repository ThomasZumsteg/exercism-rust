// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    d: Direction,
    x: isize,
    y: isize,
}

impl Robot {
    #[allow(unused_variables)]
    pub fn new(x: isize, y: isize, d: Direction) -> Self {
        Robot {x: x, y: y, d: d}
    }

    pub fn turn_right(self) -> Self {
        Robot::new(self.x, self.y, match self.d {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        })
    }

    pub fn turn_left(self) -> Self {
        Robot::new(self.x, self.y, match self.d {
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::North => Direction::West,
            Direction::East => Direction::North,
        })
    }

    pub fn advance(self) -> Self {
        let (y, x) = match self.d {
            Direction::North => (1, 0),
            Direction::East => (0, 1),
            Direction::South => (-1, 0),
            Direction::West => (0, -1),
        };
        Robot::new(self.x + x, self.y + y, self.d)
    }

    #[allow(unused_variables)]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = self;
        for c in instructions.chars() {
            match c {
                'L' => robot = robot.turn_left(),
                'R' => robot = robot.turn_right(),
                'A' => robot = robot.advance(),
                _ => panic!("Not a valid command"),
            }
        }
        robot
    }

    pub fn position(&self) -> (isize, isize) { (self.x, self.y) }

    pub fn direction(&self) -> &Direction { &self.d }
}
