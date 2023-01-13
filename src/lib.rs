use std::collections::{HashSet, VecDeque};
use crate::direction::Direction;
use crate::vec2d::Vec2D;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};
use cons_list::ConsList;

mod direction;
mod vec2d;

/// A mars rover described by a position in a 2D grid and a direction (North,East,South,West)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Rover {
    position: Vec2D,
    direction: Direction,
}

impl Rover {
    /// Creates a new rover at the given x and y coordinates facing the given direction
    pub fn new(x: i32, y: i32, direction: Direction) -> Self {
        let res = Self {
            position: Vec2D(x, y),
            direction,
        };
        println!("Landed at {}", res);
        res
    }

    /// Turns the given direction into a delta-vector for modifying the rover's position
    fn dir_to_vec(dir: Direction) -> Vec2D {
        match dir {
            Direction::North => Vec2D(0, 1),
            Direction::East => Vec2D(1, 0),
            Direction::South => Vec2D(0, -1),
            Direction::West => Vec2D(-1, 0),
        }
    }

    /// Moves the rover one step forward towards the current direction from the current position
    pub fn forward(self, obstacles: &HashSet<Vec2D>) -> Result<Self,Self> {
        self.try_move(obstacles,self.position.add(Self::dir_to_vec(self.direction)))
    }

    /// Moves the rover one step backward from the current direction from the current position
    pub fn backward(self, obstacles: &HashSet<Vec2D>) -> Result<Self,Self> {
        self.try_move(obstacles, self.position.sub(Self::dir_to_vec(self.direction)))
    }

    fn try_move(self, obstacles: &HashSet<Vec2D>, new_pos: Vec2D) -> Result<Self,Self> {
        if obstacles.contains(&new_pos) {
            Err(self)
        } else {
            Ok(Self {
                position: new_pos,
                ..self
            })
        }
    }

    /// Turns the rover 90° to the right from the current direction
    pub fn turn_right(self) -> Self {
        Self {
            direction: self.direction.turn_right(),
            ..self
        }
    }

    /// Turns the rover 90° to the left from the current direction
    pub fn turn_left(self) -> Self {
        Self {
            direction: self.direction.turn_left(),
            ..self
        }
    }

    pub fn get_directions(&self, target: Vec2D, obstacles: &HashSet<Vec2D>) -> String {
        let mut q:VecDeque<(Rover, ConsList<char>)> = VecDeque::new();
        let mut visited: HashSet<Rover> = HashSet::new();

        q.push_back((*self,ConsList::new()));

        while let Some((rover,l)) = q.pop_front() {
            if rover.position == target {
                let mut v:Vec<char> = l.into_iter().map(|c| *c).collect();
                v.reverse();
                return v.into_iter().collect();
                // DONE
            } else if visited.insert(rover) {
                q.push_back((rover.turn_left(), l.append('L')));
                q.push_back((rover.turn_right(), l.append('R')));
                if let Ok(n) = rover.forward(obstacles) {
                    q.push_back((n,l.append('F')));
                }
                if let Ok(n) = rover.backward(obstacles) {
                    q.push_back((n, l.append('B')));
                }
            }
        }
        String::new()
    }

    /// Moves the rover according to the given command sequence and returns
    /// the modified rover. If an invalid command is supplied, the unmodified rover is returned.
    ///
    /// Valid commands are:
    /// - F: Move forward in the current direction
    /// - B: Move backward in the current direction
    /// - L: Turn left from the current direction
    /// - R: Turn right from the current direction
    pub fn process_sequence<T: AsRef<str>>(self, sequence: T, obstacles: &HashSet<Vec2D>) -> Result<Self, Self> {
        let res = sequence.as_ref().chars().try_fold(self, |r, c| match c {
            'F' => r.forward(obstacles),
            'B' => r.backward(obstacles),
            'R' => Ok(r.turn_right()),
            'L' => Ok(r.turn_left()),
            _ => Err(self),
        });
        match res {
            Ok(r) => println!("Moved to {}", r),
            Err(r) => println!("Stopped at {}", r),
        }
        res
    }


    pub fn move_all(rovers: &[Rover], commands: &[char], obstacles: &HashSet<Vec2D>) -> Vec<Rover> {
        let mut with_rover = obstacles.clone();
        for r in rovers {
            with_rover.insert(r.position);
        }

        let mut result = Vec::new();

        rovers.into_iter().zip(commands.into_iter()).for_each(|(r,c ):(&Rover, &char)| {
           with_rover.remove(&r.position);
           let rover = match c {
               'F' => {
                   match r.forward(&with_rover) {
                       Ok(res) => res,
                       Err(res) => res,
                   }
               },
               'B' => {
                   match r.backward(&with_rover) {
                       Ok(res) => res,
                       Err(res) => res,
                   }
               },
               'L' => r.turn_left(),
               'R' => r.turn_right(),
               _ => *r,
           };
            with_rover.insert(rover.position);
            result.push(rover);
        });
        result
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
        let obstacles = HashSet::new();
        let start = Rover::new(0,0, Direction::North);
        assert!(matches!(start.forward(&obstacles).unwrap(), Rover { position: Vec2D(0,1), direction: Direction::North }));
        assert!(matches!(start.backward(&obstacles).unwrap(), Rover { position: Vec2D(0,-1), direction: Direction::North }));
        assert!(matches!(start.turn_left(), Rover { position: Vec2D(0,0), direction: Direction::West }));
        assert!(matches!(start.turn_right(), Rover { position: Vec2D(0,0), direction: Direction::East }));

    }

