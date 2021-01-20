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

type Piece = (Color, Size);

#[derive(Debug)]
struct Bank {
    available_amounts: HashMap<Piece, u8>,
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
}

impl fmt::Display for Bank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bank - ")?;
        for color in ALL_COLORS.iter() {
            write!(f, "{}: ", color)?;
            for (index, size) in ALL_SIZES.iter().enumerate() {
                write!(f, "{} {}", self.num_available((*color, *size)), size)?;
                if index < ALL_SIZES.len() - 1 {
                    write!(f, ", ")?;
                }
            }
            write!(f, "; ")?;
        }
        write!(f, "")
    }
}

type PlayerIndex = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum State {
    Setup(PlayerIndex),
    Turn(PlayerIndex),
    Finished(PlayerIndex), // The winner's index
}

struct SetupMove {
    player: PlayerIndex,
    stars: [Piece; 2],
    ship: Piece,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum InputError {
    WrongState,
    WrongPlayer,
    PieceUnavailable,
}

const NUM_PLAYERS: u8 = 2;

#[derive(Debug)]
struct Game {
    state: State,
    bank: Bank,
    // systems
}

impl Game {
    pub fn new() -> Game {
        Game {
            bank: Bank::full(),
            state: State::Setup(0),
        }
    }

    pub fn setup(&mut self, setup_move: &SetupMove) -> Result<(), InputError> {
        let player = setup_move.player;
        match self.state {
            State::Setup(player) => self.setup_unchecked(setup_move),
            State::Setup(_) => Err(InputError::WrongPlayer),
            _ => Err(InputError::WrongState),
        }
    }

    fn setup_unchecked(&mut self, setup_move: &SetupMove) -> Result<(), InputError> {
        self.bank.remove(setup_move.stars[0])?;
        self.bank.remove(setup_move.stars[1])?;
        self.bank.remove(setup_move.ship)?;
        // TODO update systems
        let next_player = setup_move.player + 1;
        if next_player < NUM_PLAYERS {
            self.state = State::Setup(next_player);
        } else {
            self.state = State::Turn(0);
        }
        Ok(())
    }
}

fn main() {
    let mut game = Game::new();
    println!("new game? {:?}\n", game);

    game.setup(&SetupMove{ player: 0, stars: [(Color::RED, Size::SMALL), (Color::YELLOW, Size::MEDIUM)], ship: (Color::GREEN, Size::LARGE) });
    println!("advanced game? {:?}", game);
}
