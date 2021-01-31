mod common;
mod bank;
mod system;
mod inputs;
mod game;

use std::io;
use common::*;
use inputs::*;
use game::*;
use crate::inputs::ColorAction::{GreenAction, YellowAction, BlueAction, RedAction};
use crate::common::Color::*;
use crate::common::Size::*;
use std::str::SplitWhitespace;

fn main() {
    let mut game = Game::new();

    let mut input = String::new();
    loop {
        println!("{}", game);
        print!(" >\n");
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut tokens = input.split_whitespace();
        let first_token = tokens.next();
        let finished = match first_token {
            None => false,
            Some("quit") => true,
            Some("setup") => input_setup(tokens, &mut game),
            Some("free") => input_free(tokens, &mut game),
            Some("sac") => input_sacrifice(tokens, &mut game),
            Some("catastrophe") => input_catastrophe(tokens, &mut game),
            Some("end") => input_end(&mut game),
            Some("red") => input_red(tokens, &mut game),
            Some("green") => input_green(tokens, &mut game),
            Some("blue") => input_blue(tokens, &mut game),
            Some("yellow") => input_yellow(tokens, &mut game),
            Some(first_token) => {
                println!("Unknown input: {}", first_token);
                false
            },
        };
        if finished {
            break;
        }
    }
}

fn input_free(mut tokens: SplitWhitespace, game: &mut Game) -> bool {
    let color_input = tokens.next();
    if let None = color_input {
        println!("Malformed input, color not specified");
        return false;
    }
    let color_input = color_input.unwrap();
    let color = parse_color(color_input);
    if let None = color {
        println!("Color not recognized: {}", color_input);
        return false;
    }
    let color = color.unwrap();
    let system_input = tokens.next();
    if let None = system_input {
        println!("Malformed input, system not specified");
        return false;
    }
    let system_input = system_input.unwrap();
    let parse_result = system_input.parse::<u8>();
    if let Err(error) = parse_result {
        println!("System ID {} is not a number: {:?}", system_input, error);
        return false;
    }
    let system = parse_result.unwrap();
    let result = game.free_move(system, color);
    if let Err(error) = result {
        println!("Failed to pick a free action: {:?}", error);
    }
    false
}

fn input_yellow(p0: SplitWhitespace, p1: &mut Game) -> bool {
    unimplemented!()
}

fn input_blue(p0: SplitWhitespace, p1: &mut Game) -> bool {
    unimplemented!()
}

fn input_green(p0: SplitWhitespace, p1: &mut Game) -> bool {
    unimplemented!()
}

fn input_red(p0: SplitWhitespace, p1: &mut Game) -> bool {
    unimplemented!()
}

fn input_catastrophe(p0: SplitWhitespace, p1: &mut Game) -> bool {
    unimplemented!()
}

fn input_sacrifice(p0: SplitWhitespace, p1: &mut Game) -> bool {
    unimplemented!()
}

fn input_setup(mut tokens: SplitWhitespace, game: &mut Game) -> bool {
    let star1 = parse_next_token_as_piece(&mut tokens, "star 1");
    if let None = star1 {
        return false;
    }
    let star1 = star1.unwrap();
    let star2 = parse_next_token_as_piece(&mut tokens, "star 2");
    if let None = star2 {
        return false;
    }
    let star2 = star2.unwrap();
    let ship = parse_next_token_as_piece(&mut tokens, "starting ship");
    if let None = ship {
        return false;
    }
    let ship = ship.unwrap();
    let result = game.setup(&SetupMove { ship, stars: [star1, star2] });
    if let Err(error) = result {
        println!("Setup attempt failed: {:?}", error);
        return false;
    }
    false
}

fn input_end(game: &mut Game) -> bool {
    game.end_turn();
    false
}

fn parse_next_token_as_piece(tokens: &mut SplitWhitespace, description: &str) -> Option<Piece> {
    let piece_input = tokens.next();
    if let None = piece_input {
        println!("Malformed input, {} not specified", description);
        return None;
    }
    let piece_input = piece_input.unwrap();
    let piece = parse_piece(piece_input);
    if let None = piece {
        println!("{} is not a recognized piece: {}", description, piece_input);
        return None;
    }
    piece
}

fn parse_piece(string: &str) -> Option<Piece> {
    if string.len() != 2 {
        return None;
    }
    let size_char = string.get(0..1).unwrap();
    let color_char = string.get(1..2).unwrap();
    let size = parse_size(size_char);
    let color = parse_color(color_char);
    if let None = size {
        return None;
    }
    if let None = color {
        return None;
    }
    Some(Piece { size: size.unwrap(), color: color.unwrap() })
}

fn parse_size(string: &str) -> Option<Size> {
     match string {
        "s" => Some(SMALL),
        "m" => Some(MEDIUM),
        "l" => Some(LARGE),
        _ => None,
    }
}

fn parse_color(string: &str) -> Option<Color> {
    match string {
        "r" => Some(RED),
        "g" => Some(GREEN),
        "b" => Some(BLUE),
        "y" => Some(YELLOW),
        _ => None,
    }
}