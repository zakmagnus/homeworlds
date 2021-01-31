use std::fmt;
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
            State::Setup(player) => self.setup_unchecked(player, setup_move),
            _ => Err(InputError::WrongState),
        }
    }

    fn setup_unchecked(&mut self, player: PlayerIndex, setup_move: &SetupMove) -> Result<(), InputError> {
        self.bank.remove(setup_move.stars[0])?;
        self.bank.remove(setup_move.stars[1])?;
        self.bank.remove(setup_move.ship)?;

        let mut homeworld = System::new_homeworld(setup_move.stars, player);
        homeworld.add_ship(player, setup_move.ship);
        self.systems.push(homeworld);

        let next_player = player + 1;
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

    pub fn sacrifice(&mut self, system: SystemIndex, ship: Piece) -> Result<(), InputError> {
        match self.state {
            State::Turn(player, TurnPhase::Started) => {
                let system_data = self.systems.get_mut(system as usize);
                match system_data {
                    None => Err(InputError::BadSystem),
                    Some(system_data) => {
                        system_data.remove_ship(player, ship)?;
                        self.evaporate_system_if_necessary(system);
                        self.state = State::Turn(player, TurnPhase::Sacrifice(ship.color, ship.size.to_u8()));
                        self.end_game_if_necessary();
                        Ok(())
                    }
                }
            },
            State::Turn(_, _) => Err(InputError::WrongPhase),
            _ => Err(InputError::WrongState),
        }
    }

    pub fn catastrophe(&mut self, system: SystemIndex, color: Color) -> Result<(), InputError> {
        if let State::Setup(_) | State::Finished(_) = self.state {
            return Err(InputError::WrongState);
        }
        let system_data = self.systems.get_mut(system as usize);
        match system_data {
            None => Err(InputError::BadSystem),
            Some(system_data) => {
                let color_count = system_data.color_count(color);
                if color_count < CATASTROPHE_COUNT {
                    return Err(InputError::NotCatastrophicEnough);
                }
                let result = system_data.catastrophe(color, &mut self.bank);
                if let CatastropheResult::SystemEvaporated = result {
                    self.systems.remove(system as usize);
                }
                self.end_game_if_necessary();
                Ok(())
            }
        }
    }

    // TODO this should be able to return something, at least for yellow discoveries
    pub fn action(&mut self, action: Action) -> Result<(), InputError> {
        match self.state {
            State::Turn(player, TurnPhase::FreeMove(system, color)) => {
                Game::check_free_move(system, color, action)?;
                self.check_action(player, action)?;
                self.action_unchecked(player, action)?;
                self.state = State::Turn(player, TurnPhase::Done);
                self.end_game_if_necessary();
                Ok(())
            },
            State::Turn(player, TurnPhase::Sacrifice(color, moves_left)) => {
                Game::check_sacrifice(color, moves_left, action)?;
                self.check_action(player, action)?;
                self.action_unchecked(player, action)?;
                let moves_left = moves_left - 1;
                self.state = if moves_left > 0 {
                    State::Turn(player, TurnPhase::Sacrifice(color, moves_left))
                } else {
                    State::Turn(player, TurnPhase::Done)
                };
                self.end_game_if_necessary();
                Ok(())
            },
            State::Turn(_, _) => Err(InputError::WrongPhase),
            _ => Err(InputError::WrongState),
        }
    }

    pub fn end_turn(&mut self) -> Result<(), InputError> {
        match self.state {
            State::Turn(player, TurnPhase::Done) => {
                let next_player = (player + 1) % NUM_PLAYERS;
                self.state = State::Turn(next_player, TurnPhase::Started);
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
            ColorAction::YellowAction(yellow_action_input) => self.yellow_action(player, action.system, action.ship, &yellow_action_input),
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

    fn yellow_action(&mut self, player: PlayerIndex, system: SystemIndex, ship: Piece, yellow_action_input: &YellowActionInput)
        -> Result<(), InputError> {
        // TODO this should not return an error after mutating data
        let target_system = self.get_yellow_target(system, yellow_action_input)?;
        target_system.add_ship(player, ship);
        let system_data = self.systems.get_mut(system as usize).unwrap();
        system_data.remove_ship(player, ship)?;
        self.evaporate_system_if_necessary(system);
        Ok(())
    }

    fn evaporate_system_if_necessary(&mut self, system: SystemIndex) {
        let system_data = self.systems.get_mut(system as usize).unwrap();
        if system_data.is_empty() {
            for &star in system_data.stars().iter() {
                self.bank.add(star);
            }
            self.systems.remove(system as usize);
        }
    }

    fn get_yellow_target(&mut self, system: SystemIndex, yellow_action_input: &YellowActionInput)
        -> Result<&mut System, InputError> {
        let system = self.systems.get(system as usize).unwrap();
        match yellow_action_input {
            YellowActionInput::Existing(existing_system_index) => {
                let existing_system = self.systems.get(*existing_system_index as usize);
                match existing_system {
                    None => Err(InputError::BadSystem),
                    Some(existing_system) => {
                        if !system.is_adjacent(existing_system) {
                            return Err(InputError::SystemsNotAdjacent);
                        }
                        Ok(self.systems.get_mut(*existing_system_index as usize).unwrap())
                    }
                }
            },
            YellowActionInput::Discover(new_star) => {
                let new_system = System::new(*new_star);
                if !system.is_adjacent(&new_system) {
                    return Err(InputError::SystemsNotAdjacent);
                }
                self.systems.push(new_system);
                Ok(self.systems.last_mut().unwrap())
            },
        }
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
        for star in system.stars().iter() {
            if star.color == color {
                return Ok(());
            }
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

    fn end_game_if_necessary(&mut self) -> bool {
        let winner = self.get_winner();
        if let Some(winner) = winner {
            self.state = State::Finished(winner);
            true
        } else {
            false
        }
    }

    fn get_winner(&self) -> Option<PlayerIndex> {
        let non_losers: Vec<PlayerIndex> = (0..NUM_PLAYERS).into_iter().filter(|&player| !self.is_loser(player)).collect();
        return if non_losers.len() == 1 {
            Some(*non_losers.get(0).unwrap())
        } else {
            None
        }
    }

    fn is_loser(&self, player: PlayerIndex) -> bool {
        let system_loss_status = self.systems.iter()
            .filter(|system| match system.home_player {
                None => false,
                Some(home_player) => home_player == player,
            })
            .map(|home_system| home_system.get_ships(home_system.home_player.unwrap()).is_empty())
            .next();
        match system_loss_status {
            None => false, // No home system, no ships at home system. Lose.
            Some(is_loser) => is_loser,
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::Setup(player) => write!(f, "Player {}'s setup", player),
            State::Finished(winner) => write!(f, "Game over, player {} wins", winner),
            State::Turn(player, turn_phase) => {
                write!(f, "Player {}'s turn, ", player)?;
                match turn_phase {
                    TurnPhase::Started => write!(f, "no move selected"),
                    TurnPhase::Done => write!(f, "no moves left"),
                    TurnPhase::FreeMove(system, color) =>
                        write!(f, "free {} move in system {}", color, system),
                    TurnPhase::Sacrifice(color, moves_left) =>
                        write!(f, "{} sacrifice, {} move(s) left", color, moves_left),
                }
            },
        }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\nSystems:\n", self.state)?;
        for system in self.systems.iter() {
            write!(f, " - {}\n", system)?;
        }
        Ok(())
    }
}
