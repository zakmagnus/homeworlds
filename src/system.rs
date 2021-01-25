use std::collections::HashMap;
use crate::common::*;

#[derive(Debug)]
pub struct System {
    star: Piece,
    second_star: Option<Piece>,
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

    pub fn stars(&self) -> Vec<Piece> {
        match self.second_star {
            None => vec![self.star],
            Some(second_star) => vec![self.star, second_star],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.ships.is_empty()
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
        let stars = self.stars();
        let other_stars = other_system.stars();
        for star in stars.iter() {
            for other_star in other_stars.iter() {
                if star.size == other_star.size {
                    return false;
                }
            }
        }
        true
    }
}
