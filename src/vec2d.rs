use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};

/// A 2 dimensional integer vector
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Vec2D(pub i32, pub i32);

impl Add<Self> for Vec2D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2D(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub<Self> for Vec2D {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec2D(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Display for Vec2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let v1 = Vec2D(10, 10);
        let v2 = Vec2D(1, 2);

        assert_eq!(v1.add(v2), Vec2D(11, 12))
    }

    #[test]
    fn test_sub() {
        let v1 = Vec2D(10, 10);
        let v2 = Vec2D(1, 2);

        assert_eq!(v1.sub(v2), Vec2D(9, 8))
    }
}
