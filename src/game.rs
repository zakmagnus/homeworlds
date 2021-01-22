use crate::common::*;
use crate::bank::*;
use crate::system::*;
use crate::inputs::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum State {
    Setup(PlayerIndex),
    Turn(PlayerIndex),
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