    #[test]
    fn test_south() {
        let obstacles = HashSet::new();
        let start = Rover::new(0,0, Direction::South);
        assert!(matches!(start.forward(&obstacles).unwrap(), Rover { position: Vec2D(0,-1), direction: Direction::South }));
        assert!(matches!(start.backward(&obstacles).unwrap(), Rover { position: Vec2D(0,1), direction: Direction::South }));
        assert!(matches!(start.turn_left(), Rover { position: Vec2D(0,0), direction: Direction::East }));
        assert!(matches!(start.turn_right(), Rover { position: Vec2D(0,0), direction: Direction::West }));
    }

    #[test]
    fn test_east() {
        let obstacles = HashSet::new();
        let start = Rover::new(0,0, Direction::East);
        assert!(matches!(start.forward(&obstacles).unwrap(), Rover { position: Vec2D(1,0), direction: Direction::East }));
        assert!(matches!(start.backward(&obstacles).unwrap(), Rover { position: Vec2D(-1,0), direction: Direction::East }));
        assert!(matches!(start.turn_left(), Rover { position: Vec2D(0,0), direction: Direction::North }));
        assert!(matches!(start.turn_right(), Rover { position: Vec2D(0,0), direction: Direction::South }));
    }

    #[test]
    fn test_west() {let obstacles = HashSet::new();
        let start = Rover::new(0,0, Direction::West);
        assert!(matches!(start.forward(&obstacles).unwrap(), Rover { position: Vec2D(-1,0), direction: Direction::West }));
        assert!(matches!(start.backward(&obstacles).unwrap(), Rover { position: Vec2D(1,0), direction: Direction::West }));
        assert!(matches!(start.turn_left(), Rover { position: Vec2D(0,0), direction: Direction::South }));
        assert!(matches!(start.turn_right(), Rover { position: Vec2D(0,0), direction: Direction::North }));
    }

    #[test]
    fn test_obstacle_fw() {
        let obsts = [Vec2D(0,1)].into_iter().collect();
        let start = Rover::new(0,0, Direction::North);
        assert!(start.forward(&obsts).is_err());
    }

    #[test]
    fn test_obstacle_bw() {
        let obsts = [Vec2D(0,-1)].into_iter().collect();
        let start = Rover::new(0,0, Direction::North);
        assert!(start.backward(&obsts).is_err());
    }

    #[test]
    fn test_example_sequence_obst() {
        let obstacles = [Vec2D(0,3)].into_iter().collect();
        let start = Rover::new(0, 0, Direction::North);
        let moved = start.clone().process_sequence("FFFLFFRBRF",&obstacles);
        assert!(matches!(moved, Err(Rover { position: Vec2D(0,2), direction: Direction::North })));
    }

    #[test]
    fn test_example_sequence() {
        let obstacles = HashSet::new();
        let start = Rover::new(0, 0, Direction::North);
        let moved = start.clone().process_sequence("FFFLFFRBRF",&obstacles).unwrap();
        assert_eq!(moved, Rover::new(-1, 2, Direction::East));
    }

    #[test]
    fn test_empty_sequence() {
        let obstacles = HashSet::new();
        let start = Rover::new(0, 0, Direction::North);
        assert!(matches!(
            start.process_sequence("",&obstacles),
            Ok(Rover {
                position: Vec2D(0, 0),
                direction: Direction::North
            })
        ));
    }

    #[test]
    fn test_illegal_sequence() {
        let obstacles = HashSet::new();
        let start = Rover::new(0, 0, Direction::North);
        assert!(matches!(
            start.process_sequence("FFX", &obstacles),
            Err(Rover {
                position: Vec2D(0, 0),
                direction: Direction::North
            })
        ))
    }

    #[test]
    fn test_path_no_obstacles() {
        let obstacles = HashSet::new();
        let start = Rover::new(0, 0, Direction::North);
        let r = start.get_directions(Vec2D(3,0), &obstacles );
        assert_eq!(r,"LBBB")
    }

    #[test]
    fn test_path_with_obstacles() {
        let obstacles = [Vec2D(2,0)].into_iter().collect();
        let start = Rover::new(0, 0, Direction::North);
        let r = start.get_directions(Vec2D(3,0), &obstacles );
        assert_eq!(r,"FLBBBLF");
        assert_eq!(Vec2D(3,0), start.process_sequence(r,&obstacles).unwrap().position);
    }

    #[test]
    fn test_move_all_no_obstacles() {
        let obstacles = HashSet::new();
        let rovers = [
            Rover::new(0,0, Direction::North),
            Rover::new( 1, 0, Direction::North)
        ];

        let result= Rover::move_all(&rovers, &['F','F'], &obstacles );

        assert_eq!( result, vec![Rover::new(0,1, Direction::North),
                                 Rover::new( 1, 1, Direction::North)])
    }

    #[test]
    fn test_move_all_collision_no_obstacles() {
        let obstacles = HashSet::new();
        let rovers = [
            Rover::new(0,0, Direction::East),
            Rover::new( 2, 0, Direction::West)
        ];

        let result= Rover::move_all(&rovers, &['F','F'], &obstacles );

        assert_eq!( result, vec![Rover::new(1,0, Direction::East),
                                 Rover::new( 2, 0, Direction::West)])
    }

    #[test]
    fn test_move_all_collision_with_obstacles() {
        let obstacles = [Vec2D(1,0)].into_iter().collect();
        let rovers = [
            Rover::new(0,0, Direction::East),
            Rover::new( 2, 0, Direction::West)
        ];

        let result= Rover::move_all(&rovers, &['F','F'], &obstacles );

        assert_eq!( result, vec![Rover::new(0,0, Direction::East),
                                 Rover::new( 2, 0, Direction::West)])
    }

}
