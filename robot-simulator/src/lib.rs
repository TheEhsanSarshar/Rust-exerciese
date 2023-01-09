// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { x, y, d }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        let new_direction;

        match self.direction() {
            Direction::East => new_direction = Direction::South,
            Direction::West => new_direction = Direction::North,
            Direction::North => new_direction = Direction::East,
            Direction::South => new_direction = Direction::West,
        }

        Robot {
            x: self.x,
            y: self.y,
            d: new_direction,
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        let new_direction;

        match self.direction() {
            Direction::East => new_direction = Direction::North,
            Direction::West => new_direction = Direction::South,
            Direction::North => new_direction = Direction::West,
            Direction::South => new_direction = Direction::East,
        }

        Robot {
            x: self.x,
            y: self.y,
            d: new_direction,
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        match self.direction() {
            Direction::East => Robot {
                x: self.x + 1,
                y: self.y,
                d: self.d,
            },
            Direction::West => Robot {
                x: self.x - 1,
                y: self.y,
                d: self.d,
            },
            Direction::North => Robot {
                x: self.x,
                y: self.y + 1,
                d: self.d,
            },
            Direction::South => Robot {
                x: self.x,
                y: self.y - 1,
                d: self.d,
            },
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut to_return = self;
        for each_dir in instructions.chars() {
            match each_dir {
                'A' => to_return = to_return.advance(),
                'L' => to_return = to_return.turn_left(),
                'R' => to_return = to_return.turn_right(),
                _ => panic!("invalid direction"),
            }
        }

        return to_return;
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
