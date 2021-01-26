use std::fmt;
use std::collections::HashMap;
use crate::common::*;
use crate::bank::*;

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

    pub fn color_count(&self, color: Color) -> i32 {
        let mut count = 0;
        for star in self.stars() {
            if star.color == color {
                count += 1;
            }
        }
        for ships in self.ships.values().into_iter() {
            for ship in ships {
                if ship.color == color {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn catastrophe(&mut self, color: Color, bank: &mut Bank) -> CatastropheResult {
        let no_ships_left = self.ships.values_mut().into_iter().all(|ships| {
            ships.retain(|&ship| {
                if ship.color != color {
                    return true;
                }
                bank.add(ship);
                return false;
            });
            ships.is_empty()
        });
        let mut no_stars_left = false;
        if !no_ships_left {
            let stars = self.stars();
            let stars_to_kill: Vec<Piece> = stars.iter()
                .filter(|&star| star.color == color)
                .map(|star| *star).collect();
            if stars_to_kill.len() >= stars.len() {
                // Everything in the system will get banked later because of this.
                no_stars_left = true;
            } else if !stars_to_kill.is_empty() {
                // One star dies and the other remains
                let &star_to_kill = stars_to_kill.get(0).unwrap();
                let only_star_left = stars.iter().filter(|&&star| star != star_to_kill).next().unwrap();
                bank.add(star_to_kill);
                self.star = *only_star_left;
                self.second_star = None;
            }
        }
        if no_ships_left || no_stars_left {
            // Evaporate: bank everything.
            for ships in self.ships.values() {
                for &ship in ships {
                    bank.add(ship);
                }
            }
            for star in self.stars() {
                bank.add(star);
            }
            CatastropheResult::SystemEvaporated
        } else {
            CatastropheResult::SystemStillExists
        }
    }
}

pub enum CatastropheResult {
    SystemStillExists,
    // The system should not be used after it's evaporated.
    SystemEvaporated,
}

impl fmt::Display for System {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(home_player) = self.home_player {
            write!(f, "Player {}'s homeworld, ", home_player)?;
        }
        if let Some(second_star) = self.second_star {
            write!(f, "binary stars {}/{}; ", self.star, second_star)?;
        } else {
            write!(f, "{} star; ", self.star)?;
        }
        for (player, ships) in self.ships.iter() {
            if ships.is_empty() {
                continue;
            }
            write!(f, "Player {}'s ship(s): ", player)?;
            for (index, ship) in ships.iter().enumerate() {
                write!(f, "{}", ship)?;
                if index >= ships.len() - 1 {
                    write!(f, "; ")?;
                } else {
                    write!(f, ", ")?;
                }
            }
        }
        Ok(())
    }
}