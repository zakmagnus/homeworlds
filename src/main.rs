mod common;
mod bank;
mod system;
mod inputs;
mod game;

use common::*;
use inputs::*;
use game::*;
use crate::inputs::ColorAction::{GreenAction, YellowAction, BlueAction, RedAction};
use crate::common::Color::*;
use crate::common::Size::*;

fn main() {
    let mut game = Game::new();
    println!("{}", game);

    game.setup(&SetupMove{ player: 0,
        stars: [Piece {color: RED, size: SMALL}, Piece{color: YELLOW, size: MEDIUM}],
        ship: Piece{color: GREEN, size: LARGE} }).unwrap();
    game.setup(&SetupMove{ player: 1,
        stars: [Piece {color: BLUE, size: MEDIUM}, Piece{color: GREEN, size: SMALL}],
        ship: Piece{color: YELLOW, size: LARGE} }).unwrap();
    println!("{}", game);

    game.free_move(0, GREEN).unwrap();
    game.action(Action { system: 0, ship: Piece { color: GREEN, size: LARGE },
        color_action: GreenAction { }}).unwrap();
    game.end_turn().unwrap();
    println!("{}", game);

    game.free_move(1, GREEN).unwrap();
    game.action(Action { system: 1, ship: Piece { color: YELLOW, size: LARGE },
        color_action: GreenAction { }}).unwrap();
    game.end_turn().unwrap();
    println!("{}", game);

    game.free_move(0, YELLOW).unwrap();
    game.action(Action { system: 0, ship: Piece { color: GREEN, size: SMALL },
        color_action: YellowAction(YellowActionInput::Discover(Piece { color: BLUE, size: LARGE }))}).unwrap();
    game.end_turn().unwrap();
    println!("{}", game);

    game.free_move(1, YELLOW).unwrap();
    game.action(Action { system: 1, ship: Piece { color: YELLOW, size: LARGE },
        color_action: YellowAction(YellowActionInput::Existing(2))}).unwrap();
    game.end_turn().unwrap();
    println!("{}", game);

    game.free_move(2, GREEN).unwrap();
    game.action(Action { system: 2, ship: Piece { color: GREEN, size: SMALL },
        color_action: GreenAction {} }).unwrap();
    game.end_turn().unwrap();
    println!("{}", game);

    game.free_move(2, BLUE).unwrap();
    game.action(Action { system: 2, ship: Piece { color: YELLOW, size: LARGE },
        color_action: BlueAction(RED) }).unwrap();
    game.end_turn().unwrap();
    println!("{}", game);

    game.free_move(2, GREEN).unwrap();
    game.action(Action { system: 2, ship: Piece { color: GREEN, size: SMALL },
        color_action: GreenAction {} }).unwrap();
    game.end_turn().unwrap();
    println!("{}", game);

    game.free_move(2, RED).unwrap();
    game.action(Action { system: 2, ship: Piece { color: RED, size: LARGE },
        color_action: RedAction(RedActionInput{enemy_player: 0,
            ship_to_take: Piece { color: GREEN, size: SMALL }}) }).unwrap();
    game.end_turn().unwrap();
    println!("{}", game);

    game.sacrifice(2, Piece { size: MEDIUM, color: GREEN }).unwrap();
    println!("{}", game);
    game.action(Action { system: 2, ship: Piece { color: GREEN, size: SMALL },
        color_action: GreenAction {} }).unwrap();
    println!("{}", game);
    game.action(Action { system: 2, ship: Piece { color: GREEN, size: SMALL },
        color_action: GreenAction {} }).unwrap();
    println!("{}", game);
    game.catastrophe(2, GREEN).unwrap();
    println!("{}", game);
    game.end_turn().unwrap();
    println!("{}", game);
}
