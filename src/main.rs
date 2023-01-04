use macroquad::prelude::*;

mod types;
mod parse;
mod game;
use crate::game::{GameState, game_loop};

#[macroquad::main("Chess")]
async fn main() {

    let mut gs: GameState = GameState::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").await;

    loop {
        game_loop(&mut gs);

        next_frame().await
    }
}
