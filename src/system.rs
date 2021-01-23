use std::collections::HashMap;
use crate::common::*;

#[derive(Debug)]
pub struct System {
    pub star: Piece,
    pub second_star: Option<Piece>,
    pub home_player: Option<PlayerIndex>,
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

    pub fn remove_ship(&mut self, player: PlayerIndex, ship: Piece) -> Result<(), InputError> {
        let player_ships = self.ships.get_mut(&player).unwrap();
        let ship_position = player_ships.iter().position(|player_ship| *player_ship == ship);
        if let Some(ship_position) = ship_position {
            player_ships.remove(ship_position);
            Ok(())
        } else {
            Err(InputError::NoSuchShip)
        }
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
