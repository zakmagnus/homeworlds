use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Color {
    RED,
    BLUE,
    GREEN,
    YELLOW,
}

const ALL_COLORS: [Color; 4] = [Color::RED, Color::BLUE, Color::GREEN, Color::YELLOW];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Size {
    SMALL,
    MEDIUM,
    LARGE,
}

const ALL_SIZES: [Size; 3] = [Size::SMALL, Size::MEDIUM, Size::LARGE];

#[derive(Debug)]
struct Bank {
    available_amount: HashMap<(Color, Size), u8>,
}

impl Bank {
    fn full() -> Bank {
        let mut available_amount = HashMap::new();
        for color in ALL_COLORS.iter() {
            for size in ALL_SIZES.iter() {
                available_amount.insert((*color, *size), 3);
            }
        }
        Bank { available_amount, }
    }
}

fn main() {
    let new_bank = Bank::full();
    println!("Hello, world! {:?}", new_bank);
}
