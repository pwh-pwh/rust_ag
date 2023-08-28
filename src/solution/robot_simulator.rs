// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl From<i32> for Direction {
    fn from(value: i32) -> Self {
        let value = value%4;
        match value {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            _ => panic!()
        }
    }
}

impl Direction {
    fn to_int(&self)->i32 {
        match self {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3
        }
    }
}

pub struct Robot {
    x:i32,
    y:i32,
    dir:Direction
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self {
            x,y,
            dir:d
        }
    }

    #[must_use]
    pub fn turn_right(mut self) -> Self {
        let dI = self.dir.to_int()+1 + 4;
        self.dir = dI.into();
        self
    }

    #[must_use]
    pub fn turn_left(mut self) -> Self {
        let dI = self.dir.to_int()-1 + 4;
        self.dir = dI.into();
        self
    }

    #[must_use]
    pub fn advance(mut self) -> Self {
        match self.dir {
            Direction::North => {
                self.y+=1
            },
            Direction::East => {
                self.x+=1
            },
            Direction::South => {
                self.y-=1
            },
            Direction::West => {
                self.x-=1
            },
        }
        self
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut owner = self;
        for x in instructions.bytes() {
            match x {
                b'R' => {
                    owner =owner.turn_right()
                },
                b'A' => {
                    owner = owner.advance();
                },
                b'L' => {
                    owner = owner.turn_left();
                },
                _ => panic!()
            }
        }
        owner
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x,self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        println!("test")
    }
}