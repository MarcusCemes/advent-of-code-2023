use std::{
    fmt::Display,
    ops::{Add, AddAssign, Sub},
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coords {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UCoords {
    pub x: usize,
    pub y: usize,
}

impl Coords {
    pub fn new(x: i64, y: i64) -> Coords {
        Coords { x, y }
    }

    pub fn norm_l1(&self) -> u64 {
        self.x.unsigned_abs() + self.y.unsigned_abs()
    }

    pub fn ucoords(&self, bounds: &UCoords) -> Option<UCoords> {
        let x: usize = self.x.try_into().ok()?;
        let y: usize = self.y.try_into().ok()?;
        (x < bounds.x && y < bounds.y).then_some(UCoords { x, y })
    }
}

impl Add for Coords {
    type Output = Coords;

    fn add(self, rhs: Coords) -> Self::Output {
        Coords {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Coords {
    fn add_assign(&mut self, rhs: Coords) {
        *self = *self + rhs;
    }
}

impl Sub for Coords {
    type Output = Coords;

    fn sub(self, rhs: Coords) -> Self::Output {
        Coords {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Display for Coords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl From<(i64, i64)> for Coords {
    fn from(val: (i64, i64)) -> Coords {
        Coords { x: val.0, y: val.1 }
    }
}

impl UCoords {
    pub fn new(x: usize, y: usize) -> UCoords {
        UCoords { x, y }
    }
}

impl From<(usize, usize)> for UCoords {
    fn from(val: (usize, usize)) -> UCoords {
        UCoords { x: val.0, y: val.1 }
    }
}

impl From<UCoords> for Coords {
    fn from(val: UCoords) -> Coords {
        Coords {
            x: val.x as i64,
            y: val.y as i64,
        }
    }
}
