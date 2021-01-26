use std::fmt;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    RED,
    BLUE,
    GREEN,
    YELLOW,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Color::RED => "Red",
            Color::BLUE => "Blue",
            Color::GREEN => "Green",
            Color::YELLOW => "Yellow",
        };
        write!(f, "{}", name)
    }
}

pub const ALL_COLORS: [Color; 4] = [Color::RED, Color::BLUE, Color::GREEN, Color::YELLOW];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord)]
pub enum Size {
    SMALL,
    MEDIUM,
    LARGE,
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Size::SMALL => "Small",
            Size::MEDIUM => "Medium",
            Size::LARGE => "Large",
        };
        write!(f, "{}", name)
    }
}

impl Size {
    pub fn to_u8(&self) -> u8 {
        match self {
            Size::SMALL => 1,
            Size::MEDIUM => 2,
            Size::LARGE => 3,
        }
    }
}

impl PartialOrd<Size> for Size {
    fn partial_cmp(&self, other: &Size) -> Option<Ordering> {
        Some(self.to_u8().cmp(&other.to_u8()))
    }
}

pub const ALL_SIZES: [Size; 3] = [Size::SMALL, Size::MEDIUM, Size::LARGE];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Piece {
    pub color: Color,
    pub size: Size,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.size, self.color)
    }
}

pub type PlayerIndex = u8;
pub type SystemIndex = u8;

pub const NUM_PLAYERS: u8 = 2;

pub const CATASTROPHE_COUNT: i32 = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputError {
    WrongState,
    WrongPhase,
    WrongPlayer,
    PieceUnavailable,
    WrongActionColor,
    WrongSystem,
    NoSuchShip,
    BadSystem,
    FreeActionUnavailable,
    NoActionsLeft,
    WrongColor,
    ShipTooBig,
    SystemsNotAdjacent,
    BadPiece,
    NotCatastrophicEnough,
}
