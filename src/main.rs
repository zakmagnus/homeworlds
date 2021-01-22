mod common;
mod bank;
mod system;
mod inputs;
mod game;

use std::fmt;
use std::collections::HashMap;
use common::*;
use bank::*;
use system::*;
use inputs::*;
use game::*;

fn main() {
    let mut game = Game::new();
    println!("new game? {:?}\n", game);

    game.setup(&SetupMove{ player: 0, stars: [Piece {color: Color::RED, size: Size::SMALL}, Piece{color: Color::YELLOW, size: Size::MEDIUM}], ship: Piece{color: Color::GREEN, size: Size::LARGE} });
    println!("advanced game? {:?}", game);

    let big = System::new(Piece {color: Color::RED, size: Size::LARGE});
    let medium = System::new(Piece {color: Color::RED, size: Size::MEDIUM});
    let small = System::new(Piece {color: Color::RED, size: Size::SMALL});
    let homeworld = &System::new_homeworld([Piece {color: Color::RED, size: Size::SMALL}, Piece{color: Color::YELLOW, size: Size::MEDIUM}], 0);
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
