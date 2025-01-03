use core::fmt;
use strum_macros::EnumIter;

pub enum Direction {
    Top, // from top to bottom
    Left, // from down left to up right
    Right, // from down right to up left
}

pub fn max_number(direction: &Direction) -> u32 {
    match direction {
        Direction::Top => NumTop::N9 as u32,
        Direction::Left => NumLeft::N7 as u32,
        Direction::Right => NumRight::N8 as u32,
    }
}

#[repr(i32)]
#[derive(PartialEq, Eq, Debug, Copy, Hash, Clone, EnumIter)]
pub enum NumTop {
    N1 = 1,
    N5 = 5,
    N9 = 9,
}

impl NumTop {
    pub fn from_int(num: i32) -> Result<NumTop, ()> {
        match num {
            1 => Ok(NumTop::N1),
            5 => Ok(NumTop::N5),
            9 => Ok(NumTop::N9),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Hash, Clone, EnumIter)]
pub enum NumLeft {
    N2 = 2,
    N6 = 6,
    N7 = 7,
}

impl NumLeft {
    pub fn from_int(num: i32) -> Result<NumLeft, ()> {
        match num {
            2 => Ok(NumLeft::N2),
            6 => Ok(NumLeft::N6),
            7 => Ok(NumLeft::N7),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Hash, Clone, EnumIter)]
pub enum NumRight {
    N3 = 3,
    N4 = 4,
    N8 = 8,
}

impl NumRight {
    pub fn from_int(num: i32) -> Result<NumRight, ()> {
        match num {
            3 => Ok(NumRight::N3),
            4 => Ok(NumRight::N4),
            8 => Ok(NumRight::N8),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Tile {
    pub top: NumTop,
    pub left: NumLeft,
    pub right: NumRight,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Tile({} {} {})",
            self.top as i32, self.left as i32, self.right as i32
        )
    }
}

#[macro_export]
macro_rules! tile {
    ($top:expr, $left:expr, $right:expr) => {
        tile::Tile {
            top: tile::NumTop::from_int($top).unwrap(),
            left: tile::NumLeft::from_int($left).unwrap(),
            right: tile::NumRight::from_int($right).unwrap(),
        }
    };
}
