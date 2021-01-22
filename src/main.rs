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
type SystemIndex = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum State {
    Setup(PlayerIndex),
    Turn(PlayerIndex),
    Finished(PlayerIndex), // The winner's index
}

#[derive(Debug)]
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
    WrongActionColor,
    WrongSystem,
    NoSuchShip,
    BadSystem,
    FreeActionUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct DeclareCatastrophe {
    system: SystemIndex,
    color: Color,
}

#[derive(Debug)]
enum TurnInput {
    Free(FreeMove),
    Sacrifice(SacrificeMove),
}

#[derive(Debug)]
struct FreeMove {
    system: SystemIndex,
    color: Color,
    actions: Vec<Action>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Action {
    Catastrophe(DeclareCatastrophe),
    RedAction(RedActionInput),
    //TODO all the other colors
}

#[derive(Debug)]
struct SacrificeMove {
    //TODO
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct RedActionInput {
    system: SystemIndex,
    ship: Piece,
    enemy_player: PlayerIndex,
    ship_to_take: Piece,
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

    fn no_ships() -> HashMap<PlayerIndex, Vec<Piece>> {
        let mut initial_ships = HashMap::new();
        for player_index in 0..NUM_PLAYERS {
            initial_ships.insert(player_index, Vec::new());
        }
        initial_ships
    }

    pub fn add_ship(&mut self, player: PlayerIndex, ship: Piece) {
        self.ships.get_mut(&player).unwrap().push(ship);
    }

    pub fn remove_ship(&mut self, player: PlayerIndex, ship: Piece) {
        let player_ships = self.ships.get_mut(&player).unwrap();
        let ship_position = player_ships.iter().position(|player_ship| *player_ship == ship).unwrap();
        player_ships.remove(ship_position);
    }

    pub fn has_ship(&self, player: PlayerIndex, ship: Piece) -> bool {
        self.ships.get(&player).unwrap().iter().any(|player_ship| ship == *player_ship)
    }

    pub fn get_ships(&self, player: PlayerIndex) -> &Vec<Piece> {
        self.ships.get(&player).unwrap()
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

    // TODO SetupMove packs the player into the input, pick one way and stick to it
    pub fn turn(&mut self, player: PlayerIndex, turn: &TurnInput) -> Result<(), InputError> {
        match self.state {
            State::Turn(player) => {
                match turn {
                    TurnInput::Free(free_move) => self.free_move(player, free_move),
                    TurnInput::Sacrifice(sacrifice_move) => self.sacrifice_move(player, sacrifice_move),
                }
            },
            State::Turn(_) => Err(InputError::WrongPlayer),
            _ => Err(InputError::WrongState),
        }
    }

    fn free_move(&mut self, player: PlayerIndex, FreeMove { system, color, actions }: &FreeMove) -> Result<(), InputError> {
        self.check_free_move_available(player, *system, *color)?;
        for action in actions.iter() {
            match action {
                Action::RedAction(red_action_input) if *color == Color::RED => {
                    Game::check_system(*system, red_action_input.system)?;
                    return self.red_action(player, red_action_input);
                },
                Action::RedAction(_) => Err(InputError::WrongActionColor),
                _ => Ok(()) // TODO
            }?;
        }
        Ok(()) // TODO check the right amount of action happened
    }

    fn red_action(&mut self, player: PlayerIndex, RedActionInput { system, ship, enemy_player, ship_to_take }: &RedActionInput) -> Result<(), InputError> {
        if player == *enemy_player {
            return Err(InputError::WrongPlayer);
        }
        let system_data = self.systems.get_mut(*system as usize).unwrap();
        if !system_data.has_ship(player, *ship) {
            return Err(InputError::NoSuchShip);
        }
        if !system_data.has_ship(*enemy_player, *ship_to_take) {
            return Err(InputError::NoSuchShip);
        }
        system_data.remove_ship(*enemy_player, *ship_to_take);
        system_data.add_ship(player, *ship_to_take);
        // TODO check for win ?
        Ok(())
    }

    fn check_system(system: SystemIndex, input_system: SystemIndex) -> Result<(), InputError> {
        if system != input_system {
            return Err(InputError::WrongSystem);
        }
        Ok(())
    }

    fn check_free_move_available(&self, player: PlayerIndex, system: SystemIndex, color: Color) -> Result<(), InputError> {
        let system = self.systems.get(system as usize);
        if let None = system {
            return Err(InputError::BadSystem);
        }
        let system = system.unwrap();
        let available_ships = system.get_ships(player);
        if available_ships.is_empty() {
            return Err(InputError::FreeActionUnavailable);
        }
        if system.star.color == color {
            return Ok(());
        }
        if system.second_star.map_or(false, |star| star.color == color) {
            return Ok(());
        }
        if available_ships.iter().any(|ship| ship.color == color) {
            return Ok(());
        }
        return Err(InputError::FreeActionUnavailable);
    }

    fn sacrifice_move(&mut self, player: PlayerIndex, sacrifice_move: &SacrificeMove) -> Result<(), InputError> {
        Ok(()) // TODO
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
