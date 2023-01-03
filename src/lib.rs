use crate::direction::Direction;
use crate::vec2d::Vec2D;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};

mod direction;
mod vec2d;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rover {
    position: Vec2D,
    direction: Direction,
}

impl Rover {
    pub fn new(x: i32, y: i32, direction: Direction) -> Self {
        let res = Self {
            position: Vec2D(x, y),
            direction,
        };
        println!("Landed at {}", res);
        res
    }

    fn dir_to_vec(dir: Direction) -> Vec2D {
        match dir {
            Direction::North => Vec2D(0, 1),
            Direction::East => Vec2D(1, 0),
            Direction::South => Vec2D(0, -1),
            Direction::West => Vec2D(-1, 0),
        }
    }

    pub fn forward(self) -> Self {
        Self {
            position: self.position.add(Self::dir_to_vec(self.direction)),
            ..self
        }
    }

    pub fn backward(self) -> Self {
        Self {
            position: self.position.sub(Self::dir_to_vec(self.direction)),
            ..self
        }
    }

    pub fn turn_right(self) -> Self {
        Self {
            direction: self.direction.turn_right(),
            ..self
        }
    }

    pub fn turn_left(self) -> Self {
        Self {
            direction: self.direction.turn_left(),
            ..self
        }
    }

    /// Moves the rover according to the given command sequence and returns
    /// the modified rover. If an invalid command is supplied, the unmodified rover is returned.
    ///
    /// Valid commands are:
    /// - F: Move forward in the current direction
    /// - B: Move backward in the current direction
    /// - L: Turn left from the current direction
    /// - R: Turn right from the current direction
    pub fn process_sequence<T: AsRef<str>>(self, sequence: T) -> Result<Self, Self> {
        let res = sequence.as_ref().chars().try_fold(self, |r, c| match c {
            'F' => Ok(r.forward()),
            'B' => Ok(r.backward()),
            'R' => Ok(r.turn_right()),
            'L' => Ok(r.turn_left()),
            _ => Err(self),
        });
        match res {
            Ok(r) => println!("Moved to {}", r),
            Err(r) => println!("Stayed at {}", r),
        }
        res
    }
}

impl Display for Rover {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.position, self.direction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_north() {
        let start = Rover::new(0,0, Direction::North);
        assert!(matches!(start.forward(), Rover { position: Vec2D(0,1), direction: Direction::North }));
        assert!(matches!(start.backward(), Rover { position: Vec2D(0,-1), direction: Direction::North }));
        assert!(matches!(start.turn_left(), Rover { position: Vec2D(0,0), direction: Direction::West }));
        assert!(matches!(start.turn_right(), Rover { position: Vec2D(0,0), direction: Direction::East }));

    }

    #[test]
    fn test_south() {
        let start = Rover::new(0,0, Direction::South);
        assert!(matches!(start.forward(), Rover { position: Vec2D(0,-1), direction: Direction::South }));
        assert!(matches!(start.backward(), Rover { position: Vec2D(0,1), direction: Direction::South }));
        assert!(matches!(start.turn_left(), Rover { position: Vec2D(0,0), direction: Direction::East }));
        assert!(matches!(start.turn_right(), Rover { position: Vec2D(0,0), direction: Direction::West }));
    }

    #[test]
    fn test_east() {
        let start = Rover::new(0,0, Direction::East);
        assert!(matches!(start.forward(), Rover { position: Vec2D(1,0), direction: Direction::East }));
        assert!(matches!(start.backward(), Rover { position: Vec2D(-1,0), direction: Direction::East }));
        assert!(matches!(start.turn_left(), Rover { position: Vec2D(0,0), direction: Direction::North }));
        assert!(matches!(start.turn_right(), Rover { position: Vec2D(0,0), direction: Direction::South }));
    }

    #[test]
    fn test_west() {
        let start = Rover::new(0,0, Direction::West);
        assert!(matches!(start.forward(), Rover { position: Vec2D(-1,0), direction: Direction::West }));
        assert!(matches!(start.backward(), Rover { position: Vec2D(1,0), direction: Direction::West }));
        assert!(matches!(start.turn_left(), Rover { position: Vec2D(0,0), direction: Direction::South }));
        assert!(matches!(start.turn_right(), Rover { position: Vec2D(0,0), direction: Direction::North }));
    }

    #[test]
    fn test_example_sequence() {
        let start = Rover::new(0, 0, Direction::North);
        let moved = start.clone().process_sequence("FFFLFFRBRF").unwrap();
        assert_eq!(moved, Rover::new(-1, 2, Direction::East));
    }

    #[test]
    fn test_empty_sequence() {
        let start = Rover::new(0, 0, Direction::North);
        assert!(matches!(
            start.process_sequence(""),
            Ok(Rover {
                position: Vec2D(0, 0),
                direction: Direction::North
            })
        ));
    }

    #[test]
    fn test_illegal_sequence() {
        let start = Rover::new(0, 0, Direction::North);
        assert!(matches!(
            start.process_sequence("FFX"),
            Err(Rover {
                position: Vec2D(0, 0),
                direction: Direction::North
            })
        ))
    }
}
