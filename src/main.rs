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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Piece {
    color: Color,
    size: Size,
}

#[derive(Debug)]
struct Bank {
    available_amounts: HashMap<Piece, u8>,
}

impl Bank {
    fn full() -> Bank {
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
}

impl fmt::Display for Bank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bank - ")?;
        for color in ALL_COLORS.iter() {
            write!(f, "{}: ", color)?;
            for (index, size) in ALL_SIZES.iter().enumerate() {
                write!(f, "{} {}", self.num_available(Piece {color: *color, size: *size}), size)?;
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

#[derive(Debug)]
struct System {
    star: Piece,
    second_star: Option<Piece>,
    home_player: Option<PlayerIndex>,
    ships: HashMap<PlayerIndex, Vec<Piece>>,
}

impl System {
    pub fn new_homeworld(stars: [Piece; 2], player: PlayerIndex) -> System {
        System {
            star: stars[0],
            second_star: Some(stars[1]),
            home_player: Some(player),
            ships: System::no_ships(),
        }
    }

    pub fn new(star: Piece) -> System {
        System {
            star,
            second_star: None,
            home_player: None,
            ships: System::no_ships(),
        }
    }

    pub fn add_ship(&mut self, player: PlayerIndex, ship: Piece) {
        self.ships.get_mut(&player).unwrap().push(ship);
    }

    pub fn is_adjacent(&self, other_system: &System) -> bool {
        if self.star.size == other_system.star.size {
            return false;
        }
        if let Some(second_star) = self.second_star {
            if second_star.size == other_system.star.size {
                return false;
            }
        }
        if let Some(other_second_star) = other_system.second_star {
            if self.star.size == other_second_star.size {
                return false;
            }
            if let Some(second_star) = self.second_star {
                if second_star.size == other_second_star.size {
                    return false;
                }
            }
        }
        true
    }

    fn no_ships() -> HashMap<PlayerIndex, Vec<Piece>> {
        let mut initial_ships = HashMap::new();
        for player_index in 0..NUM_PLAYERS {
            initial_ships.insert(player_index, Vec::new());
        }
        initial_ships
    }
}

const NUM_PLAYERS: u8 = 2;

#[derive(Debug)]
struct Game {
    state: State,
    bank: Bank,
    systems: Vec<System>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            bank: Bank::full(),
            state: State::Setup(0),
            systems: Vec::new(),
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

        let mut homeworld = System::new_homeworld(setup_move.stars, setup_move.player);
        homeworld.add_ship(setup_move.player, setup_move.ship);
        self.systems.push(homeworld);

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

    game.setup(&SetupMove{ player: 0, stars: [Piece {color: Color::RED, size: Size::SMALL}, Piece{color: Color::YELLOW, size: Size::MEDIUM}], ship: Piece{color: Color::GREEN, size: Size::LARGE} });
    println!("advanced game? {:?}", game);

    let big = System::new(Piece {color: Color::RED, size: Size::LARGE});
    let medium = System::new(Piece {color: Color::RED, size: Size::MEDIUM});
    let small = System::new(Piece {color: Color::RED, size: Size::SMALL});
    let homeworld = game.systems.get(0).unwrap();
    println!("b b {}", big.is_adjacent(&big));
    println!("b m {}", big.is_adjacent(&medium));
    println!("b s {}", big.is_adjacent(&small));
    println!("b h {}", big.is_adjacent(homeworld));

    println!("m b {}", medium.is_adjacent(&big));
    println!("m m {}", medium.is_adjacent(&medium));
    println!("m s {}", medium.is_adjacent(&small));
    println!("m h {}", medium.is_adjacent(homeworld));

    println!("s b {}", small.is_adjacent(&big));
    println!("s m {}", small.is_adjacent(&medium));
    println!("s s {}", small.is_adjacent(&small));
    println!("s h {}", small.is_adjacent(homeworld));

    println!("h b {}", homeworld.is_adjacent(&big));
    println!("h m {}", homeworld.is_adjacent(&medium));
    println!("h s {}", homeworld.is_adjacent(&small));
    println!("h h {}", homeworld.is_adjacent(homeworld));

    let hw2 = System::new_homeworld([Piece {color: Color::RED, size: Size::MEDIUM}, Piece {color: Color::RED, size: Size::LARGE}], 0);
    println!("h1 h2 {}", homeworld.is_adjacent(&hw2));
    println!("h2 h1 {}", hw2.is_adjacent(homeworld));
}
