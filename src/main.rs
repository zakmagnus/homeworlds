use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Color {
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

const ALL_COLORS: [Color; 4] = [Color::RED, Color::BLUE, Color::GREEN, Color::YELLOW];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Size {
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

const ALL_SIZES: [Size; 3] = [Size::SMALL, Size::MEDIUM, Size::LARGE];

#[derive(Debug)]
struct Bank {
    available_amounts: HashMap<(Color, Size), u8>,
}

impl Bank {
    fn full() -> Bank {
        let mut available_amounts = HashMap::new();
        for color in ALL_COLORS.iter() {
            for size in ALL_SIZES.iter() {
                available_amounts.insert((*color, *size), 3);
            }
        }
        Bank { available_amounts, }
    }

    pub fn num_available(&self, color: Color, size: Size) -> u8 {
        *self.available_amounts.get(&(color, size)).unwrap()
    }
}

impl fmt::Display for Bank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bank - ")?;
        for color in ALL_COLORS.iter() {
            write!(f, "{}: ", color)?;
            for (index, size) in ALL_SIZES.iter().enumerate() {
                write!(f, "{} {}", self.num_available(*color, *size), size)?;
                if index < ALL_SIZES.len() - 1 {
                    write!(f, ", ")?;
                }
            }
            write!(f, "; ")?;
        }
        write!(f, "")
    }
}

fn main() {
    let new_bank = Bank::full();
    println!("{}", new_bank);
}
