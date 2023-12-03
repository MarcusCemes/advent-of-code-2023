use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coords {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UCoords {
    pub x: usize,
    pub y: usize,
}

impl Coords {
    pub fn shift(&self, by: Coords) -> Coords {
        Coords {
            x: self.x + by.x,
            y: self.y + by.y,
        }
    }

    pub fn left(&self) -> Coords {
        Coords {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn right(&self) -> Coords {
        Coords {
            x: self.x + 1,
            y: self.y,
        }
    }

    pub fn up(&self) -> Coords {
        Coords {
            x: self.x,
            y: self.y - 1,
        }
    }

    pub fn down(&self) -> Coords {
        Coords {
            x: self.x,
            y: self.y + 1,
        }
    }

    pub fn to_ucoords(&self, bounds: &UCoords) -> Option<UCoords> {
        let x: usize = self.x.try_into().ok()?;
        let y: usize = self.y.try_into().ok()?;
        (x < bounds.x && y < bounds.y).then_some(UCoords { x, y })
    }
}

impl Display for Coords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
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
