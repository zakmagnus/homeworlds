use std::fmt;
use std::collections::HashMap;
use crate::common::*;

#[derive(Debug)]
pub struct Bank {
    available_amounts: HashMap<Piece, u8>,
}

impl Bank {
    pub fn full() -> Bank {
        let mut available_amounts = HashMap::new();
        for color in ALL_COLORS.iter() {
            for size in ALL_SIZES.iter() {
                available_amounts.insert(Piece {color: *color, size: *size}, 3);
            }
        }
        Bank { available_amounts, }
    }

    pub fn num_available(&self, piece: Piece) -> u8 {
        *self.available_amounts.get(&piece).unwrap()
    }

    pub fn remove(&mut self, piece: Piece) -> Result<(), InputError> {
        if self.num_available(piece) < 1 {
            return Err(InputError::PieceUnavailable);
        }
        *self.available_amounts.get_mut(&piece).unwrap() -= 1;
        Ok(())
    }

    pub fn remove_several(&mut self, pieces: &[Piece]) -> Result<(), InputError> {
        let mut requested_counts = HashMap::new();
        for piece in pieces.iter() {
            let count = requested_counts.get_mut(piece);
            match count {
                None => {
                    requested_counts.insert(*piece, 1);
                },
                Some(count) => {
                    *count += 1;
                },
            }
        }
        for (piece, count) in requested_counts.iter() {
            if self.num_available(*piece) < *count {
                return Err(InputError::PieceUnavailable);
            }
        }
        for (piece, count) in requested_counts.iter() {
            *self.available_amounts.get_mut(piece).unwrap() -= count;
        }
        Ok(())
    }

    pub fn add(&mut self, piece: Piece) -> Result<(), InputError> {
        if self.num_available(piece) >= 3 {
            return Err(InputError::BadPiece);
        }
        *self.available_amounts.get_mut(&piece).unwrap() += 1;
        Ok(())
    }
}

impl fmt::Display for Bank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bank: ")?;
        for (color_index, color) in ALL_COLORS.iter().enumerate() {
            write!(f, "{} ", color)?;
            for (size_index, size) in ALL_SIZES.iter().enumerate() {
                let size_string = size.to_string();
                let size_string = size_string.get(0..1).unwrap();
                write!(f, "{}{}", self.num_available(Piece {color: *color, size: *size}), size_string)?;
                if size_index < ALL_SIZES.len() - 1 {
                    write!(f, " ")?;
                }
            }
            if (color_index < ALL_COLORS.len() - 1) {
                write!(f, " | ")?;
            }
        }
        write!(f, "")
    }
}
