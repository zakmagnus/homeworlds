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
    let mut last_input_failed = false;
    loop {
        if !last_input_failed {
            println!("{}", game);
        }
        last_input_failed = false;
        print!(" >\n");
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut tokens = input.split_whitespace();
        let first_token = tokens.next();
        let result = match first_token {
            None => Err("".into()),
            Some("quit") => Ok(true),
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
                Err(format!("Unknown input: {}", first_token))
            },
        };
        match result {
            Err(error_message) => {
                println!("{}", error_message);
                last_input_failed = true;
            },
            Ok(finished) => {
                if finished {
                    break;
                }
            },
        }
    }
}

fn input_free(mut tokens: SplitWhitespace, game: &mut Game) -> Result<bool, String> {
    let color = parse_next_token_as(&mut tokens, parse_color, "color")?;
    let system = parse_next_token_as(&mut tokens, parse_system, "system ID")?;
    let result = game.free_move(system, color);
    if let Err(error) = result {
        return Err(format!("Failed to pick a free action: {:?}", error));
    }
    Ok(false)
}

fn input_yellow(p0: SplitWhitespace, p1: &mut Game) -> Result<bool, String> {
    unimplemented!()
}

fn input_blue(p0: SplitWhitespace, p1: &mut Game) -> Result<bool, String> {
    unimplemented!()
}

fn input_green(p0: SplitWhitespace, p1: &mut Game) -> Result<bool, String> {
    unimplemented!()
}

fn input_red(p0: SplitWhitespace, p1: &mut Game) -> Result<bool, String> {
    unimplemented!()
}

fn input_catastrophe(p0: SplitWhitespace, p1: &mut Game) -> Result<bool, String> {
    unimplemented!()
}

fn input_sacrifice(p0: SplitWhitespace, p1: &mut Game) -> Result<bool, String> {
    unimplemented!()
}

fn input_setup(mut tokens: SplitWhitespace, game: &mut Game) -> Result<bool, String> {
    let star1 = parse_next_token_as(&mut tokens, parse_piece, "star 1")?;
    let star2 = parse_next_token_as(&mut tokens, parse_piece, "star 2")?;
    let ship = parse_next_token_as(&mut tokens, parse_piece, "starting ship")?;
    let result = game.setup(&SetupMove { ship, stars: [star1, star2] });
    match result {
        Err(error) => Err(format!("Setup attempt failed: {:?}", error)),
        Ok(()) => Ok(false),
    }
}

fn input_end(game: &mut Game) -> Result<bool, String> {
    let result = game.end_turn();
    match result {
        Err(error) => Err(format!("Failed to end turn: {:?}", error)),
        Ok(()) => Ok(false),
    }
}

fn parse_next_token_as<T>(tokens: &mut SplitWhitespace, parse: fn(&str) -> Result<T, String>, description: &str) -> Result<T, String> {
    let piece_input = tokens.next();
    match piece_input {
        None => Err(format!("Malformed input, {} not specified", description)),
        Some(piece_input) => parse(piece_input),
    }
}

fn parse_piece(string: &str) -> Result<Piece, String> {
    if string.len() != 2 {
        return Err(format!("Unrecognized as a piece: {}", string));
    }
    let size_char = string.get(0..1).unwrap();
    let color_char = string.get(1..2).unwrap();
    let size = parse_size(size_char)?;
    let color = parse_color(color_char)?;
    Ok(Piece { size, color })
}

fn parse_size(string: &str) -> Result<Size, String> {
     match string {
        "s" => Ok(SMALL),
        "m" => Ok(MEDIUM),
        "l" => Ok(LARGE),
        _ => Err(format!("Not recognized as a size: {}", string)),
    }
}

fn parse_color(string: &str) -> Result<Color, String> {
    match string {
        "r" => Ok(RED),
        "g" => Ok(GREEN),
        "b" => Ok(BLUE),
        "y" => Ok(YELLOW),
        _ => Err(format!("Not recognized as a color: {}", string)),
    }
}

fn parse_system(string: &str) -> Result<SystemIndex, String> {
    let parse_result = string.parse::<u8>();
    match parse_result {
        Err(error) => Err(format!("System ID {} is not a number: {:?}", string, error)),
        Ok(system) => Ok(system),
    }
}