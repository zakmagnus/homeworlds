use crate::common::*;
use crate::bank::*;
use crate::system::*;
use crate::inputs::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum State {
    Setup(PlayerIndex),
    Turn(PlayerIndex, TurnPhase),
    Finished(PlayerIndex), // The winner's index
}

#[derive(Debug)]
pub struct Game {
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
        match self.state {
            State::Setup(player) if player == setup_move.player => self.setup_unchecked(setup_move),
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
            self.state = State::Turn(0, TurnPhase::Started);
        }
        Ok(())
    }

    pub fn free_move(&mut self, system: SystemIndex, color: Color) -> Result<(), InputError> {
        match self.state {
            State::Turn(player, TurnPhase::Started) => {
                self.check_free_move_available(player, system, color)?;
                self.state = State::Turn(player, TurnPhase::FreeMove(system, color));
                return Ok(());
            },
            State::Turn(_, _) => Err(InputError::WrongPhase),
            _ => Err(InputError::WrongState),
        }
    }

    pub fn action(&mut self, action: Action) -> Result<(), InputError> {
        match self.state {
            State::Turn(player, TurnPhase::FreeMove(system, color)) => {
                Game::check_free_move(system, color, action)?;
                self.check_action(player, action)?;
                self.action_unchecked(player, action)?;
                self.state = State::Turn(player, TurnPhase::Done);
                Ok(())
            },
            State::Turn(player, TurnPhase::Sacrifice(color, moves_left)) => {
                Game::check_sacrifice(color, moves_left, action)?;
                self.check_action(player, action)?;
                self.action_unchecked(player, action)?;
                self.state = State::Turn(player, TurnPhase::Sacrifice(color, moves_left - 1));
                Ok(())
            },
            State::Turn(_, _) => Err(InputError::WrongPhase),
            _ => Err(InputError::WrongState),
        }
    }

    fn check_action(&self, player: PlayerIndex, action: Action) -> Result<(), InputError> {
        let system = self.systems.get(action.system as usize);
        match system {
            None => Err(InputError::BadSystem),
            Some(system) => {
                if !system.has_ship(player, action.ship) {
                    return Err(InputError::NoSuchShip);
                }
                Ok(())
            }
        }
    }

    fn action_unchecked(&mut self, player: PlayerIndex, action: Action) -> Result<(), InputError> {
        match action.color_action {
            ColorAction::RedAction(red_action_input) => self.red_action(player, action.system, action.ship, &red_action_input),
            ColorAction::BlueAction(new_color) => self.blue_action(player, action.system, action.ship, new_color),
            ColorAction::GreenAction => self.green_action(player, action.system, action.ship),
            ColorAction::YellowAction(_) => Ok(()) // TODO
        }
    }

    fn red_action(&mut self, player: PlayerIndex, system: SystemIndex, ship: Piece, RedActionInput { enemy_player, ship_to_take }: &RedActionInput) -> Result<(), InputError> {
        if player == *enemy_player {
            return Err(InputError::WrongPlayer);
        }
        if ship_to_take.size > ship.size {
            return Err(InputError::ShipTooBig);
        }
        let system = self.systems.get_mut(system as usize).unwrap();
        system.remove_ship(*enemy_player, *ship_to_take)?;
        system.add_ship(player, *ship_to_take);
        // TODO check for win ?
        Ok(())
    }

    fn blue_action(&mut self, player: PlayerIndex, system: SystemIndex, ship: Piece, new_color: Color)
        -> Result<(), InputError> {
        if ship.color == new_color {
            return Err(InputError::WrongColor);
        }
        let system = self.systems.get_mut(system as usize).unwrap();
        system.remove_ship(player, ship)?;
        let new_ship = Piece { color: new_color, size: ship.size };
        system.add_ship(player, new_ship);
        Ok(())
    }

    fn green_action(&mut self, player: PlayerIndex, system: SystemIndex, ship: Piece) -> Result<(), InputError> {
        let system = self.systems.get_mut(system as usize).unwrap();
        let possible_new_ships = [
            Piece { color: ship.color, size: Size::SMALL },
            Piece { color: ship.color, size: Size::MEDIUM },
            Piece { color: ship.color, size: Size::LARGE }];
        for new_ship in possible_new_ships.iter() {
            if self.bank.num_available(*new_ship) > 0 {
                self.bank.remove(*new_ship);
                system.add_ship(player, *new_ship);
                return Ok(());
            }
        }
        Err(InputError::PieceUnavailable)
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

    fn check_free_move(system: SystemIndex, color: Color, action: Action) -> Result<(), InputError> {
        if system != action.system {
            return Err(InputError::WrongSystem);
        }
        Game::check_action_color(color, action.color_action)
    }

    fn check_sacrifice(color: Color, moves_left: u8, action: Action) -> Result<(), InputError> {
        if moves_left <= 0 {
            return Err(InputError::NoActionsLeft);
        }
        Game::check_action_color(color, action.color_action)
    }

    fn check_action_color(color: Color, color_action: ColorAction) -> Result<(), InputError> {
        match color_action {
            ColorAction::RedAction(_) if color != Color::RED => Err(InputError::WrongActionColor),
            ColorAction::BlueAction(_) if color != Color::BLUE => Err(InputError::WrongActionColor),
            ColorAction::GreenAction if color != Color::GREEN => Err(InputError::WrongActionColor),
            ColorAction::YellowAction(_) if color != Color::YELLOW => Err(InputError::WrongActionColor),
            _ => Ok(())
        }
    }
}
