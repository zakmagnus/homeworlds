mod common;
mod bank;
mod system;
mod inputs;
mod game;

use common::*;
use system::*;
use inputs::*;
use game::*;
use crate::inputs::ColorAction::GreenAction;

fn main() {
    let mut game = Game::new();
    println!("{}", game);

    game.setup(&SetupMove{ player: 0,
        stars: [Piece {color: Color::RED, size: Size::SMALL}, Piece{color: Color::YELLOW, size: Size::MEDIUM}],
        ship: Piece{color: Color::GREEN, size: Size::LARGE} }).unwrap();
    println!("{}", game);
    game.setup(&SetupMove{ player: 1,
        stars: [Piece {color: Color::BLUE, size: Size::MEDIUM}, Piece{color: Color::GREEN, size: Size::SMALL}],
        ship: Piece{color: Color::YELLOW, size: Size::LARGE} }).unwrap();
    println!("{}", game);

    game.free_move(0, Color::GREEN).unwrap();
    println!("{}", game);
    game.action(Action { system: 0, ship: Piece { color: Color::GREEN, size: Size::LARGE },
        color_action: GreenAction { }}).unwrap();
    println!("{}", game);
    game.end_turn().unwrap();
    println!("{}", game);

    game.free_move(1, Color::GREEN).unwrap();
    println!("{}", game);
    game.action(Action { system: 1, ship: Piece { color: Color::YELLOW, size: Size::LARGE },
        color_action: GreenAction { }}).unwrap();
    println!("{}", game);
    game.end_turn().unwrap();
    println!("{}", game);


}
